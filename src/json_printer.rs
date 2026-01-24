// NOTE: this was AI generated to quickly test the idea for the structural editor.
//   If this is to be kept it has to be reviewed and possibly rewritten.

use crate::{ast::*, core::*, parser::*};
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

pub struct JsonPrinter {
    parser: &'static Parser,
}

impl JsonPrinter {
    pub fn new(parser: &'static Parser) -> Self {
        JsonPrinter { parser }
    }

    pub fn print_file(&mut self) {
        stdout_enable_buffering();

        let mut items = Vec::new();
        for expr in &self.parser.ast {
            items.push(self.json_top_level_expr(expr));
        }

        stdout_write("{\n  \"ast\": [\n");
        for (i, item) in items.iter().enumerate() {
            stdout_write("    ");
            stdout_write(&item);
            if i < items.len() - 1 {
                stdout_write(",");
            }
            stdout_write("\n");
        }
        stdout_write("  ]\n}\n");

        stdout_disable_buffering();
    }

    fn json_top_level_expr(&self, expr: &TopLevelExpr) -> String {
        match expr {
            TopLevelExpr::FnDef(e) => {
                format!(
                    r#"{{"kind": "FnDef", "exported": {}, "decl": {}, "body": {}}}"#,
                    e.exported,
                    self.json_fn_decl(&e.decl),
                    self.json_code_block(&e.body)
                )
            }
            TopLevelExpr::Include(e) => {
                format!(
                    r#"{{"kind": "Include", "file_path": {}, "alias": {}, "with_extern": {}}}"#,
                    self.json_string(e.file_path.get_repr(self.parser.lexer.source)),
                    self.json_option(&e.alias.as_ref().map(|a| a.repr.to_string())),
                    e.with_extern
                )
            }
            TopLevelExpr::Import(e) => {
                let items: String = e
                    .items
                    .iter()
                    .enumerate()
                    .map(|(i, item)| {
                        let json = match item {
                            ImportItem::FnDecl(decl) => format!(
                                r#"{{"kind": "FnDecl", "decl": {}}}"#,
                                self.json_fn_decl(decl)
                            ),
                            ImportItem::Memory(mem) => format!(
                                r#"{{"kind": "Memory", "memory": {}}}"#,
                                self.json_memory_def(mem)
                            ),
                        };
                        if i < e.items.len() - 1 {
                            format!("{},", json)
                        } else {
                            json
                        }
                    })
                    .collect();

                format!(
                    r#"{{"kind": "Import", "module_name": {}, "items": [{}]}}"#,
                    self.json_string(e.module_name.get_repr(self.parser.lexer.source)),
                    items
                )
            }
            TopLevelExpr::GlobalDef(e) => {
                format!(
                    r#"{{"kind": "GlobalDef", "global_name": {}, "global_value": {}}}"#,
                    self.json_string(&e.global_name.repr),
                    self.json_code_expr(&e.global_value)
                )
            }
            TopLevelExpr::StructDef(e) => {
                let fields: String = e
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(i, field)| {
                        let json = format!(
                            r#"{{"name": {}, "type": {}}}"#,
                            self.json_string(&field.field_name.repr),
                            self.json_type_expr(&field.field_type)
                        );
                        if i < e.fields.len() - 1 {
                            format!("{},", json)
                        } else {
                            json
                        }
                    })
                    .collect();

                format!(
                    r#"{{"kind": "StructDef", "name": {}, "fields": [{}]}}"#,
                    self.json_string(&e.struct_name.repr),
                    fields
                )
            }
            TopLevelExpr::EnumDef(e) => {
                let variants: String = e
                    .variants
                    .iter()
                    .enumerate()
                    .map(|(i, variant)| {
                        let json = format!(
                            r#"{{"name": {}, "type": {}}}"#,
                            self.json_string(&variant.variant_name.repr),
                            self.json_option(
                                &variant
                                    .variant_type
                                    .as_ref()
                                    .map(|t| self.json_type_expr(t))
                            )
                        );
                        if i < e.variants.len() - 1 {
                            format!("{},", json)
                        } else {
                            json
                        }
                    })
                    .collect();

                format!(
                    r#"{{"kind": "EnumDef", "name": {}, "variant_type": {}, "variants": [{}]}}"#,
                    self.json_string(&e.enum_name.repr),
                    self.json_option(&e.variant_type.as_ref().map(|t| self.json_type_expr(t))),
                    variants
                )
            }
            TopLevelExpr::TypeDef(e) => {
                format!(
                    r#"{{"kind": "TypeDef", "name": {}, "type": {}}}"#,
                    self.json_string(&e.type_name.repr),
                    self.json_type_expr(&e.type_value)
                )
            }
            TopLevelExpr::ConstDef(e) => {
                format!(
                    r#"{{"kind": "ConstDef", "name": {}, "value": {}}}"#,
                    self.json_string(&e.const_name.repr),
                    self.json_code_expr(&e.const_value)
                )
            }
            TopLevelExpr::MemoryDef(e) => {
                format!(
                    r#"{{"kind": "MemoryDef", "memory": {}}}"#,
                    self.json_memory_def(e)
                )
            }
            TopLevelExpr::TryExport(e) => {
                format!(
                    r#"{{"kind": "TryExport", "in_name": {}, "out_name": {}, "from_root": {}}}"#,
                    self.json_string(&e.in_name.repr),
                    self.json_string(e.out_name.get_repr(self.parser.lexer.source)),
                    e.from_root
                )
            }
            TopLevelExpr::MacroDef(e) => {
                let params: String = e
                    .macro_params
                    .iter()
                    .enumerate()
                    .map(|(i, param)| {
                        let json = self.json_fn_param(param);
                        if i < e.macro_params.len() - 1 {
                            format!("{},", json)
                        } else {
                            json
                        }
                    })
                    .collect();

                let type_params: String = e
                    .macro_type_params
                    .iter()
                    .enumerate()
                    .map(|(i, tp)| {
                        let json = self.json_string(tp);
                        if i < e.macro_type_params.len() - 1 {
                            format!("{},", json)
                        } else {
                            json
                        }
                    })
                    .collect();

                format!(
                    r#"{{"kind": "MacroDef", "name": {}, "type_params": [{}], "params": [{}], "return_type": {}, "body": {}}}"#,
                    self.json_string(&e.macro_name.repr),
                    type_params,
                    params,
                    self.json_option(&e.return_type.as_ref().map(|t| self.json_type_expr(t))),
                    self.json_code_block(&e.body)
                )
            }
        }
    }

    fn json_fn_decl(&self, fn_decl: &FnDeclExpr) -> String {
        let params: String = fn_decl
            .fn_params
            .iter()
            .enumerate()
            .map(|(i, param)| {
                let json = self.json_fn_param(param);
                if i < fn_decl.fn_params.len() - 1 {
                    format!("{},", json)
                } else {
                    json
                }
            })
            .collect();

        format!(
            r#"{{"name": {}, "params": [{}], "return_type": {}}}"#,
            self.json_string(&fn_decl.fn_name.repr),
            params,
            self.json_option(&fn_decl.return_type.as_ref().map(|t| self.json_type_expr(t)))
        )
    }

    fn json_fn_param(&self, param: &FnParam) -> String {
        let param_type = match &param.param_type {
            FnParamType::Self_ => r#""self""#.to_string(),
            FnParamType::SelfRef => r#""&self""#.to_string(),
            FnParamType::Type { expr } => {
                format!(
                    r#"{{"kind": "Type", "type": {}}}"#,
                    self.json_type_expr(expr)
                )
            }
            FnParamType::Infer { name } => {
                format!(r#"{{"kind": "Infer", "name": {}}}"#, self.json_string(name))
            }
        };

        format!(
            r#"{{"name": {}, "type": {}}}"#,
            self.json_string(&param.param_name.repr),
            param_type
        )
    }

    fn json_memory_def(&self, mem: &MemoryDefExpr) -> String {
        format!(
            r#"{{"exported": {}, "params": {}}}"#,
            mem.exported,
            self.json_code_expr_map(&mem.params)
        )
    }

    fn json_type_expr(&self, type_expr: &TypeExpr) -> String {
        match type_expr {
            TypeExpr::Named(e) => {
                format!(
                    r#"{{"kind": "Named", "name": {}}}"#,
                    self.json_string(&e.name.repr)
                )
            }
            TypeExpr::Pointer(e) => {
                format!(
                    r#"{{"kind": "Pointer", "pointee": {}}}"#,
                    self.json_type_expr(&e.pointee)
                )
            }
            TypeExpr::SequencePointer(e) => {
                format!(
                    r#"{{"kind": "SequencePointer", "pointee": {}}}"#,
                    self.json_type_expr(&e.pointee)
                )
            }
            TypeExpr::Container(e) => {
                let items: String = e
                    .items
                    .iter()
                    .enumerate()
                    .map(|(i, item)| {
                        let json = self.json_type_expr(item);
                        if i < e.items.len() - 1 {
                            format!("{},", json)
                        } else {
                            json
                        }
                    })
                    .collect();

                format!(
                    r#"{{"kind": "Container", "container": {}, "items": [{}]}}"#,
                    self.json_type_expr(&e.container),
                    items
                )
            }
        }
    }

    fn json_code_block(&self, code_block: &CodeBlock) -> String {
        let exprs: String = code_block
            .exprs
            .iter()
            .enumerate()
            .map(|(i, expr)| {
                let json = self.json_code_expr(expr);
                if i < code_block.exprs.len() - 1 {
                    format!("{},", json)
                } else {
                    json
                }
            })
            .collect();

        format!(r#"{{"expressions": [{}]}}"#, exprs)
    }

    fn json_code_expr(&self, expr: &CodeExpr) -> String {
        match expr {
            CodeExpr::BoolLiteral(e) => {
                format!(r#"{{"kind": "BoolLiteral", "value": {}}}"#, e.value)
            }
            CodeExpr::CharLiteral(e) => {
                format!(
                    r#"{{"kind": "CharLiteral", "value": {}}}"#,
                    self.json_string(&e.repr)
                )
            }
            CodeExpr::NullLiteral(_) => r#"{"kind": "NullLiteral"}"#.to_string(),
            CodeExpr::IntLiteral(e) => {
                let tag_str = if let Some(tag) = &e.tag {
                    self.json_string(tag)
                } else {
                    "null".to_string()
                };
                format!(
                    r#"{{"kind": "IntLiteral", "value": {}, "tag": {}}}"#,
                    self.json_string(&e.repr),
                    tag_str
                )
            }
            CodeExpr::StringLiteral(e) => {
                format!(
                    r#"{{"kind": "StringLiteral", "value": {}}}"#,
                    self.json_string(&e.repr)
                )
            }
            CodeExpr::ArrayLiteral(e) => {
                let items: String = e
                    .items
                    .iter()
                    .enumerate()
                    .map(|(i, item)| {
                        let json = self.json_code_expr(item);
                        if i < e.items.len() - 1 {
                            format!("{},", json)
                        } else {
                            json
                        }
                    })
                    .collect();

                format!(
                    r#"{{"kind": "ArrayLiteral", "item_type": {}, "items": [{}]}}"#,
                    self.json_type_expr(&e.item_type),
                    items
                )
            }
            CodeExpr::ResultLiteral(e) => {
                let result_type_str = if let Some(rt) = &e.result_type {
                    format!(
                        r#"{{"ok": {}, "err": {}}}"#,
                        self.json_type_expr(&rt.ok),
                        self.json_type_expr(&rt.err)
                    )
                } else {
                    "null".to_string()
                };

                format!(
                    r#"{{"kind": "ResultLiteral", "is_ok": {}, "result_type": {}, "value": {}}}"#,
                    e.is_ok,
                    result_type_str,
                    self.json_option(&e.value.as_ref().map(|v| self.json_code_expr(v)))
                )
            }
            CodeExpr::Ident(e) => {
                format!(
                    r#"{{"kind": "Ident", "value": {}}}"#,
                    self.json_string(&e.repr)
                )
            }
            CodeExpr::Let(e) => {
                format!(
                    r#"{{"kind": "Let", "local_name": {}, "value": {}}}"#,
                    self.json_string(&e.local_name.repr),
                    self.json_code_expr(&e.value)
                )
            }
            CodeExpr::Return(e) => {
                format!(
                    r#"{{"kind": "Return", "value": {}}}"#,
                    self.json_option(&e.expr.as_ref().map(|v| self.json_code_expr(v)))
                )
            }
            CodeExpr::InfixOp(e) => {
                let op = e.op_loc.read_span(self.parser.lexer.source);
                format!(
                    r#"{{"kind": "InfixOp", "op": {}, "lhs": {}, "rhs": {}}}"#,
                    self.json_string(op),
                    self.json_code_expr(&e.lhs),
                    self.json_code_expr(&e.rhs)
                )
            }
            CodeExpr::ExprPipe(e) => {
                let op = e.op_loc.read_span(self.parser.lexer.source);
                format!(
                    r#"{{"kind": "ExprPipe", "op": {}, "lhs": {}, "rhs": {}}}"#,
                    self.json_string(op),
                    self.json_code_expr(&e.lhs),
                    self.json_code_expr(&e.rhs)
                )
            }
            CodeExpr::PrefixOp(e) => {
                let op = e.op_loc.read_span(self.parser.lexer.source);
                format!(
                    r#"{{"kind": "PrefixOp", "op": {}, "operand": {}}}"#,
                    self.json_string(op),
                    self.json_code_expr(&e.expr)
                )
            }
            CodeExpr::If(e) => {
                let cond_str = match &e.cond {
                    IfCond::Expr(expr) => self.json_code_expr(expr),
                    IfCond::Match(header) => self.json_match_header(header),
                };

                let else_block_str = match &e.else_block {
                    ElseBlock::None => "null".to_string(),
                    ElseBlock::Else(block) => self.json_code_block(block),
                    ElseBlock::ElseIf(if_expr) => self.json_code_expr(if_expr),
                };

                format!(
                    r#"{{"kind": "If", "condition": {}, "then_block": {}, "else_block": {}}}"#,
                    cond_str,
                    self.json_code_block(&e.then_block),
                    else_block_str
                )
            }
            CodeExpr::Match(e) => {
                format!(
                    r#"{{"kind": "Match", "header": {}, "else_branch": {}}}"#,
                    self.json_match_header(&e.header),
                    self.json_code_block(&e.else_branch)
                )
            }
            CodeExpr::While(e) => {
                let cond_str = if let Some(cond) = &e.cond {
                    self.json_code_expr(cond)
                } else {
                    "null".to_string()
                };

                format!(
                    r#"{{"kind": "WhileLoop", "condition": {}, "body": {}}}"#,
                    cond_str,
                    self.json_code_block(&e.body)
                )
            }
            CodeExpr::For(e) => {
                format!(
                    r#"{{"kind": "ForLoop", "counter": {}, "start": {}, "end": {}, "body": {}}}"#,
                    self.json_string(&e.counter.repr),
                    self.json_code_expr(&e.start),
                    self.json_code_expr(&e.end),
                    self.json_code_block(&e.body)
                )
            }
            CodeExpr::Break(_) => r#"{"kind": "Break"}"#.to_string(),
            CodeExpr::Continue(_) => r#"{"kind": "Continue"}"#.to_string(),
            CodeExpr::DoWith(e) => {
                format!(
                    r#"{{"kind": "DoWith", "body": {}, "args": {}}}"#,
                    self.json_code_expr(&e.body),
                    self.json_code_expr_list(&e.args)
                )
            }
            CodeExpr::Dbg(e) => {
                format!(
                    r#"{{"kind": "Dbg", "message": {}}}"#,
                    self.json_string(e.message.get_repr(self.parser.lexer.source))
                )
            }
            CodeExpr::Defer(e) => {
                format!(
                    r#"{{"kind": "Defer", "expr": {}}}"#,
                    self.json_code_expr(&e.expr)
                )
            }
            CodeExpr::Cast(e) => {
                format!(
                    r#"{{"kind": "Cast", "expr": {}, "casted_to": {}}}"#,
                    self.json_code_expr(&e.expr),
                    self.json_type_expr(&e.casted_to)
                )
            }
            CodeExpr::StructLiteral(e) => {
                format!(
                    r#"{{"kind": "StructLiteral", "struct_name": {}, "body": {}}}"#,
                    self.json_string(&e.struct_name.repr),
                    self.json_code_expr_map(&e.body)
                )
            }
            CodeExpr::Assign(e) => {
                format!(
                    r#"{{"kind": "Assign", "lhs": {}, "rhs": {}}}"#,
                    self.json_code_expr(&e.lhs),
                    self.json_code_expr(&e.rhs)
                )
            }
            CodeExpr::FieldAccess(e) => {
                format!(
                    r#"{{"kind": "FieldAccess", "lhs": {}, "field_name": {}}}"#,
                    self.json_code_expr(&e.lhs),
                    self.json_string(&e.field_name.repr)
                )
            }
            CodeExpr::Catch(e) => {
                format!(
                    r#"{{"kind": "Catch", "lhs": {}, "error_bind": {}, "catch_body": {}}}"#,
                    self.json_code_expr(&e.lhs),
                    self.json_string(&e.error_bind.repr),
                    self.json_code_block(&e.catch_body)
                )
            }
            CodeExpr::Paren(e) => {
                format!(
                    r#"{{"kind": "Paren", "expr": {}, "has_trailing_comma": {}}}"#,
                    self.json_code_expr(&e.expr),
                    e.has_trailing_comma
                )
            }
            CodeExpr::FnCall(e) => {
                format!(
                    r#"{{"kind": "FnCall", "fn_name": {}, "args": {}}}"#,
                    self.json_string(&e.fn_name.repr),
                    self.json_code_expr_list(&e.args)
                )
            }
            CodeExpr::MethodCall(e) => {
                format!(
                    r#"{{"kind": "MethodCall", "lhs": {}, "method_name": {}, "args": {}}}"#,
                    self.json_code_expr(&e.lhs),
                    self.json_string(&e.field_name.repr),
                    self.json_code_expr_list(&e.args)
                )
            }
            CodeExpr::MacroFnCall(e) => {
                let type_args: String = e
                    .type_args
                    .iter()
                    .enumerate()
                    .map(|(i, arg)| {
                        let json = self.json_type_expr(arg);
                        if i < e.type_args.len() - 1 {
                            format!("{},", json)
                        } else {
                            json
                        }
                    })
                    .collect();

                format!(
                    r#"{{"kind": "MacroFnCall", "fn_name": {}, "type_args": [{}], "args": {}}}"#,
                    self.json_string(&e.fn_name.repr),
                    type_args,
                    self.json_code_expr_list(&e.args)
                )
            }
            CodeExpr::IntrinsicCall(e) => {
                let type_args: String = e
                    .type_args
                    .iter()
                    .enumerate()
                    .map(|(i, arg)| {
                        let json = self.json_type_expr(arg);
                        if i < e.type_args.len() - 1 {
                            format!("{},", json)
                        } else {
                            json
                        }
                    })
                    .collect();

                format!(
                    r#"{{"kind": "IntrinsicCall", "fn_name": {}, "type_args": [{}], "args": {}}}"#,
                    self.json_string(&e.fn_name.repr),
                    type_args,
                    self.json_code_expr_list(&e.args)
                )
            }
            CodeExpr::MacroMethodCall(e) => {
                let type_args: String = e
                    .type_args
                    .iter()
                    .enumerate()
                    .map(|(i, arg)| {
                        let json = self.json_type_expr(arg);
                        if i < e.type_args.len() - 1 {
                            format!("{},", json)
                        } else {
                            json
                        }
                    })
                    .collect();

                format!(
                    r#"{{"kind": "MacroMethodCall", "lhs": {}, "method_name": {}, "type_args": [{}], "args": {}}}"#,
                    self.json_code_expr(&e.lhs),
                    self.json_string(&e.field_name.repr),
                    type_args,
                    self.json_code_expr_list(&e.args)
                )
            }
            CodeExpr::Sizeof(e) => {
                format!(
                    r#"{{"kind": "Sizeof", "type": {}}}"#,
                    self.json_type_expr(&e.type_expr)
                )
            }
            CodeExpr::PropagateError(e) => {
                format!(
                    r#"{{"kind": "PropagateError", "expr": {}}}"#,
                    self.json_code_expr(&e.expr)
                )
            }
        }
    }

    fn json_code_expr_map(&self, map: &CodeExprMap) -> String {
        let fields: String = map
            .fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let json = format!(
                    r#"{{"key": {}, "value": {}}}"#,
                    self.json_string(&field.key),
                    self.json_code_expr(&field.value)
                );
                if i < map.fields.len() - 1 {
                    format!("{},", json)
                } else {
                    json
                }
            })
            .collect();

        format!(r#"{{"fields": [{}]}}"#, fields)
    }

    fn json_match_header(&self, header: &MatchHeader) -> String {
        format!(
            r#"{{"variant_name": {}, "variant_bind": {}, "expr": {}}}"#,
            self.json_string(&header.variant_name.repr),
            self.json_string(&header.variant_bind.repr),
            self.json_code_expr(&header.expr_to_match)
        )
    }

    fn json_code_expr_list(&self, list: &CodeExprList) -> String {
        let items: String = list
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let json = self.json_code_expr(item);
                if i < list.items.len() - 1 {
                    format!("{},", json)
                } else {
                    json
                }
            })
            .collect();

        format!(r#"{{"items": [{}]}}"#, items)
    }

    fn json_string(&self, s: &str) -> String {
        let mut result = String::from("\"");
        for c in s.chars() {
            match c {
                '"' => result.push_str("\\\""),
                '\\' => result.push_str("\\\\"),
                '\n' => result.push_str("\\n"),
                '\r' => result.push_str("\\r"),
                '\t' => result.push_str("\\t"),
                _ => result.push(c),
            }
        }
        result.push('"');
        result
    }

    fn json_option<T>(&self, opt: &Option<T>) -> String
    where
        T: ToString,
    {
        match opt {
            Some(val) => val.to_string(),
            None => "null".to_string(),
        }
    }
}
