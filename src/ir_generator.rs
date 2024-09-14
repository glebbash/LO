use crate::{ast::*, core::*, lexer::*, parser_v2::*, wasm::*};
use alloc::{boxed::Box, format, string::String, vec::Vec};

#[derive(Clone, PartialEq)]
pub enum LoType {
    Never,
    Void,
    Bool,
    U32,
}

impl core::fmt::Display for LoType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LoType::Never => f.write_str("never"),
            LoType::Void => f.write_str("void"),
            LoType::Bool => f.write_str("bool"),
            LoType::U32 => f.write_str("u32"),
        }
    }
}

#[derive(Default)]
pub struct CodeBlock {
    pub exprs: Vec<LoExpr>,
    pub scope: LoScope,
}

pub enum LoExpr {
    Casted {
        expr: Box<LoExpr>,
        casted_to: LoType,
    },

    Void,
    U32Const {
        value: u32,
    },
    Return {
        expr: Box<LoExpr>,
    },
    BinaryOp {
        kind: WasmBinaryOpKind,
        lhs: Box<LoExpr>,
        rhs: Box<LoExpr>,
    },
    VarLoad {
        name: String,
        var_type: LoType,
    },
    If {
        cond: Box<LoExpr>,
        then_block: CodeBlock,
        else_block: Option<CodeBlock>,
    },
    Call {
        fn_name: String,
        args: Vec<LoExpr>,
        return_type: LoType,
    },
}

impl LoExpr {
    pub fn get_type(&self) -> LoType {
        match self {
            LoExpr::Casted { casted_to, .. } => casted_to.clone(),

            LoExpr::Void { .. } => LoType::Void,
            LoExpr::U32Const { .. } => LoType::U32,
            LoExpr::Return { .. } => LoType::Never,
            LoExpr::BinaryOp { lhs, .. } => lhs.get_type(),
            LoExpr::VarLoad { var_type, .. } => var_type.clone(),
            LoExpr::If { .. } => LoType::Void,
            LoExpr::Call { return_type, .. } => return_type.clone(),
        }
    }
}

#[derive(Default)]
pub struct LoScope {
    pub vars: Vec<LoVar>,
    pub fn_defs: Vec<LoFnDef>,
}

pub struct LoVar {
    pub name: String,
    pub type_: LoType,
}

pub struct LoFnDef {
    pub name: String,
    pub inputs: Vec<LoType>,
    pub output: LoType,
    pub exported: bool,
    pub body: CodeBlock,
}

#[derive(Default)]
struct LoScopeStack {
    scopes: Vec<LoScope>,
}

impl LoScopeStack {
    fn push(&mut self, scope: LoScope) {
        self.scopes.push(scope);
    }

    fn pop(&mut self) -> LoScope {
        self.scopes.pop().unwrap()
    }

    fn top(&mut self) -> &mut LoScope {
        let parent_index = self.scopes.len() - 1;
        &mut self.scopes[parent_index]
    }

    fn get_var(&self, name: &str) -> Option<&LoVar> {
        for scope in self.scopes.iter().rev() {
            for var in &scope.vars {
                if var.name == *name {
                    return Some(var);
                }
            }
        }
        None
    }

    fn get_fn_def(&self, name: &str) -> Option<&LoFnDef> {
        for scope in self.scopes.iter().rev() {
            for var in &scope.fn_defs {
                if var.name == *name {
                    return Some(var);
                }
            }
        }

        None
    }

    fn get_fn_def_mut(&mut self, name: &str) -> Option<&mut LoFnDef> {
        for scope in self.scopes.iter_mut().rev() {
            for var in &mut scope.fn_defs {
                if var.name == *name {
                    return Some(var);
                }
            }
        }

        None
    }
}

#[derive(Default)]
pub struct IRGenerator {
    pub errors: LoErrorManager,
    ss: LoScopeStack,
}

impl IRGenerator {
    pub fn generate_ir(&mut self) -> Result<LoScope, LoError> {
        Ok(self.ss.pop())
    }

    pub fn process_file(&mut self, file: &FileInfo) -> Result<(), LoError> {
        if self.ss.scopes.len() == 0 {
            self.ss.push(LoScope::default());
        }

        for expr in &file.ast.exprs {
            match expr {
                TopLevelExpr::Include(_) => {} // skip, processed earlier
                TopLevelExpr::FnDef(fn_def) => self.process_fn_def(fn_def)?,
                TopLevelExpr::Import(_) => return Err(LoError::todo(file!(), line!())),
                TopLevelExpr::GlobalDef(_) => return Err(LoError::todo(file!(), line!())),
                TopLevelExpr::StructDef(_) => return Err(LoError::todo(file!(), line!())),
                TopLevelExpr::TypeDef(_) => return Err(LoError::todo(file!(), line!())),
                TopLevelExpr::ConstDef(_) => return Err(LoError::todo(file!(), line!())),
                TopLevelExpr::MemoryDef(_) => return Err(LoError::todo(file!(), line!())),
            }
        }

        Ok(())
    }

    fn process_fn_def(&mut self, fn_def: &FnDefExpr) -> Result<(), LoError> {
        let return_type = match &fn_def.decl.return_type {
            Some(return_type) => self.build_type(return_type)?,
            _ => LoType::Void,
        };

        let mut scope = LoScope::default();

        let mut lo_inputs = Vec::new();
        for fn_param in &fn_def.decl.fn_params {
            for var in &scope.vars {
                if var.name == fn_param.name {
                    self.errors.report(LoError {
                        message: format!("Duplicate function parameter name: {}", fn_param.name),
                        loc: fn_param.loc.clone(),
                    });
                    continue;
                }
            }

            let Some(param_type_expr) = &fn_param.type_ else {
                return Err(LoError::todo(file!(), line!()));
            };
            let param_type = self.build_type(param_type_expr)?;
            lo_inputs.push(param_type.clone());

            scope.vars.push(LoVar {
                name: fn_param.name.clone(),
                type_: param_type,
            });
        }

        self.ss.top().fn_defs.push(LoFnDef {
            name: fn_def.decl.fn_name.repr.clone(),
            inputs: lo_inputs,
            output: return_type,
            exported: fn_def.exported,
            body: CodeBlock::default(),
        });

        self.ss.push(scope);

        let exprs = self.build_code_block(&fn_def.body)?;

        let scope = self.ss.pop();
        self.ss
            .get_fn_def_mut(&fn_def.decl.fn_name.repr)
            .unwrap()
            .body = CodeBlock { exprs, scope };

        Ok(())
    }

    fn build_type(&mut self, type_expr: &TypeExpr) -> Result<LoType, LoError> {
        match type_expr {
            TypeExpr::U32 => Ok(LoType::U32),
            TypeExpr::AliasOrStruct { .. } => Err(LoError::todo(file!(), line!())),
            TypeExpr::Result { .. } => Err(LoError::todo(file!(), line!())),
        }
    }

    fn build_code_block(&mut self, code_block: &CodeBlockExpr) -> Result<Vec<LoExpr>, LoError> {
        let mut exprs = Vec::new();
        for expr in &code_block.exprs {
            exprs.push(self.build_code_expr(expr)?);
        }

        Ok(exprs)
    }

    fn build_code_expr(&mut self, code_expr: &CodeExpr) -> Result<LoExpr, LoError> {
        match code_expr {
            CodeExpr::Return(return_expr) => {
                let mut lo_expr = LoExpr::Void;
                if let Some(return_expr) = &return_expr.expr {
                    lo_expr = self.build_code_expr(&return_expr)?;
                }

                Ok(LoExpr::Return {
                    expr: Box::new(lo_expr),
                })
            }
            CodeExpr::IntLiteral(int_expr) => Ok(LoExpr::U32Const {
                value: int_expr.value,
            }),
            CodeExpr::Ident(var_load) => {
                let Some(var) = self.ss.get_var(&var_load.repr) else {
                    return Err(LoError {
                        message: format!("Cannot read unknown variable: {}", var_load.repr),
                        loc: var_load.loc.clone(),
                    });
                };

                Ok(LoExpr::VarLoad {
                    name: var.name.clone(),
                    var_type: var.type_.clone(),
                })
            }
            CodeExpr::BinaryOp(BinaryOpExpr {
                op_tag,
                lhs,
                rhs,
                loc,
            }) => {
                let lo_lhs = self.build_code_expr(lhs)?;
                let lo_rhs = self.build_code_expr(rhs)?;

                let lhs_type = lo_lhs.get_type();
                let rhs_type = lo_rhs.get_type();
                if lhs_type != rhs_type {
                    return Err(LoError {
                        message: format!(
                            "Operands have different types: {lhs_type} and {rhs_type}"
                        ),
                        loc: loc.clone(),
                    });
                }

                fn unsupported_op(
                    type_: &LoType,
                    op_tag: &InfixOpTag,
                    loc: &LoLocation,
                ) -> LoError {
                    let op = op_tag.to_str();
                    LoError {
                        message: format!(
                            "Operator {op} is not applicable to operands of type {type_}",
                        ),
                        loc: loc.clone(),
                    }
                }

                let mut cast_to = None;
                let kind = match op_tag {
                    InfixOpTag::Less => {
                        cast_to = Some(LoType::Bool);
                        match lhs_type {
                            LoType::U32 => WasmBinaryOpKind::I32_LT_U,
                            _ => return Err(unsupported_op(&lhs_type, op_tag, loc)),
                        }
                    }

                    InfixOpTag::Equal
                    | InfixOpTag::NotEqual
                    | InfixOpTag::Greater
                    | InfixOpTag::LessEqual
                    | InfixOpTag::GreaterEqual => return Err(LoError::todo(file!(), line!())),

                    InfixOpTag::And | InfixOpTag::Or => {
                        return Err(LoError::todo(file!(), line!()))
                    }

                    InfixOpTag::Add => match lhs_type {
                        LoType::U32 => WasmBinaryOpKind::I32_ADD,
                        _ => return Err(unsupported_op(&lhs_type, op_tag, loc)),
                    },
                    InfixOpTag::Sub => match lhs_type {
                        LoType::U32 => WasmBinaryOpKind::I32_SUB,
                        _ => return Err(unsupported_op(&lhs_type, op_tag, loc)),
                    },
                    InfixOpTag::Mul => match lhs_type {
                        LoType::U32 => WasmBinaryOpKind::I32_MUL,
                        _ => return Err(unsupported_op(&lhs_type, op_tag, loc)),
                    },

                    InfixOpTag::ShiftRight => match lhs_type {
                        LoType::U32 => WasmBinaryOpKind::I32_SHR_U,
                        _ => return Err(unsupported_op(&lhs_type, op_tag, loc)),
                    },

                    InfixOpTag::Div
                    | InfixOpTag::Mod
                    | InfixOpTag::BitAnd
                    | InfixOpTag::BitOr
                    | InfixOpTag::ShiftLeft => return Err(LoError::todo(file!(), line!())),

                    InfixOpTag::Assign
                    | InfixOpTag::AddAssign
                    | InfixOpTag::SubAssign
                    | InfixOpTag::MulAssign
                    | InfixOpTag::DivAssign
                    | InfixOpTag::ModAssign
                    | InfixOpTag::BitAndAssign
                    | InfixOpTag::BitOrAssign
                    | InfixOpTag::ShiftLeftAssign
                    | InfixOpTag::ShiftRightAssign
                    | InfixOpTag::Cast
                    | InfixOpTag::FieldAccess
                    | InfixOpTag::Catch
                    | InfixOpTag::ErrorPropagation => {
                        return Err(LoError::unreachable(file!(), line!()))
                    }
                };

                let expr = LoExpr::BinaryOp {
                    kind,
                    lhs: Box::new(lo_lhs),
                    rhs: Box::new(lo_rhs),
                };

                if let Some(cast_to) = cast_to {
                    return Ok(LoExpr::Casted {
                        expr: Box::new(expr),
                        casted_to: cast_to,
                    });
                }

                Ok(expr)
            }
            CodeExpr::If(IfExpr {
                cond,
                then_block,
                else_block,
                ..
            }) => {
                let lo_cond = self.build_code_expr(cond)?;

                self.ss.push(LoScope::default());
                let then_exprs = self.build_code_block(&then_block)?;
                let scope = self.ss.pop();
                let lo_then_block = CodeBlock {
                    exprs: then_exprs,
                    scope,
                };

                let lo_else_block = match &else_block {
                    ElseBlock::Else(else_block) => {
                        self.ss.push(LoScope::default());
                        let exprs = self.build_code_block(&else_block)?;
                        let scope = self.ss.pop();
                        let code_block = CodeBlock { exprs, scope };
                        Some(code_block)
                    }
                    ElseBlock::ElseIf(expr) => {
                        let mut exprs = Vec::new();
                        self.ss.push(LoScope::default());
                        exprs.push(self.build_code_expr(expr)?);
                        let scope = self.ss.pop();
                        let else_block = CodeBlock { exprs, scope };
                        Some(else_block)
                    }
                    ElseBlock::None => None,
                };

                Ok(LoExpr::If {
                    cond: Box::new(lo_cond),
                    then_block: lo_then_block,
                    else_block: lo_else_block,
                })
            }
            CodeExpr::FnCall(FnCallExpr { fn_name, args, loc }) => {
                let mut arg_types = Vec::new();
                let mut lo_args = Vec::new();
                for arg in args {
                    let lo_arg = self.build_code_expr(arg)?;
                    arg_types.push(lo_arg.get_type());
                    lo_args.push(lo_arg);
                }

                let Some(fn_def) = self.ss.get_fn_def(&fn_name.repr) else {
                    return Err(LoError {
                        message: format!("Trying to call unknown function {}", fn_name.repr),
                        loc: loc.clone(),
                    });
                };

                if arg_types != fn_def.inputs {
                    return Err(LoError {
                        message: format!(
                            "Invalid function parameters: {}, expected: {}",
                            ListDisplay(&arg_types),
                            ListDisplay(&fn_def.inputs)
                        ),
                        loc: loc.clone(),
                    });
                }

                Ok(LoExpr::Call {
                    fn_name: fn_name.repr.clone(),
                    args: lo_args,
                    return_type: fn_def.output.clone(),
                })
            }
            CodeExpr::BoolLiteral(_) => Err(LoError::todo(file!(), line!())),
            CodeExpr::Local(_) => Err(LoError::todo(file!(), line!())),
            CodeExpr::Loop(_) => Err(LoError::todo(file!(), line!())),
            CodeExpr::Break(_) => Err(LoError::todo(file!(), line!())),
            CodeExpr::ForLoop(_) => Err(LoError::todo(file!(), line!())),
            CodeExpr::Continue(_) => Err(LoError::todo(file!(), line!())),
            CodeExpr::StringLiteral(_) => Err(LoError::todo(file!(), line!())),
            CodeExpr::Dbg(_) => Err(LoError::todo(file!(), line!())),
            CodeExpr::Defer(_) => Err(LoError::todo(file!(), line!())),
            CodeExpr::Cast(_) => Err(LoError::todo(file!(), line!())),
            CodeExpr::StructInit(_) => Err(LoError::todo(file!(), line!())),
            CodeExpr::Assign(_) => Err(LoError::todo(file!(), line!())),
            CodeExpr::FieldAccess(_) => Err(LoError::todo(file!(), line!())),
            CodeExpr::MethodCall(_) => Err(LoError::todo(file!(), line!())),
            CodeExpr::Catch(_) => Err(LoError::todo(file!(), line!())),
        }
    }
}
