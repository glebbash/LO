#![allow(dead_code)] // TODO: remove

use crate::{ast::*, common::*, registry::*, wasm::*};

pub struct CodeBlockContext {
    pub exprs: &'static [CodeExpr],
    pub node_id_offset: usize,
    pub symbol_id_offset: usize,
}

pub struct TypeChecker {
    pub reporter: UBRef<Reporter>,
    pub registry: UBRef<Registry>,

    pub type_lookup: BTreeMap<Type, TypeId>,
}

impl TypeChecker {
    pub fn new(registry: &mut Registry) -> Self {
        Self {
            registry: UBRef::new(&mut *registry),
            reporter: UBRef::new(&mut *registry.reporter),

            type_lookup: Default::default(),
        }
    }

    pub fn check_all(&mut self) {
        for module in self.registry.modules.relax_mut() {
            self.pass_collect_own_symbols(module);
        }

        for module in self.registry.modules.relax_mut() {
            self.pass_collect_all_symbols(module);
        }

        for module in self.registry.modules.relax() {
            self.pass_build_type_def_values(module);
        }

        for module in self.registry.modules.relax_mut() {
            self.pass_type_check_top_level_exprs(module);
        }
    }

    fn pass_collect_own_symbols(&mut self, module: &mut Module) {
        for expr in &*module.parser.ast {
            match expr {
                TopLevelExpr::Intrinsic(_) => {} // skip, not interested

                TopLevelExpr::Type(type_def) => {
                    if let TypeDefValue::Alias(_) = &type_def.value {
                        let _ = self.registry.be_mut().define_symbol(
                            &mut module.ctx,
                            type_def.name.repr,
                            SymbolType::TypeAlias,
                            self.registry.type_aliases.len(),
                            type_def.name.loc,
                        );

                        self.registry.type_aliases.push(Type::Never); // placeholder
                        continue;
                    }

                    if let TypeDefValue::Struct { .. } = &type_def.value {
                        let _ = self.registry.be_mut().define_symbol(
                            &mut module.ctx,
                            type_def.name.repr,
                            SymbolType::Struct,
                            self.registry.structs.len(),
                            type_def.name.loc,
                        );

                        self.registry.structs.push(StructDef {
                            struct_name: type_def.name.repr,
                            fields: Vec::new(),
                            fully_defined: false,
                        });
                        continue;
                    }

                    let TypeDefValue::Enum {
                        variant_type: variant_type_in_decl,
                        variants,
                    } = &type_def.value
                    else {
                        unreachable!()
                    };

                    let Ok(_) = self.registry.be_mut().define_symbol(
                        &mut module.ctx,
                        type_def.name.repr,
                        SymbolType::Enum,
                        self.registry.enums.len(),
                        type_def.name.loc,
                    ) else {
                        continue;
                    };

                    let mut variant_type = Type::Void;
                    if let Some(type_expr) = &variant_type_in_decl {
                        match self.registry.build_type(&module.ctx, type_expr) {
                            Ok(t) => variant_type = t,
                            Err(err) => {
                                variant_type = Type::Never;
                                self.reporter.error(&err);
                            }
                        }
                    }

                    self.registry.enums.push(EnumDef {
                        enum_name: type_def.name.repr,
                        variant_type: variant_type.clone(),
                        variants: Vec::new(), // placeholder
                    });

                    for (variant, variant_index) in variants.iter().zip(0..) {
                        let constructor_name = self.alloc_str(format!(
                            "{}::{}",
                            type_def.name.repr, variant.variant_name.repr
                        ));

                        let enum_index = self.registry.enums.len() - 1;

                        if variant_type != Type::Void {
                            let _ = self.registry.be_mut().define_symbol(
                                &mut module.ctx,
                                constructor_name,
                                SymbolType::EnumConstructor,
                                self.registry.enum_ctors.len(),
                                variant.variant_name.loc,
                            );

                            self.registry.enum_ctors.push(EnumConstructor {
                                enum_index,
                                variant_index,
                            });
                        } else {
                            let Ok(_) = self.registry.be_mut().define_symbol(
                                &mut module.ctx,
                                constructor_name,
                                SymbolType::Const,
                                self.registry.constants.len(),
                                variant.variant_name.loc,
                            ) else {
                                continue;
                            };

                            let mut instrs = Vec::new();
                            instrs.push(WasmInstr::I32Const {
                                value: variant_index as i32,
                            });

                            self.registry.constants.push(ConstDef {
                                const_name: constructor_name,
                                code_unit: CodeUnit {
                                    type_: Type::EnumInstance { enum_index },
                                    instrs,
                                },
                                loc: variant.variant_name.loc,
                            })
                        }
                    }
                }
                TopLevelExpr::Fn(fn_def) => {
                    if fn_def.is_inline {
                        let _ = self.registry.be_mut().define_symbol(
                            &mut module.ctx,
                            fn_def.decl.fn_name.repr,
                            SymbolType::InlineFn,
                            self.registry.inline_fns.len(),
                            fn_def.decl.fn_name.loc,
                        );

                        self.registry.inline_fns.push(fn_def.relax());
                        continue;
                    }

                    let _ = self.registry.be_mut().define_symbol(
                        &mut module.ctx,
                        fn_def.decl.fn_name.repr,
                        SymbolType::Function,
                        self.registry.functions.len(),
                        fn_def.decl.fn_name.loc,
                    );

                    match &fn_def.value {
                        FnExprValue::Body(body) => {
                            let mut exported_as = Vec::new();
                            if fn_def.exported {
                                exported_as.push(String::from(fn_def.decl.fn_name.repr));
                            }

                            self.registry.be_mut().functions.push(FnInfo {
                                fn_name: fn_def.decl.fn_name.repr,
                                fn_type: FnType {
                                    inputs: Vec::new(),
                                    output: Type::Void,
                                },
                                fn_params: Vec::new(),
                                fn_source: FnSource::Guest {
                                    module_index: module.index,
                                    lo_fn_index: self.registry.functions.len(),
                                    body: body.relax(),
                                },
                                exported_as,
                                wasm_fn_index: u32::MAX, // placeholder
                                definition_loc: fn_def.decl.fn_name.loc,
                            });
                        }
                        FnExprValue::ImportFrom(imported_from) => {
                            let module_name = imported_from.get_value(module.source);
                            let method_name = fn_def.decl.fn_name.parts.last().unwrap();
                            let external_fn_name = method_name.read_span(module.source);

                            self.registry.functions.push(FnInfo {
                                fn_name: fn_def.decl.fn_name.repr,
                                fn_type: FnType {
                                    inputs: Vec::new(),
                                    output: Type::Void,
                                },
                                fn_params: Vec::new(),
                                fn_source: FnSource::Host {
                                    module_name: module_name.clone(),
                                    external_fn_name,
                                },
                                exported_as: Vec::new(),
                                wasm_fn_index: u32::MAX, // not known at this point
                                definition_loc: fn_def.decl.fn_name.loc,
                            });
                        }
                    }
                }
                TopLevelExpr::Let(let_expr) if let_expr.is_inline => {
                    let _ = self.registry.be_mut().define_symbol(
                        &mut module.ctx,
                        let_expr.name.repr,
                        SymbolType::Const,
                        self.registry.constants.len(),
                        let_expr.name.loc,
                    );

                    self.registry.constants.push(ConstDef {
                        const_name: let_expr.name.repr,
                        code_unit: CodeUnit {
                            type_: Type::Never, // placeholder
                            instrs: Vec::new(), // placeholder
                        },
                        loc: let_expr.name.loc,
                    });
                }
                TopLevelExpr::Let(let_expr) => {
                    let _ = self.registry.be_mut().define_symbol(
                        &mut module.ctx,
                        let_expr.name.repr,
                        SymbolType::Global,
                        self.registry.globals.len(),
                        let_expr.name.loc,
                    );

                    self.registry.globals.push(GlobalDef {
                        module_ctx: module.ctx.relax(),
                        def_expr: let_expr.relax(),
                        global_type: Type::Never, // placeholder
                        global_index: 0,          // placeholder
                    });
                }
            }
        }
    }

    fn pass_collect_all_symbols(&mut self, module: &mut Module) {
        // scope for module's imports
        self.registry
            .init_scope_from_parent_and_push(&mut module.scope_stack, ScopeType::Global);

        self.inline_includes(module.relax_mut(), module, &mut String::from(""), true);
    }

    fn inline_includes(
        &mut self,
        includer: &mut Module,
        includee: &Module,
        prefix: &mut String,
        force_go_deeper: bool,
    ) {
        if includer.index != includee.index {
            let target_scope = includer.scope_stack.last_mut().unwrap();

            for symbol in &includee.scope_stack[1].symbols {
                let mut included_symbol = symbol.clone();
                included_symbol.name = self.alloc_str(format!("{}{}", prefix, symbol.name));
                target_scope.symbols.push(included_symbol)
            }
        }

        let original_prefix_len = prefix.len();
        for include in &includee.includes {
            if !(include.with_extern || force_go_deeper) {
                continue;
            }

            if let Some(alias) = &include.alias {
                prefix.push_str(&alias);
                prefix.push_str("::");
            }

            let included_module = self.registry.modules[include.module_index].relax();
            self.inline_includes(includer, included_module, prefix, false);
            prefix.truncate(original_prefix_len);
        }
    }

    fn pass_build_type_def_values(&mut self, module: &Module) {
        'exprs: for expr in &*module.parser.ast {
            match expr {
                TopLevelExpr::Type(type_def) => {
                    if let TypeDefValue::Alias(type_expr) = &type_def.value {
                        let type_value = self.registry.build_type(&module.ctx, &type_expr);
                        let type_value = catch!(type_value, err, {
                            self.reporter.error(&err);
                            continue;
                        });

                        let symbol = self
                            .registry
                            .current_scope(&module.ctx)
                            .get_symbol(&type_def.name.repr)
                            .unwrap()
                            .relax();
                        let SymbolType::TypeAlias = symbol.type_ else {
                            continue;
                        };
                        self.registry.type_aliases[symbol.col_index] = type_value;
                        continue;
                    }

                    if let TypeDefValue::Struct { fields } = &type_def.value {
                        let mut struct_fields = Vec::<StructField>::new();
                        let mut struct_primitives_count = 0;
                        let mut struct_aligment = 1;

                        'fields: for field in fields {
                            for existing_field in &struct_fields {
                                if existing_field.field_name == field.field_name.repr {
                                    self.reporter.error(&Error {
                                        message: format!(
                                            "Cannot redefine struct field '{}', already defined at {}",
                                            field.field_name.repr,
                                            existing_field.loc.to_string(&self.reporter.fm),
                                        ),
                                        loc: field.field_name.loc,
                                    });
                                    continue 'fields;
                                }
                            }

                            let field_index = struct_primitives_count;
                            let field_type_res = self.registry.build_type_check_ref(
                                &module.ctx,
                                &field.field_type,
                                false,
                                field.field_type.loc(),
                            );
                            let field_type = catch!(field_type_res, err, {
                                self.reporter.error(&err);
                                continue 'exprs;
                            });
                            let mut field_layout = TypeLayout::new();
                            self.registry
                                .get_type_layout(&field_type, &mut field_layout);

                            struct_aligment = u32::max(struct_aligment, field_layout.alignment);
                            struct_primitives_count += field_layout.primities_count;

                            struct_fields.push(StructField {
                                field_name: field.field_name.repr,
                                field_type: field_type.clone(),
                                field_layout,
                                field_index,
                                byte_offset: 0, // will be set during field alignment
                                loc: field.field_name.loc,
                            });
                        }

                        // align fields
                        let mut byte_offset = 0;
                        for field in &mut struct_fields {
                            byte_offset = align(byte_offset, field.field_layout.alignment);

                            field.byte_offset = byte_offset;

                            byte_offset += field.field_layout.byte_size;
                        }

                        let symbol = self
                            .registry
                            .current_scope(&module.ctx)
                            .get_symbol(&type_def.name.repr)
                            .unwrap()
                            .relax();
                        let SymbolType::Struct = symbol.type_ else {
                            continue;
                        };
                        let struct_ = &mut self.registry.structs[symbol.col_index];

                        struct_.fields.append(&mut struct_fields);
                        struct_.fully_defined = true;
                        continue;
                    }

                    let TypeDefValue::Enum {
                        variant_type: _,
                        variants,
                    } = &type_def.value
                    else {
                        unreachable!()
                    };

                    let symbol = self
                        .registry
                        .current_scope(&module.ctx)
                        .get_symbol(&type_def.name.repr)
                        .unwrap()
                        .relax();
                    let SymbolType::Enum = symbol.type_ else {
                        continue;
                    };
                    let enum_ = self.registry.enums[symbol.col_index].relax_mut();

                    'variants: for variant in variants.iter() {
                        for existing_variant in &enum_.variants {
                            if existing_variant.variant_name == variant.variant_name.repr {
                                self.reporter.error(&Error {
                                    message: format!(
                                        "Cannot redefine enum variant '{}', already defined at {}",
                                        variant.variant_name.repr,
                                        existing_variant.loc.to_string(&self.reporter.fm),
                                    ),
                                    loc: variant.variant_name.loc,
                                });
                                continue 'variants;
                            }
                        }

                        let mut variant_type = Type::Void;
                        if let Some(variant_type_expr) = &variant.variant_type {
                            match self.registry.build_type(&module.ctx, variant_type_expr) {
                                Ok(t) => variant_type = t,
                                Err(err) => {
                                    self.reporter.error(&err);
                                    variant_type = Type::Never
                                }
                            }
                        }

                        if !self
                            .registry
                            .is_type_compatible(&enum_.variant_type, &variant_type)
                        {
                            self.reporter.error(&Error {
                                message: format!(
                                    "Enum variant is not compatible with {}",
                                    TypeFmt(&*self.registry, &enum_.variant_type)
                                ),
                                loc: variant.variant_name.loc,
                            });
                        }

                        enum_.variants.push(EnumVariant {
                            variant_name: variant.variant_name.repr,
                            variant_type,
                            loc: variant.variant_name.loc,
                        });
                    }
                }
                _ => {} // skip, not interested
            }
        }
    }

    fn pass_type_check_top_level_exprs(&mut self, module: &mut Module) {
        for expr in &*module.parser.ast {
            match expr {
                TopLevelExpr::Type(_) => {} // skip, processed in previous passes

                TopLevelExpr::Fn(fn_def) => {
                    if fn_def.is_inline {
                        // skip, processed in pass_collect_all_symbols
                        continue;
                    }

                    let symbol = self
                        .registry
                        .current_scope(&module.ctx)
                        .get_symbol(&fn_def.decl.fn_name.repr)
                        .unwrap()
                        .relax();
                    let SymbolType::Function = symbol.type_ else {
                        continue;
                    };
                    let fn_info = self.registry.functions[symbol.col_index].relax_mut();

                    let self_type = self.registry.get_fn_self_type(
                        &module.ctx,
                        &fn_def.decl.fn_name,
                        &fn_def.decl.params,
                    );

                    if let Some(return_type) = &fn_def.decl.return_type {
                        fn_info.fn_type.output =
                            catch!(self.registry.build_type(&module.ctx, return_type), err, {
                                self.reporter.error(&err);
                                continue;
                            })
                    }

                    for fn_param in &fn_def.decl.params {
                        let param_type = self.registry.get_fn_param_type(
                            &module.ctx,
                            fn_param,
                            &self_type,
                            false,
                        );
                        let param_type = catch!(param_type, err, {
                            self.reporter.error(&err);
                            continue;
                        });
                        fn_info.fn_type.inputs.push(param_type.clone());

                        fn_info.fn_params.push(FnParameter {
                            param_name: fn_param.param_name.repr,
                            param_type: param_type.clone(),
                            loc: fn_param.param_name.loc,
                        });
                    }
                }
                TopLevelExpr::Let(let_expr) => {
                    let symbol = self
                        .registry
                        .current_scope(&module.ctx)
                        .get_symbol(&let_expr.name.repr)
                        .unwrap()
                        .relax();

                    if let_expr.is_inline {
                        let const_ = self.registry.constants[symbol.col_index].relax_mut();

                        let const_type =
                            match self.registry.get_expr_type(&module.ctx, &let_expr.value) {
                                Ok(x) => x,
                                Err(err) => {
                                    self.reporter.error(&err);
                                    continue;
                                }
                            };
                        const_.code_unit.type_ = const_type;
                        continue;
                    }

                    let SymbolType::Global = symbol.type_ else {
                        continue;
                    };
                    let global = self.registry.globals[symbol.col_index].relax_mut();

                    // TODO: check for valid const expression

                    let value_type = self.registry.get_expr_type(&module.ctx, &let_expr.value);
                    let value_type = catch!(value_type, err, {
                        self.reporter.error(&err);
                        continue;
                    });
                    global.global_type = value_type;

                    if self.reporter.in_inspection_mode {
                        let global_name = &let_expr.name.repr;

                        self.reporter.print_inspection(&InspectInfo {
                            message: format!(
                                "global {global_name}: {}",
                                TypeFmt(&*self.registry, &global.global_type)
                            ),
                            loc: let_expr.name.loc,
                            linked_loc: None,
                        });
                    }
                }
                TopLevelExpr::Intrinsic(intrinsic) => {
                    if intrinsic.fn_name.repr == "include" {
                        // skip, was processed in `Compiler.include`
                        continue;
                    }

                    if intrinsic.fn_name.repr == "use_memory" {
                        // skip, will be processed in `Compiler.codegen`
                        continue;
                    }

                    if intrinsic.fn_name.repr == "export_existing" {
                        let mut from_root = false;
                        let mut in_name = None;
                        let mut out_name = None;

                        if intrinsic.type_args.len() != 0 {
                            self.reporter.error(&bad_signature(&intrinsic.fn_name));
                        }

                        for arg in &intrinsic.args.items {
                            if let CodeExpr::Ident(ident) = arg
                                && ident.repr == "from_root"
                            {
                                from_root = true;
                                continue;
                            }

                            if let CodeExpr::Ident(ident) = arg {
                                in_name = Some(ident.relax());
                                continue;
                            }

                            let CodeExpr::Assign(AssignExpr { lhs, rhs, .. }) = arg else {
                                self.reporter.error(&bad_signature(&intrinsic.fn_name));
                                continue;
                            };

                            let CodeExpr::Ident(key) = &**lhs else {
                                self.reporter.error(&bad_signature(&intrinsic.fn_name));
                                continue;
                            };

                            if key.repr == "out" {
                                let CodeExpr::StringLiteral(value) = &**rhs else {
                                    self.reporter.error(&bad_signature(&intrinsic.fn_name));
                                    continue;
                                };

                                out_name = Some(value.relax());
                                continue;
                            }

                            self.reporter.error(&bad_signature(&intrinsic.fn_name));
                            continue;
                        }

                        let Some(in_name) = in_name else {
                            self.reporter.error(&bad_signature(&intrinsic.fn_name));
                            continue;
                        };

                        let mut target_module = module as &Module;
                        if from_root {
                            target_module = &self.registry.modules.last().unwrap();
                        }

                        let Ok(fn_info) = self.registry.get_fn_info_for_call(
                            &target_module.ctx,
                            &in_name.repr,
                            &in_name.loc,
                        ) else {
                            // don't report any errors if function to be exported
                            //   could be in another module but only one module is inspected
                            if from_root && self.reporter.in_inspection_mode {
                                continue;
                            }

                            self.reporter.error(&Error {
                                message: format!("Can't export unknown symbol {}", in_name.repr),
                                loc: in_name.loc,
                            });
                            continue;
                        };

                        if self.reporter.in_inspection_mode {
                            self.reporter.print_inspection(&InspectInfo {
                                message: String::from(in_name.repr),
                                loc: in_name.loc,
                                linked_loc: Some(fn_info.definition_loc.clone()),
                            });
                        }

                        let exported_as = if let Some(out_name) = out_name {
                            out_name.value.clone()
                        } else {
                            String::from(in_name.repr)
                        };

                        fn_info.be_mut().exported_as.push(exported_as);

                        continue;

                        fn bad_signature(fn_name: &IdentExpr) -> Error {
                            Error {
                                message: format!(
                                    "Invalid call, expected signature: @{}(<ident>, [out = <str-literal>, from_root])",
                                    fn_name.repr
                                ),
                                loc: fn_name.loc,
                            }
                        }
                    }

                    self.reporter.error(&Error {
                        message: format!("Unknown intrinsic: {}", intrinsic.fn_name.repr),
                        loc: intrinsic.loc,
                    });
                }
            }
        }
    }

    pub fn build_type_id(&mut self, type_: Type) -> TypeId {
        if let Some(&id) = self.type_lookup.get(&type_) {
            return id;
        }

        let id = self.registry.types.len();
        self.type_lookup.insert(type_.clone(), id);
        self.registry.types.push(type_);
        id
    }

    pub fn alloc_str(&mut self, value: String) -> &'static str {
        let str_ref = value.as_str().relax();
        self.registry.allocated_strings.push(value);
        str_ref
    }
}
