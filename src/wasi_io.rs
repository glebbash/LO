use alloc::vec::Vec;

pub fn fd_read(fd: u32) -> Vec<u8> {
    let mut source = Vec::<u8>::new();
    let mut chunk = [0; 256];

    let in_vec = [wasi::Iovec {
        buf: chunk.as_mut_ptr(),
        buf_len: chunk.len(),
    }];

    loop {
        let nread = unsafe { wasi::fd_read(fd, &in_vec) }.unwrap();

        if nread == 0 {
            break;
        }

        source.extend(&chunk[0..nread]);
    }

    source
}

pub fn fd_write(fd: u32, message: &[u8]) {
    let err_vec = [wasi::Ciovec {
        buf: message.as_ptr(),
        buf_len: message.len(),
    }];

    unsafe { wasi::fd_write(fd, &err_vec) }.unwrap();
}
