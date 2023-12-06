use alloc::{string::String, vec, vec::Vec};
use core::ffi::CStr;
use wasi::*;

const CWD_PREOPEN_FD: u32 = 3;

pub struct WasiArgs {
    size: usize,
    argv: Vec<*mut u8>,
    _argv_buf: Vec<u8>,
}

impl WasiArgs {
    pub fn load() -> Result<Self, wasi::Errno> {
        let (argv_size, argv_buf_size) = unsafe { wasi::args_sizes_get() }?;

        let mut argv = vec![core::ptr::null::<u8>() as *mut u8; argv_size];
        let mut _argv_buf = vec![0u8; argv_buf_size];
        unsafe { wasi::args_get(argv.as_mut_ptr() as *mut *mut u8, _argv_buf.as_mut_ptr()) }?;

        Ok(Self {
            size: argv_size,
            argv,
            _argv_buf,
        })
    }

    pub fn len(&self) -> usize {
        return self.size;
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        if index >= self.len() {
            return None;
        }

        unsafe { CStr::from_ptr(self.argv[index] as *const i8).to_str().ok() }
    }
}

pub fn proc_exit(exit_code: u32) -> ! {
    unsafe { wasi::proc_exit(exit_code) };
    unreachable!(); // needed for typesystem
}

pub fn fd_open(file_path: &str) -> Result<u32, Errno> {
    unsafe { path_open(CWD_PREOPEN_FD, 0, &file_path, 0, 2, 0, 0) }
}

pub fn stdin_read() -> Vec<u8> {
    fd_read_all_and_close(FD_STDIN)
}

pub fn fd_read_all_and_close(fd: u32) -> Vec<u8> {
    let mut output = Vec::<u8>::new();
    let mut chunk = [0; 256];

    let in_vec = [Iovec {
        buf: chunk.as_mut_ptr(),
        buf_len: chunk.len(),
    }];

    loop {
        let nread = unsafe { fd_read(fd, &in_vec) }.unwrap();

        if nread == 0 {
            break;
        }

        output.extend(&chunk[0..nread]);
    }

    if fd != 0 {
        let _ = unsafe { wasi::fd_close(fd) };
    }

    output
}

pub fn stdout_write(message: &[u8]) {
    fputs(FD_STDOUT, message);
}

pub fn stderr_write(message: &[u8]) {
    fputs(FD_STDERR, message);
}

pub fn fputs(fd: u32, message: &[u8]) {
    let out_vec = [Ciovec {
        buf: message.as_ptr(),
        buf_len: message.len(),
    }];

    unsafe { fd_write(fd, &out_vec) }.unwrap();
}

#[allow(dead_code)]
pub fn debug(msg: String) {
    unsafe {
        fd_write(
            FD_STDERR,
            &[
                Ciovec {
                    buf: msg.as_ptr(),
                    buf_len: msg.as_bytes().len(),
                },
                Ciovec {
                    buf: "\n".as_ptr(),
                    buf_len: 1,
                },
            ],
        )
        .unwrap();
    }
}
