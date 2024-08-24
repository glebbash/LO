use crate::{core::*, ir_generator::*, wasm::*};
use alloc::{format, string::String, vec::Vec};

#[derive(Clone)]
pub enum LoValue {
    Never,
    Void,
    Bool { value: bool },
    U32 { value: u32 },
}

impl core::fmt::Display for LoValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LoValue::Never => f.write_str("<never>"),
            LoValue::Void => f.write_str("<void>"),
            LoValue::Bool { value } => f.write_fmt(format_args!("{value}")),
            LoValue::U32 { value } => f.write_fmt(format_args!("{value}")),
        }
    }
}

#[derive(Default)]
pub struct Eval {
    ss: EvalScopeStack,
}

impl Eval {
    pub fn eval(scope: LoScope) -> Result<LoValue, LoError> {
        let mut interpreter = Eval::default();
        let eval_scope = interpreter.make_scope(&scope);
        interpreter.ss.push(eval_scope);

        let result = interpreter.eval_expr(&LoExpr::Call {
            fn_name: format!("main"),
            args: Vec::new(),
            return_type: LoType::Void,
        });

        interpreter.ss.pop();

        Ok(result)
    }

    fn eval_expr(&mut self, expr: &LoExpr) -> LoValue {
        match expr {
            LoExpr::Casted { expr, .. } => self.eval_expr(expr),
            LoExpr::U32Const { value } => LoValue::U32 { value: *value },
            LoExpr::Return { expr } => {
                let value = self.eval_expr(expr);
                self.ss.set_return(value);
                LoValue::Never
            }
            LoExpr::BinaryOp { kind, lhs, rhs } => {
                let lhs_value = self.eval_expr(lhs);
                let rhs_value = self.eval_expr(rhs);

                match kind {
                    WasmBinaryOpKind::I32_LT_U => match (lhs_value, rhs_value) {
                        (LoValue::U32 { value: a }, LoValue::U32 { value: b }) => {
                            LoValue::Bool { value: a < b }
                        }
                        (l, r) => panic!("Unsupported operation: {l} < {r}"),
                    },

                    WasmBinaryOpKind::I32_ADD => match (lhs_value, rhs_value) {
                        (LoValue::U32 { value: a }, LoValue::U32 { value: b }) => {
                            LoValue::U32 { value: a + b }
                        }
                        (l, r) => panic!("Unsupported operation: {l} + {r}"),
                    },
                    WasmBinaryOpKind::I32_SUB => match (lhs_value, rhs_value) {
                        (LoValue::U32 { value: a }, LoValue::U32 { value: b }) => {
                            LoValue::U32 { value: a - b }
                        }
                        (l, r) => panic!("Unsupported operation: {l} - {r}"),
                    },
                    WasmBinaryOpKind::I32_MUL => match (lhs_value, rhs_value) {
                        (LoValue::U32 { value: a }, LoValue::U32 { value: b }) => {
                            LoValue::U32 { value: a * b }
                        }
                        (l, r) => panic!("Unsupported operation: {l} * {r}"),
                    },

                    WasmBinaryOpKind::I32_EQ
                    | WasmBinaryOpKind::I32_NE
                    | WasmBinaryOpKind::I32_LT_S
                    | WasmBinaryOpKind::I32_GT_S
                    | WasmBinaryOpKind::I32_GT_U
                    | WasmBinaryOpKind::I32_LE_S
                    | WasmBinaryOpKind::I32_LE_U
                    | WasmBinaryOpKind::I32_GE_S
                    | WasmBinaryOpKind::I32_GE_U => todo!(),

                    WasmBinaryOpKind::I32_DIV_S
                    | WasmBinaryOpKind::I32_DIV_U
                    | WasmBinaryOpKind::I32_REM_S
                    | WasmBinaryOpKind::I32_REM_U
                    | WasmBinaryOpKind::I32_AND
                    | WasmBinaryOpKind::I32_OR
                    | WasmBinaryOpKind::I32_SHL
                    | WasmBinaryOpKind::I32_SHR_S
                    | WasmBinaryOpKind::I32_SHR_U => todo!(),

                    WasmBinaryOpKind::I64_EQ
                    | WasmBinaryOpKind::I64_NE
                    | WasmBinaryOpKind::I64_LT_S
                    | WasmBinaryOpKind::I64_LT_U
                    | WasmBinaryOpKind::I64_GT_S
                    | WasmBinaryOpKind::I64_GT_U
                    | WasmBinaryOpKind::I64_LE_S
                    | WasmBinaryOpKind::I64_LE_U
                    | WasmBinaryOpKind::I64_GE_S
                    | WasmBinaryOpKind::I64_GE_U
                    | WasmBinaryOpKind::F32_EQ
                    | WasmBinaryOpKind::F32_NE
                    | WasmBinaryOpKind::F32_LT
                    | WasmBinaryOpKind::F32_GT
                    | WasmBinaryOpKind::F32_LE
                    | WasmBinaryOpKind::F32_GE
                    | WasmBinaryOpKind::F64_EQ
                    | WasmBinaryOpKind::F64_NE
                    | WasmBinaryOpKind::F64_LT
                    | WasmBinaryOpKind::F64_GT
                    | WasmBinaryOpKind::F64_LE
                    | WasmBinaryOpKind::F64_GE
                    | WasmBinaryOpKind::I64_ADD
                    | WasmBinaryOpKind::I64_SUB
                    | WasmBinaryOpKind::I64_MUL
                    | WasmBinaryOpKind::I64_DIV_S
                    | WasmBinaryOpKind::I64_DIV_U
                    | WasmBinaryOpKind::I64_REM_S
                    | WasmBinaryOpKind::I64_REM_U
                    | WasmBinaryOpKind::I64_AND
                    | WasmBinaryOpKind::I64_OR
                    | WasmBinaryOpKind::I64_SHL
                    | WasmBinaryOpKind::I64_SHR_S
                    | WasmBinaryOpKind::I64_SHR_U
                    | WasmBinaryOpKind::F32_ADD
                    | WasmBinaryOpKind::F32_SUB
                    | WasmBinaryOpKind::F32_MUL
                    | WasmBinaryOpKind::F32_DIV
                    | WasmBinaryOpKind::F64_ADD
                    | WasmBinaryOpKind::F64_SUB
                    | WasmBinaryOpKind::F64_MUL
                    | WasmBinaryOpKind::F64_DIV => todo!(),
                }
            }
            LoExpr::VarLoad { name, .. } => {
                let var = self.ss.get_var(name).unwrap();
                var.value.clone()
            }
            LoExpr::If {
                cond,
                then_block,
                else_block,
            } => {
                let cond_res = self.eval_expr(cond);
                if let LoValue::Bool { value: true } = cond_res {
                    let eval_scope = self.make_scope(&then_block.scope);
                    self.ss.push(eval_scope);
                    let res = self.eval_exprs(&then_block.exprs);
                    self.ss.pop();
                    res
                } else if let Some(else_block) = else_block {
                    let eval_scope = self.make_scope(&else_block.scope);
                    self.ss.push(eval_scope);
                    let res = self.eval_exprs(&else_block.exprs);
                    self.ss.pop();
                    res
                } else {
                    LoValue::Void
                }
            }
            LoExpr::Call { fn_name, args, .. } => {
                let fn_def = self.ss.get_fn_def(fn_name).unwrap();

                let mut eval_scope = self.make_scope(unsafe { &(*(*fn_def).code_block).scope });
                eval_scope.type_ = ScopeType::Function;
                for (arg, index) in args.iter().zip(0..) {
                    let value = self.eval_expr(arg);
                    eval_scope.vars[index].value = value;
                }
                self.ss.push(eval_scope);

                self.eval_exprs(unsafe { &(*(*fn_def).code_block).exprs });

                let scope = self.ss.pop();
                if let Some(return_value) = scope.return_value {
                    return_value
                } else {
                    LoValue::Void
                }
            }
        }
    }

    fn eval_exprs(&mut self, exprs: &Vec<LoExpr>) -> LoValue {
        for expr in exprs {
            let value = self.eval_expr(expr);
            if let LoValue::Never = value {
                return LoValue::Never;
            }
        }

        LoValue::Void
    }

    fn make_scope(&mut self, scope: &LoScope) -> EvalScope {
        let mut fn_defs = Vec::new();
        for lo_fn_def in &scope.fn_defs {
            fn_defs.push(EvalFnDef {
                name: lo_fn_def.name.clone(),
                code_block: &lo_fn_def.body,
            });
        }

        let mut vars = Vec::new();
        for lo_var in &scope.vars {
            vars.push(EvalVar {
                name: lo_var.name.clone(),
                value: LoValue::Void,
            });
        }

        EvalScope {
            vars,
            fn_defs,
            return_value: None,
            type_: ScopeType::Block,
        }
    }
}

struct EvalVar {
    name: String,
    value: LoValue,
}

struct EvalFnDef {
    name: String,
    code_block: *const CodeBlock,
}

#[derive(Default)]
enum ScopeType {
    #[default]
    Block,
    Function,
}

#[derive(Default)]
struct EvalScope {
    vars: Vec<EvalVar>,
    fn_defs: Vec<EvalFnDef>,
    return_value: Option<LoValue>,
    type_: ScopeType,
}

#[derive(Default)]
struct EvalScopeStack {
    scopes: Vec<EvalScope>,
}

impl EvalScopeStack {
    fn push(&mut self, scope: EvalScope) {
        if self.scopes.len() > 1000 {
            panic!("Stack overflow\n");
        }

        self.scopes.push(scope);
    }

    fn pop(&mut self) -> EvalScope {
        self.scopes.pop().unwrap()
    }

    fn set_return(&mut self, value: LoValue) {
        for scope in self.scopes.iter_mut().rev() {
            if let ScopeType::Function = scope.type_ {
                scope.return_value = Some(value);
                break;
            }
        }
    }

    fn get_var(&self, name: &str) -> Option<&EvalVar> {
        for scope in self.scopes.iter().rev() {
            for var in &scope.vars {
                if var.name == *name {
                    return Some(var);
                }
            }
        }
        None
    }

    fn get_fn_def(&self, name: &str) -> Option<*const EvalFnDef> {
        for scope in self.scopes.iter().rev() {
            for var in &scope.fn_defs {
                if var.name == *name {
                    return Some(var as *const EvalFnDef);
                }
            }
        }

        None
    }
}
