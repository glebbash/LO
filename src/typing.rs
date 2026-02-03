#![allow(dead_code)] // TODO: remove

use core::usize;

use crate::{ast::*, common::*, lexer::*, registry::*};

pub type ExprInfoId = usize;
pub const EXPR_INFO_ID_EMPTY: ExprInfoId = usize::MAX;

type TyScopeId = usize;

pub struct TyContext {
    pub module_id: usize,
    pub scope_id: TyScopeId,
    pub expr_id_offset: usize,
}

#[derive(Clone)]
pub struct TyScope {
    pub parent_id: Option<TyScopeId>,
    pub id: usize,
    pub kind: TyScopeKind,
    pub symbols: Vec<TySymbol>,
}

#[derive(Clone, PartialEq)]
pub enum TyScopeKind {
    Global,
    Function,
    Block,
    Loop,
    ForLoop,
    InlineFn,
}

#[derive(Clone)]
pub struct TySymbol {
    pub scope_id: usize,
    pub name: &'static str,
    pub kind: SymbolKind,
    pub col_index: usize,
    pub loc: Loc,
}

pub struct ModuleInfo {
    module_id: usize,
    ctx: TyContext,
}

#[derive(Clone)]
pub struct TyLocal {
    pub type_id: TypeId,
    pub definition_loc: Loc,
}

#[derive(Default)]
pub struct Typer {
    pub reporter: UBRef<Reporter>,
    pub registry: UBRef<Registry>,

    pub scopes: UBCell<Vec<TyScope>>,
    pub module_info: UBCell<Vec<ModuleInfo>>,
    pub type_lookup: UBCell<BTreeMap<Type, TypeId>>,
    pub type_aliases: UBCell<Vec<TypeId>>, // indexed by `col_index` when `kind = TypeAlias`
    pub locals: Vec<TyLocal>,              // indexed by `col_index` when `kind = Local`
    pub globals: Vec<GlobalDef>,           // indexed by `col_index` when `kind = Global`
    pub constants: Vec<ConstDef>,          // indexed by `col_index` when `kind = Const`
    pub functions: Vec<FnInfo>,            // indexed by `col_index` when `kind = Function`
    pub inline_fns: Vec<&'static FnExpr>,  // indexed by `col_index` when `kind = InlineFn`
    pub structs: Vec<StructDef>,           // indexed by `col_index` when `kind = Struct`
    pub enums: Vec<EnumDef>,               // indexed by `col_index` when `kind = Enum`
    pub enum_ctors: Vec<EnumConstructor>,  // indexed by `col_index` when `kind = EnumConstructor`

    pub global_scope: Scope,
}

impl Typer {
    pub fn new(registry: &mut Registry) -> Self {
        let mut it = Self::default();
        it.registry = UBRef::new(&mut *registry);
        it.reporter = UBRef::new(&mut *registry.reporter);
        it.global_scope.kind = ScopeKind::Global;
        it.init();
        it
    }

    pub fn init(&mut self) {
        let global_scope_id = self.new_scope(TyScopeKind::Global, None);

        self.module_info.reserve(self.registry.modules.len());
        for module in &self.registry.modules {
            self.module_info.be_mut().push(ModuleInfo {
                module_id: module.id,
                ctx: TyContext {
                    module_id: module.id,
                    scope_id: self.new_scope(TyScopeKind::Global, Some(global_scope_id)),
                    expr_id_offset: 0,
                },
            });
        }

        self.extend_expr_info_storage(self.registry.expr_count);

        self.add_builtin_type(Type::Never);
        self.add_builtin_type(Type::Void);
        self.add_builtin_type(Type::Bool);
        self.add_builtin_type(Type::U8);
        self.add_builtin_type(Type::I8);
        self.add_builtin_type(Type::U16);
        self.add_builtin_type(Type::I16);
        self.add_builtin_type(Type::U32);
        self.add_builtin_type(Type::I32);
        self.add_builtin_type(Type::F32);
        self.add_builtin_type(Type::U64);
        self.add_builtin_type(Type::I64);
        self.add_builtin_type(Type::F64);
    }

    fn extend_expr_info_storage(&self, expr_count: usize) {
        self.registry.be_mut().expr_info.resize(
            self.registry.expr_info.len() + expr_count,
            EXPR_INFO_ID_EMPTY,
        );
    }

    #[inline]
    fn add_builtin_type(&mut self, type_: Type) {
        self.build_type_id(&type_);

        let col_index = self.registry.type_aliases.len();
        self.global_scope.be_mut().symbols.push(Symbol {
            scope_id: self.global_scope.id,
            name: type_.to_str().unwrap(),
            kind: SymbolKind::TypeAlias,
            col_index,
            loc: Loc::internal(),
        });

        self.scopes.be_mut()[0].symbols.push(TySymbol {
            scope_id: 0,
            name: type_.to_str().unwrap(),
            kind: SymbolKind::TypeAlias,
            col_index,
            loc: Loc::internal(),
        });
        self.registry.type_aliases.push(type_);
    }

    pub fn type_all(&mut self) {
        for module in self.module_info.be_mut().relax_mut() {
            self.pass_collect_own_symbols(&module.ctx);
        }

        for module in self.module_info.be_mut().relax_mut() {
            self.pass_collect_included_symbols(&module.ctx);
        }

        for module in self.registry.modules.relax() {
            self.pass_build_type_def_values(module);
        }

        for module in self.registry.modules.relax_mut() {
            self.pass_process_top_level_exprs(module);
        }

        for module in self.module_info.be_mut().relax_mut() {
            self.pass_type_fns(&module.ctx);
        }
    }

    fn pass_collect_own_symbols(&mut self, ctx: &TyContext) {
        let module = self.registry.modules[ctx.module_id].relax_mut();

        module.scope_stack.push(self.global_scope.clone());

        // scope for module's own symbols
        self.registry.enter_scope(&module.ctx, ScopeKind::Global);

        for expr in &*module.parser.ast {
            match expr {
                TopLevelExpr::Intrinsic(_) => {} // skip, not interested

                TopLevelExpr::Type(type_def) => {
                    if let TypeDefValue::Alias(_) = &type_def.value {
                        let _ = self.define_symbol_compat(
                            ctx,
                            type_def.name.repr,
                            SymbolKind::TypeAlias,
                            type_def.name.loc,
                        );

                        self.registry.type_aliases.push(Type::Never); // placeholder
                        continue;
                    }

                    if let TypeDefValue::Struct { .. } = &type_def.value {
                        let _ = self.define_symbol_compat(
                            ctx,
                            type_def.name.repr,
                            SymbolKind::Struct,
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

                    let Ok(_) = self.define_symbol_compat(
                        ctx,
                        type_def.name.repr,
                        SymbolKind::Enum,
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

                        let _ = self.define_symbol_compat(
                            ctx,
                            constructor_name,
                            SymbolKind::EnumConstructor,
                            variant.variant_name.loc,
                        );

                        self.registry.enum_ctors.push(EnumConstructor {
                            enum_index,
                            variant_index,
                            loc: variant.variant_name.loc,
                        });
                    }
                }
                TopLevelExpr::Fn(fn_def) => {
                    if fn_def.is_inline {
                        let _ = self.define_symbol_compat(
                            ctx,
                            fn_def.decl.fn_name.repr,
                            SymbolKind::InlineFn,
                            fn_def.decl.fn_name.loc,
                        );

                        self.registry.inline_fns.push(fn_def.relax());
                        continue;
                    }

                    let _ = self.define_symbol_compat(
                        ctx,
                        fn_def.decl.fn_name.repr,
                        SymbolKind::Function,
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
                                    module_index: module.id,
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
                    let _ = self.define_symbol_compat(
                        ctx,
                        let_expr.name.repr,
                        SymbolKind::Const,
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
                    let _ = self.define_symbol_compat(
                        ctx,
                        let_expr.name.repr,
                        SymbolKind::Global,
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

    fn pass_collect_included_symbols(&mut self, ctx: &TyContext) {
        self.inline_includes(ctx, ctx, &mut String::from(""), true);

        // TODO: remove when migrated
        {
            let module = self.registry.modules[ctx.module_id].relax_mut();

            // scope for module's imports
            self.registry
                .init_scope_from_parent_and_push(&mut module.scope_stack, ScopeKind::Global);

            self.inline_includes_old(module.relax_mut(), module, &mut String::from(""), true);
        }
    }

    // TODO: old version was using an interlayer scope to store includes in
    //   but this version is using module's scope to store inclues in
    //   check if this is fine and fix if it isn't
    fn inline_includes(
        &mut self,
        includer: &TyContext,
        includee: &TyContext,
        prefix: &mut String,
        force_go_deeper: bool,
    ) {
        if includer.module_id != includee.module_id {
            let target_scope = self.scopes[includer.scope_id].relax_mut();

            for symbol in self.scopes[includee.scope_id].symbols.relax() {
                let mut included_symbol = symbol.clone();
                included_symbol.name = self.alloc_str(format!("{}{}", prefix, symbol.name));
                target_scope.symbols.push(included_symbol)
            }
        }

        let original_prefix_len = prefix.len();
        for include in self.registry.modules[includee.module_id].includes.relax() {
            if !(include.with_extern || force_go_deeper) {
                continue;
            }

            if let Some(alias) = &include.alias {
                prefix.push_str(&alias);
                prefix.push_str("::");
            }

            let sub_includee = self.module_info[include.module_index].relax();
            self.inline_includes(includer, &sub_includee.ctx, prefix, false);
            prefix.truncate(original_prefix_len);
        }
    }

    fn inline_includes_old(
        &mut self,
        includer: &mut Module,
        includee: &Module,
        prefix: &mut String,
        force_go_deeper: bool,
    ) {
        if includer.id != includee.id {
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
            self.inline_includes_old(includer, included_module, prefix, false);
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
                        let SymbolKind::TypeAlias = symbol.kind else {
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
                                            "Cannot redefine struct field '{}', previously defined at {}",
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
                        let SymbolKind::Struct = symbol.kind else {
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
                    let SymbolKind::Enum = symbol.kind else {
                        continue;
                    };
                    let enum_ = self.registry.enums[symbol.col_index].relax_mut();

                    'variants: for variant in variants.iter() {
                        for existing_variant in &enum_.variants {
                            if existing_variant.variant_name == variant.variant_name.repr {
                                self.reporter.error(&Error {
                                    message: format!(
                                        "Cannot redefine enum variant '{}', previously defined at {}",
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

    fn pass_process_top_level_exprs(&mut self, module: &mut Module) {
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
                    let SymbolKind::Function = symbol.kind else {
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

                    let SymbolKind::Global = symbol.kind else {
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

                    if intrinsic.fn_name.repr.starts_with("inspect_") {
                        // skip, processed elsewhere
                        continue;
                    }

                    self.reporter.error(&Error {
                        message: format!("Unknown intrinsic: {}", intrinsic.fn_name.repr),
                        loc: intrinsic.loc,
                    });
                }
            }
        }
    }

    fn pass_type_fns(&mut self, ctx: &TyContext) {
        for expr in self.registry.modules[ctx.module_id]
            .parser
            .ast
            .be_mut()
            .relax_mut()
        {
            let TopLevelExpr::Fn(FnExpr {
                is_inline: false,
                value: FnExprValue::Body(body),
                ..
            }) = expr
            else {
                continue;
            };

            let fn_ctx = self.child_ctx(&ctx, TyScopeKind::Function);

            self.type_code_block(&fn_ctx, body);
        }
    }

    fn type_code_block(&mut self, ctx: &TyContext, block: &CodeBlock) {
        let block_ctx = self.child_ctx(ctx, TyScopeKind::Block);

        for expr in &block.exprs {
            self.type_code_expr(&block_ctx, expr);
        }
    }

    fn type_code_expr(&mut self, ctx: &TyContext, expr: &CodeExpr) {
        match expr {
            CodeExpr::BoolLiteral(bool_literal) => {
                self.store_type(ctx, bool_literal.id, &Type::Bool);
            }
            CodeExpr::CharLiteral(char_literal) => {
                self.store_type(ctx, char_literal.id, &Type::U8);
            }
            CodeExpr::NullLiteral(null_literal) => {
                self.store_type(ctx, null_literal.id, &Type::Null);
            }
            CodeExpr::IntLiteral(int_literal) => {
                let Some(_tag) = int_literal.tag else {
                    return self.store_type(ctx, int_literal.id, &Type::U32);
                };

                // TODO: enable when symbols are properly stored
                // let tag_type = catch!(self.get_type_or_err(ctx, tag, &int_literal.loc), err, {
                //     self.reporter.error(&err);
                //     return self.store_type(ctx, int_literal.id, &Type::U32);
                // });

                // catch!(self.is_64_bit_int_tag(&tag_type, &int_literal.loc), err, {
                //     self.reporter.error(&err);
                //     return self.store_type(ctx, int_literal.id, &Type::U32);
                // });
            }
            CodeExpr::StringLiteral(_str_literal) => {
                // TODO: enable when symbols are properly stored
                // let Some(symbol) = self.get_symbol(ctx.scope_id, "str") else {
                //     return self.reporter.error(&Error {
                //         message: format!("Cannot use strings with no `str` struct defined"),
                //         loc: str_literal.loc,
                //     });
                // };

                // let str_type = Type::StructInstance {
                //     struct_index: symbol.col_index,
                // };
                // self.store_type(ctx, str_literal.id, &str_type)
            }
            CodeExpr::StructLiteral(struct_literal) => {
                for field in &struct_literal.body.fields {
                    self.type_code_expr(ctx, &field.value);
                }

                // TODO: enable when symbols are properly stored
                // let Some(symbol) = self.get_symbol(ctx.scope_id, &struct_literal.struct_name.repr)
                // else {
                //     return self.reporter.error(&Error {
                //         message: format!("Unknown struct: {}", struct_literal.struct_name.repr),
                //         loc: struct_literal.loc,
                //     });
                // };

                // let struct_type = Type::StructInstance {
                //     struct_index: symbol.col_index,
                // };
                // return self.store_type(ctx, struct_literal.id, &struct_type);
            }
            CodeExpr::ArrayLiteral(_array_literal) => {
                // TODO: implement
            }
            CodeExpr::ResultLiteral(_result_literal) => {
                // TODO: implement
            }
            CodeExpr::Ident(ident) => {
                let Some(symbol) = self.get_symbol(ctx.scope_id, ident.repr) else {
                    // TODO: report error
                    return;
                };

                let SymbolKind::Local = symbol.kind else {
                    // TODO: handle other variants
                    return;
                };

                self.store_type_id(ctx, ident.id, self.locals[symbol.col_index].type_id);
            }
            CodeExpr::Let(let_expr) => {
                self.type_code_expr(ctx, &let_expr.value);

                let Some(value_type_id) = self.load_type_id(ctx, let_expr.value.id()) else {
                    self.store_type(ctx, let_expr.id, &Type::Void);
                    return;
                };

                if self.registry.types[value_type_id] == Type::Never {
                    self.store_type(ctx, let_expr.id, &Type::Never);
                }

                if let_expr.name.repr == "_" {
                    return;
                }

                let _ = self.define_symbol(
                    ctx,
                    let_expr.name.repr,
                    SymbolKind::Local,
                    let_expr.name.loc,
                );
                self.locals.push(TyLocal {
                    type_id: value_type_id,
                    definition_loc: let_expr.name.loc,
                });
            }
            CodeExpr::InfixOp(infix_op) => {
                // TODO: implement

                self.type_code_expr(ctx, &infix_op.lhs);
                self.type_code_expr(ctx, &infix_op.rhs);

                match infix_op.op_tag {
                    InfixOpTag::Equal
                    | InfixOpTag::NotEqual
                    | InfixOpTag::Less
                    | InfixOpTag::Greater
                    | InfixOpTag::LessEqual
                    | InfixOpTag::GreaterEqual
                    | InfixOpTag::And
                    | InfixOpTag::Or => {
                        self.store_type(ctx, infix_op.id, &Type::Bool);
                    }

                    InfixOpTag::Add
                    | InfixOpTag::Sub
                    | InfixOpTag::Mul
                    | InfixOpTag::Div
                    | InfixOpTag::Mod
                    | InfixOpTag::BitAnd
                    | InfixOpTag::BitOr
                    | InfixOpTag::ShiftLeft
                    | InfixOpTag::ShiftRight => {
                        if let Some(type_id) = self.load_type_id(ctx, infix_op.lhs.id()) {
                            self.store_type_id(ctx, infix_op.id, type_id);
                        }
                    }

                    InfixOpTag::AddAssign
                    | InfixOpTag::SubAssign
                    | InfixOpTag::MulAssign
                    | InfixOpTag::DivAssign
                    | InfixOpTag::ModAssign
                    | InfixOpTag::BitAndAssign
                    | InfixOpTag::BitOrAssign
                    | InfixOpTag::ShiftLeftAssign
                    | InfixOpTag::ShiftRightAssign => {
                        self.store_type(ctx, infix_op.id, &Type::Void);
                    }

                    // have their own CodeExpr variants
                    InfixOpTag::Cast
                    | InfixOpTag::Assign
                    | InfixOpTag::FieldAccess
                    | InfixOpTag::Catch
                    | InfixOpTag::ErrorPropagation
                    | InfixOpTag::Pipe => unreachable!(),
                }
            }
            CodeExpr::PrefixOp(prefix_op) => {
                // TODO: implement

                self.type_code_expr(ctx, &prefix_op.expr);
            }
            CodeExpr::Cast(cast) => {
                // TODO: implement

                self.type_code_expr(ctx, &cast.expr);
            }
            CodeExpr::Assign(assign) => {
                // TODO: implement

                self.store_type(ctx, assign.id, &Type::Void);
                self.type_code_expr(ctx, &assign.lhs);
                self.type_code_expr(ctx, &assign.rhs);
            }
            CodeExpr::FieldAccess(field_access) => {
                // TODO: implement

                self.type_code_expr(ctx, &field_access.lhs);
            }
            CodeExpr::PropagateError(prop_error) => {
                // TODO: implement

                self.type_code_expr(ctx, &prop_error.expr);
            }
            CodeExpr::FnCall(call) => {
                // TODO: implement

                for arg in &call.args.items {
                    self.type_code_expr(ctx, arg);
                }
            }
            CodeExpr::MethodCall(call) => {
                // TODO: implement

                self.type_code_expr(ctx, &call.lhs);

                for arg in &call.args.items {
                    self.type_code_expr(ctx, arg);
                }
            }
            CodeExpr::InlineFnCall(call) => {
                for arg in &call.args.items {
                    self.type_code_expr(ctx, arg);
                }

                let Some(symbol) = self.get_symbol(ctx.scope_id, call.fn_name.repr) else {
                    return self.reporter.error(&Error {
                        message: format!("Unknown inline fn: {}", call.fn_name.repr),
                        loc: call.fn_name.loc,
                    });
                };

                let inline_fn_def = self.registry.inline_fns[symbol.col_index];

                let FnExprValue::Body(body) = &inline_fn_def.value else {
                    unreachable!()
                };

                let mut inline_fn_ctx = self.child_ctx(ctx, TyScopeKind::InlineFn);
                inline_fn_ctx.expr_id_offset = self.registry.expr_info.len() - body.expr_id_start;
                self.extend_expr_info_storage(body.expr_id_end - body.expr_id_start);

                self.type_code_block(&inline_fn_ctx, body);
            }
            CodeExpr::InlineMethodCall(call) => {
                // TODO: implement

                self.type_code_expr(ctx, &call.lhs);

                for arg in &call.args.items {
                    self.type_code_expr(ctx, arg);
                }
            }
            CodeExpr::IntrinsicCall(_call) => {
                // TODO: implement
            }
            CodeExpr::Return(return_expr) => {
                self.store_type(ctx, return_expr.id, &Type::Never);

                if let Some(return_value) = &return_expr.expr {
                    self.type_code_expr(ctx, &return_value);
                }
            }
            CodeExpr::If(if_expr) => {
                // TODO: implement

                match &if_expr.cond {
                    IfCond::Expr(code_expr) => {
                        self.type_code_expr(ctx, code_expr);
                    }
                    IfCond::Match(match_header) => {
                        self.type_code_expr(ctx, &match_header.expr_to_match);
                    }
                }

                self.type_code_block(ctx, &if_expr.then_block);

                match &if_expr.else_block {
                    ElseBlock::None => {}
                    ElseBlock::Else(code_block) => {
                        self.type_code_block(ctx, code_block);
                    }
                    ElseBlock::ElseIf(code_expr) => {
                        self.type_code_expr(ctx, code_expr);
                    }
                }
            }
            CodeExpr::While(while_expr) => {
                // TODO: implement

                if let Some(cond) = &while_expr.cond {
                    self.type_code_expr(ctx, cond);
                }

                self.type_code_block(ctx, &while_expr.body);
            }
            CodeExpr::For(for_expr) => {
                // TODO: implement

                self.type_code_expr(ctx, &for_expr.start);
                self.type_code_expr(ctx, &for_expr.end);
                self.type_code_block(ctx, &for_expr.body);
            }
            CodeExpr::Break(_break_expr) => {
                // TODO: implement
            }
            CodeExpr::Continue(_continue_expr) => {
                // TODO: implement
            }
            CodeExpr::Defer(defer) => {
                // TODO: implement

                self.type_code_expr(ctx, &defer.expr);
            }
            CodeExpr::Catch(catch) => {
                // TODO: implement

                self.type_code_expr(ctx, &catch.lhs);
                self.type_code_block(ctx, &catch.catch_body);
            }
            CodeExpr::Match(match_expr) => {
                // TODO: implement

                self.type_code_expr(ctx, &match_expr.header.expr_to_match);
                self.type_code_block(ctx, &match_expr.else_branch);
            }
            CodeExpr::Paren(paren) => {
                self.type_code_expr(ctx, &paren.expr);

                if let Some(expr_type_id) = self.load_type_id(ctx, paren.expr.id()) {
                    self.store_type_id(ctx, paren.id, expr_type_id);
                }
            }
            CodeExpr::DoWith(do_with) => {
                // TODO: implement

                self.type_code_expr(ctx, &do_with.body);

                for arg in &do_with.args.items {
                    self.type_code_expr(ctx, arg);
                }
            }
            CodeExpr::Pipe(pipe) => {
                // TODO: implement

                self.type_code_expr(ctx, &pipe.lhs);
                self.type_code_expr(ctx, &pipe.rhs);
            }
            CodeExpr::Sizeof(sizeof) => {
                // TODO: implement

                self.store_type(ctx, sizeof.id, &Type::U32);
            }
        }
    }

    // TODO: check if there is a better place to print inspections from here
    fn get_type_or_err(&self, ctx: &TyContext, type_name: &str, loc: &Loc) -> Result<Type, Error> {
        let Some(symbol) = self.get_symbol(ctx.scope_id, type_name) else {
            return Err(Error {
                message: format!("Unknown type: {}", type_name),
                loc: *loc,
            });
        };

        match symbol.kind {
            SymbolKind::Struct => {
                if self.reporter.in_inspection_mode {
                    self.reporter.print_inspection(&InspectInfo {
                        message: format!("struct {type_name} {{ ... }}"),
                        loc: *loc,
                        linked_loc: Some(symbol.loc),
                    });
                }

                Ok(Type::StructInstance {
                    struct_index: symbol.col_index,
                })
            }
            SymbolKind::Enum => {
                if self.reporter.in_inspection_mode {
                    self.reporter.print_inspection(&InspectInfo {
                        message: format!("enum {type_name} {{ ... }}"),
                        loc: *loc,
                        linked_loc: Some(symbol.loc),
                    });
                }

                Ok(Type::EnumInstance {
                    enum_index: symbol.col_index,
                })
            }
            SymbolKind::TypeAlias => {
                let type_id = self.type_aliases[symbol.col_index];
                let type_ = &self.registry.types[type_id];

                // don't print inspection for built-ins
                if self.reporter.in_inspection_mode && symbol.loc.file_index != 0 {
                    self.reporter.print_inspection(&InspectInfo {
                        message: format!("type {type_name} = {}", TypeFmt(&*self.registry, &type_)),
                        loc: *loc,
                        linked_loc: Some(symbol.loc),
                    });
                }

                Ok(type_.clone())
            }
            SymbolKind::Local
            | SymbolKind::Global
            | SymbolKind::Const
            | SymbolKind::Function
            | SymbolKind::InlineFn
            | SymbolKind::EnumConstructor => Err(Error {
                message: format!("Symbol is not a type: {}", type_name),
                loc: *loc,
            }),
        }
    }

    fn is_64_bit_int_tag(&self, tag_type: &Type, loc: &Loc) -> Result<bool, Error> {
        match tag_type {
            Type::U64 | Type::I64 => Ok(true),
            Type::U8 | Type::I8 | Type::U16 | Type::I16 | Type::U32 | Type::I32 => Ok(false),
            other => Err(Error {
                message: format!("{} is not a valid int tag", TypeFmt(&*self.registry, other)),
                loc: *loc,
            }),
        }
    }

    // TODO: remove this once migrated
    fn define_symbol_compat(
        &self,
        ctx: &TyContext,
        symbol_name: &'static str,
        symbol_kind: SymbolKind,
        symbol_loc: Loc,
    ) -> Result<(), &TySymbol> {
        let _ = self.registry.be_mut().define_symbol(
            &self.registry.modules[ctx.module_id].ctx,
            symbol_name,
            symbol_kind,
            symbol_loc,
        );

        self.define_symbol(ctx, symbol_name, symbol_kind, symbol_loc)
    }

    fn define_symbol(
        &self,
        ctx: &TyContext,
        symbol_name: &'static str,
        symbol_kind: SymbolKind,
        symbol_loc: Loc,
    ) -> Result<(), &TySymbol> {
        let symbol_col_index = match &symbol_kind {
            SymbolKind::TypeAlias => self.type_aliases.len(),
            SymbolKind::Struct => self.structs.len(),
            SymbolKind::Enum => self.enums.len(),
            SymbolKind::Local => self.locals.len(),
            SymbolKind::Global => self.globals.len(),
            SymbolKind::Const => self.constants.len(),
            SymbolKind::InlineFn => self.inline_fns.len(),
            SymbolKind::Function => self.functions.len(),
            SymbolKind::EnumConstructor => self.enum_ctors.len(),
        };

        if let Some(existing_symbol) = self.get_symbol(ctx.scope_id, symbol_name)
            && existing_symbol.scope_id == ctx.scope_id
        {
            self.reporter.error(&Error {
                message: format!(
                    // TODO: drop prefix after migration
                    "(ty) Cannot redefine {}, previously defined at {}",
                    symbol_name,
                    existing_symbol.loc.to_string(&self.reporter.fm)
                ),
                loc: symbol_loc,
            });
            return Err(&existing_symbol);
        }

        self.scopes[ctx.scope_id].symbols.be_mut().push(TySymbol {
            scope_id: ctx.scope_id,
            name: symbol_name,
            kind: symbol_kind,
            col_index: symbol_col_index,
            loc: symbol_loc,
        });

        Ok(())
    }

    fn get_symbol(&self, scope_id: TyScopeId, symbol_name: &str) -> Option<&TySymbol> {
        for symbol in self.scopes[scope_id].symbols.iter().rev() {
            if symbol.name == symbol_name {
                return Some(symbol);
            }
        }

        if let Some(parent_id) = &self.scopes[scope_id].parent_id {
            return self.get_symbol(*parent_id, symbol_name);
        }

        None
    }

    fn child_ctx(&self, parent: &TyContext, scope_kind: TyScopeKind) -> TyContext {
        TyContext {
            module_id: parent.module_id,
            scope_id: self.new_scope(scope_kind, Some(parent.scope_id)),
            expr_id_offset: 0,
        }
    }

    fn new_scope(&self, kind: TyScopeKind, parent_id: Option<TyScopeId>) -> usize {
        self.scopes.be_mut().push(TyScope {
            parent_id,
            id: self.scopes.len(),
            kind,
            symbols: Vec::new(),
        });

        self.scopes.len() - 1
    }

    fn load_type_id(&self, ctx: &TyContext, expr_id: ExprId) -> Option<TypeId> {
        let info_id = self.registry.expr_info[ctx.expr_id_offset + expr_id];
        if info_id == EXPR_INFO_ID_EMPTY {
            return None;
        }

        return Some(info_id);
    }

    fn store_type(&self, ctx: &TyContext, expr_id: ExprId, type_: &Type) {
        let type_id = self.build_type_id(type_);
        self.store_type_id(ctx, expr_id, type_id)
    }

    fn store_type_id(&self, ctx: &TyContext, expr_id: ExprId, type_id: TypeId) {
        self.registry.be_mut().expr_info[ctx.expr_id_offset + expr_id] = type_id;
    }

    fn build_type_id(&self, type_: &Type) -> TypeId {
        if let Some(&id) = self.type_lookup.get(type_) {
            return id;
        }

        let id = self.registry.types.len();
        self.type_lookup.be_mut().insert(type_.clone(), id);
        self.registry.be_mut().types.push(type_.clone());
        id
    }

    fn alloc_str(&mut self, value: String) -> &'static str {
        let str_ref = value.as_str().relax();
        self.registry.allocated_strings.push(value);
        str_ref
    }
}
