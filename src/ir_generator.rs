use crate::{ast::*, core::*, parser_v2::*, wasm::*};
use alloc::{format, vec::Vec};

#[derive(Default)]
pub struct IRGenerator {
    wasm_module: WasmModule,
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
        let mut fn_type = WasmFnType {
            inputs: Vec::new(),
            outputs: Vec::new(),
        };

        let return_type = self.build_type(&fn_def.return_type)?;
        fn_type.outputs.push(return_type);

        let type_index = self.wasm_module.types.len() as u32;
        self.wasm_module.types.push(fn_type);

        let fn_index = self.wasm_module.functions.len() as u32;
        self.wasm_module.functions.push(type_index);

        let mut instrs = Vec::new();
        self.build_code_block(&mut instrs, &fn_def.body)?;

        self.wasm_module.codes.push(WasmFn {
            locals: Vec::new(),
            expr: WasmExpr { instrs },
        });

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
        out_instrs: &mut Vec<WasmInstr>,
        code_block: &CodeBlockExpr,
    ) -> Result<(), LoError> {
        for expr in &code_block.exprs {
            self.build_code_expr(out_instrs, expr)?;
        }

        Ok(())
    }

    fn build_code_expr(
        &mut self,
        out_instrs: &mut Vec<WasmInstr>,
        code_expr: &CodeExpr,
    ) -> Result<(), LoError> {
        match code_expr {
            CodeExpr::Return(return_expr) => {
                self.build_code_expr(out_instrs, &return_expr.expr)?;
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
        }

        Ok(())
    }
}
