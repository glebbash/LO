use crate::{ir_generator::*, wasm::*};
use alloc::{string::String, vec::Vec};

#[derive(Default)]
pub struct CodeGenerator {
    wasm_module: WasmModule,
    ss: WasmScopeStack,
    fn_count: u32,
}

impl CodeGenerator {
    pub fn generate(scope: LoScope) -> WasmModule {
        let mut generator = CodeGenerator::default();

        generator.generate_all(scope);

        generator.wasm_module
    }

    fn generate_all(&mut self, scope: LoScope) {
        let wasm_scope = self.make_scope(&scope);
        self.ss.push(wasm_scope);

        for fn_def in &scope.fn_defs {
            let mut fn_type = WasmFnType {
                inputs: Vec::new(),
                outputs: Vec::new(),
            };

            for lo_type in &fn_def.inputs {
                self.lower_type(&lo_type, &mut fn_type.inputs);
            }
            self.lower_type(&fn_def.output, &mut fn_type.outputs);

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

            let wasm_scope = self.make_scope(&fn_def.body.scope);
            self.ss.push(wasm_scope);

            self.lower_exprs(&fn_def.body.exprs, &mut fn_code.expr.instrs);
            self.ss.pop();

            let type_index = self.wasm_module.types.len() as u32;
            self.wasm_module.types.push(fn_type);

            let fn_index = self.wasm_module.functions.len() as u32;
            self.wasm_module.functions.push(type_index);

            self.wasm_module.codes.push(fn_code);

            if fn_def.exported {
                self.wasm_module.exports.push(WasmExport {
                    export_type: WasmExportType::Func,
                    export_name: fn_def.name.clone(),
                    exported_item_index: fn_index,
                })
            }

            self.wasm_module.debug_fn_info.push(WasmDebugFnInfo {
                fn_index,
                fn_name: fn_def.name.clone(),
            });
        }

        self.ss.pop();
    }

    fn lower_type(&mut self, lo_type: &LoType, wasm_types: &mut Vec<WasmType>) {
        match lo_type {
            LoType::Never => {}
            LoType::Void => {}
            LoType::Bool => wasm_types.push(WasmType::I32),
            LoType::U32 => wasm_types.push(WasmType::I32),
        }
    }

    fn lower_exprs(&mut self, exprs: &Vec<LoExpr>, instrs: &mut Vec<WasmInstr>) {
        for expr in exprs {
            self.lower(expr, instrs);
        }
    }

    fn lower(&mut self, expr: &LoExpr, instrs: &mut Vec<WasmInstr>) {
        match expr {
            LoExpr::Casted { expr, .. } => {
                self.lower(expr, instrs);
            }
            LoExpr::Void => {}
            LoExpr::Unreachable => {
                instrs.push(WasmInstr::Unreachable);
            }
            LoExpr::U32Const { value } => {
                instrs.push(WasmInstr::I32Const {
                    value: *value as i32,
                });
            }
            LoExpr::Return { expr } => {
                self.lower(expr, instrs);
                instrs.push(WasmInstr::Return);
            }
            LoExpr::BinaryOp { kind, lhs, rhs } => {
                self.lower(lhs, instrs);
                self.lower(rhs, instrs);
                instrs.push(WasmInstr::BinaryOp { kind: kind.clone() });
            }
            LoExpr::VarLoad { name, .. } => {
                let var = self.ss.get_var(&name).unwrap();
                for i in 0..var.count {
                    instrs.push(WasmInstr::LocalGet {
                        local_index: var.index + i,
                    });
                }
            }
            LoExpr::If {
                cond,
                then_block,
                else_block,
            } => {
                self.lower(cond, instrs);
                instrs.push(WasmInstr::BlockStart {
                    block_kind: WasmBlockKind::If,
                    block_type: WasmBlockType::NoOut,
                });
                self.lower_exprs(&then_block.exprs, instrs);
                if let Some(else_block) = else_block {
                    instrs.push(WasmInstr::Else);
                    self.lower_exprs(&else_block.exprs, instrs);
                }
                instrs.push(WasmInstr::BlockEnd);
            }
            LoExpr::Call { fn_name, args, .. } => {
                self.lower_exprs(args, instrs);
                let fn_index = self.ss.get_fn_def(&fn_name).unwrap().index;
                instrs.push(WasmInstr::Call { fn_index });
            }
        }
    }

    fn make_scope(&mut self, scope: &LoScope) -> WasmScope {
        let mut fn_defs = Vec::new();
        for lo_fn_def in &scope.fn_defs {
            fn_defs.push(WasmFnDef {
                name: lo_fn_def.name.clone(),
                index: self.fn_count,
            });
            self.fn_count += 1;
        }

        let mut vars = Vec::new();
        for lo_var in &scope.vars {
            let mut wasm_types = Vec::new();
            self.lower_type(&lo_var.type_, &mut wasm_types);

            vars.push(WasmVar {
                name: lo_var.name.clone(),
                index: vars.len() as u32,
                count: wasm_types.len() as u32,
            });
        }

        WasmScope { vars, fn_defs }
    }
}

struct WasmVar {
    name: String,
    index: u32,
    count: u32,
}

struct WasmFnDef {
    name: String,
    index: u32,
}

#[derive(Default)]
struct WasmScope {
    vars: Vec<WasmVar>,
    fn_defs: Vec<WasmFnDef>,
}

#[derive(Default)]
struct WasmScopeStack {
    scopes: Vec<WasmScope>,
}

impl WasmScopeStack {
    fn push(&mut self, scope: WasmScope) {
        self.scopes.push(scope);
    }

    fn pop(&mut self) -> WasmScope {
        self.scopes.pop().unwrap()
    }

    fn get_var(&self, name: &str) -> Option<&WasmVar> {
        for scope in self.scopes.iter().rev() {
            for var in &scope.vars {
                if var.name == *name {
                    return Some(var);
                }
            }
        }
        None
    }

    fn get_fn_def(&self, name: &str) -> Option<&WasmFnDef> {
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
