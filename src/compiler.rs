#[no_mangle]
pub extern "C" fn compile(script: *const u8, script_len: usize) -> i32 {
    let str = read_ascii_str(script, script_len).unwrap();
    let res: i32 = str.parse().unwrap();
    res
}

fn read_ascii_str<'a>(chars: *const u8, chars_len: usize) -> Option<&'a str> {
    unsafe { core::str::from_utf8(core::slice::from_raw_parts(chars, chars_len)).ok() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (script, script_len) = make_ascii_str("1");

        let result = compile(script, script_len);

        assert_eq!(result, 1);
    }

    fn make_ascii_str(data: &str) -> (*const u8, usize) {
        return (data.as_bytes().as_ptr(), data.as_bytes().len());
    }
}
