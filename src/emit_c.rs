use crate::{ir::*, utils::stdout_write};
use alloc::format;

/*
*experimental*: emit C code
currently can only supports defining structs and function definitions
to generate function bodies AST is needed
*/
pub fn emit_c(ctx: &ModuleContext) {
    stdout_write(format!("#define bool _Bool\n").as_bytes());
    stdout_write(format!("#define true 1\n").as_bytes());
    stdout_write(format!("#define false 0\n\n").as_bytes());

    stdout_write(format!("#define never void __attribute__((noreturn))\n").as_bytes());
    stdout_write(format!("#define Void struct {{}}\n").as_bytes());
    stdout_write(format!("#define u8 unsigned char\n").as_bytes());
    stdout_write(format!("#define i8 signed char\n").as_bytes());
    stdout_write(format!("#define u16 unsigned short\n").as_bytes());
    stdout_write(format!("#define i16 signed short\n").as_bytes());
    stdout_write(format!("#define u32 unsigned int\n").as_bytes());
    stdout_write(format!("#define i32 signed int\n").as_bytes());
    stdout_write(format!("#define u64 unsigned long\n").as_bytes());
    stdout_write(format!("#define i64 signed long\n").as_bytes());
    stdout_write(format!("#define f32 float\n").as_bytes());
    stdout_write(format!("#define f64 double\n\n").as_bytes());

    stdout_write(format!("#define Result(T, E) struct {{ T value; E error; }}\n\n").as_bytes());

    for struct_def in &ctx.struct_defs {
        let struct_name = &struct_def.name;
        stdout_write(
            format!("struct {struct_name} {{\n")
                .replace("::", "_")
                .as_bytes(),
        );

        for field in &struct_def.fields {
            let field_name = &field.name;
            let field_type = CDisplayForLoType(&field.value_type);
            stdout_write(format!("    {field_type} {field_name};\n").as_bytes());
        }

        stdout_write(format!("}};\n\n").as_bytes());
    }

    for (fn_name, fn_def) in &ctx.fn_defs {
        let return_type = CDisplayForLoType(&fn_def.type_.output);
        let fn_params = CDisplayForFnParams(&fn_def.fn_params);
        stdout_write(
            format!("{return_type} {fn_name}({fn_params});\n\n")
                .replace("::", "_")
                .as_bytes(),
        );
    }
}

struct CDisplayForLoType<'a>(&'a LoType);

impl<'a> core::fmt::Display for CDisplayForLoType<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            LoType::Never => f.write_str("never"),
            LoType::Void => f.write_str("void"),
            LoType::Bool => f.write_str("bool"),
            LoType::U8 => f.write_str("u8"),
            LoType::I8 => f.write_str("i8"),
            LoType::U16 => f.write_str("u16"),
            LoType::I16 => f.write_str("i16"),
            LoType::U32 => f.write_str("u32"),
            LoType::I32 => f.write_str("i32"),
            LoType::F32 => f.write_str("f32"),
            LoType::U64 => f.write_str("u64"),
            LoType::I64 => f.write_str("i64"),
            LoType::F64 => f.write_str("f64"),
            LoType::Pointer(pointee) => {
                f.write_fmt(format_args!("{}*", CDisplayForLoType(pointee)))
            }
            LoType::Tuple(types) => {
                f.write_str("void* /* (")?;
                let mut types_iter = types.iter();
                if let Some(item) = types_iter.next() {
                    f.write_fmt(format_args!("{}", CDisplayForLoType(item)))?;
                }
                for item in types_iter {
                    f.write_str(", ")?;
                    f.write_fmt(format_args!("{}", CDisplayForLoType(item)))?;
                }
                f.write_str(") */")
            }
            LoType::StructInstance { name } => f.write_fmt(format_args!("struct {name}")),
            LoType::Result { ok_type, err_type } => {
                // TODO: figure out a better way?
                let ok_type = if let LoType::Void = **ok_type {
                    LoType::MacroTypeArg {
                        name: format!("Void"),
                    }
                } else {
                    *ok_type.clone()
                };

                f.write_fmt(format_args!(
                    "Result({}, {})",
                    CDisplayForLoType(&ok_type),
                    CDisplayForLoType(err_type)
                ))?;
                Ok(())
            }
            LoType::MacroTypeArg { name } => f.write_str(&name),
        }
    }
}

struct CDisplayForFnParams<'a>(&'a [FnParam]);

impl<'a> core::fmt::Display for CDisplayForFnParams<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (fn_param, index) in self.0.iter().zip(0..) {
            if index != 0 {
                f.write_str(", ")?;
            }

            f.write_fmt(format_args!(
                "{} {}",
                CDisplayForLoType(&fn_param.type_),
                fn_param.name
            ))?;
        }
        Ok(())
    }
}
