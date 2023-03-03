extern crate alloc;

use alloc::vec::Vec;

#[repr(C)]
pub struct RawVec {
    data: *const u8,
    length: usize,
    capacity: usize,
}

#[no_mangle]
pub extern "C" fn compile(script_ptr: *const u8, script_len: usize) -> RawVec {
    // TODO handle input :)
    let script = read_ascii_str(script_ptr, script_len).unwrap();
    let _num_from_script: i32 = script.parse().unwrap();

    let mut wasm_binary = Vec::<u8>::new();

    {
        // wasm magic number
        wasm_binary.extend_from_slice(b"\0asm");

        // version
        wasm_binary.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
    }

    {
        // type section
        wasm_binary.extend_from_slice(&[0x01]);

        // section size
        wasm_binary.extend_from_slice(&[0x05]);

        // number of types
        wasm_binary.extend_from_slice(&[0x01]);

        // func
        wasm_binary.extend_from_slice(&[0x60]);

        // number of parameters
        wasm_binary.extend_from_slice(&[0x00]);

        // number of results
        wasm_binary.extend_from_slice(&[0x01]);

        // result type (i32)
        wasm_binary.extend_from_slice(&[0x7f]);
    }

    {
        // function section
        wasm_binary.extend_from_slice(&[0x03]);

        // section size
        wasm_binary.extend_from_slice(&[0x02]);

        // number of functions
        wasm_binary.extend_from_slice(&[0x01]);

        // index of the function
        wasm_binary.extend_from_slice(&[0x00]);
    }

    {
        // exports section
        wasm_binary.extend_from_slice(&[0x07]);

        // section size
        wasm_binary.extend_from_slice(&[0x08]);

        // number of exports
        wasm_binary.extend_from_slice(&[0x01]);

        // length of export name
        wasm_binary.extend_from_slice(&[0x04]);

        // export name
        wasm_binary.extend_from_slice(b"main");

        // export kind (function)
        wasm_binary.extend_from_slice(&[0x00]);

        // index of the exported function
        wasm_binary.extend_from_slice(&[0x00]);
    }

    {
        // code section (ID 10)
        wasm_binary.extend_from_slice(&[0x0a]);

        // section size
        wasm_binary.extend_from_slice(&[0x07]);

        // number of functions
        wasm_binary.extend_from_slice(&[0x01]);

        // function body size
        wasm_binary.extend_from_slice(&[0x05]);

        // number of local declarations
        wasm_binary.extend_from_slice(&[0x00]);

        // i32.const
        wasm_binary.extend_from_slice(&[0x41]);

        // i32.literal
        wasm_binary.extend_from_slice(&[0x2a]);

        // return
        wasm_binary.extend_from_slice(&[0x0f]);

        // end of function
        wasm_binary.extend_from_slice(&[0x0b]);
    }

    let (data, size, capacity) = wasm_binary.into_raw_parts();

    RawVec {
        data,
        length: size,
        capacity,
    }
}

#[no_mangle]
pub extern "C" fn free_binary(wasm_binary: RawVec) {
    let _vec = unsafe {
        Vec::<u8>::from_raw_parts(
            wasm_binary.data as *mut u8,
            wasm_binary.length,
            wasm_binary.capacity,
        )
    };
}

fn read_ascii_str<'a>(chars: *const u8, chars_len: usize) -> Option<&'a str> {
    unsafe { core::str::from_utf8(core::slice::from_raw_parts(chars, chars_len)).ok() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use std::io::Write;

        let (script, script_len) = make_ascii_str("1");

        let result = compile(script, script_len);

        let a2 = unsafe { std::slice::from_raw_parts(result.data, result.length) };

        let mut f1 = std::fs::File::create("tmp/main.wasm").unwrap();
        f1.write_all(a2).unwrap();
        f1.flush().unwrap();

        // TODO: add assertions
    }

    fn make_ascii_str(data: &str) -> (*const u8, usize) {
        return (data.as_bytes().as_ptr(), data.as_bytes().len());
    }
}
