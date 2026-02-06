use core::usize;

use crate::{ast::*, common::*, lexer::*, registry::*, wasm::WasmType};

pub type ExprInfo = usize;
pub const EXPR_INFO_INVALID: ExprInfo = usize::MAX;

type TyScopeId = usize;

pub struct TyContext {
    pub module_id: usize,
    pub scope_id: TyScopeId,
    pub fn_index: Option<usize>,
    pub expr_info_offset: usize,
}

#[allow(dead_code)] // TODO: remove `#[allow(dead_code)]`
#[derive(Clone)]
pub struct TyScope {
    pub parent_id: Option<TyScopeId>,
    pub id: usize,
    pub kind: ScopeKind,
    pub symbols: Vec<TySymbol>,
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
    ctx: TyContext,
}

#[derive(Clone)]
pub struct TyLocal {
    pub type_id: TypeId,
}

pub struct ICallInfo {
    pub return_type_id: TypeId,
    pub inner_expr_offset: usize,
}

// TODO: optimize types copied from registry
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
        let global_scope_id = self.new_scope(ScopeKind::Global, None);

        self.module_info.reserve(self.registry.modules.len());
        for module in &self.registry.modules {
            self.module_info.be_mut().push(ModuleInfo {
                ctx: TyContext {
                    module_id: module.id,
                    scope_id: self.new_scope(ScopeKind::Global, Some(global_scope_id)),
                    fn_index: None,
                    expr_info_offset: 0,
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
            EXPR_INFO_INVALID,
        );
    }

    #[inline]
    fn add_builtin_type(&mut self, type_: Type) {
        self.scopes.be_mut()[0].symbols.push(TySymbol {
            scope_id: 0,
            name: type_.to_str().unwrap(),
            kind: SymbolKind::TypeAlias,
            col_index: self.type_aliases.len(),
            loc: Loc::internal(),
        });

        let type_id = self.build_type_id(&type_);
        self.type_aliases.push(type_id);

        // TODO: remove after migration
        {
            self.global_scope.be_mut().symbols.push(Symbol {
                scope_id: self.global_scope.id,
                name: type_.to_str().unwrap(),
                kind: SymbolKind::TypeAlias,
                col_index: self.type_aliases.len() - 1,
                loc: Loc::internal(),
            });
            self.registry.type_aliases.push(type_.clone());
        }
    }

    pub fn type_all(&mut self) {
        for module in self.module_info.be_mut().relax_mut() {
            self.pass_collect_own_symbols(&module.ctx);
        }

        for module in self.module_info.be_mut().relax_mut() {
            self.pass_collect_included_symbols(&module.ctx);
        }

        for module in self.module_info.be_mut().relax_mut() {
            self.pass_build_type_def_values(&module.ctx);
        }

        for module in self.module_info.be_mut().relax_mut() {
            self.pass_process_top_level_exprs(&module.ctx);
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
                    match &type_def.value {
                        TypeDefValue::Struct { .. } => {
                            let _ = self.define_symbol_compat(
                                ctx,
                                type_def.name.repr,
                                SymbolKind::Struct,
                                type_def.name.loc,
                            );

                            self.structs.push(StructDef {
                                struct_name: type_def.name.repr,
                                fields: Vec::new(),
                                fully_defined: false,
                            });

                            // TODO: remove after migration
                            {
                                self.registry.structs.push(StructDef {
                                    struct_name: type_def.name.repr,
                                    fields: Vec::new(),
                                    fully_defined: false,
                                });
                            }
                        }
                        TypeDefValue::Enum {
                            variant_type: variant_type_in_decl,
                            variants,
                        } => {
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
                                        self.report_error(&err);
                                    }
                                }
                            }

                            self.enums.push(EnumDef {
                                enum_name: type_def.name.repr,
                                variant_type: variant_type.clone(),
                                variants: Vec::new(), // placeholder
                            });

                            // TODO: remove after migration
                            {
                                self.registry.enums.push(EnumDef {
                                    enum_name: type_def.name.repr,
                                    variant_type: variant_type.clone(),
                                    variants: Vec::new(), // placeholder
                                });
                            }

                            for (variant, variant_index) in variants.iter().zip(0..) {
                                let constructor_name = self.alloc_str(format!(
                                    "{}::{}",
                                    type_def.name.repr, variant.variant_name.repr
                                ));

                                let enum_index = self.enums.len() - 1;

                                let _ = self.define_symbol_compat(
                                    ctx,
                                    constructor_name,
                                    SymbolKind::EnumConstructor,
                                    variant.variant_name.loc,
                                );

                                self.enum_ctors.push(EnumConstructor {
                                    enum_index,
                                    variant_index,
                                    loc: variant.variant_name.loc,
                                });

                                // TODO: remove after migration
                                {
                                    self.registry.enum_ctors.push(EnumConstructor {
                                        enum_index,
                                        variant_index,
                                        loc: variant.variant_name.loc,
                                    });
                                }
                            }
                        }
                        TypeDefValue::Alias(_) => {
                            let _ = self.define_symbol_compat(
                                ctx,
                                type_def.name.repr,
                                SymbolKind::TypeAlias,
                                type_def.name.loc,
                            );

                            self.type_aliases.push(TYPE_ID_INVALID); // placeholder

                            // TODO: remove after migration
                            {
                                self.registry.type_aliases.push(Type::Never); // placeholder
                            }
                        }
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

                        self.inline_fns.push(fn_def.relax());

                        // TODO: remove after migration
                        {
                            self.registry.inline_fns.push(fn_def.relax());
                        }
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

                            // TODO: remove after migration
                            {
                                self.registry.be_mut().functions.push(FnInfo {
                                    fn_name: fn_def.decl.fn_name.repr,
                                    fn_type: FnType {
                                        inputs: Vec::new(),
                                        output: Type::Void,
                                    },
                                    fn_params: Vec::new(),
                                    fn_source: FnSource::Guest {
                                        module_id: module.id,
                                        lo_fn_index: self.registry.functions.len(),
                                        body: body.relax(),
                                    },
                                    exported_as: exported_as.clone(),
                                    wasm_fn_index: u32::MAX, // placeholder
                                    definition_loc: fn_def.decl.fn_name.loc,
                                });
                            }

                            self.functions.push(FnInfo {
                                fn_name: fn_def.decl.fn_name.repr,
                                fn_type: FnType {
                                    inputs: Vec::new(),
                                    output: Type::Void,
                                },
                                fn_params: Vec::new(),
                                fn_source: FnSource::Guest {
                                    module_id: module.id,
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

                            // TODO: remove after migration
                            {
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

                            self.functions.push(FnInfo {
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
                TopLevelExpr::Let(let_expr) => {
                    if let_expr.is_inline {
                        let _ = self.define_symbol_compat(
                            ctx,
                            let_expr.name.repr,
                            SymbolKind::Const,
                            let_expr.name.loc,
                        );

                        // TODO: remove after migration
                        {
                            self.registry.constants.push(ConstDef {
                                const_name: let_expr.name.repr,
                                code_unit: CodeUnit {
                                    type_: Type::Never, // placeholder
                                    instrs: Vec::new(), // placeholder
                                },
                                loc: let_expr.name.loc,
                            });
                        }

                        self.constants.push(ConstDef {
                            const_name: let_expr.name.repr,
                            code_unit: CodeUnit {
                                type_: Type::Never, // placeholder
                                instrs: Vec::new(), // placeholder
                            },
                            loc: let_expr.name.loc,
                        });
                        continue;
                    }

                    let _ = self.define_symbol_compat(
                        ctx,
                        let_expr.name.repr,
                        SymbolKind::Global,
                        let_expr.name.loc,
                    );

                    // TODO: remove after migration
                    {
                        self.registry.globals.push(GlobalDef {
                            module_ctx: module.ctx.relax(),
                            def_expr: let_expr.relax(),
                            global_type: Type::Never, // placeholder
                            global_index: 0,          // placeholder
                        });
                    }

                    self.globals.push(GlobalDef {
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

            let sub_includee = self.module_info[include.module_id].relax();
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

            let included_module = self.registry.modules[include.module_id].relax();
            self.inline_includes_old(includer, included_module, prefix, false);
            prefix.truncate(original_prefix_len);
        }
    }

    fn pass_build_type_def_values(&mut self, ctx: &TyContext) {
        let module = self.registry.modules[ctx.module_id].relax_mut();

        'exprs: for expr in &*module.parser.ast {
            match expr {
                TopLevelExpr::Type(type_def) => {
                    match &type_def.value {
                        TypeDefValue::Struct { fields } => {
                            let mut struct_fields = Vec::<StructField>::new();
                            let mut struct_primitives_count = 0;
                            let mut struct_aligment = 1;

                            'fields: for field in fields {
                                for existing_field in &struct_fields {
                                    if existing_field.field_name == field.field_name.repr {
                                        self.report_error(&Error {
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
                                    self.report_error(&err);
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
                                .get_symbol(ctx.scope_id, type_def.name.repr)
                                .unwrap()
                                .relax();

                            let SymbolKind::Struct = symbol.kind else {
                                continue;
                            };

                            // TODO: remove when migrated
                            {
                                let struct_def = &mut self.registry.structs[symbol.col_index];
                                struct_def.fields.extend(struct_fields.clone());
                                struct_def.fully_defined = true;
                            }

                            let struct_def = &mut self.structs[symbol.col_index];
                            struct_def.fields.append(&mut struct_fields);
                            struct_def.fully_defined = true;
                        }
                        TypeDefValue::Enum {
                            variant_type: _,
                            variants,
                        } => {
                            let symbol = self
                                .registry
                                .current_scope(&module.ctx)
                                .get_symbol(&type_def.name.repr)
                                .unwrap()
                                .relax();
                            let SymbolKind::Enum = symbol.kind else {
                                continue;
                            };
                            let enum_ = self.enums[symbol.col_index].relax_mut();

                            'variants: for variant in variants.iter() {
                                for existing_variant in &enum_.variants {
                                    if existing_variant.variant_name == variant.variant_name.repr {
                                        self.report_error(&Error {
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
                                            self.report_error(&err);
                                            variant_type = Type::Never
                                        }
                                    }
                                }

                                if !is_type_compatible(&enum_.variant_type, &variant_type) {
                                    self.report_error(&Error {
                                        message: format!(
                                            "Enum variant is not compatible with {}",
                                            TypeFmt(&*self.registry, &enum_.variant_type)
                                        ),
                                        loc: variant.variant_name.loc,
                                    });
                                }

                                // TODO: remove after migration
                                {
                                    self.registry.enums[symbol.col_index].variants.push(
                                        EnumVariant {
                                            variant_name: variant.variant_name.repr,
                                            variant_type: variant_type.clone(),
                                            loc: variant.variant_name.loc,
                                        },
                                    );
                                }

                                enum_.variants.push(EnumVariant {
                                    variant_name: variant.variant_name.repr,
                                    variant_type,
                                    loc: variant.variant_name.loc,
                                });
                            }
                        }
                        TypeDefValue::Alias(type_expr) => {
                            let type_value = self.registry.build_type(&module.ctx, &type_expr);
                            let type_value = catch!(type_value, err, {
                                self.report_error(&err);
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

                            self.type_aliases[symbol.col_index] = self.build_type_id(&type_value);

                            // TODO: remove after migration
                            {
                                self.registry.type_aliases[symbol.col_index] = type_value;
                            }
                        }
                    }
                }
                _ => {} // skip, not interested
            }
        }
    }

    fn pass_process_top_level_exprs(&mut self, ctx: &TyContext) {
        let module = self.registry.modules[ctx.module_id].relax_mut();

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

                    // TODO: remove after migration
                    {
                        let fn_info = self.registry.functions[symbol.col_index].relax_mut();

                        let self_type = self.registry.get_fn_self_type(
                            &module.ctx,
                            &fn_def.decl.fn_name,
                            &fn_def.decl.params,
                        );

                        if let Some(return_type) = &fn_def.decl.return_type {
                            fn_info.fn_type.output =
                                catch!(self.registry.build_type(&module.ctx, return_type), err, {
                                    self.report_error(&err);
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
                                self.report_error(&err);
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

                    let fn_info = self.functions[symbol.col_index].relax_mut();

                    let self_type =
                        self.get_fn_self_type(ctx, &fn_def.decl.fn_name, &fn_def.decl.params);

                    if let Some(return_type) = &fn_def.decl.return_type {
                        fn_info.fn_type.output = catch!(self.build_type(ctx, return_type), err, {
                            self.report_error(&err);
                            continue;
                        })
                    }

                    for fn_param in &fn_def.decl.params {
                        let param_type = self.get_fn_param_type(ctx, fn_param, &self_type, false);
                        let param_type = catch!(param_type, err, {
                            self.report_error(&err);
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

                    // TODO: check for valid const expression (in case of global)
                    let value_type = match self.type_code_expr_and_load(ctx, &let_expr.value) {
                        Ok(x) => x.clone(),
                        Err(err) => {
                            self.report_error(&err);
                            continue;
                        }
                    };

                    if let_expr.is_inline {
                        // TODO: remove after migration
                        {
                            let const_ = self.registry.constants[symbol.col_index].relax_mut();
                            const_.code_unit.type_ = value_type.clone();
                        }

                        let const_ = self.constants[symbol.col_index].relax_mut();

                        const_.code_unit.type_ = value_type;
                        continue;
                    }

                    let SymbolKind::Global = symbol.kind else {
                        continue;
                    };

                    // TODO: remove after migration
                    {
                        let global = self.registry.globals[symbol.col_index].relax_mut();
                        global.global_type = value_type.clone();
                    }

                    let global = self.globals[symbol.col_index].relax_mut();
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
                        // TODO: use inline let unwrapping to get int literal value
                        // skip, will be processed in `Compiler.codegen_top_level_leftover`
                        continue;
                    }

                    if intrinsic.fn_name.repr == "export_existing" {
                        let mut from_root = false;
                        let mut in_name = None;
                        let mut out_name = None;

                        if intrinsic.type_args.len() != 0 {
                            self.report_error(&bad_signature(&intrinsic.fn_name));
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
                                self.report_error(&bad_signature(&intrinsic.fn_name));
                                continue;
                            };

                            let CodeExpr::Ident(key) = &**lhs else {
                                self.report_error(&bad_signature(&intrinsic.fn_name));
                                continue;
                            };

                            if key.repr == "out" {
                                let CodeExpr::StringLiteral(value) = &**rhs else {
                                    self.report_error(&bad_signature(&intrinsic.fn_name));
                                    continue;
                                };

                                out_name = Some(value.relax());
                                continue;
                            }

                            self.report_error(&bad_signature(&intrinsic.fn_name));
                            continue;
                        }

                        let Some(in_name) = in_name else {
                            self.report_error(&bad_signature(&intrinsic.fn_name));
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

                            self.report_error(&Error {
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

                    self.report_error(&Error {
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
                decl,
                value: FnExprValue::Body(body),
                ..
            }) = expr
            else {
                continue;
            };

            let Some(symbol) = self.get_symbol(ctx.scope_id, decl.fn_name.repr) else {
                // TODO: what's the proper way to handle this
                //   maybe need to iterate on self.functions instead of ast
                continue;
            };

            let ctx = &self.child_ctx(&ctx, ScopeKind::Function);
            ctx.be_mut().fn_index = Some(symbol.col_index);

            let self_type = self.get_fn_self_type(ctx, &decl.fn_name, &decl.params);

            for param in &decl.params {
                if let Err(_) = self.define_symbol(
                    ctx,
                    &param.param_name.repr,
                    SymbolKind::Local,
                    param.param_name.loc,
                ) {
                    continue;
                };

                let param_type = catch!(
                    self.get_fn_param_type(ctx, param, &self_type, false),
                    err,
                    {
                        self.report_error(&err);
                        continue;
                    }
                );

                self.locals.push(TyLocal {
                    type_id: self.build_type_id(&param_type),
                });
            }

            self.type_code_block(ctx, body);
        }
    }

    fn type_code_block(&self, ctx: &TyContext, block: &CodeBlock) -> Type {
        let ctx = &self.child_ctx(ctx, ScopeKind::Block);

        let mut diverges = false;

        // TODO: cache this
        let never_type_id = self.build_type_id(&Type::Never);

        for expr in &block.exprs {
            self.report_if_err(self.type_code_expr(ctx, expr));
            let Some(type_id) = self.load_type_id(ctx, expr) else {
                continue;
            };
            diverges = diverges || type_id == never_type_id;
        }

        if diverges {
            return Type::Never;
        }

        // TODO: store type in code_block.id?
        Type::Void
    }

    fn type_code_expr(&self, ctx: &TyContext, expr: &CodeExpr) -> Result<(), Error> {
        match expr {
            CodeExpr::BoolLiteral(bool_literal) => {
                self.store_type(ctx, bool_literal.id, &Type::Bool);
                return Ok(());
            }
            CodeExpr::CharLiteral(char_literal) => {
                self.store_type(ctx, char_literal.id, &Type::U8);
                return Ok(());
            }
            CodeExpr::NullLiteral(null_literal) => {
                self.store_type(ctx, null_literal.id, &Type::Null);
                return Ok(());
            }
            CodeExpr::IntLiteral(int_literal) => {
                let Some(tag) = int_literal.tag else {
                    self.store_type(ctx, int_literal.id, &Type::U32);
                    return Ok(());
                };

                let tag_type = catch!(self.get_type_or_err(ctx, tag, &int_literal.loc), err, {
                    self.report_error(&err);
                    self.store_type(ctx, int_literal.id, &Type::U32);
                    return Ok(());
                });

                if let Err(err) = is_64_bit_int_tag(&self.registry, &tag_type, &int_literal.loc) {
                    self.report_error(&err);
                    self.store_type(ctx, int_literal.id, &Type::U32);
                    return Ok(());
                }

                self.store_type(ctx, int_literal.id, &tag_type);

                return Ok(());
            }
            CodeExpr::StringLiteral(str_literal) => {
                let Some(symbol) = self.get_symbol(ctx.scope_id, "str") else {
                    return Err(Error {
                        message: format!("Cannot use strings with no `str` struct defined"),
                        loc: str_literal.loc,
                    });
                };

                let str_type = Type::StructInstance {
                    struct_index: symbol.col_index,
                };
                self.store_type(ctx, str_literal.id, &str_type);
                return Ok(());
            }
            CodeExpr::StructLiteral(struct_literal) => {
                // TODO: check that fields match struct definition
                for field in &struct_literal.body.fields {
                    self.report_if_err(self.type_code_expr(ctx, &field.value));
                }

                let Some(symbol) = self.get_symbol(ctx.scope_id, &struct_literal.struct_name.repr)
                else {
                    return Err(Error {
                        message: format!("Unknown struct: {}", struct_literal.struct_name.repr),
                        loc: struct_literal.loc,
                    });
                };

                let struct_type = Type::StructInstance {
                    struct_index: symbol.col_index,
                };
                self.store_type(ctx, struct_literal.id, &struct_type);
                return Ok(());
            }
            CodeExpr::ArrayLiteral(array_literal) => {
                // TODO: check items match array item type
                for item_expr in &array_literal.items {
                    self.report_if_err(self.type_code_expr(ctx, &item_expr));
                }

                let item_type = self.build_type(ctx, &array_literal.item_type)?;

                let array_type = Type::SequencePointer {
                    pointee: Box::new(item_type),
                };
                self.store_type(ctx, array_literal.id, &array_type);
                return Ok(());
            }
            CodeExpr::ResultLiteral(result_literal) => {
                // TODO: check this matches expected result type
                if let Some(value) = &result_literal.value {
                    self.report_if_err(self.type_code_expr(ctx, &value));
                }

                let result = self.get_result_literal_type(
                    ctx,
                    &result_literal.result_type,
                    &result_literal.loc,
                )?;
                self.store_type(ctx, result_literal.id, &Type::Result(result));

                return Ok(());
            }
            CodeExpr::Ident(ident) => {
                let var_type_id = self.get_variable_type_id(ctx, ident)?;
                self.store_expr_info(ctx, ident.id, var_type_id);
                return Ok(());
            }
            CodeExpr::Let(let_expr) => {
                self.report_if_err(self.type_code_expr(ctx, &let_expr.value));

                let Some(value_type_id) = self.load_type_id(ctx, &let_expr.value) else {
                    self.store_type(ctx, let_expr.id, &Type::Void);
                    return Ok(());
                };

                if self.registry.types[value_type_id] == Type::Never {
                    self.store_type(ctx, let_expr.id, &Type::Never);
                } else {
                    self.store_type(ctx, let_expr.id, &Type::Void);
                }

                if let_expr.name.repr == "_" {
                    return Ok(());
                }

                let _ = self.define_symbol(
                    ctx,
                    let_expr.name.repr,
                    SymbolKind::Local,
                    let_expr.name.loc,
                );
                self.locals.be_mut().push(TyLocal {
                    type_id: value_type_id,
                });
                return Ok(());
            }
            CodeExpr::InfixOp(infix_op) => {
                // TODO: implement

                self.report_if_err(self.type_code_expr(ctx, &infix_op.lhs));
                self.report_if_err(self.type_code_expr(ctx, &infix_op.rhs));

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
                        return Ok(());
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
                        if let Some(type_id) = self.load_type_id(ctx, &infix_op.lhs) {
                            self.store_expr_info(ctx, infix_op.id, type_id);
                        }
                        return Ok(());
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
                        return Ok(());
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
                let expr_type = self.type_code_expr_and_load(ctx, &prefix_op.expr)?;

                match &prefix_op.op_tag {
                    PrefixOpTag::Not => {
                        self.store_type(ctx, prefix_op.id, &Type::Bool);
                        return Ok(());
                    }
                    PrefixOpTag::Reference => {
                        let type_ = Type::Pointer {
                            pointee: Box::new(expr_type.clone()),
                        };
                        self.store_type(ctx, prefix_op.id, &type_);
                        return Ok(());
                    }
                    PrefixOpTag::Dereference => {
                        let (Type::Pointer { pointee } | Type::SequencePointer { pointee }) =
                            expr_type
                        else {
                            return Err(Error {
                                message: format!(
                                    "Cannot dereference expr of type {}",
                                    TypeFmt(&*self.registry, &expr_type)
                                ),
                                loc: prefix_op.loc,
                            });
                        };
                        self.store_type(ctx, prefix_op.id, pointee);
                        return Ok(());
                    }
                    PrefixOpTag::Positive | PrefixOpTag::Negative => match expr_type {
                        Type::U8 => {
                            self.store_type(ctx, prefix_op.id, &Type::I8);
                            return Ok(());
                        }
                        Type::U16 => {
                            self.store_type(ctx, prefix_op.id, &Type::I16);
                            return Ok(());
                        }
                        Type::U32 => {
                            self.store_type(ctx, prefix_op.id, &Type::I32);
                            return Ok(());
                        }
                        Type::U64 => {
                            self.store_type(ctx, prefix_op.id, &Type::I64);
                            return Ok(());
                        }
                        Type::Never
                        | Type::Null
                        | Type::Void
                        | Type::Bool
                        | Type::I8
                        | Type::I16
                        | Type::I32
                        | Type::F32
                        | Type::I64
                        | Type::F64
                        | Type::Pointer { pointee: _ }
                        | Type::SequencePointer { pointee: _ }
                        | Type::StructInstance { struct_index: _ }
                        | Type::EnumInstance { enum_index: _ }
                        | Type::Result(_)
                        | Type::Container(_) => {
                            // TODO: report error if negating unsupported type
                            self.store_type(ctx, prefix_op.id, expr_type);
                            return Ok(());
                        }
                    },
                }
            }
            CodeExpr::Cast(cast) => {
                self.report_if_err(self.type_code_expr(ctx, &cast.expr));

                let casted_to = self.build_type(ctx, &cast.casted_to)?;
                self.store_type(ctx, cast.id, &casted_to);

                return Ok(());
            }
            CodeExpr::Assign(assign) => {
                // TODO: implement

                self.store_type(ctx, assign.id, &Type::Void);
                self.report_if_err(self.type_code_expr(ctx, &assign.lhs));
                self.report_if_err(self.type_code_expr(ctx, &assign.rhs));
                return Ok(());
            }
            CodeExpr::FieldAccess(field_access) => {
                // TODO: implement

                let lhs_type = self.type_code_expr_and_load(ctx, &field_access.lhs)?;
                let field = self.get_struct_or_struct_ref_field(&lhs_type, field_access)?;
                self.store_type(ctx, field_access.id, &field.field_type);
                return Ok(());
            }
            CodeExpr::FnCall(call) => {
                if let Some(symbol) = self.get_symbol(ctx.scope_id, &call.fn_name.repr) {
                    if let SymbolKind::EnumConstructor = symbol.kind {
                        let ctor = &self.enum_ctors[symbol.col_index];

                        // TODO: validate arg with enum expectation
                        for arg in &call.args.items {
                            self.report_if_err(self.type_code_expr(ctx, arg));
                        }

                        self.store_type(
                            ctx,
                            call.id,
                            &Type::EnumInstance {
                                enum_index: ctor.enum_index,
                            },
                        );

                        return Ok(());
                    }
                }

                self.type_fn_call(
                    ctx,
                    call.fn_name.repr,
                    None,
                    &call.args.items,
                    call.id,
                    &call.fn_name.loc,
                )?;
                return Ok(());
            }
            CodeExpr::MethodCall(call) => {
                let lhs_type = self.type_code_expr_and_load(ctx, &call.lhs)?;
                let fn_name = self
                    .registry
                    .get_fn_name_from_method(&lhs_type, &call.field_name.repr);

                self.type_fn_call(
                    ctx,
                    &fn_name,
                    Some(&call.lhs),
                    &call.args.items,
                    call.id,
                    &call.field_name.loc,
                )?;
                return Ok(());
            }
            CodeExpr::InlineFnCall(call) => {
                self.type_inline_fn_call(
                    ctx,
                    &call.fn_name.repr,
                    &call.type_args,
                    None,
                    &call.args.items,
                    call.id,
                    &call.fn_name.loc,
                )?;
                return Ok(());
            }
            CodeExpr::InlineMethodCall(call) => {
                let lhs_type = self.type_code_expr_and_load(ctx, &call.lhs)?;
                let inline_fn_name = self
                    .registry
                    .get_fn_name_from_method(&lhs_type, &call.field_name.repr);

                self.type_inline_fn_call(
                    ctx,
                    &inline_fn_name,
                    &call.type_args,
                    Some(&call.lhs),
                    &call.args.items,
                    call.id,
                    &call.field_name.loc,
                )?;
                return Ok(());
            }
            // TODO: check arguments
            CodeExpr::IntrinsicCall(call) => {
                // TODO: is this always needed?
                for arg in &call.args.items {
                    self.report_if_err(self.type_code_expr(ctx, arg));
                }

                if call.fn_name.repr == "unreachable" {
                    self.store_type(ctx, call.id, &Type::Never);
                    return Ok(());
                }

                if call.fn_name.repr == "memory_size" {
                    self.store_type(ctx, call.id, &Type::I32);
                    return Ok(());
                }

                if call.fn_name.repr == "memory_grow" {
                    self.store_type(ctx, call.id, &Type::I32);
                    return Ok(());
                }

                if call.fn_name.repr == "memory_copy" {
                    self.store_type(ctx, call.id, &Type::Void);
                    return Ok(());
                }

                if call.fn_name.repr == "data_size" {
                    self.store_type(ctx, call.id, &Type::U32);
                    return Ok(());
                }

                if call.fn_name.repr == "embed_file" {
                    self.store_type(
                        ctx,
                        call.id,
                        &Type::SequencePointer {
                            pointee: Box::new(Type::U8),
                        },
                    );
                    return Ok(());
                }

                if call.fn_name.repr == "const_slice_len" {
                    self.store_type(ctx, call.id, &Type::U32);
                    return Ok(());
                }

                if call.fn_name.repr == "inline_fn_call_loc" {
                    self.store_type(
                        ctx,
                        call.id,
                        &Type::StructInstance {
                            struct_index: self.get_symbol(ctx.scope_id, "str").unwrap().col_index,
                        },
                    );
                    return Ok(());
                }

                if call.fn_name.repr == "get_ok" {
                    if call.args.items.len() == 1 {
                        let arg_type = self.type_code_expr_and_load(ctx, &call.args.items[0])?;

                        if let Type::Result(result) = arg_type.relax() {
                            self.store_type(ctx, call.id, &result.ok);
                        };
                        return Ok(());
                    }

                    return Err(Error {
                        message: format!(
                            "Invalid arguments for @{}(items: Result(T, E)): T",
                            call.fn_name.repr,
                        ),
                        loc: call.fn_name.loc,
                    });
                }

                if call.fn_name.repr == "get_err" {
                    if call.args.items.len() == 1 {
                        let arg_type = self.type_code_expr_and_load(ctx, &call.args.items[0])?;

                        if let Type::Result(result) = arg_type.relax() {
                            self.store_type(ctx, call.id, &result.err);
                        };

                        return Ok(());
                    }

                    return Err(Error {
                        message: format!(
                            "Invalid arguments for @{}(items: Result(T, E)): E",
                            call.fn_name.repr,
                        ),
                        loc: call.fn_name.loc,
                    });
                }

                if call.fn_name.repr.starts_with("inspect_") {
                    self.store_type(ctx, call.id, &Type::Void);
                    return Ok(());
                }

                self.report_error(&Error {
                    message: format!("Unknown intrinsic: {}", call.fn_name.repr),
                    loc: call.fn_name.loc,
                });
                return Ok(());
            }
            CodeExpr::Return(return_expr) => {
                self.store_type(ctx, return_expr.id, &Type::Never);

                if let Some(return_value) = &return_expr.expr {
                    self.report_if_err(self.type_code_expr(ctx, &return_value));
                }
                return Ok(());
            }
            CodeExpr::If(if_expr) => {
                // TODO: implement

                let mut updated_then_ctx = None;

                match &if_expr.cond {
                    IfCond::Expr(code_expr) => {
                        self.report_if_err(self.type_code_expr(ctx, &code_expr));
                    }
                    IfCond::Match(match_header) => {
                        self.report_if_err(self.type_code_expr(ctx, &match_header.expr_to_match));

                        if let Some(symbol) =
                            self.get_symbol(ctx.scope_id, &match_header.variant_name.repr)
                        {
                            if let SymbolKind::EnumConstructor = symbol.kind {
                                let enum_ctor = &self.enum_ctors[symbol.col_index];
                                let enum_variant = &self.enums[enum_ctor.enum_index].variants
                                    [enum_ctor.variant_index];

                                let then_ctx = self.child_ctx(ctx, ScopeKind::Block);
                                self.register_block_const(
                                    &then_ctx,
                                    ConstDef {
                                        const_name: match_header.variant_bind.repr,
                                        code_unit: CodeUnit {
                                            type_: enum_variant.variant_type.clone(),
                                            instrs: Vec::new(),
                                        },
                                        loc: match_header.variant_bind.loc,
                                    },
                                );
                                updated_then_ctx = Some(then_ctx);
                            } else {
                                // TODO: update error message
                                self.report_error(&Error {
                                    message: format!("error 1"),
                                    loc: match_header.variant_name.loc,
                                });
                            };
                        } else {
                            // TODO: update error message
                            self.report_error(&Error {
                                message: format!("error 2"),
                                loc: match_header.variant_name.loc,
                            });
                        }
                    }
                }

                let then_type;
                if let Some(updated_then_ctx) = updated_then_ctx {
                    then_type = self.type_code_block(&updated_then_ctx, &if_expr.then_block);
                } else {
                    then_type = self.type_code_block(ctx, &if_expr.then_block);
                }

                let mut else_type = Type::Void;
                match &if_expr.else_block {
                    ElseBlock::None => {}
                    ElseBlock::Else(code_block) => {
                        else_type = self.type_code_block(ctx, code_block);
                    }
                    ElseBlock::ElseIf(code_expr) => {
                        self.report_if_err(self.type_code_expr(ctx, &code_expr));
                        if let Some(type_id) = self.load_type_id(ctx, &code_expr) {
                            else_type = self.registry.types[type_id].clone();
                        }
                    }
                }

                // TODO: perform this check on type ids
                if then_type == Type::Never && else_type == Type::Never {
                    self.store_type(ctx, if_expr.id, &Type::Never);
                } else {
                    self.store_type(ctx, if_expr.id, &Type::Void);
                }

                return Ok(());
            }
            CodeExpr::While(while_expr) => {
                // TODO: implement

                if let Some(cond) = &while_expr.cond {
                    self.report_if_err(self.type_code_expr(ctx, &cond));
                }

                self.type_code_block(ctx, &while_expr.body);

                self.store_type(ctx, while_expr.id, &Type::Void);

                return Ok(());
            }
            CodeExpr::For(for_expr) => {
                // TODO: implement

                self.report_if_err(self.type_code_expr(ctx, &for_expr.start));
                self.report_if_err(self.type_code_expr(ctx, &for_expr.end));

                let ctx = &self.child_ctx(ctx, ScopeKind::ForLoop);

                let Ok(_) = self.define_symbol(
                    ctx,
                    for_expr.counter.repr,
                    SymbolKind::Local,
                    for_expr.counter.loc,
                ) else {
                    unreachable!()
                };

                if let Some(counter_type_id) = self.load_type_id(ctx, &for_expr.start) {
                    self.locals.be_mut().push(TyLocal {
                        type_id: counter_type_id,
                    });
                }

                self.type_code_block(ctx, &for_expr.body);

                self.store_type(ctx, for_expr.id, &Type::Void);

                return Ok(());
            }
            CodeExpr::Break(break_expr) => {
                // TODO: add validations
                self.store_type(ctx, break_expr.id, &Type::Never);
                return Ok(());
            }
            CodeExpr::Continue(continue_expr) => {
                // TODO: add validations
                self.store_type(ctx, continue_expr.id, &Type::Never);
                return Ok(());
            }
            CodeExpr::Defer(defer) => {
                self.type_code_expr(ctx, &defer.expr)?;
                self.store_type(ctx, defer.id, &Type::Void);
                return Ok(());
            }
            CodeExpr::Catch(catch) => {
                // TODO: improve error tolerance
                let expr_type = self.type_code_expr_and_load(ctx, &catch.lhs)?;
                let result = self.assert_catchable_type(&expr_type, &catch.catch_loc)?;

                let ctx = &self.child_ctx(ctx, ScopeKind::Block);

                let Ok(_) = self.define_symbol(
                    ctx,
                    catch.error_bind.repr,
                    SymbolKind::Local,
                    catch.error_bind.loc,
                ) else {
                    unreachable!()
                };
                self.locals.be_mut().push(TyLocal {
                    type_id: self.build_type_id(&result.err),
                });

                self.type_code_block(&ctx, &catch.catch_body);

                self.store_type(ctx, catch.id, &result.ok);
                return Ok(());
            }
            CodeExpr::PropagateError(prop_error) => {
                // TODO: improve error tolerance?
                let expr_type = self.type_code_expr_and_load(ctx, &prop_error.expr)?;
                let result = self.assert_catchable_type(&expr_type, &prop_error.loc)?;
                self.store_type(ctx, prop_error.id, &result.ok);
                return Ok(());
            }
            CodeExpr::Match(match_expr) => {
                self.report_if_err(self.type_code_expr(ctx, &match_expr.header.expr_to_match));

                let else_ctx = &self.child_ctx(ctx, ScopeKind::Block);
                // TODO: assert diverges
                self.type_code_block(else_ctx, &match_expr.else_branch);

                if let Some(symbol) =
                    self.get_symbol(ctx.scope_id, &match_expr.header.variant_name.repr)
                {
                    if let SymbolKind::EnumConstructor = symbol.kind {
                        let enum_ctor = &self.enum_ctors[symbol.col_index];
                        let enum_variant =
                            &self.enums[enum_ctor.enum_index].variants[enum_ctor.variant_index];

                        if let Ok(()) = self.define_symbol(
                            ctx,
                            match_expr.header.variant_bind.repr,
                            SymbolKind::Local,
                            match_expr.header.variant_bind.loc,
                        ) {
                            self.locals.be_mut().push(TyLocal {
                                type_id: self.build_type_id(&enum_variant.variant_type),
                            });
                        };
                    } else {
                        // TODO: update error message
                        self.report_error(&Error {
                            message: format!("error 1"),
                            loc: match_expr.header.variant_name.loc,
                        });
                    };
                } else {
                    // TODO: update error message
                    self.report_error(&Error {
                        message: format!("error 2"),
                        loc: match_expr.header.variant_name.loc,
                    });
                }

                self.store_type(ctx, match_expr.id, &Type::Void);

                return Ok(());
            }
            CodeExpr::Paren(paren) => {
                self.report_if_err(self.type_code_expr(ctx, &paren.expr));

                if let Some(expr_type_id) = self.load_type_id(ctx, &paren.expr) {
                    self.store_expr_info(ctx, paren.id, expr_type_id);
                }
                return Ok(());
            }
            CodeExpr::DoWith(do_with) => {
                // TODO: implement

                for arg in &do_with.args.items {
                    self.report_if_err(self.type_code_expr(ctx, arg));
                }

                let ctx = &self.child_ctx(ctx, ScopeKind::Block);

                if let Some(it_type_id) = self.load_type_id(ctx, &do_with.args.items[0]) {
                    let Ok(_) = self.define_symbol(ctx, "it", SymbolKind::Local, do_with.with_loc)
                    else {
                        unreachable!()
                    };
                    self.locals.be_mut().push(TyLocal {
                        type_id: it_type_id,
                    });
                }

                self.report_if_err(self.type_code_expr(ctx, &do_with.body));

                self.store_type(ctx, do_with.id, &Type::Void);
                return Ok(());
            }
            CodeExpr::Pipe(pipe) => {
                // TODO: typecheck
                self.report_if_err(self.type_code_expr(ctx, &pipe.lhs));

                // TODO: check scope kind
                let ctx = &self.child_ctx(ctx, ScopeKind::InlineFn);
                if let Some(it_type_id) = self.load_type_id(ctx, &pipe.lhs) {
                    let Ok(_) = self.define_symbol(ctx, "it", SymbolKind::Local, pipe.op_loc)
                    else {
                        unreachable!()
                    };
                    self.locals.be_mut().push(TyLocal {
                        type_id: it_type_id,
                    });
                }

                self.report_if_err(self.type_code_expr(ctx, &pipe.rhs));

                if let Some(type_id) = self.load_type_id(ctx, &pipe.rhs) {
                    self.store_expr_info(ctx, pipe.id, type_id);
                } else {
                    self.store_type(ctx, pipe.id, &Type::Never);
                }

                return Ok(());
            }
            CodeExpr::Sizeof(sizeof) => {
                // TODO: check that sizeof'd type is valid
                self.store_type(ctx, sizeof.id, &Type::U32);
                return Ok(());
            }
        }
    }

    fn get_result_literal_type(
        &self,
        ctx: &TyContext,
        explicit_type: &Option<ResultTypeExpr>,
        loc: &Loc,
    ) -> Result<ResultType, Error> {
        if let Some(result_type) = explicit_type {
            let ok = Box::new(self.build_type(ctx, &result_type.ok)?);
            let err = Box::new(self.build_type(ctx, &result_type.err)?);
            return Ok(ResultType { ok, err });
        }

        let Some(fn_index) = ctx.fn_index else {
            return Err(Error {
                message: format!("Cannot create implicitly typed result in const context"),
                loc: *loc,
            });
        };

        let fn_info = &self.functions[fn_index];
        let Type::Result(result) = &fn_info.fn_type.output else {
            return Err(Error {
                message: format!(
                    "Cannot create implicitly typed result: function does not return result"
                ),
                loc: *loc,
            });
        };

        Ok(ResultType {
            ok: result.ok.clone(),
            err: result.err.clone(),
        })
    }

    fn get_variable_type_id(&self, ctx: &TyContext, var_name: &IdentExpr) -> Result<TypeId, Error> {
        let Some(symbol) = self.get_symbol(ctx.scope_id, var_name.repr) else {
            return Err(Error {
                message: format!("Unknown variable: {}", var_name.repr),
                loc: var_name.loc,
            });
        };

        match symbol.kind {
            SymbolKind::Local => {
                return Ok(self.locals[symbol.col_index].type_id);
            }
            SymbolKind::Global => {
                return Ok(self.build_type_id(&self.globals[symbol.col_index].global_type));
            }
            SymbolKind::Const => {
                return Ok(self.build_type_id(&self.constants[symbol.col_index].code_unit.type_));
            }
            SymbolKind::EnumConstructor => {
                let enum_ctor = &self.enum_ctors[symbol.col_index];
                let enum_def = &self.enums[enum_ctor.enum_index];
                let Type::Void = enum_def.variant_type else {
                    return Err(Error {
                        message: format!(
                            "Cannot construct {}, expected payload of type {}",
                            var_name.repr,
                            TypeFmt(
                                &*self.registry,
                                &enum_def.variants[enum_ctor.variant_index].variant_type
                            )
                        ),
                        loc: var_name.loc,
                    });
                };

                return Ok(self.build_type_id(&Type::EnumInstance {
                    enum_index: enum_ctor.enum_index,
                }));
            }
            _ => {
                return Err(Error {
                    message: format!(
                        "Expected variable, found {:?} '{}'",
                        symbol.kind, var_name.repr
                    ),
                    loc: var_name.loc,
                });
            }
        }
    }

    fn assert_catchable_type<'a>(
        &self,
        expr_type: &'a Type,
        loc: &Loc,
    ) -> Result<&'a ResultType, Error> {
        let Type::Result(result) = expr_type else {
            return Err(Error {
                message: format!(
                    "Cannot catch error from expr of type {}",
                    TypeFmt(&*self.registry, &expr_type)
                ),
                loc: *loc,
            });
        };

        let mut err_type_components = Vec::new();
        self.lower_type(&result.err, &mut err_type_components);
        if err_type_components != [WasmType::I32] {
            return Err(Error {
                message: format!(
                    "Invalid Result error type: {}, must lower to i32",
                    TypeFmt(&*self.registry, &result.err)
                ),
                loc: *loc,
            });
        }

        Ok(result)
    }

    fn lower_type(&self, lo_type: &Type, wasm_types: &mut Vec<WasmType>) {
        match lo_type {
            Type::Never | Type::Void => {}
            Type::Null
            | Type::Bool
            | Type::U8
            | Type::I8
            | Type::U16
            | Type::I16
            | Type::U32
            | Type::I32 => wasm_types.push(WasmType::I32),
            Type::F32 => wasm_types.push(WasmType::F32),
            Type::U64 => wasm_types.push(WasmType::I64),
            Type::I64 => wasm_types.push(WasmType::I64),
            Type::F64 => wasm_types.push(WasmType::F64),
            Type::Pointer { pointee: _ } => wasm_types.push(WasmType::I32),
            Type::SequencePointer { pointee: _ } => wasm_types.push(WasmType::I32),
            Type::StructInstance { struct_index } => {
                let struct_def = &self.structs[*struct_index];

                for field in &struct_def.fields {
                    self.lower_type(&field.field_type, wasm_types);
                }
            }
            Type::EnumInstance { enum_index } => {
                let enum_def = &self.enums[*enum_index];

                self.lower_type(&Type::U32, wasm_types);
                self.lower_type(&enum_def.variant_type, wasm_types);
            }
            Type::Result(result) => {
                self.lower_type(&result.ok, wasm_types);
                self.lower_type(&result.err, wasm_types);
            }
            Type::Container(ContainerType {
                container,
                items: _,
            }) => {
                self.lower_type(container, wasm_types);
            }
        }
    }

    fn get_struct_or_struct_ref_field(
        &self,
        mut lhs_type: &Type,
        field_access: &FieldAccessExpr,
    ) -> Result<&StructField, Error> {
        if let Type::Pointer { pointee } = &lhs_type {
            lhs_type = pointee;
        }

        let struct_index: usize;
        if let Type::StructInstance { struct_index: si } = lhs_type {
            struct_index = *si;
        } else if let Type::Container(ContainerType {
            container,
            items: _,
        }) = lhs_type
            && let Type::StructInstance { struct_index: si } = &**container
        {
            struct_index = *si;
        } else {
            return Err(Error {
                message: format!(
                    "Cannot get field '{}' on non struct: {}",
                    field_access.field_name.repr,
                    TypeFmt(&*self.registry, lhs_type),
                ),
                loc: field_access.field_name.loc,
            });
        };

        let struct_def = &self.structs[struct_index];
        let Some(field) = struct_def
            .fields
            .iter()
            .find(|f| &f.field_name == &field_access.field_name.repr)
        else {
            return Err(Error {
                message: format!(
                    "Unknown field {} in struct {}",
                    field_access.field_name.repr, struct_def.struct_name
                ),
                loc: field_access.field_name.loc,
            });
        };

        Ok(field)
    }

    fn type_fn_call(
        &self,
        ctx: &TyContext,
        fn_name: &str,
        receiver_arg: Option<&CodeExpr>,
        args: &Vec<CodeExpr>,
        call_expr_id: usize,
        loc: &Loc,
    ) -> Result<(), Error> {
        if let Some(receiver_arg) = receiver_arg {
            self.report_if_err(self.type_code_expr(ctx, receiver_arg));
        }
        for arg in args {
            self.report_if_err(self.type_code_expr(ctx, &arg));
        }

        let Some(symbol) = self.get_symbol(ctx.scope_id, fn_name) else {
            return Err(Error {
                message: format!("Unknown function: {}", fn_name),
                loc: *loc,
            });
        };

        let SymbolKind::Function = symbol.kind else {
            return Err(Error {
                message: format!(
                    "Trying to call {} which is not a function, defined at: {}",
                    fn_name,
                    symbol.loc.to_string(&self.reporter.fm)
                ),
                loc: *loc,
            });
        };

        let fn_def = &self.functions[symbol.col_index];
        self.store_type(ctx, call_expr_id, &fn_def.fn_type.output);

        return Ok(());
    }

    fn type_inline_fn_call(
        &self,
        ctx: &TyContext,
        inline_fn_name: &str,
        type_args: &Vec<TypeExpr>,
        receiver_arg: Option<&CodeExpr>,
        args: &Vec<CodeExpr>,
        call_expr_id: usize,
        loc: &Loc,
    ) -> Result<(), Error> {
        let Some(symbol) = self.get_symbol(ctx.scope_id, inline_fn_name) else {
            return Err(Error {
                message: format!("Unknown inline fn: {}", inline_fn_name),
                loc: *loc,
            });
        };
        let inline_fn_def = self.inline_fns[symbol.col_index];

        let FnExprValue::Body(body) = &inline_fn_def.value else {
            unreachable!()
        };

        let parent_ctx = ctx;
        let ctx = &mut self.child_ctx(ctx, ScopeKind::InlineFn);
        ctx.expr_info_offset = self.registry.expr_info.len() - body.expr_id_start;
        self.extend_expr_info_storage(body.expr_id_end - body.expr_id_start);

        let mut lo_type_args = Vec::new();
        for type_arg in type_args {
            lo_type_args.push(self.build_type(ctx, &type_arg)?);
        }
        if lo_type_args.len() != inline_fn_def.decl.type_params.len() {
            return Err(Error {
                message: format!(
                    "Invalid number of type args, expected {}, got {}",
                    inline_fn_def.decl.type_params.len(),
                    type_args.len()
                ),
                loc: *loc,
            });
        }
        for (i, (type_param, type_arg)) in inline_fn_def
            .decl
            .type_params
            .iter()
            .zip(lo_type_args.iter())
            .enumerate()
        {
            self.register_block_type(ctx, type_param, type_arg.clone(), *type_args[i].loc());
        }

        let mut arg_types = Vec::<Type>::new();
        if let Some(receiver_arg) = receiver_arg {
            arg_types.push(
                self.type_code_expr_and_load(parent_ctx, receiver_arg)?
                    .clone(),
            );
        }
        for arg in args {
            arg_types.push(self.type_code_expr_and_load(parent_ctx, arg)?.clone());
        }
        if arg_types.len() != inline_fn_def.decl.params.len() {
            return Err(Error {
                message: format!(
                    "Invalid number of inline fn args, expected {}, got {}",
                    inline_fn_def.decl.params.len(),
                    arg_types.len()
                ),
                loc: *loc,
            });
        }
        for (inline_fn_param, inline_fn_arg) in
            inline_fn_def.decl.params.iter().zip(arg_types.iter())
        {
            let const_def = ConstDef {
                const_name: inline_fn_param.param_name.repr,
                code_unit: CodeUnit {
                    type_: inline_fn_arg.clone(),
                    instrs: Default::default(),
                },
                loc: inline_fn_param.param_name.loc,
            };

            if let Some(type_name) = get_infer_type_name(inline_fn_param)? {
                self.register_block_type(
                    ctx,
                    type_name,
                    const_def.code_unit.type_.clone(),
                    inline_fn_param.loc,
                );
            }

            // self.register_block_const(ctx, const_def);

            let _ = self.define_symbol(ctx, const_def.const_name, SymbolKind::Local, const_def.loc);
            self.locals.be_mut().push(TyLocal {
                type_id: self.build_type_id(&const_def.code_unit.type_),
            });
        }

        let self_type = self.get_fn_self_type(
            parent_ctx,
            &inline_fn_def.decl.fn_name,
            &inline_fn_def.decl.params,
        );

        let mut inline_fn_param_types = Vec::<Type>::new();
        for inline_fn_param in &inline_fn_def.decl.params {
            let inline_fn_type = self.get_fn_param_type(ctx, inline_fn_param, &self_type, true)?;
            inline_fn_param_types.push(inline_fn_type);
        }

        if !is_types_compatible(&inline_fn_param_types, &arg_types) {
            return Err(Error {
                message: format!(
                    "Invalid inline fn args, expected [{}], got [{}]",
                    TypeListFmt(&*self.registry, &inline_fn_param_types),
                    TypeListFmt(&*self.registry, &arg_types)
                ),
                loc: *loc,
            });
        }

        let return_type = if let Some(return_type) = &inline_fn_def.decl.return_type {
            self.build_type(ctx, return_type)?
        } else {
            Type::Void
        };

        if self.reporter.in_inspection_mode {
            let mut message = String::new();

            write!(&mut message, "inline fn {inline_fn_name}").unwrap();
            if lo_type_args.len() > 0 {
                let lo_type_args = TypeListFmt(&*self.registry, &lo_type_args);
                write!(&mut message, "<{lo_type_args}>").unwrap();
            }
            write!(&mut message, "(").unwrap();

            for i in 0..inline_fn_param_types.len() {
                if i != 0 {
                    message.push_str(", ");
                }

                let param = &inline_fn_def.decl.params[i];
                message.push_str(param.param_name.repr);
                match param.param_type {
                    FnParamType::Self_ | FnParamType::SelfRef => {}
                    _ => {
                        message.push_str(": ");
                        let arg_type = TypeFmt(&*self.registry, &inline_fn_param_types[i]);
                        write!(&mut message, "{arg_type}",).unwrap();
                    }
                }
            }

            write!(
                &mut message,
                "): {}",
                TypeFmt(&*self.registry, &return_type)
            )
            .unwrap();

            self.reporter.print_inspection(&InspectInfo {
                message,
                loc: *loc,
                linked_loc: Some(inline_fn_def.decl.fn_name.loc),
            });
        }

        // TODO: remove if no longer needed for debugging
        // stderr_writeln(format!("set_offset({})", ctx.expr_info_offset));
        self.type_code_block(ctx, body);

        self.store_expr_info(
            parent_ctx,
            call_expr_id,
            self.registry.inline_fn_call_info.len(),
        );
        self.registry.inline_fn_call_info.be_mut().push(ICallInfo {
            return_type_id: self.build_type_id(&return_type),
            inner_expr_offset: ctx.expr_info_offset,
        });

        Ok(())
    }

    fn get_fn_param_type(
        &self,
        ctx: &TyContext,
        fn_param: &FnParam,
        self_type: &Option<Type>,
        infer_allowed: bool,
    ) -> Result<Type, Error> {
        match &fn_param.param_type {
            FnParamType::Self_ | FnParamType::SelfRef => {
                // SAFETY: `get_fn_self_type` does the check
                let self_type = self_type.clone().unwrap();

                if let FnParamType::Self_ = fn_param.param_type {
                    return Ok(self_type);
                }

                return Ok(Type::Pointer {
                    pointee: Box::new(self_type),
                });
            }
            FnParamType::Type { expr } => {
                if let Some(infer_type_name) = get_infer_type_name(fn_param)? {
                    if !infer_allowed {
                        return Err(Error {
                            message: format!("Infer is only allowed in inline fns"),
                            loc: fn_param.param_name.loc,
                        });
                    }

                    return self.get_type_or_err(ctx, infer_type_name, &fn_param.param_name.loc);
                }

                self.build_type(ctx, &expr)
            }
        }
    }

    fn get_fn_self_type(
        &self,
        ctx: &TyContext,
        fn_name: &IdentExpr,
        fn_params: &Vec<FnParam>,
    ) -> Option<Type> {
        let mut has_self_param = false;
        for fn_param in fn_params {
            let (FnParamType::Self_ | FnParamType::SelfRef) = fn_param.param_type else {
                continue;
            };

            has_self_param = true;

            if fn_name.parts.len() == 1 {
                self.report_error(&Error {
                    message: format!("Cannot use self param in non-method function"),
                    loc: fn_param.loc,
                });
                return Some(Type::Never);
            }
        }
        if !has_self_param {
            return None;
        }

        let mut module = &self.registry.modules[ctx.module_id];
        if fn_name.loc.file_index != module.parser.lexer.file_index {
            // fn imported from other module
            module = &self
                .registry
                .get_module_by_file_index(fn_name.loc.file_index)
                .unwrap();
        }

        let mut self_type_loc = fn_name.parts[0];
        self_type_loc.end_pos = fn_name.parts[fn_name.parts.len() - 2].end_pos;

        let self_type_name = self_type_loc.read_span(module.source);

        let self_type = catch!(
            self.get_type_or_err(ctx, &self_type_name, &self_type_loc),
            err,
            {
                self.report_error(&err);
                return Some(Type::Never);
            }
        );

        Some(self_type)
    }

    fn register_block_type(&self, ctx: &TyContext, name: &'static str, type_: Type, loc: Loc) {
        let _ = self.define_symbol(ctx, name, SymbolKind::TypeAlias, loc);
        let type_id = self.build_type_id(&type_);
        self.type_aliases.be_mut().push(type_id);
    }

    fn register_block_const(&self, ctx: &TyContext, const_def: ConstDef) {
        if const_def.const_name == "_" {
            return;
        }

        let _ = self.define_symbol(ctx, const_def.const_name, SymbolKind::Const, const_def.loc);
        self.constants.be_mut().push(const_def);
    }

    fn type_code_expr_and_load(&self, ctx: &TyContext, expr: &CodeExpr) -> Result<&Type, Error> {
        self.type_code_expr(ctx, expr)?;
        let Some(type_id) = self.load_type_id(ctx, expr) else {
            return Err(Error {
                message: format!(
                    "Compiler bug. Expression should only return ok if it stored a type."
                ),
                loc: expr.loc(),
            });
        };
        Ok(&self.registry.types[type_id])
    }

    fn build_type(&self, ctx: &TyContext, type_expr: &TypeExpr) -> Result<Type, Error> {
        return self.build_type_check_ref(ctx, type_expr, true, &Loc::internal());
    }

    // builds a type, asserting that it doesn't have infinite size
    fn build_type_check_ref(
        &self,
        ctx: &TyContext,
        type_expr: &TypeExpr,
        is_referenced: bool,
        loc: &Loc,
    ) -> Result<Type, Error> {
        match type_expr {
            TypeExpr::Named(TypeExprNamed { name }) => {
                let lo_type = self.get_type_or_err(ctx, &name.repr, &name.loc)?;
                if let Type::StructInstance { struct_index } = &lo_type {
                    let struct_def = &self.structs[*struct_index];
                    if !is_referenced && !struct_def.fully_defined {
                        return Err(Error {
                            message: format!(
                                "Cannot use partially defined struct '{}' here",
                                struct_def.struct_name
                            ),
                            loc: *loc,
                        });
                    }
                }
                Ok(lo_type)
            }
            TypeExpr::Pointer(TypeExprPointer { pointee, loc: _ }) => {
                let pointee = Box::new(self.build_type_check_ref(ctx, &pointee, true, loc)?);

                Ok(Type::Pointer { pointee })
            }
            TypeExpr::SequencePointer(TypeExprSequencePointer { pointee, loc: _ }) => {
                let pointee = Box::new(self.build_type_check_ref(ctx, &pointee, true, loc)?);

                Ok(Type::SequencePointer { pointee })
            }
            TypeExpr::Container(TypeExprContainer {
                container,
                items,
                loc: _,
            }) => {
                if let TypeExpr::Named(ident) = &**container
                    && ident.name.repr == "Result"
                {
                    if items.len() != 2 {
                        return Err(Error {
                            message: format!(
                                "Expected exactly 2 type arguments, {} was found",
                                items.len()
                            ),
                            loc: ident.name.loc,
                        });
                    }

                    let ok = Box::new(self.build_type_check_ref(ctx, &items[0], false, loc)?);
                    let err = Box::new(self.build_type_check_ref(ctx, &items[1], false, loc)?);

                    return Ok(Type::Result(ResultType { ok, err }));
                }

                if let TypeExpr::Named(named) = &**container
                    && named.name.repr == "typeof"
                {
                    if items.len() != 1 {
                        return Err(Error {
                            message: format!(
                                "Expected exactly 1 type arguments, {} was found",
                                items.len()
                            ),
                            loc: named.name.loc,
                        });
                    }

                    let TypeExpr::Named(named) = &items[0] else {
                        return Err(Error {
                            message: format!("Symbol expected"),
                            loc: *items[0].loc(),
                        });
                    };

                    let var_type = self.get_variable_type_id(ctx, &named.name)?;
                    return Ok(self.registry.types[var_type].clone());
                }

                if let TypeExpr::Named(named) = &**container
                    && named.name.repr == "itemof"
                {
                    if items.len() != 1 {
                        return Err(Error {
                            message: format!(
                                "Expected exactly 1 type arguments, {} was found",
                                items.len()
                            ),
                            loc: named.name.loc,
                        });
                    }

                    let container = self.build_type_check_ref(ctx, &items[0], true, loc)?;
                    let container = container.deref_rec();

                    let Type::Container(ContainerType {
                        container: _,
                        items,
                    }) = container
                    else {
                        return Err(Error {
                            message: format!("Expected container type"),
                            loc: *items[0].loc(),
                        });
                    };

                    return Ok(items[0].clone());
                }

                let container = self.build_type_check_ref(ctx, container, is_referenced, loc)?;

                let mut type_items = Vec::new();
                for item in items {
                    type_items.push(self.build_type_check_ref(ctx, item, true, loc)?);
                }

                Ok(Type::Container(ContainerType {
                    container: Box::new(container),
                    items: type_items,
                }))
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
            self.report_error(&Error {
                message: format!(
                    "Cannot redefine {}, previously defined at {}",
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

    fn child_ctx(&self, parent: &TyContext, scope_kind: ScopeKind) -> TyContext {
        TyContext {
            module_id: parent.module_id,
            scope_id: self.new_scope(scope_kind, Some(parent.scope_id)),
            fn_index: parent.fn_index,
            expr_info_offset: parent.expr_info_offset,
        }
    }

    fn new_scope(&self, kind: ScopeKind, parent_id: Option<TyScopeId>) -> usize {
        self.scopes.be_mut().push(TyScope {
            parent_id,
            id: self.scopes.len(),
            kind,
            symbols: Vec::new(),
        });

        self.scopes.len() - 1
    }

    fn load_type_id(&self, ctx: &TyContext, expr: &CodeExpr) -> Option<TypeId> {
        let info_id = self.registry.expr_info[ctx.expr_info_offset + expr.id()];
        if info_id == EXPR_INFO_INVALID {
            return None;
        }

        if let CodeExpr::InlineFnCall(_) | CodeExpr::InlineMethodCall(_) = expr {
            return Some(self.registry.inline_fn_call_info[info_id].return_type_id);
        }

        return Some(info_id);
    }

    fn store_type(&self, ctx: &TyContext, expr_id: ExprId, type_: &Type) {
        let type_id = self.build_type_id(type_);
        self.store_expr_info(ctx, expr_id, type_id)
    }

    fn store_expr_info(&self, ctx: &TyContext, expr_id: ExprId, expr_info_id: ExprInfo) {
        let absolute_expr_id = ctx.expr_info_offset + expr_id;
        // TODO: remove if no longer needed for debugging
        // stderr_writeln(format!(
        //     "{}store({} + {}, {})",
        //     if ctx.expr_info_offset == 0 { "" } else { "  " },
        //     ctx.expr_info_offset,
        //     expr_id,
        //     expr_info_id
        // ));
        self.registry.be_mut().expr_info[absolute_expr_id] = expr_info_id;
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

    fn report_if_err(&self, res: Result<(), Error>) {
        if let Err(err) = res {
            self.report_error(&err);
        }
    }

    // TODO: remove tag after migration
    fn report_error(&self, err: &Error) {
        let marked_error = Error {
            message: format!("(ty) {}", err.message),
            loc: err.loc.clone(),
        };
        self.reporter.error(&marked_error);
    }
}

pub fn is_64_bit_int_tag(registry: &Registry, tag_type: &Type, loc: &Loc) -> Result<bool, Error> {
    match tag_type {
        Type::U64 | Type::I64 => Ok(true),
        Type::U8 | Type::I8 | Type::U16 | Type::I16 | Type::U32 | Type::I32 => Ok(false),
        other => Err(Error {
            message: format!("{} is not a valid int tag", TypeFmt(&registry, other)),
            loc: *loc,
        }),
    }
}

pub fn get_infer_type_name(fn_param: &FnParam) -> Result<Option<&'static str>, Error> {
    let FnParamType::Type {
        expr: TypeExpr::Container(container),
    } = &fn_param.param_type
    else {
        return Ok(None);
    };

    let TypeExpr::Named(named) = &*container.container else {
        return Ok(None);
    };

    if named.name.repr != "infer" {
        return Ok(None);
    }

    if container.items.len() != 1 {
        return Err(Error {
            message: format!("Invalid `infer` call, expected 1 named type argument"),
            loc: container.loc,
        });
    }

    let TypeExpr::Named(named) = &container.items[0] else {
        return Err(Error {
            message: format!("Invalid `infer` call, expected 1 named type argument"),
            loc: container.loc,
        });
    };

    Ok(Some(named.name.repr))
}

pub fn is_types_compatible(slots: &Vec<Type>, values: &Vec<Type>) -> bool {
    if slots.len() != values.len() {
        return false;
    }

    for i in 0..slots.len() {
        if !is_type_compatible(&slots[i], &values[i]) {
            return false;
        }
    }

    true
}

pub fn is_type_compatible(slot: &Type, value: &Type) -> bool {
    if let Type::Pointer { pointee } = slot {
        if *value == Type::Null {
            return true;
        }

        if **pointee == Type::Void {
            if let Type::Pointer { pointee: _ } = value {
                return true;
            }

            if let Type::SequencePointer { pointee: _ } = value {
                return true;
            }
        }

        if let Type::Pointer {
            pointee: value_pointee,
        } = value
        {
            return is_type_compatible(pointee, value_pointee);
        }
    }

    if let Type::Container(ContainerType { container, items }) = value {
        if let Type::Container(ContainerType {
            container: slot_container,
            items: slot_items,
        }) = slot
        {
            return is_type_compatible(slot_container, container)
                && is_types_compatible(slot_items, items);
        }

        // TODO: allow this for self arguments only
        return is_type_compatible(slot, container);
    }

    if *value == Type::Never {
        return true;
    }

    slot == value
}
