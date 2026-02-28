use crate::{ast::*, common::*, lexer::*, registry::*};

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Type {
    Never,
    Null,
    Void,
    Bool,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    Pointer(PointerType),
    Struct { struct_index: usize },
    Enum { enum_index: usize },
    Seg(SegType),
    Result(ResultType),
    Container(ContainerType),
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct PointerType {
    pub pointee: TypeId,
    pub is_sequence: bool,
    pub is_nullable: bool,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct SegType {
    pub item: TypeId,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct ResultType {
    pub ok: TypeId,
    pub err: TypeId,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct ContainerType {
    pub container: TypeId,
    pub items: Vec<TypeId>,
}

impl Type {
    pub fn to_str(&self) -> Option<&'static str> {
        Some(match self {
            Type::Never => "never",
            Type::Null => "null",
            Type::Void => "void",
            Type::Bool => "bool",
            Type::U8 => "u8",
            Type::I8 => "i8",
            Type::U16 => "u16",
            Type::I16 => "i16",
            Type::U32 => "u32",
            Type::I32 => "i32",
            Type::F32 => "f32",
            Type::U64 => "u64",
            Type::I64 => "i64",
            Type::F64 => "f64",
            _ => return None,
        })
    }
}

#[derive(Clone)]
pub struct TypeLayout {
    pub primitives_count: u32,
    pub primitives: Option<Vec<Type>>,
    pub byte_size: u32,
    pub alignment: u32,
}

impl TypeLayout {
    pub fn new() -> Self {
        Self {
            primitives_count: 0,
            primitives: None,
            byte_size: 0,
            alignment: 0,
        }
    }
}

pub type ExprInfo = usize;
pub const EXPR_INFO_INVALID: ExprInfo = usize::MAX;

pub type TypeId = usize;

type TyContextRef = UBRef<TyContext>;

struct TyContext {
    module_id: usize,
    id: usize,
    parent: Option<TyContextRef>,
    kind: ScopeKind,
    fn_index: Option<usize>,
    expr_id_offset: usize,
    symbols: Vec<TySymbol>,
}

#[derive(Clone, Debug, PartialEq, Copy)]
enum TySymbolKind {
    TypeAlias,
    Struct,
    Enum,

    Local,
    Global,
    Const,

    InlineFn,
    Function,
    EnumConstructor,
}

#[derive(Clone)]
struct TySymbol {
    ctx_id: usize,
    name: &'static str,
    kind: TySymbolKind,
    col_index: usize,
    loc: Loc,
}

struct TyModuleInfo {
    ctx: TyContextRef,
}

#[derive(Default)]
pub struct Typer {
    pub registry: UBRef<Registry>,
    pub reporter: UBRef<Reporter>,

    contexts: UBCell<StableVec<TyContext>>,
    module_info: UBCell<Vec<TyModuleInfo>>,
    type_lookup: UBCell<BTreeMap<Type, TypeId>>,

    type_aliases: UBCell<Vec<TypeId>>, // indexed by `TySymbol::col_index` when `kind = TypeAlias`
    locals_types: Vec<TypeId>,         // indexed by `TySymbol::col_index` when `kind = Local`

    first_string_usage: UBCell<Option<Loc>>,
    allocated_strings: Vec<String>, // storage for all allocated `String` objects
}

impl Typer {
    pub fn new(registry: &mut Registry) -> Self {
        let mut it = Self::default();
        it.registry = UBRef::new(&mut *registry);
        it.reporter = UBRef::new(&mut *registry.reporter);
        it.init();
        it
    }

    pub fn init(&mut self) {
        let mut global_ctx = self.contexts.push(TyContext {
            id: 0,
            parent: None,
            module_id: usize::MAX, // global scope has no module
            fn_index: None,
            expr_id_offset: 0,
            kind: ScopeKind::Global,
            symbols: Vec::new(),
        });

        self.module_info.reserve(self.registry.modules.len());
        for module in &self.registry.modules {
            let mut module_ctx = self.child_ctx(global_ctx, ScopeKind::Global);
            module_ctx.module_id = module.id;

            self.module_info
                .be_mut()
                .push(TyModuleInfo { ctx: module_ctx });
        }

        self.extend_expr_info_storage(self.registry.expr_id_count);

        self.add_builtin_type(&mut global_ctx, Type::Never);
        self.add_builtin_type(&mut global_ctx, Type::Void);
        self.add_builtin_type(&mut global_ctx, Type::Bool);
        self.add_builtin_type(&mut global_ctx, Type::U8);
        self.add_builtin_type(&mut global_ctx, Type::I8);
        self.add_builtin_type(&mut global_ctx, Type::U16);
        self.add_builtin_type(&mut global_ctx, Type::I16);
        self.add_builtin_type(&mut global_ctx, Type::U32);
        self.add_builtin_type(&mut global_ctx, Type::I32);
        self.add_builtin_type(&mut global_ctx, Type::F32);
        self.add_builtin_type(&mut global_ctx, Type::U64);
        self.add_builtin_type(&mut global_ctx, Type::I64);
        self.add_builtin_type(&mut global_ctx, Type::F64);
    }

    pub fn type_all(&mut self) {
        for module in self.module_info.be_mut().relax_mut() {
            self.pass_collect_own_symbols(module.ctx);
        }

        for module in self.module_info.be_mut().relax_mut() {
            self.pass_collect_included_symbols(module.ctx);
        }

        for module in self.module_info.be_mut().relax_mut() {
            self.pass_build_type_def_values(module.ctx);
        }

        self.pass_get_str_literal_type();

        for module in self.module_info.be_mut().relax_mut() {
            self.pass_process_top_level_exprs(module.ctx);
        }

        self.pass_type_fns();

        self.run_delayed_actions();
    }

    fn run_delayed_actions(&self) {
        for module in &self.registry.modules {
            for expr in &*module.parser.ast {
                if let TopLevelExpr::Intrinsic(intrinsic) = expr
                    && intrinsic.fn_name.repr == "inspect_stats"
                {
                    if !self.reporter.in_inspection_mode {
                        return;
                    }

                    let mut lines = 0;
                    for file in &self.registry.files {
                        lines += file
                            .source
                            .as_bytes()
                            .iter()
                            .filter(|&&b| b == b'\n')
                            .count()
                    }

                    let mut msg = String::new();
                    write!(&mut msg, "LOC: {}\n", lines).unwrap();
                    write!(
                        &mut msg,
                        "expr count (original): {}\n",
                        self.registry.expr_id_count
                    )
                    .unwrap();
                    write!(
                        &mut msg,
                        "expr count (after expansion): {}\n",
                        self.registry.expr_info.len()
                    )
                    .unwrap();
                    write!(&mut msg, "unique types: {}\n", self.registry.types.len()).unwrap();

                    self.reporter.print_inspection(InspectInfo {
                        message: msg,
                        loc: intrinsic.fn_name.loc,
                        linked_loc: None,
                    });
                }
            }
        }

        if let Some(string_usage_loc) = *self.first_string_usage
            && self.registry.memory.is_none()
            // TODO: find a way to also report this in inspection mode
            && !self.reporter.in_inspection_mode
        {
            self.report_error(Error {
                message: format!("Cannot use strings with no memory defined"),
                loc: string_usage_loc,
            });
        }
    }

    fn pass_collect_own_symbols(&mut self, ctx: TyContextRef) {
        let module = self.registry.modules[ctx.module_id].relax_mut();

        for expr in &*module.parser.ast {
            match expr {
                TopLevelExpr::Intrinsic(_) => {} // skip, not interested

                TopLevelExpr::Type(type_def) => {
                    match &type_def.value {
                        TypeDefValue::Struct { .. } => {
                            let _ = self.define_symbol(
                                ctx,
                                type_def.name.repr,
                                TySymbolKind::Struct,
                                type_def.name.loc,
                            );

                            self.registry.structs.push(StructDef {
                                struct_name: type_def.name.repr,
                                fields: Vec::new(),
                                fully_defined: false,
                            });
                        }
                        TypeDefValue::Enum {
                            variant_type: variant_type_in_decl,
                            variants,
                        } => {
                            if !self.define_symbol(
                                ctx,
                                type_def.name.repr,
                                TySymbolKind::Enum,
                                type_def.name.loc,
                            ) {
                                continue;
                            };

                            let mut variant_type = &Type::Void;
                            if let Some(type_expr) = &variant_type_in_decl {
                                match self.build_type(ctx, type_expr) {
                                    Ok(type_id) => variant_type = self.get_type(type_id).relax(),
                                    Err(err) => {
                                        variant_type = &Type::Never;
                                        self.report_error(err);
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

                                let _ = self.define_symbol(
                                    ctx,
                                    constructor_name,
                                    TySymbolKind::EnumConstructor,
                                    variant.variant_name.loc,
                                );

                                self.registry.enum_ctors.push(EnumConstructor {
                                    enum_index,
                                    variant_index,
                                    loc: variant.variant_name.loc,
                                });
                            }
                        }
                        TypeDefValue::Alias(_) => {
                            let _ = self.define_symbol(
                                ctx,
                                type_def.name.repr,
                                TySymbolKind::TypeAlias,
                                type_def.name.loc,
                            );

                            self.relax_mut()
                                .type_aliases
                                .push(self.intern_type(&Type::Never)); // placeholder
                        }
                    }
                }
                TopLevelExpr::Fn(fn_def) => {
                    if fn_def.is_inline {
                        let _ = self.define_symbol(
                            ctx,
                            fn_def.decl.fn_name.repr,
                            TySymbolKind::InlineFn,
                            fn_def.decl.fn_name.loc,
                        );

                        self.registry.inline_fns.push(fn_def.relax());
                        continue;
                    }

                    let _ = self.define_symbol(
                        ctx,
                        fn_def.decl.fn_name.repr,
                        TySymbolKind::Function,
                        fn_def.decl.fn_name.loc,
                    );

                    match &fn_def.value {
                        FnExprValue::Body(body) => {
                            let mut exported_as = Vec::new();
                            if fn_def.exported {
                                exported_as.push(String::from(fn_def.decl.fn_name.repr));
                            }

                            self.registry.be_mut().functions.push(FnInfo {
                                name: fn_def.decl.fn_name.repr,
                                type_: FnType {
                                    inputs: Vec::new(),
                                    output: Type::Void,
                                },
                                params: Vec::new(),
                                source: FnSource::Guest {
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

                            self.registry.functions.push(FnInfo {
                                name: fn_def.decl.fn_name.repr,
                                type_: FnType {
                                    inputs: Vec::new(),
                                    output: Type::Void,
                                },
                                params: Vec::new(),
                                source: FnSource::Host {
                                    module_name: module_name.clone(),
                                    external_fn_name,
                                },
                                exported_as: Vec::new(),
                                wasm_fn_index: u32::MAX, // placeholder
                                definition_loc: fn_def.decl.fn_name.loc,
                            });
                        }
                    }
                }
                TopLevelExpr::Let(let_expr) => {
                    if let_expr.is_inline {
                        let _ = self.define_symbol(
                            ctx,
                            let_expr.name.repr,
                            TySymbolKind::Const,
                            let_expr.name.loc,
                        );

                        self.registry.constants.be_mut().push(ConstDef {
                            type_id: self.intern_type(&Type::Never), // placeholder
                            expr: UBRef::new(&*let_expr.value),
                            inner_expr_id_offset: ctx.expr_id_offset,
                        });
                        continue;
                    }

                    let _ = self.define_symbol(
                        ctx,
                        let_expr.name.repr,
                        TySymbolKind::Global,
                        let_expr.name.loc,
                    );

                    self.relax_mut().registry.globals.push(GlobalDef {
                        module_id: module.id,
                        value: let_expr.value.relax(),
                        type_id: self.intern_type(&Type::Never), // placeholder
                        wasm_global_index: 0,                    // placeholder
                    });
                }
            }
        }
    }

    fn pass_collect_included_symbols(&mut self, ctx: TyContextRef) {
        self.collect_included_symbols(ctx, ctx, &mut String::from(""), true);
    }

    fn collect_included_symbols(
        &mut self,
        includer: TyContextRef,
        includee: TyContextRef,
        prefix: &mut String,
        force_go_deeper: bool,
    ) {
        if includer.module_id != includee.module_id {
            for symbol in includee.symbols.relax() {
                let mut included_symbol = symbol.clone();
                if prefix.len() != 0 {
                    included_symbol.name = self.alloc_str(format!("{}{}", prefix, symbol.name));
                }

                // TODO: optimize this
                for existing_symbol in &includer.symbols {
                    if existing_symbol.ctx_id != included_symbol.ctx_id
                        && existing_symbol.name == included_symbol.name
                    {
                        self.report_error(Error {
                            message: format!(
                                "Cannot redefine {}, already defined at {}",
                                existing_symbol.name,
                                included_symbol.loc.to_string(&self.registry)
                            ),
                            loc: existing_symbol.loc,
                        });
                    }
                }

                includer.be_mut().symbols.push(included_symbol)
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
            self.collect_included_symbols(includer, sub_includee.ctx, prefix, false);
            prefix.truncate(original_prefix_len);
        }
    }

    fn pass_build_type_def_values(&mut self, ctx: TyContextRef) {
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
                                        self.report_error(Error {
                                        message: format!(
                                            "Cannot redefine struct field '{}', previously defined at {}",
                                            field.field_name.repr,
                                            existing_field.loc.to_string(&self.registry),
                                        ),
                                        loc: field.field_name.loc,
                                    });
                                        continue 'fields;
                                    }
                                }

                                let field_index = struct_primitives_count;
                                let field_type_id = self.build_type_(
                                    ctx,
                                    &field.field_type,
                                    false,
                                    &field.field_type.loc(),
                                );
                                let field_type_id = catch!(field_type_id, err, {
                                    self.report_error(err);
                                    continue 'exprs;
                                });
                                let field_type = self.get_type(field_type_id);
                                let mut field_layout = TypeLayout::new();
                                get_type_layout(&self.registry, field_type, &mut field_layout);

                                struct_aligment = u32::max(struct_aligment, field_layout.alignment);
                                struct_primitives_count += field_layout.primitives_count;

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

                            let symbol = self.get_symbol(ctx, type_def.name.repr).unwrap().relax();

                            let TySymbolKind::Struct = symbol.kind else {
                                continue;
                            };

                            let struct_def = &mut self.registry.structs[symbol.col_index];
                            struct_def.fields.append(&mut struct_fields);
                            struct_def.fully_defined = true;
                        }
                        TypeDefValue::Enum {
                            variant_type: _,
                            variants,
                        } => {
                            let symbol = self.get_symbol(ctx, &type_def.name.repr).unwrap().relax();
                            let TySymbolKind::Enum = symbol.kind else {
                                continue;
                            };
                            let enum_def = self.registry.enums[symbol.col_index].relax_mut();

                            'variants: for variant in variants.iter() {
                                for existing_variant in &enum_def.variants {
                                    if existing_variant.variant_name == variant.variant_name.repr {
                                        self.report_error(Error {
                                            message: format!(
                                                "Cannot redefine enum variant '{}', previously defined at {}",
                                                variant.variant_name.repr,
                                                existing_variant.loc.to_string(&self.registry),
                                            ),
                                            loc: variant.variant_name.loc,
                                        });
                                        continue 'variants;
                                    }
                                }

                                let variant_type_id;
                                if let Some(variant_type_expr) = &variant.variant_type {
                                    match self.build_type(ctx, variant_type_expr) {
                                        Ok(t) => variant_type_id = t,
                                        Err(err) => {
                                            self.report_error(err);
                                            variant_type_id = self.intern_type(&Type::Never);
                                        }
                                    }
                                } else {
                                    variant_type_id = self.intern_type(&Type::Void);
                                }

                                if !is_type_compatible(
                                    &self.registry,
                                    &enum_def.variant_type,
                                    self.get_type(variant_type_id),
                                ) {
                                    self.report_error(Error {
                                        message: format!(
                                            "Enum variant is not compatible with {}",
                                            self.registry.fmt(&enum_def.variant_type)
                                        ),
                                        loc: variant.variant_name.loc,
                                    });
                                }

                                enum_def.variants.push(EnumVariant {
                                    variant_name: variant.variant_name.repr,
                                    variant_type_id,
                                    loc: variant.variant_name.loc,
                                });
                            }
                        }
                        TypeDefValue::Alias(type_expr) => {
                            let type_id = self.build_type(ctx, &type_expr);
                            let type_id = catch!(type_id, err, {
                                self.report_error(err);
                                continue;
                            });

                            let symbol = self.get_symbol(ctx, &type_def.name.repr).unwrap().relax();
                            let TySymbolKind::TypeAlias = symbol.kind else {
                                continue;
                            };

                            self.type_aliases[symbol.col_index] = type_id;
                        }
                    }
                }
                _ => {} // skip, not interested
            }
        }
    }

    fn pass_get_str_literal_type(&mut self) {
        // fallback
        self.registry.str_literal_type = Some(Type::Seg(SegType {
            item: self.intern_type(&Type::U8),
        }));

        let mut str_literal_def: Option<&TypeExpr> = None;

        for module_info in self.module_info.be_mut().relax_mut() {
            let module = self.registry.modules[module_info.ctx.module_id].relax_mut();
            for expr in &*module.parser.ast {
                let TopLevelExpr::Intrinsic(intrinsic) = expr else {
                    continue;
                };

                if intrinsic.fn_name.repr != "str_literal_type" {
                    continue;
                }

                if intrinsic.type_args.len() != 1 || intrinsic.args.items.len() != 0 {
                    self.report_error(Error {
                        message: format!(
                            "Invalid call, expected signature: @<T>{}()",
                            intrinsic.fn_name.repr
                        ),
                        loc: intrinsic.fn_name.loc,
                    });
                    continue;
                }

                if let Some(str_literal_def) = &str_literal_def {
                    self.report_error(Error {
                        message: format!(
                            "Cannot redefine str literal type, first defined at {}",
                            str_literal_def.loc().to_string(&self.registry)
                        ),
                        loc: intrinsic.fn_name.loc,
                    });
                    continue;
                }
                str_literal_def = Some(intrinsic.type_args[0].relax());

                match self.build_type(module_info.ctx, str_literal_def.unwrap()) {
                    Err(err) => self.report_error(err),
                    Ok(type_id) => {
                        self.registry.str_literal_type = Some(self.get_type(type_id).clone());
                    }
                }
            }
        }
    }

    fn pass_process_top_level_exprs(&mut self, ctx: TyContextRef) {
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
                        .get_symbol(ctx, &fn_def.decl.fn_name.repr)
                        .unwrap()
                        .relax();
                    let TySymbolKind::Function = symbol.kind else {
                        continue;
                    };

                    let fn_info = self.registry.functions[symbol.col_index].relax_mut();

                    let self_type_id =
                        self.get_fn_self_type(ctx, &fn_def.decl.fn_name, &fn_def.decl.params);

                    if let Some(return_type) = &fn_def.decl.return_type {
                        let type_id = catch!(self.build_type(ctx, return_type), err, {
                            self.report_error(err);
                            continue;
                        });
                        fn_info.type_.output = self.get_type(type_id).clone();
                    }

                    for fn_param in &fn_def.decl.params {
                        if fn_param.param_name.repr == "" {
                            self.report_error(Error {
                                message: format!("Invalid fn param name `_` is not allowed here"),
                                loc: fn_param.param_name.loc,
                            });
                        }

                        let param_type_id =
                            self.get_fn_param_type(ctx, fn_param, self_type_id, false);
                        let param_type_id = catch!(param_type_id, err, {
                            self.report_error(err);
                            continue;
                        });

                        fn_info.type_.inputs.push(param_type_id);

                        fn_info.params.push(FnParameter {
                            param_name: fn_param.param_name.repr,
                            param_type: self.get_type(param_type_id).clone(),
                            loc: fn_param.param_name.loc,
                        });
                    }
                }
                TopLevelExpr::Let(let_expr) => {
                    let symbol = self.get_symbol(ctx, &let_expr.name.repr).unwrap().relax();

                    // TODO: check for valid const expression (in case of global)
                    let value_type_id =
                        catch!(self.type_code_expr_and_load(ctx, &let_expr.value), err, {
                            self.report_error(err);
                            continue;
                        });

                    if let_expr.is_inline {
                        let const_ = self.registry.constants[symbol.col_index].relax_mut();
                        const_.type_id = value_type_id;

                        if self.reporter.in_inspection_mode {
                            self.reporter.print_inspection(InspectInfo {
                                message: format!(
                                    "inline let {}: {}",
                                    &let_expr.name.repr,
                                    self.registry.fmt(self.get_type(value_type_id))
                                ),
                                loc: let_expr.name.loc,
                                linked_loc: None,
                            });
                        }

                        continue;
                    }

                    let TySymbolKind::Global = symbol.kind else {
                        continue;
                    };

                    let global = self.registry.globals[symbol.col_index].relax_mut();
                    global.type_id = value_type_id;

                    if self.reporter.in_inspection_mode {
                        let global_name = &let_expr.name.repr;

                        self.reporter.print_inspection(InspectInfo {
                            message: format!(
                                "let {global_name}: {}",
                                self.registry.fmt(self.get_type(value_type_id))
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

                    if intrinsic.fn_name.repr == "str_literal_type" {
                        // skip, was processed in `pass_get_str_literal_type`
                        continue;
                    }

                    if intrinsic.fn_name.repr == "use_memory" {
                        if let Some(existing_memory) = &self.registry.memory {
                            self.report_error(Error {
                                message: format!(
                                    "Cannot redefine memory, first defined at {}",
                                    existing_memory.loc.to_string(&self.registry)
                                ),
                                loc: intrinsic.loc,
                            });
                            continue;
                        }

                        if intrinsic.type_args.len() != 0 {
                            self.report_error(bad_signature(&intrinsic.fn_name));
                        }

                        let mut memory = MemoryInfo {
                            min_pages: None,
                            data_start: None,
                            exported: false,
                            imported_from: None,
                            loc: intrinsic.loc,
                        };

                        for arg in &intrinsic.args.items {
                            if let CodeExpr::Ident(ident) = arg
                                && ident.repr == "exported"
                            {
                                memory.exported = true;
                                continue;
                            }

                            let CodeExpr::Assign(AssignExpr {
                                lhs, rhs: value, ..
                            }) = arg
                            else {
                                self.report_error(bad_signature(&intrinsic.fn_name));
                                continue;
                            };

                            let CodeExpr::Ident(key) = &**lhs else {
                                self.report_error(bad_signature(&intrinsic.fn_name));
                                continue;
                            };

                            if key.repr == "data_start" {
                                let Some(CodeExpr::IntLiteral(IntLiteralExpr {
                                    value,
                                    tag: None,
                                    ..
                                })) = self.get_const_value(ctx, value)
                                else {
                                    self.report_error(bad_signature(&intrinsic.fn_name));
                                    continue;
                                };

                                memory.data_start = Some(*value as u32);
                                *self.registry.data_size.be_mut() = *value as u32;

                                continue;
                            }

                            if key.repr == "min_pages" {
                                let Some(CodeExpr::IntLiteral(IntLiteralExpr {
                                    value,
                                    tag: None,
                                    ..
                                })) = self.get_const_value(ctx, value)
                                else {
                                    self.report_error(bad_signature(&intrinsic.fn_name));
                                    continue;
                                };

                                memory.min_pages = Some(*value as u32);
                                continue;
                            }

                            if key.repr == "import_from" {
                                let CodeExpr::StringLiteral(str) = &**value else {
                                    self.report_error(bad_signature(&intrinsic.fn_name));
                                    continue;
                                };

                                memory.imported_from = Some(str.value.clone());
                                continue;
                            }

                            self.report_error(bad_signature(&intrinsic.fn_name));
                        }

                        self.registry.memory = Some(memory);

                        continue;

                        fn bad_signature(fn_name: &IdentExpr) -> Error {
                            Error {
                                message: format!(
                                    "Invalid call, expected signature: @{}([min_pages = <u32>, data_start = <u32>, exported, import_from = <str-literal>])",
                                    fn_name.repr
                                ),
                                loc: fn_name.loc,
                            }
                        }
                    }

                    if intrinsic.fn_name.repr == "export_existing" {
                        let mut from_root = false;
                        let mut in_name = None;
                        let mut out_name = None;

                        if intrinsic.type_args.len() != 0 {
                            self.report_error(bad_signature(&intrinsic.fn_name));
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
                                self.report_error(bad_signature(&intrinsic.fn_name));
                                continue;
                            };

                            let CodeExpr::Ident(key) = &**lhs else {
                                self.report_error(bad_signature(&intrinsic.fn_name));
                                continue;
                            };

                            if key.repr == "out" {
                                let CodeExpr::StringLiteral(value) = &**rhs else {
                                    self.report_error(bad_signature(&intrinsic.fn_name));
                                    continue;
                                };

                                out_name = Some(value.relax());
                                continue;
                            }

                            self.report_error(bad_signature(&intrinsic.fn_name));
                            continue;
                        }

                        let Some(in_name) = in_name else {
                            self.report_error(bad_signature(&intrinsic.fn_name));
                            continue;
                        };

                        let mut target_ctx = ctx;
                        if from_root {
                            target_ctx = self.module_info.last().unwrap().ctx;
                        }

                        let Ok(fn_index) =
                            self.get_fn_index_for_call(target_ctx, &in_name.repr, &in_name.loc)
                        else {
                            // don't report any errors if function to be exported
                            //   could be in another module but only one module is inspected
                            if from_root && self.reporter.in_inspection_mode {
                                continue;
                            }

                            self.report_error(Error {
                                message: format!("Can't export unknown symbol {}", in_name.repr),
                                loc: in_name.loc,
                            });
                            continue;
                        };
                        let fn_info = &self.registry.functions[fn_index];

                        if self.reporter.in_inspection_mode {
                            self.reporter.print_inspection(InspectInfo {
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

                    if intrinsic.fn_name.repr == "inspect_stats" {
                        // skip, will be processed in `run_delayed_actions`
                        continue;
                    }

                    self.report_error(Error {
                        message: format!("Unknown intrinsic: {}", intrinsic.fn_name.repr),
                        loc: intrinsic.loc,
                    });
                }
            }
        }
    }

    fn pass_type_fns(&mut self) {
        for fn_def in &self.registry.functions {
            let FnSource::Guest {
                module_id,
                lo_fn_index,
                body,
            } = fn_def.source
            else {
                continue;
            };

            let module_ctx = self.module_info[module_id].ctx;

            let ctx = self.child_ctx(module_ctx, ScopeKind::Function);
            ctx.be_mut().fn_index = Some(lo_fn_index);

            for param in &fn_def.params {
                self.define_local(
                    ctx,
                    &param.param_name,
                    self.intern_type(&param.param_type),
                    param.loc,
                );
            }

            self.type_code_block(ctx, &body, true);
        }
    }

    fn type_code_block(&self, ctx: TyContextRef, block: &CodeBlock, void_only: bool) -> Type {
        let ctx = self.child_ctx(ctx, ScopeKind::Block);

        let mut diverges = false;
        let mut diverges_naturally = false;

        for expr in &block.exprs {
            if diverges {
                self.reporter.warning(&Error {
                    message: format!("Unreachable expression"),
                    loc: expr.loc(),
                });
            }

            self.report_if_err(self.type_code_expr(ctx, expr));
            let Some(type_id) = self.load_type(ctx, expr) else {
                continue;
            };

            let expr_type = self.get_type(type_id);

            if count_primitive_components(&self.registry, expr_type) > 0 && void_only {
                self.report_error(Error {
                    message: format!(
                        "Non void expression in block. Use `let _ = <expr>` to ignore expression result."
                    ),
                    loc: expr.loc(),
                });
            }

            if *expr_type == Type::Never {
                diverges = true;
                diverges_naturally = diverges_naturally || is_naturally_divergent(expr);
            }
        }

        if !diverges_naturally && !diverges && ctx.kind == ScopeKind::Function {
            let fn_info = &self.registry.functions[ctx.fn_index.unwrap()];
            if fn_info.type_.output != Type::Void {
                self.report_error(Error {
                    // error message stolen from clang
                    message: format!("Control reaches end of non-void function"),
                    loc: fn_info.definition_loc,
                });
            }
        }

        if diverges {
            return Type::Never;
        }

        // TODO: store type in code_block.id?
        Type::Void
    }

    fn type_code_expr(&self, ctx: TyContextRef, expr: &CodeExpr) -> Result<(), Error> {
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

                let tag_type_id =
                    catch!(self.get_type_id_or_err(ctx, tag, &int_literal.loc), err, {
                        self.report_error(err);
                        self.store_type(ctx, int_literal.id, &Type::U32);
                        return Ok(());
                    });

                let tag_type = self.get_type(tag_type_id);
                if let Err(err) = is_wide_int_tag(&self.registry, tag_type, &int_literal.loc) {
                    self.report_error(err);
                    self.store_type(ctx, int_literal.id, &Type::U32);
                    return Ok(());
                }

                self.store_expr_info(ctx, int_literal.id, tag_type_id);

                return Ok(());
            }
            CodeExpr::StringLiteral(str_literal) => {
                if let None = *self.first_string_usage {
                    *self.first_string_usage.be_mut() = Some(*&str_literal.loc);
                }

                self.store_type(
                    ctx,
                    str_literal.id,
                    self.registry.str_literal_type.as_ref().unwrap(),
                );
                return Ok(());
            }
            CodeExpr::StructLiteral(struct_literal) => {
                for field in &struct_literal.body.fields {
                    self.report_if_err(self.type_code_expr(ctx, &field.value));
                }

                let Some(symbol) = self.get_symbol(ctx, &struct_literal.struct_name.repr) else {
                    return Err(Error {
                        message: format!("Unknown struct: {}", struct_literal.struct_name.repr),
                        loc: struct_literal.loc,
                    });
                };
                let TySymbolKind::Struct = symbol.kind else {
                    return Err(Error {
                        message: format!(
                            "Struct name expected, got: {:?} for symbol {}",
                            symbol.kind, struct_literal.struct_name.repr
                        ),
                        loc: struct_literal.loc,
                    });
                };
                let struct_index = symbol.col_index;

                if self.reporter.in_inspection_mode {
                    self.reporter.print_inspection(InspectInfo {
                        message: format!("struct {{ ... }}"),
                        loc: struct_literal.struct_name.loc,
                        linked_loc: Some(symbol.loc),
                    });
                }

                let struct_type = Type::Struct { struct_index };
                self.store_type(ctx, struct_literal.id, &struct_type);

                let struct_def = &self.registry.structs[struct_index].relax();

                for field_index in 0..struct_literal.body.fields.len() {
                    let field_literal = &struct_literal.body.fields[field_index];
                    let Some(struct_field) = struct_def.fields.get(field_index) else {
                        self.report_error(Error {
                            message: format!("Excess field values"),
                            loc: field_literal.loc,
                        });
                        continue;
                    };

                    if &field_literal.key != &struct_field.field_name {
                        self.report_error(Error {
                            message: format!(
                                "Unexpected struct field name, expecting: `{}`",
                                struct_field.field_name
                            ),
                            loc: field_literal.loc,
                        });
                    }

                    let Some(field_value_type_id) = self.load_type(ctx, &field_literal.value)
                    else {
                        continue;
                    };
                    let field_value_type = self.get_type(field_value_type_id);

                    if !is_type_compatible(
                        &self.registry,
                        &struct_field.field_type,
                        &field_value_type,
                    ) {
                        self.report_error(Error {
                            message: format!(
                                "Invalid type for struct field {}.{}, expected: {}, got: {}",
                                struct_literal.struct_name.repr,
                                struct_field.field_name,
                                self.registry.fmt(&struct_field.field_type,),
                                self.registry.fmt(&field_value_type),
                            ),
                            loc: field_literal.value.loc(),
                        });
                    }
                }

                if struct_literal.body.fields.len() < struct_def.fields.len() {
                    let mut missing_fields = Vec::new();
                    for i in struct_literal.body.fields.len()..struct_def.fields.len() {
                        missing_fields.push(&struct_def.fields[i].field_name)
                    }

                    self.report_error(Error {
                        message: format!("Missing struct fields: {}", ListFmt(&missing_fields)),
                        loc: struct_literal.struct_name.loc,
                    });
                }

                return Ok(());
            }
            CodeExpr::ArrayLiteral(array_literal) => {
                // TODO: check items match array item type
                for item_expr in &array_literal.items {
                    self.report_if_err(self.type_code_expr(ctx, &item_expr));
                }

                let item_type_id = self.build_type(ctx, &array_literal.item_type)?;

                let array_type = Type::Pointer(PointerType {
                    pointee: item_type_id,
                    is_sequence: true,
                    is_nullable: false,
                });
                self.store_type(ctx, array_literal.id, &array_type);
                return Ok(());
            }
            CodeExpr::ResultLiteral(result_literal) => {
                let mut value_type = &Type::Void;
                if let Some(value) = &result_literal.value
                    && let Some(value_type_id) =
                        self.report_if_err(self.type_code_expr_and_load(ctx, &value))
                {
                    value_type = self.get_type(value_type_id);
                }

                let result = self.get_result_literal_type(
                    ctx,
                    &result_literal.result_type,
                    &result_literal.loc,
                )?;
                let ok = self.get_type(result.ok);
                let err = self.get_type(result.err);

                if result_literal.is_ok && !is_type_compatible(&self.registry, ok, value_type) {
                    self.report_error(Error {
                        message: format!(
                            "Cannot create result, Ok type mismatch. Got {}, expected: {}",
                            self.registry.fmt(value_type),
                            self.registry.fmt(ok),
                        ),
                        loc: result_literal.loc,
                    });
                }

                if !result_literal.is_ok && !is_type_compatible(&self.registry, err, value_type) {
                    return Err(Error {
                        message: format!(
                            "Cannot create result, Err type mismatch. Got {}, expected: {}",
                            self.registry.fmt(value_type),
                            self.registry.fmt(err),
                        ),
                        loc: result_literal.loc,
                    });
                }

                self.store_type(ctx, result_literal.id, &Type::Result(result));

                return Ok(());
            }
            CodeExpr::Ident(ident) => {
                let var_symbol = self.get_value_info(ctx, &ident)?;
                self.store_expr_info(ctx, ident.id, self.registry.value_info.len());
                self.registry.be_mut().value_info.push(var_symbol);
                return Ok(());
            }
            CodeExpr::Let(let_expr) => {
                self.report_if_err(self.type_code_expr(ctx, &let_expr.value));

                let Some(value_type_id) = self.load_type(ctx, &let_expr.value) else {
                    self.store_type(ctx, let_expr.id, &Type::Void);
                    return Ok(());
                };

                if let Type::Never = self.get_type(value_type_id) {
                    self.store_type(ctx, let_expr.id, &Type::Never);
                } else {
                    self.store_type(ctx, let_expr.id, &Type::Void);
                }

                self.define_local(ctx, let_expr.name.repr, value_type_id, let_expr.name.loc);

                return Ok(());
            }
            CodeExpr::InfixOp(infix_op) => {
                let lhs_type_id_res = self.type_code_expr_and_load(ctx, &infix_op.lhs);
                let rhs_type_id = self.type_code_expr_and_load(ctx, &infix_op.rhs)?;
                let lhs_type_id = lhs_type_id_res?;

                if !is_type_compatible(
                    &self.registry,
                    self.get_type(lhs_type_id),
                    self.get_type(rhs_type_id),
                ) {
                    self.report_error(Error {
                        message: format!(
                            "Operands are not of the same type: lhs = {}, rhs = {}",
                            self.registry.fmt(self.get_type(lhs_type_id)),
                            self.registry.fmt(self.get_type(rhs_type_id)),
                        ),
                        loc: infix_op.op_loc.clone(),
                    });
                }

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
                        self.store_expr_info(ctx, infix_op.id, lhs_type_id);
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
                let expr_type_id = self.type_code_expr_and_load(ctx, &prefix_op.expr)?;

                match &prefix_op.op_tag {
                    PrefixOpTag::Not => {
                        if count_primitive_components(&self.registry, self.get_type(expr_type_id))
                            != 1
                        {
                            return Err(Error {
                                message: format!(
                                    "Cannot apply not operation to expr of type {}",
                                    self.registry.fmt(self.get_type(expr_type_id))
                                ),
                                loc: prefix_op.op_loc,
                            });
                        }

                        self.store_type(ctx, prefix_op.id, &Type::Bool);
                        return Ok(());
                    }
                    PrefixOpTag::Reference => {
                        let type_ = Type::Pointer(PointerType {
                            pointee: expr_type_id,
                            is_sequence: false,
                            is_nullable: false,
                        });
                        self.store_type(ctx, prefix_op.id, &type_);
                        return Ok(());
                    }
                    PrefixOpTag::Dereference => {
                        let Type::Pointer(ptr) = self.get_type(expr_type_id) else {
                            return Err(Error {
                                message: format!(
                                    "Cannot dereference expr of type {}",
                                    self.registry.fmt(self.get_type(expr_type_id))
                                ),
                                loc: prefix_op.loc,
                            });
                        };

                        if self.reporter.in_inspection_mode {
                            self.reporter.print_inspection(InspectInfo {
                                message: format!(
                                    "<deref>: {}",
                                    self.registry.fmt(self.get_type(ptr.pointee))
                                ),
                                loc: prefix_op.op_loc,
                                linked_loc: None,
                            });
                        };

                        self.store_expr_info(ctx, prefix_op.id, ptr.pointee);
                        return Ok(());
                    }
                    PrefixOpTag::Positive | PrefixOpTag::Negative => {
                        match self.get_type(expr_type_id) {
                            Type::U8 | Type::I8 => {
                                self.store_type(ctx, prefix_op.id, &Type::I8);
                                return Ok(());
                            }
                            Type::U16 | Type::I16 => {
                                self.store_type(ctx, prefix_op.id, &Type::I16);
                                return Ok(());
                            }
                            Type::U32 | Type::I32 => {
                                self.store_type(ctx, prefix_op.id, &Type::I32);
                                return Ok(());
                            }
                            Type::U64 | Type::I64 => {
                                self.store_type(ctx, prefix_op.id, &Type::I64);
                                return Ok(());
                            }
                            Type::Never
                            | Type::Null
                            | Type::Void
                            | Type::Bool
                            | Type::F32
                            | Type::F64
                            | Type::Pointer { .. }
                            | Type::Struct { struct_index: _ }
                            | Type::Enum { enum_index: _ }
                            | Type::Result(_)
                            | Type::Seg(_)
                            | Type::Container(_) => {
                                return Err(Error {
                                    message: format!(
                                        "Integer expected. Got {}",
                                        self.registry.fmt(self.get_type(expr_type_id))
                                    ),
                                    loc: prefix_op.op_loc,
                                });
                            }
                        }
                    }
                }
            }
            CodeExpr::Cast(cast) => {
                let castee_type_id =
                    self.report_if_err(self.type_code_expr_and_load(ctx, &cast.expr));

                let casted_to_type = self.get_type(self.build_type(ctx, &cast.casted_to)?);

                if let Some(castee_type_id) = castee_type_id {
                    let castee_type = self.get_type(castee_type_id);

                    let castee_type_components = self.get_primitives(castee_type);
                    let casted_to_type_components = self.get_primitives(&casted_to_type);

                    let noop_local_cast =
                        is_noop_cast(&castee_type_components, &casted_to_type_components);

                    let single_int_cast = castee_type_components.len() == 1
                        && casted_to_type_components.len() == 1
                        && is_wide_int(&castee_type_components[0]).is_some()
                        && is_wide_int(&casted_to_type_components[0]).is_some();

                    if !(single_int_cast || noop_local_cast) {
                        return Err(Error {
                            message: format!(
                                "Cannot cast from {} to {}",
                                self.registry.fmt(castee_type),
                                self.registry.fmt(&casted_to_type)
                            ),
                            loc: cast.expr.loc(),
                        });
                    }
                }

                self.store_type(ctx, cast.id, &casted_to_type);

                return Ok(());
            }
            CodeExpr::Assign(assign) => {
                // TODO: assert that lhs is writable

                let lhs_type_id =
                    self.report_if_err(self.type_code_expr_and_load(ctx, &assign.lhs));
                let rhs_type_id =
                    self.report_if_err(self.type_code_expr_and_load(ctx, &assign.rhs));

                if let Some(lhs_type_id) = lhs_type_id
                    && let Some(rhs_type_id) = rhs_type_id
                    && !is_type_compatible(
                        &self.registry,
                        self.get_type(lhs_type_id),
                        self.get_type(rhs_type_id),
                    )
                {
                    self.report_error(Error {
                        message: format!(
                            "Cannot assign {} to variable of type {}",
                            self.registry.fmt(self.get_type(rhs_type_id)),
                            self.registry.fmt(self.get_type(lhs_type_id)),
                        ),
                        loc: assign.op_loc.clone(),
                    });
                }

                self.store_type(ctx, assign.id, &Type::Void);

                return Ok(());
            }
            CodeExpr::FieldAccess(field_access) => {
                let lhs_type_id = self.type_code_expr_and_load(ctx, &field_access.lhs)?;
                let field =
                    get_struct_field(&self.registry, self.get_type(lhs_type_id), field_access)?;

                if self.reporter.in_inspection_mode {
                    self.reporter.print_inspection(InspectInfo {
                        message: format!(
                            "{}.{}: {}",
                            self.registry.fmt(&self.get_type(lhs_type_id)),
                            field.field_name,
                            self.registry.fmt(&field.field_type),
                        ),
                        loc: field_access.field_name.loc,
                        linked_loc: Some(field.loc),
                    })
                };

                self.store_type(ctx, field_access.id, &field.field_type);
                return Ok(());
            }
            CodeExpr::FnCall(call) => {
                if let Some(symbol) = self.get_symbol(ctx, &call.fn_name.repr)
                    && let TySymbolKind::EnumConstructor = symbol.kind
                {
                    let enum_ctor = &self.registry.enum_ctors[symbol.col_index];
                    let enum_def = &self.registry.enums[enum_ctor.enum_index];
                    let variant = &enum_def.variants[enum_ctor.variant_index];

                    if self.reporter.in_inspection_mode {
                        self.reporter.print_inspection(InspectInfo {
                            message: format!(
                                "{} // {}",
                                call.fn_name.repr, enum_ctor.variant_index
                            ),
                            loc: call.fn_name.loc,
                            linked_loc: Some(enum_ctor.loc),
                        });
                    }

                    for arg in &call.args.items {
                        self.report_if_err(self.type_code_expr(ctx, arg));
                    }

                    if call.args.items.len() != 1 {
                        return Err(Error {
                            message: format!(
                                "Non-void enum constructors require exactly one argument"
                            ),
                            loc: call.fn_name.loc,
                        });
                    }

                    if let Some(expr_type_id) = self.load_type(ctx, &call.args.items[0])
                        && !is_type_compatible(
                            &self.registry,
                            self.get_type(variant.variant_type_id),
                            self.get_type(expr_type_id),
                        )
                    {
                        return Err(Error {
                            message: format!(
                                "Invalid enum payload: {}, expected: {}",
                                self.registry.fmt(self.get_type(expr_type_id)),
                                self.registry.fmt(self.get_type(variant.variant_type_id)),
                            ),
                            loc: call.fn_name.loc,
                        });
                    }

                    self.store_expr_info(ctx, call.id, self.registry.call_info.len());
                    self.registry.be_mut().call_info.push(CallInfo {
                        value: CallInfoValue::EnumCtor(enum_ctor.variant_index as i32),
                        return_type_id: self.intern_type(&Type::Enum {
                            enum_index: enum_ctor.enum_index,
                        }),
                    });

                    return Ok(());
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
                let lhs_type_id = self.type_code_expr_and_load(ctx, &call.lhs)?;
                let fn_name =
                    self.get_fn_name_from_method(self.get_type(lhs_type_id), &call.field_name.repr);

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
                let lhs_type = self.get_type(self.type_code_expr_and_load(ctx, &call.lhs)?);
                let inline_fn_name = self.get_fn_name_from_method(lhs_type, &call.field_name.repr);

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
            CodeExpr::IntrinsicCall(call) => {
                if call.fn_name.repr == "unreachable" {
                    self.store_type(ctx, call.id, &Type::Never);

                    if call.args.items.len() != 0 || call.type_args.len() != 0 {
                        self.report_error(Error {
                            message: format!(
                                "Invalid arguments for `@{}(): never`",
                                call.fn_name.repr
                            ),
                            loc: call.fn_name.loc,
                        });
                    }

                    return Ok(());
                }

                if call.fn_name.repr == "memory_size" {
                    self.store_type(ctx, call.id, &Type::I32);

                    if call.args.items.len() != 0 || call.type_args.len() != 0 {
                        self.report_error(Error {
                            message: format!(
                                "Invalid arguments for `@{}(): i32`",
                                call.fn_name.repr
                            ),
                            loc: call.fn_name.loc,
                        });
                    }

                    return Ok(());
                }

                if call.fn_name.repr == "memory_grow" {
                    self.store_type(ctx, call.id, &Type::I32);

                    for arg in &call.args.items {
                        self.report_if_err(self.type_code_expr(ctx, arg));
                    }

                    if call.type_args.len() != 0 || call.args.items.len() != 1 {
                        report_bad_args(self, call);
                    }

                    if call.args.items.len() > 0
                        && let Some(type_id) = self.load_type(ctx, &call.args.items[0])
                        && *self.get_type(type_id) != Type::U32
                    {
                        report_bad_args(self, call);
                    }

                    return Ok(());

                    fn report_bad_args(self_: &Typer, call: &InlineFnCallExpr) {
                        self_.report_error(Error {
                            message: format!(
                                "Invalid arguments for `@{}(num_pages: u32): i32`",
                                call.fn_name.repr
                            ),
                            loc: call.fn_name.loc,
                        });
                    }
                }

                if call.fn_name.repr == "memory_copy" {
                    self.store_type(ctx, call.id, &Type::Void);

                    if call.type_args.len() != 0 {
                        report_bad_args(self, call);
                    }

                    let mut all_good = true;
                    let mut arg_types = Vec::new();
                    for arg in &call.args.items {
                        let Some(arg_type_id) =
                            self.report_if_err(self.type_code_expr_and_load(ctx, arg))
                        else {
                            all_good = false;
                            continue;
                        };

                        arg_types.push(self.get_type(arg_type_id));
                    }

                    if all_good && arg_types != &[&Type::U32, &Type::U32, &Type::U32] {
                        report_bad_args(self, call);
                    }

                    return Ok(());

                    fn report_bad_args(self_: &Typer, call: &InlineFnCallExpr) {
                        self_.report_error(Error {
                            message: format!(
                                "Invalid arguments for `@{}(dest: u32, source: u32: num_bytes: u32)`",
                                call.fn_name.repr
                            ),
                            loc: call.fn_name.loc,
                        });
                    }
                }

                if call.fn_name.repr == "data_size" {
                    self.store_type(ctx, call.id, &Type::U32);

                    if call.type_args.len() != 0 || call.args.items.len() != 0 {
                        self.reporter.error(Error {
                            message: format!(
                                "Invalid arguments for `@{}(): u32`",
                                call.fn_name.repr
                            ),
                            loc: call.fn_name.loc,
                        });
                    }

                    if let Some(_) = ctx.fn_index {
                        self.reporter.error(Error {
                            message: format!(
                                "`@{}(): u32` can only be used in globals",
                                call.fn_name.repr
                            ),
                            loc: call.fn_name.loc,
                        });
                    }

                    return Ok(());
                }

                if call.fn_name.repr == "embed_file" {
                    self.store_type(
                        ctx,
                        call.id,
                        &Type::Pointer(PointerType {
                            pointee: self.intern_type(&Type::U8),
                            is_sequence: true,
                            is_nullable: false,
                        }),
                    );

                    if call.type_args.len() != 0 && call.args.items.len() != 1 {
                        report_bad_args(self, call);
                    }

                    if let CodeExpr::StringLiteral(_) = &call.args.items[0] {
                    } else {
                        report_bad_args(self, call);
                    };

                    fn report_bad_args(self_: &Typer, call: &InlineFnCallExpr) {
                        self_.reporter.error(Error {
                            message: format!(
                                "Invalid arguments for `@{}(relative_file_path: str): &*u8`",
                                call.fn_name.repr,
                            ),
                            loc: call.fn_name.loc,
                        });
                    }

                    return Ok(());
                }

                if call.fn_name.repr == "const_slice_len" {
                    self.store_type(ctx, call.id, &Type::U32);

                    if call.type_args.len() != 0 && call.args.items.len() != 1 {
                        report_bad_args(self, call);
                    }

                    self.type_code_expr(ctx, &call.args.items[0])?;

                    if let Some(const_value) = self.get_const_value(ctx, &call.args.items[0])
                        && let CodeExpr::ArrayLiteral(_) = const_value
                    {
                    } else {
                        report_bad_args(self, call);
                    };

                    return Ok(());

                    fn report_bad_args(self_: &Typer, call: &InlineFnCallExpr) {
                        self_.reporter.error(Error {
                            message: format!(
                                "Invalid arguments for `@{}(items: const T[]): u32`",
                                call.fn_name.repr,
                            ),
                            loc: call.fn_name.loc,
                        });
                    }
                }

                if call.fn_name.repr == "inline_fn_call_loc" {
                    let mut inside_of_inline_fn = false;
                    let mut currrent_ctx = ctx;
                    loop {
                        if currrent_ctx.kind == ScopeKind::InlineFn {
                            inside_of_inline_fn = true;
                            break;
                        }

                        if let Some(parent) = currrent_ctx.parent {
                            currrent_ctx = parent;
                        } else {
                            break;
                        }
                    }

                    if !inside_of_inline_fn {
                        self.report_error(Error {
                            message: format!(
                                "Forbidden use of `@{}()` outside of inline fn",
                                call.fn_name.repr
                            ),
                            loc: call.fn_name.loc,
                        });
                    };

                    self.store_type(
                        ctx,
                        call.id,
                        self.registry.str_literal_type.as_ref().unwrap(),
                    );
                    return Ok(());
                }

                if call.fn_name.repr == "get_ok" {
                    if call.args.items.len() == 1 {
                        let arg_type = self.type_code_expr_and_load(ctx, &call.args.items[0])?;

                        if let Type::Result(result) = self.get_type(arg_type) {
                            self.store_type(ctx, call.id, self.get_type(result.ok));
                            return Ok(());
                        };
                    }

                    return Err(Error {
                        message: format!(
                            "Invalid arguments for `@{}(res: Result(T, E)): T`",
                            call.fn_name.repr,
                        ),
                        loc: call.fn_name.loc,
                    });
                }

                if call.fn_name.repr == "get_err" {
                    if call.args.items.len() == 1 {
                        let arg_type = self.type_code_expr_and_load(ctx, &call.args.items[0])?;

                        if let Type::Result(result) = self.get_type(arg_type) {
                            self.store_type(ctx, call.id, self.get_type(result.err));
                            return Ok(());
                        };
                    }

                    return Err(Error {
                        message: format!(
                            "Invalid arguments for `@{}(res: Result(T, E)): E`",
                            call.fn_name.repr,
                        ),
                        loc: call.fn_name.loc,
                    });
                }

                if call.fn_name.repr.starts_with("inspect_symbols") {
                    if self.reporter.in_inspection_mode {
                        let mut message = String::new();
                        for symbol in &ctx.symbols {
                            write!(message, "{} : {:?}\n", symbol.name, symbol.kind).unwrap();
                        }

                        self.reporter.print_inspection(InspectInfo {
                            message,
                            loc: call.fn_name.loc,
                            linked_loc: None,
                        });
                    }

                    self.store_type(ctx, call.id, &Type::Void);
                    return Ok(());
                }

                self.report_error(Error {
                    message: format!("Unknown intrinsic: {}", call.fn_name.repr),
                    loc: call.fn_name.loc,
                });
                return Ok(());
            }
            CodeExpr::Return(return_expr) => {
                self.store_type(ctx, return_expr.id, &Type::Never);

                if let None = ctx.fn_index {
                    self.reporter.error(Error {
                        message: format!("Cannot use `return` in const context"),
                        loc: return_expr.loc,
                    });
                };

                let mut return_type = &Type::Void;
                if let Some(return_value) = &return_expr.expr {
                    let return_type_id = self.type_code_expr_and_load(ctx, &return_value)?;
                    return_type = self.get_type(return_type_id);
                }

                let fn_return_type = &self.registry.functions[ctx.fn_index.unwrap()].type_.output;
                if !is_type_compatible(&self.registry, fn_return_type, return_type) {
                    self.reporter.error(Error {
                        message: format!(
                            "Invalid return type: {}, expected: {}",
                            self.registry.fmt(&return_type),
                            self.registry.fmt(&fn_return_type),
                        ),
                        loc: return_expr.loc,
                    });
                }

                return Ok(());
            }
            CodeExpr::If(if_expr) => {
                let mut updated_then_ctx = None;

                match &if_expr.cond {
                    IfCond::Expr(cond_expr) => {
                        if let Some(cond_type_id) =
                            self.report_if_err(self.type_code_expr_and_load(ctx, &cond_expr))
                            && *self.get_type(cond_type_id) != Type::Bool
                        {
                            self.report_error(Error {
                                message: format!(
                                    "Invalid condition type: {}, expected: {}",
                                    self.registry.fmt(self.get_type(cond_type_id)),
                                    self.registry.fmt(&Type::Bool),
                                ),
                                loc: cond_expr.loc(),
                            });
                        };
                    }
                    IfCond::Match(match_header) => {
                        let then_ctx = self.child_ctx(ctx, ScopeKind::Block);
                        if let Err(err) = self.type_match_header(then_ctx, match_header) {
                            self.report_error(err)
                        }
                        updated_then_ctx = Some(then_ctx);
                    }
                }

                let then_type;
                if let Some(updated_then_ctx) = updated_then_ctx {
                    then_type = self.type_code_block(updated_then_ctx, &if_expr.then_block, true);
                } else {
                    then_type = self.type_code_block(ctx, &if_expr.then_block, true);
                }

                let mut else_type = Type::Void;
                match &if_expr.else_block {
                    ElseBlock::None => {}
                    ElseBlock::Else(code_block) => {
                        else_type = self.type_code_block(ctx, code_block, true);
                    }
                    ElseBlock::ElseIf(code_expr) => {
                        self.report_if_err(self.type_code_expr(ctx, &code_expr));
                        if let Some(type_id) = self.load_type(ctx, &code_expr) {
                            else_type = self.get_type(type_id).clone();
                        }
                    }
                }

                if then_type == Type::Never && else_type == Type::Never {
                    self.store_type(ctx, if_expr.id, &Type::Never);
                } else {
                    self.store_type(ctx, if_expr.id, &Type::Void);
                }

                return Ok(());
            }
            CodeExpr::While(while_expr) => {
                self.type_code_block(ctx, &while_expr.body, true);

                if let Some(cond) = &while_expr.cond {
                    let cond_type = self.type_code_expr_and_load(ctx, &cond)?;

                    if *self.get_type(cond_type) != Type::Bool {
                        self.report_error(Error {
                            message: format!(
                                "Invalid condition type: {}, expected: {}",
                                self.registry.fmt(self.get_type(cond_type)),
                                self.registry.fmt(&Type::Bool),
                            ),
                            loc: cond.loc(),
                        });
                    };
                }

                self.store_type(ctx, while_expr.id, &Type::Void);

                return Ok(());
            }
            CodeExpr::For(for_expr) => {
                let maybe_start_type_id =
                    self.report_if_err(self.type_code_expr_and_load(ctx, &for_expr.start));
                let maybe_end_type_id =
                    self.report_if_err(self.type_code_expr_and_load(ctx, &for_expr.end));

                if let Some(start_type_id) = maybe_start_type_id
                    && let Some(end_type_id) = maybe_end_type_id
                    && start_type_id != end_type_id
                {
                    self.report_error(Error {
                        message: format!(
                            "Invalid range end type: {}, expected: {}",
                            self.registry.fmt(self.get_type(end_type_id)),
                            self.registry.fmt(self.get_type(start_type_id)),
                        ),
                        loc: for_expr.end.loc(),
                    });
                }

                let ctx = self.child_ctx(ctx, ScopeKind::ForLoop);

                if let Some(counter_type_id) = self.load_type(ctx, &for_expr.start) {
                    self.define_local(
                        ctx,
                        for_expr.counter.repr,
                        counter_type_id,
                        for_expr.counter.loc,
                    );

                    if is_wide_int(self.get_type(counter_type_id)).is_none() {
                        self.report_error(Error {
                            message: format!(
                                "Invalid counter. Type must be a number, got {} instead",
                                self.registry.fmt(self.get_type(counter_type_id))
                            ),
                            loc: for_expr.start.loc(),
                        });
                    }
                }

                self.type_code_block(ctx, &for_expr.body, true);

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
                let expr_type_id = self.type_code_expr_and_load(ctx, &catch.lhs)?;
                let result =
                    self.assert_catchable_type(self.get_type(expr_type_id), &catch.catch_loc)?;

                let ctx = self.child_ctx(ctx, ScopeKind::Block);

                self.define_local(ctx, catch.error_bind.repr, result.err, catch.error_bind.loc);

                let catch_type = self.type_code_block(ctx, &catch.catch_body, true);
                if catch_type != Type::Never {
                    self.report_error(Error {
                        message: format!("Catch expression must resolve to never, got other type"),
                        loc: catch.catch_loc,
                    });
                }

                self.store_expr_info(ctx, catch.id, result.ok);
                return Ok(());
            }
            CodeExpr::PropagateError(prop_error) => {
                let expr_type = self.type_code_expr_and_load(ctx, &prop_error.expr)?;
                let result =
                    self.assert_catchable_type(self.get_type(expr_type), &prop_error.loc)?;
                self.store_expr_info(ctx, prop_error.id, result.ok);
                return Ok(());
            }
            CodeExpr::Match(match_expr) => {
                let else_ctx = self.child_ctx(ctx, ScopeKind::Block);
                let else_type = self.type_code_block(else_ctx, &match_expr.else_branch, true);
                if else_type != Type::Never {
                    self.report_error(Error {
                        message: format!(
                            "Match's else block must resolve to never, got other type"
                        ),
                        loc: match_expr.else_branch.loc,
                    });
                }

                if let Err(err) = self.type_match_header(ctx, &match_expr.header) {
                    self.report_error(err)
                }

                self.store_type(ctx, match_expr.id, &Type::Void);

                return Ok(());
            }
            CodeExpr::Paren(paren) => {
                self.report_if_err(self.type_code_expr(ctx, &paren.expr));

                if let Some(expr_type_id) = self.load_type(ctx, &paren.expr) {
                    self.store_expr_info(ctx, paren.id, expr_type_id);
                }
                return Ok(());
            }
            CodeExpr::DoWith(do_with) => {
                self.store_type(ctx, do_with.id, &Type::Void);

                if do_with.args.items.len() == 0 {
                    self.report_error(Error {
                        message: format!("do-with expressions must have at least one argument"),
                        loc: do_with.with_loc.clone(),
                    });
                    return Ok(());
                };

                for arg in &do_with.args.items {
                    self.report_if_err(self.type_code_expr(ctx, arg));
                }

                let Some(it_type_id) = self.load_type(ctx, &do_with.args.items[0]) else {
                    return Ok(());
                };

                let ctx = self.child_ctx(ctx, ScopeKind::InlineFn);
                self.define_local(ctx, "it", it_type_id, do_with.with_loc);

                for arg in do_with.args.items.iter().skip(1) {
                    if let Some(arg_type_id) = self.load_type(ctx, arg)
                        && arg_type_id != it_type_id
                    {
                        self.report_error(Error {
                            message: format!(
                                "do-with argument type mismatch. expected: {}, got: {}",
                                self.registry.fmt(self.get_type(it_type_id)),
                                self.registry.fmt(self.get_type(arg_type_id)),
                            ),
                            loc: arg.loc(),
                        });
                    }
                }

                self.report_if_err(self.type_code_expr(ctx, &do_with.body));

                return Ok(());
            }
            CodeExpr::Pipe(pipe) => {
                self.report_if_err(self.type_code_expr(ctx, &pipe.lhs));

                let ctx = self.child_ctx(ctx, ScopeKind::InlineFn);
                if let Some(it_type_id) = self.load_type(ctx, &pipe.lhs) {
                    self.define_local(ctx, "it", it_type_id, pipe.op_loc);
                }

                self.report_if_err(self.type_code_expr(ctx, &pipe.rhs));

                if let Some(type_id) = self.load_type(ctx, &pipe.rhs) {
                    self.store_expr_info(ctx, pipe.id, type_id);
                } else {
                    self.store_type(ctx, pipe.id, &Type::Never);
                }

                return Ok(());
            }
            CodeExpr::Sizeof(sizeof) => {
                self.report_if_err(self.build_type(ctx, &sizeof.type_expr));
                self.store_type(ctx, sizeof.id, &Type::U32);
                return Ok(());
            }
        }
    }

    fn get_result_literal_type(
        &self,
        ctx: TyContextRef,
        explicit_type: &Option<ResultTypeExpr>,
        loc: &Loc,
    ) -> Result<ResultType, Error> {
        if let Some(result_type) = explicit_type {
            let ok = self.build_type(ctx, &result_type.ok)?;
            let err = self.build_type(ctx, &result_type.err)?;
            return Ok(ResultType { ok, err });
        }

        let Some(fn_index) = ctx.fn_index else {
            return Err(Error {
                message: format!("Cannot create implicitly typed result in const context"),
                loc: *loc,
            });
        };

        let fn_info = &self.registry.functions[fn_index];
        let Type::Result(result) = &fn_info.type_.output else {
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

    fn get_value_info(&self, ctx: TyContextRef, var_name: &IdentExpr) -> Result<ValueInfo, Error> {
        let Some(symbol) = self.get_symbol(ctx, var_name.repr) else {
            return Err(Error {
                message: format!("Unknown variable: {}", var_name.repr),
                loc: var_name.loc,
            });
        };

        match symbol.kind {
            TySymbolKind::Local => {
                let local_type_id = self.locals_types[symbol.col_index];

                if self.reporter.in_inspection_mode {
                    self.reporter.print_inspection(InspectInfo {
                        message: format!(
                            "let {}: {}",
                            var_name.repr,
                            self.registry.fmt(self.get_type(local_type_id))
                        ),
                        loc: var_name.loc,
                        linked_loc: Some(symbol.loc),
                    })
                };

                return Ok(ValueInfo {
                    kind: ValueKind::Local,
                    col_index: self.locals_types.len(),
                    type_id: local_type_id,
                });
            }
            TySymbolKind::Global => {
                let global = &self.registry.globals[symbol.col_index];

                if self.reporter.in_inspection_mode {
                    self.reporter.print_inspection(InspectInfo {
                        message: format!(
                            "let {}: {}",
                            var_name.repr,
                            self.registry.fmt(self.get_type(global.type_id))
                        ),
                        loc: var_name.loc,
                        linked_loc: Some(symbol.loc),
                    });
                }

                return Ok(ValueInfo {
                    kind: ValueKind::Global,
                    col_index: symbol.col_index,
                    type_id: global.type_id,
                });
            }
            TySymbolKind::Const => {
                let const_def = &self.registry.constants[symbol.col_index];

                if self.reporter.in_inspection_mode {
                    self.reporter.print_inspection(InspectInfo {
                        message: format!(
                            "inline let {}: {}",
                            var_name.repr,
                            self.registry.fmt(self.get_type(const_def.type_id))
                        ),
                        loc: var_name.loc,
                        linked_loc: Some(symbol.loc),
                    })
                }

                return Ok(ValueInfo {
                    kind: ValueKind::Const,
                    col_index: symbol.col_index,
                    type_id: const_def.type_id,
                });
            }
            TySymbolKind::EnumConstructor => {
                let enum_ctor = &self.registry.enum_ctors[symbol.col_index];
                let enum_def = &self.registry.enums[enum_ctor.enum_index];
                let enum_variant = &enum_def.variants[enum_ctor.variant_index];

                let Type::Void = enum_def.variant_type else {
                    return Err(Error {
                        message: format!(
                            "Cannot construct {}, expected payload of type {}",
                            var_name.repr,
                            self.registry
                                .fmt(self.get_type(enum_variant.variant_type_id))
                        ),
                        loc: var_name.loc,
                    });
                };

                let var_type = &Type::Enum {
                    enum_index: enum_ctor.enum_index,
                };

                if self.reporter.in_inspection_mode {
                    self.reporter.print_inspection(InspectInfo {
                        message: format!(
                            "inline let {}: {}",
                            var_name.repr,
                            self.registry.fmt(&var_type)
                        ),
                        loc: var_name.loc,
                        linked_loc: Some(enum_ctor.loc),
                    })
                }

                return Ok(ValueInfo {
                    kind: ValueKind::EnumConstructor,
                    col_index: symbol.col_index,
                    type_id: self.intern_type(var_type),
                });
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

    fn get_fn_name_from_method(&self, receiver_type: &Type, method_name: &str) -> String {
        let receiver_type = deref_rec(&self.registry, receiver_type);

        match receiver_type {
            Type::Seg(_) => format!("seg::{method_name}",),
            Type::Container(ctr) => format!(
                "{}::{method_name}",
                self.registry.fmt(self.get_type(ctr.container))
            ),
            _ => format!("{}::{method_name}", self.registry.fmt(receiver_type)),
        }
    }

    fn type_match_header(
        &self,
        ctx: TyContextRef,
        match_header: &MatchHeader,
    ) -> Result<(), Error> {
        let Some(symbol) = self.get_symbol(ctx, &match_header.variant_name.repr) else {
            return Err(Error {
                message: format!("Unknown symbol"),
                loc: match_header.variant_name.loc,
            });
        };

        let TySymbolKind::EnumConstructor = symbol.kind else {
            return Err(Error {
                message: format!("Not an enum variant"),
                loc: match_header.variant_name.loc,
            });
        };

        let enum_ctor = &self.registry.enum_ctors[symbol.col_index];
        let enum_variant =
            &self.registry.enums[enum_ctor.enum_index].variants[enum_ctor.variant_index];

        let expr_to_match_type_id =
            self.report_if_err(self.type_code_expr_and_load(ctx, &match_header.expr_to_match));
        if let Some(expr_to_match_type_id) = expr_to_match_type_id {
            let expr_to_match_type = self.get_type(expr_to_match_type_id);
            let expected_expr_to_match_type = Type::Enum {
                enum_index: enum_ctor.enum_index,
            };
            if !is_type_compatible(
                &self.registry,
                &expected_expr_to_match_type,
                &expr_to_match_type,
            ) {
                self.report_error(Error {
                    message: format!(
                        "Unexpected type to match, expected: {}, got: {}",
                        self.registry.fmt(&expected_expr_to_match_type),
                        self.registry.fmt(&expr_to_match_type)
                    ),
                    loc: match_header.expr_to_match.loc(),
                });
            }
        };

        if self.reporter.in_inspection_mode {
            self.reporter.print_inspection(InspectInfo {
                message: format!(
                    "{}\n{}({})",
                    self.registry.fmt(&Type::Enum {
                        enum_index: enum_ctor.enum_index
                    }),
                    match_header.variant_name.repr,
                    self.registry
                        .fmt(self.get_type(enum_variant.variant_type_id))
                ),
                loc: match_header.variant_name.loc,
                linked_loc: Some(enum_variant.loc),
            });
        }

        self.define_local(
            ctx,
            match_header.variant_bind.repr,
            enum_variant.variant_type_id,
            match_header.variant_bind.loc,
        );

        self.store_expr_info(
            ctx,
            match_header.variant_bind.id,
            self.registry.value_info.len(),
        );
        self.registry.value_info.be_mut().push(ValueInfo {
            kind: ValueKind::EnumConstructor,
            col_index: symbol.col_index,
            type_id: match_header.variant_bind.id,
        });

        Ok(())
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
                    self.registry.fmt(&expr_type)
                ),
                loc: *loc,
            });
        };

        let err_type_components = self.get_primitives(self.get_type(result.err));
        if err_type_components.len() != 1 || !is_noop_cast(&err_type_components, &[Type::U32]) {
            return Err(Error {
                message: format!(
                    "Invalid Result error type: {}, must fit into U32 local",
                    self.registry.fmt(self.get_type(result.err))
                ),
                loc: *loc,
            });
        }

        Ok(result)
    }

    // emits a sequence of `Bool | I* | U* | F*` primitive types
    fn get_primitives(&self, type_: &Type) -> Vec<Type> {
        let mut layout = TypeLayout::new();
        layout.primitives = Some(Vec::new());

        get_type_layout(&self.registry, type_, &mut layout);

        layout.primitives.unwrap()
    }

    fn type_fn_call(
        &self,
        ctx: TyContextRef,
        fn_name: &str,
        receiver_arg: Option<&CodeExpr>,
        args: &Vec<CodeExpr>,
        call_expr_id: usize,
        loc: &Loc,
    ) -> Result<(), Error> {
        let mut all_good = true;
        let mut arg_types = Vec::new();
        if let Some(receiver_arg) = receiver_arg {
            if let Some(type_id) =
                self.report_if_err(self.type_code_expr_and_load(ctx, receiver_arg))
            {
                arg_types.push(type_id)
            } else {
                all_good = false;
            };
        }
        for arg in args {
            if let Some(type_id) = self.report_if_err(self.type_code_expr_and_load(ctx, arg)) {
                arg_types.push(type_id)
            } else {
                all_good = false;
            };
        }

        let fn_index = self.get_fn_index_for_call(ctx, fn_name, loc)?;
        let fn_info = &self.registry.functions[fn_index];

        self.store_expr_info(ctx, call_expr_id, self.registry.call_info.len());
        self.registry.be_mut().call_info.push(CallInfo {
            value: CallInfoValue::FnCall(fn_index),
            return_type_id: self.intern_type(&fn_info.type_.output),
        });

        if all_good && !is_types_compatible(&self.registry, &fn_info.type_.inputs, &arg_types) {
            return Err(Error {
                message: format!(
                    "Invalid function arguments for function {}: [{}], expected [{}]",
                    fn_info.name,
                    self.registry.fmt_many(&arg_types),
                    self.registry.fmt_many(&fn_info.type_.inputs),
                ),
                loc: *loc,
            });
        }

        if self.reporter.in_inspection_mode {
            let mut message = String::new();

            write!(&mut message, "fn {fn_name}(").unwrap();

            for (param, i) in fn_info.params.iter().zip(0..) {
                if i != 0 {
                    message.push_str(", ");
                }

                message.push_str(&param.param_name);
                message.push_str(": ");
                write!(&mut message, "{}", self.registry.fmt(&param.param_type)).unwrap();
            }

            let return_type = self.registry.fmt(&fn_info.type_.output);
            write!(&mut message, "): {}", return_type).unwrap();

            self.reporter.print_inspection(InspectInfo {
                message,
                loc: *loc,
                linked_loc: Some(fn_info.definition_loc.clone()),
            });
        }

        return Ok(());
    }

    fn get_fn_index_for_call(
        &self,
        ctx: TyContextRef,
        fn_name: &str,
        loc: &Loc,
    ) -> Result<usize, Error> {
        let Some(symbol) = self.get_symbol(ctx, fn_name) else {
            return Err(Error {
                message: format!("Unknown function: {}", fn_name),
                loc: *loc,
            });
        };

        let TySymbolKind::Function = symbol.kind else {
            return Err(Error {
                message: format!(
                    "Trying to call {} which is not a function, defined at: {}",
                    fn_name,
                    symbol.loc.to_string(&self.registry)
                ),
                loc: *loc,
            });
        };

        Ok(symbol.col_index)
    }

    fn type_inline_fn_call(
        &self,
        ctx: TyContextRef,
        inline_fn_name: &str,
        type_arg_exprs: &Vec<TypeExpr>,
        receiver_arg: Option<&CodeExpr>,
        arg_exprs: &Vec<CodeExpr>,
        call_expr_id: usize,
        loc: &Loc,
    ) -> Result<(), Error> {
        let Some(symbol) = self.get_symbol(ctx, inline_fn_name) else {
            return Err(Error {
                message: format!("Unknown inline fn: {}", inline_fn_name),
                loc: *loc,
            });
        };
        let inline_fn_def = self.registry.inline_fns[symbol.col_index];
        let FnExprValue::Body(body) = &inline_fn_def.value else {
            unreachable!()
        };

        let parent_ctx = ctx;
        let mut ctx = self.child_ctx(ctx, ScopeKind::InlineFn);

        let mut type_args = Vec::new();
        for type_arg in type_arg_exprs {
            type_args.push(self.build_type(parent_ctx, type_arg)?.clone());
        }
        if type_args.len() != inline_fn_def.decl.type_params.len() {
            return Err(Error {
                message: format!(
                    "Invalid number of type args, expected {}, got {}",
                    inline_fn_def.decl.type_params.len(),
                    type_arg_exprs.len()
                ),
                loc: *loc,
            });
        }
        for (i, (type_param, type_arg)) in inline_fn_def
            .decl
            .type_params
            .iter()
            .zip(type_args.iter())
            .enumerate()
        {
            self.define_type(
                ctx,
                type_param,
                self.get_type(*type_arg),
                type_arg_exprs[i].loc(),
            );
        }

        // TODO!: figure out why this has to be done here and not near `type_code_block` call
        let inner_expr_id_offset = self.registry.expr_info.len() - body.expr_id_start;
        self.extend_expr_info_storage(body.expr_id_count);

        let mut has_receiver = false;
        let mut arg_type_ids = Vec::new();
        if let Some(receiver_arg) = receiver_arg {
            has_receiver = true;
            arg_type_ids.push(self.type_code_expr_and_load(parent_ctx, receiver_arg)?);
        }
        for arg in arg_exprs {
            arg_type_ids.push(self.type_code_expr_and_load(parent_ctx, arg)?);
        }
        if arg_type_ids.len() != inline_fn_def.decl.params.len() {
            return Err(Error {
                message: format!(
                    "Invalid number of inline fn args, expected {}, got {}",
                    inline_fn_def.decl.params.len(),
                    arg_type_ids.len()
                ),
                loc: *loc,
            });
        }
        for i in 0..arg_type_ids.len() {
            let inline_fn_param = &inline_fn_def.decl.params[i];
            let type_id = &arg_type_ids[i];

            if let Some(type_name) = get_infer_type_name(inline_fn_param)? {
                self.define_type(ctx, type_name, self.get_type(*type_id), inline_fn_param.loc);
            }

            let arg_expr = if has_receiver && i == 0 {
                receiver_arg.unwrap()
            } else {
                &arg_exprs[if has_receiver { i - 1 } else { i }]
            };

            let param_name = inline_fn_param.param_name.repr;
            let param_loc = inline_fn_param.param_name.loc;
            self.define_symbol(ctx, param_name, TySymbolKind::Const, param_loc);
            self.registry.constants.be_mut().push(ConstDef {
                type_id: *type_id,
                expr: UBRef::new(arg_expr),
                inner_expr_id_offset: parent_ctx.expr_id_offset,
            });
        }

        let self_type = self.get_fn_self_type(
            parent_ctx,
            &inline_fn_def.decl.fn_name,
            &inline_fn_def.decl.params,
        );

        // TODO!: figure out why this is needed
        ctx.expr_id_offset = 0;

        let mut inline_fn_param_types = Vec::new();
        for inline_fn_param in &inline_fn_def.decl.params {
            let inline_fn_type = self.get_fn_param_type(ctx, inline_fn_param, self_type, true)?;
            inline_fn_param_types.push(inline_fn_type);
        }

        if !is_types_compatible(&self.registry, &inline_fn_param_types, &arg_type_ids) {
            return Err(Error {
                message: format!(
                    "Invalid inline fn args, expected [{}], got [{}]",
                    self.registry.fmt_many(&inline_fn_param_types),
                    self.registry.fmt_many(&arg_type_ids)
                ),
                loc: *loc,
            });
        }

        let return_type = if let Some(return_type) = &inline_fn_def.decl.return_type {
            self.get_type(self.build_type(ctx, return_type)?)
        } else {
            &Type::Void
        };

        ctx.expr_id_offset = inner_expr_id_offset;

        self.type_code_block(ctx, body, false);

        self.store_expr_info(
            parent_ctx,
            call_expr_id,
            self.registry.inline_call_info.len(),
        );
        self.registry
            .inline_call_info
            .be_mut()
            .push(InlineCallInfo {
                inline_fn_index: symbol.col_index,
                inner_expr_id_offset,
                return_type_id: self.intern_type(&return_type),
            });

        if self.reporter.in_inspection_mode {
            let mut message = String::new();

            write!(&mut message, "inline fn {inline_fn_name}").unwrap();
            if type_args.len() > 0 {
                write!(&mut message, "<{}>", self.registry.fmt_many(&type_args)).unwrap();
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
                        let arg_type = self.registry.fmt(self.get_type(inline_fn_param_types[i]));
                        write!(&mut message, "{arg_type}",).unwrap();
                    }
                }
            }

            write!(&mut message, "): {}", self.registry.fmt(&return_type)).unwrap();

            self.reporter.print_inspection(InspectInfo {
                message,
                loc: *loc,
                linked_loc: Some(inline_fn_def.decl.fn_name.loc),
            });
        }

        Ok(())
    }

    fn get_fn_param_type(
        &self,
        ctx: TyContextRef,
        fn_param: &FnParam,
        self_type_id: Option<TypeId>,
        infer_allowed: bool,
    ) -> Result<TypeId, Error> {
        match &fn_param.param_type {
            FnParamType::Self_ | FnParamType::SelfRef => {
                // SAFETY: `get_fn_self_type` does the check
                let self_type_id = self_type_id.unwrap();

                if let FnParamType::Self_ = fn_param.param_type {
                    return Ok(self_type_id);
                }

                return Ok(self.intern_type(&Type::Pointer(PointerType {
                    pointee: self_type_id,
                    is_sequence: false,
                    is_nullable: false,
                })));
            }
            FnParamType::Type { expr } => {
                if let Some(infer_type_name) = get_infer_type_name(fn_param)? {
                    if !infer_allowed {
                        return Err(Error {
                            message: format!("Infer is only allowed in inline fns"),
                            loc: fn_param.param_name.loc,
                        });
                    }

                    return self.get_type_id_or_err(ctx, infer_type_name, &fn_param.param_name.loc);
                }

                self.build_type(ctx, &expr)
            }
        }
    }

    fn get_fn_self_type(
        &self,
        ctx: TyContextRef,
        fn_name: &IdentExpr,
        fn_params: &Vec<FnParam>,
    ) -> Option<TypeId> {
        let mut has_self_param = false;
        for fn_param in fn_params {
            let (FnParamType::Self_ | FnParamType::SelfRef) = fn_param.param_type else {
                continue;
            };

            has_self_param = true;

            if fn_name.parts.len() == 1 {
                self.report_error(Error {
                    message: format!("Cannot use self param in non-method function"),
                    loc: fn_param.loc,
                });
                return None;
            }
        }
        if !has_self_param {
            return None;
        }

        let mut fn_module = &self.registry.modules[ctx.module_id];

        // fn imported from other module
        if fn_name.loc.file_id != fn_module.parser.lexer.file_id {
            let module_id = self.registry.get_module_id_by_file_id(fn_name.loc.file_id);
            fn_module = &self.registry.modules[module_id];
        }

        let mut self_type_loc = fn_name.parts[0];
        self_type_loc.end_pos = fn_name.parts[fn_name.parts.len() - 2].end_pos;

        let self_type_name = self_type_loc.read_span(fn_module.source);

        let self_type_id = catch!(
            self.get_type_id_or_err(ctx, &self_type_name, &self_type_loc),
            err,
            {
                self.report_error(err);
                return None;
            }
        );

        Some(self_type_id)
    }

    fn define_type(&self, ctx: TyContextRef, name: &'static str, type_: &Type, loc: Loc) {
        let _ = self.define_symbol(ctx, name, TySymbolKind::TypeAlias, loc);
        let type_id = self.intern_type(type_);
        self.type_aliases.be_mut().push(type_id);
    }

    fn define_local(&self, ctx: TyContextRef, name: &'static str, type_id: TypeId, loc: Loc) {
        if self.reporter.in_inspection_mode {
            self.reporter.print_inspection(InspectInfo {
                message: format!(
                    "let {}: {}",
                    name,
                    self.registry.fmt(self.get_type(type_id))
                ),
                loc: loc,
                linked_loc: None,
            })
        };

        if name != "_" && self.define_symbol(ctx, name, TySymbolKind::Local, loc) {
            self.locals_types.be_mut().push(type_id);
        }
    }

    fn type_code_expr_and_load(&self, ctx: TyContextRef, expr: &CodeExpr) -> Result<TypeId, Error> {
        self.type_code_expr(ctx, expr)?;

        let Some(type_id) = self.load_type(ctx, expr) else {
            self.reporter.abort_due_to_compiler_bug(
                "Expression should only return ok if it stored a type",
                expr.loc(),
            );
        };

        Ok(type_id)
    }

    fn build_type(&self, ctx: TyContextRef, expr: &TypeExpr) -> Result<TypeId, Error> {
        self.build_type_(ctx, expr, true, &Loc::internal())
    }

    fn build_type_(
        &self,
        ctx: TyContextRef,
        expr: &TypeExpr,
        is_referenced: bool,
        loc: &Loc,
    ) -> Result<TypeId, Error> {
        self.type_type_expr(ctx, expr, is_referenced, loc)?;

        let Some(type_id) = self.registry.get_expr_info(ctx.expr_id_offset, expr.id()) else {
            self.reporter.abort_due_to_compiler_bug(
                "Type expression should only return ok if it stored a type",
                expr.loc(),
            );
        };

        Ok(type_id)
    }

    // builds a type, asserting that it doesn't have infinite size
    fn type_type_expr(
        &self,
        ctx: TyContextRef,
        expr: &TypeExpr,
        is_referenced: bool,
        loc: &Loc,
    ) -> Result<(), Error> {
        match expr {
            TypeExpr::Named(ident) => {
                let type_id = self.get_type_id_or_err(ctx, &ident.repr, &ident.loc)?;
                if let Type::Struct { struct_index } = self.get_type(type_id) {
                    let struct_def = &self.registry.structs[*struct_index];
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
                self.store_expr_info(ctx, ident.id, type_id);
                Ok(())
            }
            TypeExpr::Pointer(ptr) => {
                let pointee = self.build_type_(ctx, &ptr.pointee, true, loc)?;

                self.store_type(
                    ctx,
                    ptr.id,
                    &Type::Pointer(PointerType {
                        pointee,
                        is_sequence: ptr.is_sequence,
                        is_nullable: ptr.is_nullable,
                    }),
                );
                Ok(())
            }
            TypeExpr::Container(ctr) => {
                if let TypeExpr::Named(ident) = &*ctr.container
                    && ident.repr == "Result"
                {
                    if ctr.items.len() != 2 {
                        return Err(Error {
                            message: format!(
                                "Expected exactly 2 type arguments, {} was found",
                                ctr.items.len()
                            ),
                            loc: ident.loc,
                        });
                    }

                    let ok = self.build_type_(ctx, &ctr.items[0], false, loc)?;
                    let err = self.build_type_(ctx, &ctr.items[1], false, loc)?;

                    self.store_type(ctx, ctr.id, &Type::Result(ResultType { ok, err }));
                    return Ok(());
                }

                if let TypeExpr::Named(ident) = &*ctr.container
                    && ident.repr == "seg"
                {
                    if ctr.items.len() != 1 {
                        return Err(Error {
                            message: format!(
                                "Expected exactly 2 type arguments, {} was found",
                                ctr.items.len()
                            ),
                            loc: ident.loc,
                        });
                    }

                    let item = self.build_type_(ctx, &ctr.items[0], false, loc)?;

                    self.store_type(ctx, ctr.id, &Type::Seg(SegType { item }));
                    return Ok(());
                }

                if let TypeExpr::Named(ident) = &*ctr.container
                    && ident.repr == "typeof"
                {
                    if ctr.items.len() != 1 {
                        return Err(Error {
                            message: format!(
                                "Expected exactly 1 type arguments, {} was found",
                                ctr.items.len()
                            ),
                            loc: ident.loc,
                        });
                    }

                    let TypeExpr::Named(ident) = &ctr.items[0] else {
                        return Err(Error {
                            message: format!("Symbol expected"),
                            loc: ctr.items[0].loc(),
                        });
                    };

                    let var_symbol = self.get_value_info(ctx, &ident)?;
                    self.store_expr_info(ctx, ctr.id, var_symbol.type_id);
                    return Ok(());
                }

                if let TypeExpr::Named(ident) = &*ctr.container
                    && ident.repr == "itemof"
                {
                    if ctr.items.len() != 1 {
                        return Err(Error {
                            message: format!(
                                "Expected exactly 1 type arguments, {} was found",
                                ctr.items.len()
                            ),
                            loc: ident.loc,
                        });
                    }

                    let container = self.build_type_(ctx, &ctr.items[0], true, loc)?;
                    let container = deref_rec(&self.registry, self.get_type(container));

                    let Type::Container(ctr_type) = container else {
                        return Err(Error {
                            message: format!("Expected container type"),
                            loc: ctr.items[0].loc(),
                        });
                    };

                    self.store_expr_info(ctx, ctr.id, ctr_type.items[0]);
                    return Ok(());
                }

                let container = self.build_type_(ctx, &ctr.container, is_referenced, loc)?;

                let mut items = Vec::new();
                for item in &ctr.items {
                    items.push(self.build_type_(ctx, item, true, loc)?);
                }

                self.store_type(
                    ctx,
                    ctr.id,
                    &Type::Container(ContainerType { container, items }),
                );
                Ok(())
            }
        }
    }

    fn get_type_id_or_err(
        &self,
        ctx: TyContextRef,
        type_name: &str,
        loc: &Loc,
    ) -> Result<TypeId, Error> {
        let Some(symbol) = self.get_symbol(ctx, type_name) else {
            return Err(Error {
                message: format!("Unknown type: {}", type_name),
                loc: *loc,
            });
        };

        match symbol.kind {
            TySymbolKind::Struct => {
                if self.reporter.in_inspection_mode {
                    self.reporter.print_inspection(InspectInfo {
                        message: format!("struct {type_name} {{ ... }}"),
                        loc: *loc,
                        linked_loc: Some(symbol.loc),
                    });
                }

                Ok(self.intern_type(&Type::Struct {
                    struct_index: symbol.col_index,
                }))
            }
            TySymbolKind::Enum => {
                if self.reporter.in_inspection_mode {
                    self.reporter.print_inspection(InspectInfo {
                        message: format!("enum {type_name} {{ ... }}"),
                        loc: *loc,
                        linked_loc: Some(symbol.loc),
                    });
                }

                Ok(self.intern_type(&Type::Enum {
                    enum_index: symbol.col_index,
                }))
            }
            TySymbolKind::TypeAlias => {
                let type_id = self.type_aliases[symbol.col_index];

                // don't print inspection for built-ins
                if self.reporter.in_inspection_mode && symbol.loc.file_id != 0 {
                    self.reporter.print_inspection(InspectInfo {
                        message: format!(
                            "type {type_name} = {}",
                            self.registry.fmt(self.get_type(type_id))
                        ),
                        loc: *loc,
                        linked_loc: Some(symbol.loc),
                    });
                }

                Ok(type_id)
            }
            TySymbolKind::Local
            | TySymbolKind::Global
            | TySymbolKind::Const
            | TySymbolKind::Function
            | TySymbolKind::InlineFn
            | TySymbolKind::EnumConstructor => Err(Error {
                message: format!("Symbol is not a type: {}", type_name),
                loc: *loc,
            }),
        }
    }

    fn get_const_value(&self, ctx: TyContextRef, expr: &CodeExpr) -> Option<&CodeExpr> {
        match expr {
            CodeExpr::BoolLiteral(_)
            | CodeExpr::CharLiteral(_)
            | CodeExpr::NullLiteral(_)
            | CodeExpr::IntLiteral(_)
            | CodeExpr::StringLiteral(_)
            | CodeExpr::StructLiteral(_)
            | CodeExpr::ArrayLiteral(_)
            | CodeExpr::ResultLiteral(_) => {
                // all literals are valid const values
                return Some(expr.relax());
            }

            CodeExpr::Ident(ident_expr) => {
                let Some(symbol) = self.get_symbol(ctx, ident_expr.repr) else {
                    return None;
                };

                if let TySymbolKind::Const = symbol.kind {
                    return Some(&self.registry.constants[symbol.col_index].expr);
                }
            }

            _ => { /* fallthrough */ }
        }

        self.report_error(Error {
            message: format!("Expression not allowed in const context"),
            loc: expr.loc(),
        });
        None
    }

    fn alloc_str(&mut self, value: String) -> &'static str {
        let str_ref = value.as_str().relax();
        self.allocated_strings.push(value);
        str_ref
    }

    fn extend_expr_info_storage(&self, expr_count: usize) {
        self.registry.be_mut().expr_info.resize(
            self.registry.expr_info.len() + expr_count,
            EXPR_INFO_INVALID,
        );
    }

    #[inline]
    fn add_builtin_type(&mut self, ctx: &mut TyContext, type_: Type) {
        ctx.symbols.push(TySymbol {
            ctx_id: 0,
            name: type_.to_str().unwrap(),
            kind: TySymbolKind::TypeAlias,
            col_index: self.type_aliases.len(),
            loc: Loc::internal(),
        });

        let type_id = self.intern_type(&type_);
        self.type_aliases.push(type_id);
    }

    fn define_symbol<'a>(
        &self,
        ctx: TyContextRef,
        symbol_name: &'static str,
        symbol_kind: TySymbolKind,
        symbol_loc: Loc,
    ) -> bool {
        let symbol_col_index = match &symbol_kind {
            TySymbolKind::Local => self.locals_types.len(),
            TySymbolKind::Global => self.registry.globals.len(),
            TySymbolKind::Const => self.registry.constants.len(),

            TySymbolKind::InlineFn => self.registry.inline_fns.len(),
            TySymbolKind::Function => self.registry.functions.len(),

            TySymbolKind::TypeAlias => self.type_aliases.len(),
            TySymbolKind::Struct => self.registry.structs.len(),
            TySymbolKind::Enum => self.registry.enums.len(),
            TySymbolKind::EnumConstructor => self.registry.enum_ctors.len(),
        };

        if let Some(existing_symbol) = self.get_symbol(ctx, symbol_name)
            && existing_symbol.ctx_id == ctx.id
        {
            self.report_error(Error {
                message: format!(
                    "Cannot redefine {}, previously defined at {}",
                    symbol_name,
                    existing_symbol.loc.to_string(&self.registry)
                ),
                loc: symbol_loc,
            });
            return false;
        }

        ctx.symbols.be_mut().push(TySymbol {
            ctx_id: ctx.id,
            name: symbol_name,
            kind: symbol_kind,
            col_index: symbol_col_index,
            loc: symbol_loc,
        });

        true
    }

    fn get_symbol(&self, ctx: TyContextRef, symbol_name: &str) -> Option<&'static TySymbol> {
        for symbol in ctx.symbols.iter().rev() {
            if symbol.name == symbol_name {
                return Some(symbol.relax());
            }
        }

        if let Some(parent) = ctx.parent {
            return self.get_symbol(parent, symbol_name);
        }

        None
    }

    fn child_ctx(&self, parent: TyContextRef, scope_kind: ScopeKind) -> TyContextRef {
        self.contexts.be_mut().push(TyContext {
            id: self.contexts.len(),
            parent: Some(parent),
            kind: scope_kind,
            module_id: parent.module_id,
            fn_index: parent.fn_index,
            expr_id_offset: parent.expr_id_offset,
            symbols: Vec::new(),
        })
    }

    fn load_type(&self, ctx: TyContextRef, expr: &CodeExpr) -> Option<TypeId> {
        self.registry.get_expr_type(ctx.expr_id_offset, expr)
    }

    fn store_type(&self, ctx: TyContextRef, expr_id: ExprId, type_: &Type) {
        let type_id = self.intern_type(type_);
        self.store_expr_info(ctx, expr_id, type_id)
    }

    fn store_expr_info(&self, ctx: TyContextRef, expr_id: ExprId, expr_info: ExprInfo) {
        let absolute_expr_id = ctx.expr_id_offset + expr_id;
        self.registry.be_mut().expr_info[absolute_expr_id] = expr_info;
    }

    fn intern_type(&self, type_: &Type) -> TypeId {
        if let Some(&id) = self.type_lookup.get(type_) {
            return id;
        }

        let id = self.registry.types.len();
        self.type_lookup.be_mut().insert(type_.clone(), id);
        self.registry.be_mut().types.push(type_.clone());
        id
    }

    fn get_type(&self, type_id: TypeId) -> &'static Type {
        self.registry.get_type(type_id)
    }

    fn report_if_err<T>(&self, res: Result<T, Error>) -> Option<T> {
        match res {
            Ok(value) => Some(value),
            Err(err) => {
                self.report_error(err);
                None
            }
        }
    }

    fn report_error(&self, err: Error) {
        self.reporter.error(err);
    }
}

pub struct IncludeInfo {
    pub file_path: UBRef<StringLiteralExpr>,
    pub alias: Option<String>,
    pub with_extern: bool,
}
pub fn get_include_info(expr: &TopLevelExpr) -> Result<Option<IncludeInfo>, Error> {
    let TopLevelExpr::Intrinsic(InlineFnCallExpr {
        id: _,
        fn_name: intrinsic,
        type_args,
        args,
        loc: _,
    }) = expr
    else {
        return Ok(None);
    };
    if intrinsic.repr != "include" {
        return Ok(None);
    }

    if type_args.len() != 0 {
        return Err(bad_signature(intrinsic));
    }

    let mut file_path = None;
    let mut alias = None;
    let mut with_extern = false;

    for arg in &args.items {
        if let CodeExpr::StringLiteral(str) = arg {
            file_path = Some(str.relax());
            continue;
        }

        if let CodeExpr::Ident(ident) = arg
            && ident.repr == "with_extern"
        {
            with_extern = true;
            continue;
        }

        if let CodeExpr::Assign(AssignExpr { lhs, rhs, .. }) = arg
            && let CodeExpr::Ident(IdentExpr { repr: "alias", .. }) = &**lhs
            && let CodeExpr::StringLiteral(str) = &**rhs
        {
            alias = Some(str.value.clone());
            continue;
        }
    }

    let Some(file_path) = file_path else {
        return Err(bad_signature(intrinsic));
    };

    return Ok(Some(IncludeInfo {
        file_path: UBRef::new(file_path),
        alias,
        with_extern,
    }));

    fn bad_signature(fn_name: &IdentExpr) -> Error {
        Error {
            message: format!(
                "Invalid call, expected signature: @{}(<str-literal>, [with_extern, alias = <str-literal>])",
                fn_name.repr
            ),
            loc: fn_name.loc,
        }
    }
}

pub fn is_noop_cast(primitives1: &[Type], primitives2: &[Type]) -> bool {
    if primitives1.len() != primitives2.len() {
        return false;
    }

    for (p1, p2) in primitives1.iter().zip(primitives2) {
        if let Some(p1_wide) = is_wide_int(p1)
            && let Some(p2_wide) = is_wide_int(p2)
            && p1_wide == p2_wide
        {
        } else {
            return false;
        }
    }

    true
}

pub fn count_primitive_components(registry: &Registry, type_: &Type) -> u32 {
    let layout = &mut TypeLayout::new();
    get_type_layout(registry, type_, layout);
    layout.primitives_count
}

pub fn get_type_layout(registry: &Registry, type_: &Type, layout: &mut TypeLayout) {
    match type_ {
        Type::Never | Type::Void => {
            layout.alignment = u32::max(layout.alignment, 1);
        }
        Type::Bool | Type::U8 | Type::I8 => {
            if let Some(primitives) = &mut layout.primitives {
                primitives.push(type_.clone())
            }
            layout.primitives_count += 1;
            layout.alignment = u32::max(layout.alignment, 1);
            layout.byte_size = align(layout.byte_size, 1) + 1;
        }
        Type::U16 | Type::I16 => {
            if let Some(primitives) = &mut layout.primitives {
                primitives.push(type_.clone())
            }
            layout.primitives_count += 1;
            layout.alignment = u32::max(layout.alignment, 2);
            layout.byte_size = align(layout.byte_size, 2) + 2;
        }
        Type::U32 | Type::I32 | Type::F32 | Type::Null | Type::Pointer { .. } => {
            if let Some(primitives) = &mut layout.primitives {
                if let Type::Null | Type::Pointer { .. } = type_ {
                    primitives.push(Type::U32)
                } else {
                    primitives.push(type_.clone())
                }
            }
            layout.primitives_count += 1;
            layout.alignment = u32::max(layout.alignment, 4);
            layout.byte_size = align(layout.byte_size, 4) + 4;
        }
        Type::U64 | Type::I64 | Type::F64 => {
            if let Some(primitives) = &mut layout.primitives {
                primitives.push(type_.clone())
            }
            layout.primitives_count += 1;
            layout.alignment = u32::max(layout.alignment, 8);
            layout.byte_size = align(layout.byte_size, 8) + 8;
        }
        Type::Struct { struct_index } => {
            let struct_def = &registry.structs[*struct_index];

            for field in &struct_def.fields {
                get_type_layout(registry, &field.field_type, layout);
            }

            layout.alignment = u32::max(layout.alignment, 1);
            layout.byte_size = align(layout.byte_size, layout.alignment);
        }
        Type::Enum { enum_index } => {
            let enum_def = &registry.enums[*enum_index];

            get_type_layout(registry, &Type::U32, layout);
            get_type_layout(registry, &enum_def.variant_type, layout);

            layout.byte_size = align(layout.byte_size, layout.alignment);
        }
        Type::Result(result) => {
            get_type_layout(registry, registry.get_type(result.ok), layout);
            get_type_layout(registry, registry.get_type(result.err), layout);

            layout.byte_size = align(layout.byte_size, layout.alignment);
        }
        Type::Seg(seg) => {
            get_type_layout(registry, &Type::U32, layout);
            get_type_layout(registry, registry.get_type(seg.item), layout);

            layout.byte_size = align(layout.byte_size, layout.alignment);
        }
        Type::Container(ctr) => {
            get_type_layout(registry, registry.get_type(ctr.container), layout);
        }
    }
}

pub fn deref_rec<'a>(registry: &'a Registry, type_: &'a Type) -> &'a Type {
    match type_ {
        Type::Pointer(ptr) => deref_rec(registry, registry.get_type(ptr.pointee)),
        _ => type_,
    }
}

fn is_wide_int_tag(registry: &Registry, tag_type: &Type, loc: &Loc) -> Result<bool, Error> {
    let Some(wide) = is_wide_int(tag_type) else {
        return Err(Error {
            message: format!("{} is not a valid int tag", registry.fmt(tag_type)),
            loc: *loc,
        });
    };
    return Ok(wide);
}

pub fn is_wide_int(type_: &Type) -> Option<bool> {
    match type_ {
        Type::U64 | Type::I64 => Some(true),
        Type::Bool
        | Type::U8
        | Type::I8
        | Type::U16
        | Type::I16
        | Type::U32
        | Type::I32
        | Type::Null
        | Type::Pointer { .. } => Some(false),
        _ => None,
    }
}

pub fn get_struct_field<'a>(
    registry: &'a Registry,
    mut lhs_type: &Type,
    field_access: &FieldAccessExpr,
) -> Result<&'a StructField, Error> {
    if let Type::Pointer(ptr) = &lhs_type
        && !ptr.is_sequence
    {
        lhs_type = registry.get_type(ptr.pointee);
    }

    if let Type::Container(ctr) = &lhs_type {
        lhs_type = registry.get_type(ctr.container);
    }

    let Type::Struct { struct_index } = lhs_type else {
        return Err(Error {
            message: format!(
                "Cannot get field '{}' on non struct: {}",
                field_access.field_name.repr,
                registry.fmt(lhs_type),
            ),
            loc: field_access.field_name.loc,
        });
    };

    let struct_def = &registry.structs[*struct_index];
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

pub fn get_infer_type_name(fn_param: &FnParam) -> Result<Option<&'static str>, Error> {
    let FnParamType::Type {
        expr: TypeExpr::Container(ctr),
    } = &fn_param.param_type
    else {
        return Ok(None);
    };

    let TypeExpr::Named(ident) = &*ctr.container else {
        return Ok(None);
    };

    if ident.repr != "infer" {
        return Ok(None);
    }

    if ctr.items.len() != 1 {
        return Err(Error {
            message: format!("Invalid `infer` call, expected 1 named type argument"),
            loc: ctr.loc,
        });
    }

    let TypeExpr::Named(ident) = &ctr.items[0] else {
        return Err(Error {
            message: format!("Invalid `infer` call, expected 1 named type argument"),
            loc: ctr.loc,
        });
    };

    Ok(Some(ident.repr))
}

pub fn is_types_compatible(registry: &Registry, slots: &[TypeId], values: &[TypeId]) -> bool {
    if slots.len() != values.len() {
        return false;
    }

    for i in 0..slots.len() {
        if !is_type_compatible(
            registry,
            registry.get_type(slots[i]),
            registry.get_type(values[i]),
        ) {
            return false;
        }
    }

    true
}

pub fn is_type_compatible(registry: &Registry, slot: &Type, value: &Type) -> bool {
    if let Type::Pointer(ptr) = slot
        && !ptr.is_sequence
    {
        if *value == Type::Null {
            return true;
        }

        if let Type::Void = registry.get_type(ptr.pointee) {
            if let Type::Pointer(_) = value {
                return true;
            }
        }

        if let Type::Pointer(value_ptr) = value {
            return is_type_compatible(
                registry,
                registry.get_type(ptr.pointee),
                registry.get_type(value_ptr.pointee),
            );
        }
    }

    if let Type::Container(ctr) = value {
        if let Type::Container(slot_ctr) = slot {
            return is_type_compatible(
                registry,
                registry.get_type(slot_ctr.container),
                registry.get_type(ctr.container),
            ) && is_types_compatible(registry, &slot_ctr.items, &ctr.items);
        }

        // TODO: allow this for self arguments only
        return is_type_compatible(registry, slot, registry.get_type(ctr.container));
    }

    if *value == Type::Never {
        return true;
    }

    slot == value
}

pub fn is_naturally_divergent(expr: &CodeExpr) -> bool {
    match expr {
        CodeExpr::BoolLiteral(_)
        | CodeExpr::CharLiteral(_)
        | CodeExpr::NullLiteral(_)
        | CodeExpr::IntLiteral(_)
        | CodeExpr::StringLiteral(_)
        | CodeExpr::StructLiteral(_)
        | CodeExpr::ArrayLiteral(_)
        | CodeExpr::ResultLiteral(_)
        | CodeExpr::Ident(_)
        | CodeExpr::Let(_)
        | CodeExpr::Cast(_)
        | CodeExpr::Assign(_)
        | CodeExpr::FieldAccess(_)
        | CodeExpr::PropagateError(_)
        | CodeExpr::FnCall(_)
        | CodeExpr::MethodCall(_)
        | CodeExpr::InlineFnCall(_)
        | CodeExpr::If(_)
        | CodeExpr::Sizeof(_)
        | CodeExpr::While(_)
        | CodeExpr::For(_)
        | CodeExpr::Defer(_)
        | CodeExpr::InlineMethodCall(_) => false,

        CodeExpr::Break(_) | CodeExpr::Continue(_) | CodeExpr::Return(_) => true,

        CodeExpr::Paren(paren_expr) => is_naturally_divergent(&paren_expr.expr),
        CodeExpr::IntrinsicCall(intrinsic) => intrinsic.fn_name.repr == "unreachable",

        CodeExpr::Catch(catch_) => is_naturally_divergent(&catch_.lhs),
        CodeExpr::InfixOp(infix) => {
            is_naturally_divergent(&infix.lhs) || is_naturally_divergent(&infix.rhs)
        }
        CodeExpr::PrefixOp(prefix) => is_naturally_divergent(&prefix.expr),
        CodeExpr::Match(match_) => is_naturally_divergent(&match_.header.expr_to_match),
        CodeExpr::Pipe(pipe) => {
            is_naturally_divergent(&pipe.lhs) || is_naturally_divergent(&pipe.rhs)
        }

        CodeExpr::DoWith(do_with_) => {
            let mut divergent = is_naturally_divergent(&do_with_.body);
            for expr in &do_with_.args.items {
                divergent = divergent || is_naturally_divergent(expr);
            }
            divergent
        }
    }
}

pub struct TypeFmt<'a> {
    pub registry: &'a Registry,
    pub type_: &'a Type,
}

impl<'a> core::fmt::Display for TypeFmt<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.type_ {
            Type::Never
            | Type::Null
            | Type::Void
            | Type::Bool
            | Type::U8
            | Type::I8
            | Type::U16
            | Type::I16
            | Type::U32
            | Type::I32
            | Type::F32
            | Type::U64
            | Type::I64
            | Type::F64 => write!(f, "{}", self.type_.to_str().unwrap()),
            Type::Pointer(ptr) => {
                write!(f, "&")?;
                if ptr.is_nullable {
                    write!(f, "?")?;
                }
                if ptr.is_sequence {
                    write!(f, "[]")?;
                }
                write!(
                    f,
                    "{}",
                    self.registry.fmt(self.registry.get_type(ptr.pointee))
                )
            }
            Type::Struct { struct_index } => {
                f.write_str(&self.registry.structs[*struct_index].struct_name)
            }
            Type::Enum { enum_index } => f.write_str(&self.registry.enums[*enum_index].enum_name),
            Type::Result(result) => {
                write!(
                    f,
                    "Result({}, {})",
                    self.registry.fmt(self.registry.get_type(result.ok)),
                    self.registry.fmt(self.registry.get_type(result.err))
                )
            }
            Type::Seg(seg) => {
                write!(
                    f,
                    "seg({})",
                    self.registry.fmt(self.registry.get_type(seg.item)),
                )
            }
            Type::Container(ctr) => {
                write!(
                    f,
                    "{}",
                    self.registry.fmt(self.registry.get_type(ctr.container))
                )?;
                write!(f, "(")?;
                for (i, item) in ctr.items.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", self.registry.fmt(self.registry.get_type(*item)))?;
                }
                write!(f, ")")
            }
        }
    }
}

pub struct TypeListFmt<'a> {
    pub registry: &'a Registry,
    pub type_ids: &'a [TypeId],
}

impl<'a> core::fmt::Display for TypeListFmt<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (i, item) in self.type_ids.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", self.registry.fmt(self.registry.get_type(*item)))?;
        }
        Ok(())
    }
}

struct StableVec<T> {
    chunks: Vec<Vec<T>>,
    chunk_size: usize,
}

impl<T> Default for StableVec<T> {
    fn default() -> Self {
        Self {
            chunks: Default::default(),
            chunk_size: 8,
        }
    }
}

impl<T> StableVec<T> {
    fn push(&mut self, value: T) -> UBRef<T> {
        if self.chunks.len() == 0 || self.chunks.last().unwrap().len() == self.chunk_size {
            self.chunks.push(Vec::with_capacity(self.chunk_size))
        }

        let last_chunk = self.chunks.last_mut().unwrap();
        last_chunk.push(value);
        UBRef::new(last_chunk.last_mut().unwrap())
    }

    fn len(&self) -> usize {
        let chunks_len = self.chunks.len();
        if chunks_len == 0 {
            return 0;
        }

        (chunks_len - 1) * self.chunk_size + self.chunks[chunks_len - 1].len()
    }
}
