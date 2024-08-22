use crate::{ast::*, core::*, parser_v2::*, wasm::*};
use alloc::{format, string::String, vec::Vec};

#[derive(Default)]
pub struct IRGenerator {
    wasm_module: WasmModule,
    pub errors: LoErrorManager,
}

struct Variable {
    name: String,
    index: u32,
}

struct CodeScope {
    locals: Vec<Variable>,
}

impl IRGenerator {
    pub fn process_file(&mut self, file: &FileInfo) -> Result<(), LoError> {
        for expr in &file.ast.exprs {
            match expr {
                TopLevelExpr::Include(_) => {} // skip, processed earlier

                TopLevelExpr::FnDef(fn_def) => self.process_fn_def(fn_def)?,
            }
        }

        Ok(())
    }

    fn process_fn_def(&mut self, fn_def: &FnDefExpr) -> Result<(), LoError> {
        let mut scope = CodeScope { locals: Vec::new() };

        let mut fn_type = WasmFnType {
            inputs: Vec::new(),
            outputs: Vec::new(),
        };

        let return_type = self.build_type(&fn_def.return_type)?;
        fn_type.outputs.push(return_type);

        for (fn_param, index) in fn_def.fn_params.iter().zip(0..) {
            for local in &scope.locals {
                if local.name == fn_param.name {
                    self.errors.report(LoError {
                        message: format!("Duplicate function parameter name: {}", fn_param.name),
                        loc: fn_param.loc.clone(),
                    });
                    continue;
                }
            }

            let param_type = self.build_type(&fn_param.type_)?;
            fn_type.inputs.push(param_type.clone());

            scope.locals.push(Variable {
                name: fn_param.name.clone(),
                index: index as u32,
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

        self.build_code_block(&scope, &mut fn_code.expr.instrs, &fn_def.body)?;
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

        Ok(())
    }

    pub fn generate(&mut self) -> Result<&WasmModule, LoError> {
        Ok(&self.wasm_module)
    }

    fn build_type(&mut self, type_expr: &TypeExpr) -> Result<WasmType, LoError> {
        match type_expr {
            TypeExpr::U32 => Ok(WasmType::I32),
        }
    }

    fn build_code_block(
        &mut self,
        scope: &CodeScope,
        out_instrs: &mut Vec<WasmInstr>,
        code_block: &CodeBlockExpr,
    ) -> Result<(), LoError> {
        for expr in &code_block.exprs {
            self.build_code_expr(scope, out_instrs, expr)?;
        }

        Ok(())
    }

    fn build_code_expr(
        &mut self,
        scope: &CodeScope,
        out_instrs: &mut Vec<WasmInstr>,
        code_expr: &CodeExpr,
    ) -> Result<(), LoError> {
        match code_expr {
            CodeExpr::Return(return_expr) => {
                self.build_code_expr(scope, out_instrs, &return_expr.expr)?;
                out_instrs.push(WasmInstr::Return);
            }
            CodeExpr::IntLiteral(int_expr) => {
                let value = u32::from_str_radix(&int_expr.value, 10).map_err(|_| LoError {
                    message: format!("Invalid int literal"),
                    loc: int_expr.loc.clone(),
                })?;

                out_instrs.push(WasmInstr::I32Const {
                    value: value as i32,
                })
            }
            CodeExpr::VarLoad(var_load) => {
                for local in &scope.locals {
                    if local.name == var_load.name {
                        out_instrs.push(WasmInstr::LocalGet {
                            local_index: local.index,
                        });
                        return Ok(());
                    }
                }

                return Err(LoError {
                    message: format!("Cannot read unknown variable: {}", var_load.name),
                    loc: var_load.loc.clone(),
                });
            }
            CodeExpr::Add(AddExpr { lhs, rhs, .. }) => {
                self.build_code_expr(scope, out_instrs, lhs)?;
                self.build_code_expr(scope, out_instrs, rhs)?;
                out_instrs.push(WasmInstr::BinaryOp {
                    kind: WasmBinaryOpKind::I32_ADD,
                });
                return Ok(());
            }
        }

        Ok(())
    }
}
