use crate::{ast::*, core::*, parser_v2::*, wasm::*};
use alloc::{boxed::Box, format, string::String, vec::Vec};

#[derive(Clone, PartialEq)]
enum LoType {
    U32,
}

impl LoType {
    fn emit_components(&self, out: &mut Vec<WasmType>) {
        match self {
            LoType::U32 => out.push(WasmType::I32),
        }
    }
}

impl core::fmt::Display for LoType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LoType::U32 => f.write_str("u32"),
        }
    }
}

enum LoExpr {
    U32Const { value: u32 },
    Return { expr: Box<LoExpr> },
    Add { lhs: Box<LoExpr>, rhs: Box<LoExpr> },
    VarLoad { name: String, var_type: LoType },
}

impl LoExpr {
    fn get_type(&self) -> LoType {
        match self {
            LoExpr::U32Const { .. } => LoType::U32,
            LoExpr::Return { expr } => expr.get_type(),
            LoExpr::Add { lhs, .. } => lhs.get_type(),
            LoExpr::VarLoad { var_type, .. } => var_type.clone(),
        }
    }

    fn lower(&self, scope: &LoScope, instrs: &mut Vec<WasmInstr>) {
        match self {
            LoExpr::U32Const { value } => instrs.push(WasmInstr::I32Const {
                value: *value as i32,
            }),
            LoExpr::Return { expr } => {
                expr.lower(scope, instrs);
                instrs.push(WasmInstr::Return);
            }
            LoExpr::Add { lhs, rhs } => {
                lhs.lower(scope, instrs);
                rhs.lower(scope, instrs);
                instrs.push(WasmInstr::BinaryOp {
                    kind: WasmBinaryOpKind::I32_ADD,
                })
            }
            LoExpr::VarLoad { name, .. } => {
                let mut local_index = 0;
                for var in &scope.variables {
                    if var.name == *name {
                        local_index = var.index;
                    }
                }

                instrs.push(WasmInstr::LocalGet { local_index })
            }
        }
    }
}

#[derive(Default)]
pub struct IRGenerator {
    wasm_module: WasmModule,
    pub errors: LoErrorManager,
}

struct LoVariable {
    name: String,
    type_: LoType,
    index: u32,
}

struct LoScope {
    variables: Vec<LoVariable>,
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
        let mut scope = LoScope {
            variables: Vec::new(),
        };

        let mut fn_type = WasmFnType {
            inputs: Vec::new(),
            outputs: Vec::new(),
        };

        let return_type = self.build_type(&fn_def.return_type)?;
        return_type.emit_components(&mut fn_type.outputs);

        for fn_param in &fn_def.fn_params {
            for var in &scope.variables {
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

            scope.variables.push(LoVariable {
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

    fn build_type(&mut self, type_expr: &TypeExpr) -> Result<LoType, LoError> {
        match type_expr {
            TypeExpr::U32 => Ok(LoType::U32),
        }
    }

    fn build_code_block(
        &mut self,
        scope: &LoScope,
        out_instrs: &mut Vec<WasmInstr>,
        code_block: &CodeBlockExpr,
    ) -> Result<(), LoError> {
        for expr in &code_block.exprs {
            let lo_expr = self.build_code_expr(scope, expr)?;
            lo_expr.lower(scope, out_instrs);
        }

        Ok(())
    }

    fn build_code_expr(
        &mut self,
        scope: &LoScope,
        code_expr: &CodeExpr,
    ) -> Result<LoExpr, LoError> {
        match code_expr {
            CodeExpr::Return(return_expr) => {
                let lo_expr = self.build_code_expr(scope, &return_expr.expr)?;

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
                for var in &scope.variables {
                    if var.name == var_load.name {
                        return Ok(LoExpr::VarLoad {
                            name: var.name.clone(),
                            var_type: var.type_.clone(),
                        });
                    }
                }

                Err(LoError {
                    message: format!("Cannot read unknown variable: {}", var_load.name),
                    loc: var_load.loc.clone(),
                })
            }
            CodeExpr::Add(AddExpr { lhs, rhs, loc }) => {
                let lo_lhs = self.build_code_expr(scope, lhs)?;
                let lo_rhs = self.build_code_expr(scope, rhs)?;

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

                Ok(LoExpr::Add {
                    lhs: Box::new(lo_lhs),
                    rhs: Box::new(lo_rhs),
                })
            }
        }
    }
}
