pub type Err = u32;
pub const ERR_SUCCESS: Err = 0;
pub const ERR_AGAIN: Err = 6;

pub const FD_STDIN: u32 = 0;
pub const FD_STDOUT: u32 = 1;
pub const FD_STDERR: u32 = 2;

#[repr(C)]
#[derive(Default)]
pub struct Prestat {
    pub tag: u8,
    pub pr_name_len: u32,
}

#[repr(C)]
#[derive(Default)]
pub struct Fdstat {
    pub fs_filetype: u8,
    pub fs_flags: u16,
    pub fs_rights_base: u64,
    pub fs_rights_inheriting: u64,
}

#[repr(C)]
pub struct IOVec {
    pub base: *const u8,
    pub size: u32,
}

#[repr(C)]
pub struct IOVecMut {
    pub base: *mut u8,
    pub size: u32,
}

#[link(wasm_import_module = "wasi_snapshot_preview1")]
unsafe extern "C" {
    // @param dirfd ---------------- The file descriptor representing the directory that the file is located in.
    // @param dirflags ------------- Flags specifying how the path will be resolved.
    // @param path ----------------- A wasm pointer to a null-terminated string containing the path of the file or directory to open.
    // @param path_len ------------- The length of the path string.
    // @param oflags --------------- Flags specifying how the file will be opened.
    // @param fs_rights_base ------- The rights of the created file descriptor.
    // @param fs_rights_inheriting - The rights of file descriptors derived from the created file descriptor.
    // @param fdflags -------------- The flags of the file descriptor.
    // @param fd ------------------- A wasm pointer to a wasiFd variable where the new file descriptor will be stored.
    pub fn path_open(
        dirfd: u32,
        dirflags: u32,
        path: *const u8,
        path_len: u32,
        oflags: u32,
        fs_rights_base: u64,
        fs_rights_inheriting: u64,
        fdflags: u32,
        fd: *mut u32,
    ) -> Err;

    // @param fd ------- file_descriptor
    // @param iovs ----- The pointer to the iov array
    // @param iovs_len - Amount of wasi::IOVec
    // @param nread ---- Pointer to store the number of bytes read
    pub fn fd_read(fd: u32, iovs: *mut IOVecMut, iovs_len: u32, nread: *mut u32) -> Err;

    // @param fd ------- file_descriptor
    // @param iovs ----- The pointer to the iov array
    // @param iovs_len - Amount of wasi::IOVec
    // @param nwritten - Pointer to store the number of bytes written
    pub fn fd_write(fd: u32, iovs: *const IOVec, iovs_len: u32, nwritten: *mut u32) -> Err;

    // @param fd - file_descriptor
    pub fn fd_close(fd: u32) -> Err;

    // @param argc ---------- Pointer to where the number of arguments will be written.
    // @param argv_buf_size - Pointer to where the size of the argument string data will be written.
    pub fn args_sizes_get(argc: *mut u32, argv_buf_size: *mut u32) -> Err;

    // @param argv ----- Pointer to a buffer where the argument pointers will be written.
    // @param argv_buf - Pointer to a buffer where the argument string data will be written.
    pub fn args_get(argv: *mut *mut u8, argv_buf: *mut u8) -> Err;

    // @param code - The exit code to return to the operating system.
    pub fn proc_exit(exit_code: u32) -> !;

    // non-essentials:

    // @param fd -- The preopened file descriptor to query.
    // @param buf - A pointer to a Prestat structure where the metadata will be written.
    pub fn fd_prestat_get(fd: u32, buf: *mut Prestat) -> Err;

    // @param fd ------- The preopened file descriptor to query.
    // @param path ----- A pointer to a buffer where the directory name will be written.
    // @param path_len - The maximum length of the buffer.
    pub fn fd_prestat_dir_name(fd: u32, path: *mut u8, path_len: u32) -> Err;

    // @param fd -- The file descriptor whose metadata will be accessed.
    // @param buf - A WebAssembly pointer to a memory location where the metadata will be written.
    pub fn fd_fdstat_get(fd: u32, buf: *mut Fdstat) -> Err;
}
