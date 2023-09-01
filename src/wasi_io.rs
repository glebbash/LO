use alloc::{string::String, vec::Vec};
use wasi::{
    fd_read, fd_write, path_open, proc_exit, Ciovec, Errno, Iovec, FD_STDERR, FD_STDIN, FD_STDOUT,
};

const CWD_PREOPEN_FD: u32 = 3;

pub fn exit(exit_code: u32) {
    unsafe { proc_exit(exit_code) };
}

pub fn open(file_path: &str) -> Result<u32, Errno> {
    unsafe { path_open(CWD_PREOPEN_FD, 0, &file_path, 0, 2, 0, 0) }
}

pub fn stdin_read() -> Vec<u8> {
    fd_read_all(FD_STDIN)
}

pub fn fd_read_all(fd: u32) -> Vec<u8> {
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
            &[Ciovec {
                buf: msg.as_ptr(),
                buf_len: msg.as_bytes().len(),
            }],
        )
        .unwrap();
    }
}
