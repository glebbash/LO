#![no_std]

#[no_mangle]
pub extern "C" fn compile(script: *const u8, script_len: usize) -> i32 {
    let str = read_wasm_str(script, script_len).unwrap();
    let res: i32 = str.parse().unwrap();
    res
}

fn read_wasm_str<'a>(chars: *const u8, chars_len: usize) -> Option<&'a str> {
    unsafe { core::str::from_utf8(core::slice::from_raw_parts(chars, chars_len)).ok() }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    // if this throws compiler error, try: rustup target add wasm32-unknown-unknown
    core::arch::wasm32::unreachable()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (script, script_len) = build_wasm_str("1");

        let result = compile(script, script_len);

        assert_eq!(result, 1);
    }

    fn build_wasm_str(data: &str) -> (*const u8, usize) {
        return (data.as_bytes().as_ptr(), data.as_bytes().len());
    }
}
