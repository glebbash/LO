use crate::{ast::*, core::*, lexer::InfixOpTag, parser_v2::*, wasm::*};
use alloc::{boxed::Box, format, string::String, vec::Vec};

#[derive(Clone, PartialEq)]
enum LoType {
    Never,
    Void,
    Bool,
    U32,
}

impl LoType {
    fn emit_components(&self, out: &mut Vec<WasmType>) {
        match self {
            LoType::Never => {}
            LoType::Void => {}
            LoType::Bool => out.push(WasmType::I32),
            LoType::U32 => out.push(WasmType::I32),
        }
    }
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

enum LoExpr {
    Casted {
        expr: Box<LoExpr>,
        casted_to: LoType,
    },

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
        then_block: Vec<LoExpr>,
        else_block: Option<Vec<LoExpr>>,
    },
    Call {
        fn_name: String,
        args: Vec<LoExpr>,
        return_type: LoType,
    },
}

impl LoExpr {
    fn get_type(&self) -> LoType {
        match self {
            LoExpr::Casted { casted_to, .. } => casted_to.clone(),

            LoExpr::U32Const { .. } => LoType::U32,
            LoExpr::Return { .. } => LoType::Never,
            LoExpr::BinaryOp { lhs, .. } => lhs.get_type(),
            LoExpr::VarLoad { var_type, .. } => var_type.clone(),
            LoExpr::If { .. } => LoType::Void,
            LoExpr::Call { return_type, .. } => return_type.clone(),
        }
    }

    fn lower_all(exprs: &Vec<Self>, ss: &LoScopeStack, instrs: &mut Vec<WasmInstr>) {
        for expr in exprs {
            expr.lower(ss, instrs);
        }
    }

    fn lower(&self, ss: &LoScopeStack, instrs: &mut Vec<WasmInstr>) {
        match self {
            LoExpr::Casted { expr, .. } => {
                expr.lower(ss, instrs);
            }
            LoExpr::U32Const { value } => {
                instrs.push(WasmInstr::I32Const {
                    value: *value as i32,
                });
            }
            LoExpr::Return { expr } => {
                expr.lower(ss, instrs);
                instrs.push(WasmInstr::Return);
            }
            LoExpr::BinaryOp { kind, lhs, rhs } => {
                lhs.lower(ss, instrs);
                rhs.lower(ss, instrs);
                instrs.push(WasmInstr::BinaryOp { kind: kind.clone() });
            }
            LoExpr::VarLoad { name, .. } => {
                let local_index = ss.get_var(&name).unwrap().index;
                instrs.push(WasmInstr::LocalGet { local_index });
            }
            LoExpr::If {
                cond,
                then_block,
                else_block,
            } => {
                cond.lower(ss, instrs);
                instrs.push(WasmInstr::BlockStart {
                    block_kind: WasmBlockKind::If,
                    block_type: WasmBlockType::NoOut,
                });
                LoExpr::lower_all(then_block, ss, instrs);
                if let Some(else_block) = else_block {
                    instrs.push(WasmInstr::Else);
                    LoExpr::lower_all(else_block, ss, instrs);
                }
                instrs.push(WasmInstr::BlockEnd);
            }
            LoExpr::Call { fn_name, args, .. } => {
                LoExpr::lower_all(args, ss, instrs);
                let fn_index = ss.get_fn_def(&fn_name).unwrap().index;
                instrs.push(WasmInstr::Call { fn_index });
            }
        }
    }
}

#[derive(Default)]
pub struct IRGenerator {
    wasm_module: WasmModule,
    pub errors: LoErrorManager,
}

#[derive(Default)]
struct LoScopeStack {
    scopes: Vec<LoScope>,
}

#[derive(Default)]
struct LoScope {
    vars: Vec<LoVar>,
    fn_defs: Vec<LoFnDef>,
}

struct LoVar {
    name: String,
    type_: LoType,
    index: u32,
}

struct LoFnDef {
    name: String,
    inputs: Vec<LoType>,
    output: LoType,
    index: u32,
}

impl LoScopeStack {
    fn add_new(&mut self) -> &mut LoScope {
        self.scopes.push(LoScope::default());
        self.scopes.last_mut().unwrap()
    }

    fn drop_last(&mut self) {
        self.scopes.pop();
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
}

impl IRGenerator {
    pub fn process_file(&mut self, file: &FileInfo) -> Result<(), LoError> {
        let mut ss = LoScopeStack::default();

        ss.add_new();

        for expr in &file.ast.exprs {
            match expr {
                TopLevelExpr::Include(_) => {} // skip, processed earlier

                TopLevelExpr::FnDef(fn_def) => self.process_fn_def(&mut ss, fn_def)?,
            }
        }

        ss.drop_last();

        Ok(())
    }

    fn process_fn_def(&mut self, ss: &mut LoScopeStack, fn_def: &FnDefExpr) -> Result<(), LoError> {
        let scope = ss.add_new();

        let mut fn_type = WasmFnType {
            inputs: Vec::new(),
            outputs: Vec::new(),
        };

        let return_type = self.build_type(&fn_def.return_type)?;
        return_type.emit_components(&mut fn_type.outputs);

        let mut lo_inputs = Vec::new();
        for fn_param in &fn_def.fn_params {
            for var in &scope.vars {
                if var.name == fn_param.name {
                    self.errors.report(LoError {
                        message: format!("Duplicate function parameter name: {}", fn_param.name),
                        loc: fn_param.loc.clone(),
                    });
                    continue;
                }
            }

            let local_index = fn_type.inputs.len() as u32;

            let param_type = self.build_type(&fn_param.type_)?;
            param_type.emit_components(&mut fn_type.inputs);
            lo_inputs.push(param_type.clone());

            scope.vars.push(LoVar {
                name: fn_param.name.clone(),
                type_: param_type,
                index: local_index,
            });
        }

        let mut fn_code = WasmFn {
            locals: Vec::new(),
            expr: WasmExpr { instrs: Vec::new() },
        };

        for input_type in &fn_type.inputs {
            if let Some(locals) = fn_code.locals.last_mut() {
                if locals.value_type == *input_type {
                    locals.count += 1;
                    continue;
                }
            }

            fn_code.locals.push(WasmLocals {
                value_type: input_type.clone(),
                count: 1,
            });
        }

        let type_index = self.wasm_module.types.len() as u32;
        self.wasm_module.types.push(fn_type);

        let fn_index = self.wasm_module.functions.len() as u32;
        self.wasm_module.functions.push(type_index);

        let parent_scope_index = ss.scopes.len() - 2;
        ss.scopes[parent_scope_index].fn_defs.push(LoFnDef {
            name: fn_def.fn_name.clone(),
            inputs: lo_inputs,
            output: return_type,
            index: fn_index,
        });

        let exprs = self.build_code_block(ss, &fn_def.body)?;
        LoExpr::lower_all(&exprs, ss, &mut fn_code.expr.instrs);
        self.wasm_module.codes.push(fn_code);

        if fn_def.exported {
            self.wasm_module.exports.push(WasmExport {
                export_type: WasmExportType::Func,
                export_name: fn_def.fn_name.clone(),
                exported_item_index: fn_index,
            })
        }

        self.wasm_module.debug_fn_info.push(WasmDebugFnInfo {
            fn_index,
            fn_name: fn_def.fn_name.clone(),
        });

        ss.drop_last();

        Ok(())
    }

    pub fn generate(&mut self) -> Result<&WasmModule, LoError> {
        Ok(&self.wasm_module)
    }

    fn build_type(&mut self, type_expr: &TypeExpr) -> Result<LoType, LoError> {
        match type_expr {
            TypeExpr::U32 => Ok(LoType::U32),
        }
    }

    fn build_code_block(
        &mut self,
        ss: &mut LoScopeStack,
        code_block: &CodeBlockExpr,
    ) -> Result<Vec<LoExpr>, LoError> {
        let mut lo_exprs = Vec::new();
        for expr in &code_block.exprs {
            lo_exprs.push(self.build_code_expr(ss, expr)?);
        }
        Ok(lo_exprs)
    }

    fn build_code_expr(
        &mut self,
        ss: &mut LoScopeStack,
        code_expr: &CodeExpr,
    ) -> Result<LoExpr, LoError> {
        match code_expr {
            CodeExpr::Return(return_expr) => {
                let lo_expr = self.build_code_expr(ss, &return_expr.expr)?;

                Ok(LoExpr::Return {
                    expr: Box::new(lo_expr),
                })
            }
            CodeExpr::IntLiteral(int_expr) => {
                let value = u32::from_str_radix(&int_expr.value, 10).map_err(|_| LoError {
                    message: format!("Invalid int literal"),
                    loc: int_expr.loc.clone(),
                })?;

                Ok(LoExpr::U32Const { value })
            }
            CodeExpr::VarLoad(var_load) => {
                let Some(var) = ss.get_var(&var_load.name) else {
                    return Err(LoError {
                        message: format!("Cannot read unknown variable: {}", var_load.name),
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
                let lo_lhs = self.build_code_expr(ss, lhs)?;
                let lo_rhs = self.build_code_expr(ss, rhs)?;

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

                    InfixOpTag::Div
                    | InfixOpTag::Mod
                    | InfixOpTag::BitAnd
                    | InfixOpTag::BitOr
                    | InfixOpTag::ShiftLeft
                    | InfixOpTag::ShiftRight => return Err(LoError::todo(file!(), line!())),

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
                let lo_cond = self.build_code_expr(ss, cond)?;
                let lo_then_block = self.build_code_block(ss, &then_block)?;
                let lo_else_block = match &else_block {
                    ElseBlock::Else(else_block) => Some(self.build_code_block(ss, &else_block)?),
                    ElseBlock::ElseIf(expr) => {
                        let mut exprs = Vec::new();
                        exprs.push(self.build_code_expr(ss, expr)?);
                        Some(exprs)
                    }
                    ElseBlock::None => None,
                };

                Ok(LoExpr::If {
                    cond: Box::new(lo_cond),
                    then_block: lo_then_block,
                    else_block: lo_else_block,
                })
            }
            CodeExpr::Call(CallExpr { fn_name, args, loc }) => {
                let mut arg_types = Vec::new();
                let mut lo_args = Vec::new();
                for arg in args {
                    let lo_arg = self.build_code_expr(ss, arg)?;
                    arg_types.push(lo_arg.get_type());
                    lo_args.push(lo_arg);
                }

                let Some(fn_def) = ss.get_fn_def(&fn_name) else {
                    return Err(LoError {
                        message: format!("Trying to call unknown function {fn_name}"),
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
                    fn_name: fn_name.clone(),
                    args: lo_args,
                    return_type: fn_def.output.clone(),
                })
            }
        }
    }
}
