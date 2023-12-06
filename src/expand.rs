use crate::{ast::*, parser::*, wasi_io::*};
use alloc::{
    collections::{BTreeMap, BTreeSet},
    format,
    string::String,
    vec,
    vec::Vec,
};
use core::str;

#[derive(Default)]
struct ExpandContext {
    pub included_modules: BTreeSet<String>,
    pub aliases: BTreeMap<String, Vec<SExpr>>,
}

pub fn expand(raw_exprs: Vec<SExpr>) -> Result<Vec<SExpr>, LoleError> {
    let mut ctx = ExpandContext::default();
    let mut exprs = vec![];

    for expr in raw_exprs {
        expand_top_level_expr(&mut ctx, expr, &mut exprs)?;
    }

    return Ok(exprs);
}

fn expand_top_level_expr(
    ctx: &mut ExpandContext,
    expr: SExpr,
    exprs: &mut Vec<SExpr>,
) -> Result<(), LoleError> {
    let SExpr::List { value: items, .. } = &expr else {
        expand_expr(ctx, expr, exprs);
        return Ok(());
    };

    let [SExpr::Atom { value: op, loc: op_loc, kind: AtomKind::Symbol }, other @ ..] = &items[..] else {
        expand_expr(ctx, expr, exprs);
        return Ok(());
    };

    match op.as_str() {
        "alias" => match other {
            [SExpr::Atom {
                value: alias,
                loc: alias_loc,
                kind: AtomKind::Symbol,
            }, other @ ..] => {
                if let Some(_) = ctx.aliases.insert(alias.clone(), (*other).to_vec()) {
                    return Err(LoleError {
                        message: format!("Duplicate alias: {alias}"),
                        loc: alias_loc.clone(),
                    });
                };
            }
            _ => {
                return Err(LoleError {
                    message: format!("Invalid arguments for {op}"),
                    loc: op_loc.clone(),
                });
            }
        },
        "mod" => match other {
            [SExpr::Atom {
                value: mod_name,
                loc: mod_name_loc,
                kind: _,
            }] => {
                if !ctx.included_modules.insert(mod_name.clone()) {
                    // do not include module twice
                    return Ok(());
                };

                let file_name = format!("{}.lole", mod_name);
                let mod_fd = open(&file_name).map_err(|err| LoleError {
                    message: format!("Cannot load file {file_name}: {err}"),
                    loc: mod_name_loc.clone(),
                })?;

                let source_buf = fd_read_all(mod_fd);
                let source = str::from_utf8(source_buf.as_slice()).unwrap();

                let raw_exprs = parse(&file_name, source)?;

                for expr in raw_exprs {
                    expand_top_level_expr(ctx, expr, exprs)?;
                }
            }
            _ => {
                return Err(LoleError {
                    message: format!("Invalid arguments for {op}"),
                    loc: op_loc.clone(),
                });
            }
        },
        _ => {
            expand_expr(ctx, expr, exprs);
        }
    }

    Ok(())
}

fn expand_expr(ctx: &mut ExpandContext, expr: SExpr, exprs: &mut Vec<SExpr>) {
    match expr {
        SExpr::Atom {
            value,
            kind: AtomKind::Symbol,
            ..
        } if ctx.aliases.contains_key(&value) => {
            let replacement = ctx.aliases.get(&value).unwrap().clone();
            for expr in replacement {
                expand_expr(ctx, expr, exprs);
            }
        }
        SExpr::List { value, loc } => {
            let mut sub_exprs = vec![];
            for expr in value {
                expand_expr(ctx, expr, &mut sub_exprs);
            }
            exprs.push(SExpr::List {
                value: sub_exprs,
                loc,
            });
        }
        _ => {
            exprs.push(expr);
        }
    }
}
