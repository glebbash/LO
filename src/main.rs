mod validation_err;
use validation_err::ValidationError;

use inkwell::context::Context;
use std::error::Error;

// fn main() -> Result<(), Box<dyn Error>> {
//     let module_str = compile_module()?;
//     println!("Module: {}", module_str.to_string());
//     Ok(())
// }

#[no_mangle]
pub extern "C" fn compile() {
    match compile_module() {
        Ok(ir) => println!("{}", ir),
        Err(msg) => println!("Error: {}", msg),
    }
}

pub fn compile_module<'a>() -> Result<String, Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("my_module");

    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let sum = module.add_function("sum", fn_type, None);

    let entry = context.append_basic_block(sum, "entry");

    let builder = context.create_builder();
    builder.position_at_end(entry);

    let a = sum.get_nth_param(0).unwrap().into_int_value();
    let b = sum.get_nth_param(1).unwrap().into_int_value();
    let tmp = builder.build_int_add(a, b, "tmp");
    builder.build_return(Some(&tmp));

    if let Err(msg) = module.verify() {
        return Err(ValidationError::new(msg.to_string()).into());
    }

    Ok(module.to_string())
}
