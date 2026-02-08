use crate::common::*;

pub struct FileInfo {
    pub index: usize,
    pub included_times: usize,
    pub absolute_path: String,
    pub source: String,
}

pub struct FileManager {
    pub files: Vec<FileInfo>,
}

impl Default for FileManager {
    fn default() -> Self {
        let mut files = Vec::new();
        files.push(FileInfo {
            index: 0,
            included_times: 0,
            absolute_path: String::from("<internal>"),
            source: String::from(""),
        });
        Self { files }
    }
}

impl FileManager {
    pub fn include_file(&mut self, relative_path: &str, loc: &Loc) -> Result<usize, Error> {
        let absolute_path = self.resolve_path(relative_path, loc);

        for file in &mut self.files {
            if file.absolute_path == absolute_path {
                file.included_times += 1;
                return Ok(file.index);
            }
        }

        let file_contents =
            fs::file_read_utf8(&absolute_path).map_err(|message| Error { message, loc: *loc })?;

        let file_id = self.files.len();
        self.files.push(FileInfo {
            index: file_id,
            included_times: 1,
            absolute_path: absolute_path.into(),
            source: file_contents,
        });

        Ok(file_id)
    }

    pub fn resolve_path(&self, file_path: &str, loc: &Loc) -> String {
        let relative_to = &self.files[loc.file_id].absolute_path;

        if !file_path.starts_with('.') {
            return file_path.into();
        }

        let mut path_items = relative_to.split('/').collect::<Vec<_>>();
        path_items.pop(); // remove `relative_to`'s file name

        path_items.extend(file_path.split('/'));

        let mut i = 0;
        loop {
            if i >= path_items.len() {
                break;
            }

            if path_items[i] == "." {
                path_items.remove(i);
                continue;
            }

            if path_items[i] == ".." && i > 0 {
                i -= 1;
                path_items.remove(i);
                path_items.remove(i);
                continue;
            }

            i += 1;
        }

        path_items.join("/")
    }
}

const CWD_PREOPEN_FD: u32 = 3;

pub struct WasiArgs {
    pub size: usize,
    pub argv: Vec<*mut u8>,
    pub _argv_buf: Vec<u8>,
}

impl WasiArgs {
    pub fn load() -> Result<Self, u32> {
        let mut argc = 0u32;
        let mut argv_buf_size = 0u32;
        let err =
            unsafe { wasi::args_sizes_get(&mut argc as *mut u32, &mut argv_buf_size as *mut u32) };
        if err != wasi::ERR_SUCCESS {
            return Err(err);
        }

        let mut argv = vec![core::ptr::null::<u8>() as *mut u8; argc as usize];
        let mut _argv_buf = vec![0u8; argv_buf_size as usize];
        if argc != 0 {
            let err = unsafe { wasi::args_get(argv.as_mut_ptr(), _argv_buf.as_mut_ptr()) };
            if err != wasi::ERR_SUCCESS {
                return Err(err);
            }
        }

        Ok(Self {
            size: argc as usize,
            argv,
            _argv_buf,
        })
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        if index >= self.size {
            return None;
        }

        unsafe { CStr::from_ptr(self.argv[index] as *const i8).to_str().ok() }
    }
}

pub fn proc_exit(exit_code: u32) -> ! {
    unsafe { wasi::proc_exit(exit_code) };
}

pub mod fs {
    use super::*;

    /// Hack for https://github.com/microsoft/vscode-wasm/issues/161
    pub fn unlock_fs() -> Result<(), u32> {
        let mut prestat = wasi::Prestat::default();
        let err =
            unsafe { wasi::fd_prestat_get(CWD_PREOPEN_FD, &mut prestat as *mut wasi::Prestat) };
        if err != wasi::ERR_SUCCESS {
            return Err(err);
        }

        let path_len = prestat.pr_name_len;
        let mut path_buf = vec![0u8; path_len as usize];
        let err =
            unsafe { wasi::fd_prestat_dir_name(CWD_PREOPEN_FD, path_buf.as_mut_ptr(), path_len) };
        if err != wasi::ERR_SUCCESS {
            return Err(err);
        }

        let mut fdstat = wasi::Fdstat::default();
        let _err = unsafe { wasi::fd_fdstat_get(CWD_PREOPEN_FD, &mut fdstat as *mut wasi::Fdstat) };

        let mut prestat2 = wasi::Prestat::default();
        let _err = unsafe {
            wasi::fd_prestat_get(CWD_PREOPEN_FD + 1, &mut prestat2 as *mut wasi::Prestat)
        };

        Ok(())
    }

    static mut FS_UNLOCKED: bool = false;

    pub fn file_read_utf8(file_path: &str) -> Result<String, String> {
        let bytes = file_read(file_path)?;

        let Ok(chars) = String::from_utf8(bytes) else {
            return Err(format!("Contents of `{file_path}` are not valid UTF-8"));
        };

        return Ok(chars);
    }

    pub fn file_read(file_path: &str) -> Result<Vec<u8>, String> {
        if file_path == "<stdin>" {
            return fd_read_all(wasi::FD_STDIN)
                .map_err(|err| format!("Cannot read <stdin>: error code = {err}"));
        };

        if unsafe { !FS_UNLOCKED } {
            unlock_fs().map_err(|err| format!("Error unlocking fs: error code = {err}"))?;
            unsafe { FS_UNLOCKED = true };
        }

        let fd = fd_open(&file_path)
            .map_err(|err| format!("Cannot open file {file_path}: error code = {err}"))?;

        let bytes =
            fd_read_all(fd).map_err(|err| format!("Cannot read file {file_path}: {err}"))?;

        let err = unsafe { wasi::fd_close(fd) };
        if err != wasi::ERR_SUCCESS {
            return Err(format!("Cannot close file {file_path}: error code = {err}"));
        };

        return Ok(bytes);
    }

    fn fd_open(file_path: &str) -> Result<u32, u32> {
        let mut fd = 0;
        let err = unsafe {
            wasi::path_open(
                CWD_PREOPEN_FD,
                1,
                file_path.as_ptr(),
                file_path.len() as u32,
                0,
                264240830,
                268435455,
                0,
                &mut fd as *mut u32,
            )
        };
        if err != wasi::ERR_SUCCESS {
            return Err(err);
        }

        Ok(fd)
    }

    fn fd_read_all(fd: u32) -> Result<Vec<u8>, String> {
        let mut output = Vec::<u8>::new();
        let mut chunk = [0; 256];

        let mut in_vec = wasi::IOVecMut {
            base: chunk.as_mut_ptr(),
            size: chunk.len() as u32,
        };

        loop {
            let mut nread = 0u32;
            let err = unsafe {
                wasi::fd_read(
                    fd,
                    &mut in_vec as *mut wasi::IOVecMut,
                    1,
                    &mut nread as *mut u32,
                )
            };
            if err != wasi::ERR_SUCCESS {
                // stdin is empty
                if fd == 0 && err == wasi::ERR_AGAIN {
                    break;
                }

                return Err(alloc::format!("Error reading file: fd={fd}, err={err}\n"));
            }

            if nread == 0 {
                break;
            }

            output.extend(&chunk[0..nread as usize]);
        }

        Ok(output)
    }
}

#[thread_local]
static STDOUT_BUFFER: UBCell<Option<Vec<u8>>> = UBCell::new(None);
const STDOUT_BUFFER_SIZE: usize = 4096;

pub fn stdout_enable_buffering() {
    *STDOUT_BUFFER.be_mut() = Some(Vec::with_capacity(STDOUT_BUFFER_SIZE));
}

pub fn stdout_disable_buffering() {
    if let Some(buffer) = &mut *STDOUT_BUFFER.be_mut() {
        if !buffer.is_empty() {
            fputs(wasi::FD_STDOUT, &buffer);
        }
    }
    *STDOUT_BUFFER.be_mut() = None;
}

pub fn stdout_writeln(message: impl AsRef<[u8]>) {
    stdout_write(message);
    stdout_write("\n");
}

pub fn stdout_write(message: impl AsRef<[u8]>) {
    let message_bytes = message.as_ref();

    let Some(buffer) = &mut *STDOUT_BUFFER.be_mut() else {
        fputs(wasi::FD_STDOUT, message_bytes);
        return;
    };

    if buffer.len() + message_bytes.len() > STDOUT_BUFFER_SIZE {
        if !buffer.is_empty() {
            fputs(wasi::FD_STDOUT, buffer);
            buffer.clear();
        }
    }

    if message_bytes.len() >= STDOUT_BUFFER_SIZE {
        fputs(wasi::FD_STDOUT, message_bytes);
        return;
    }

    buffer.extend(message_bytes);
}

pub fn stderr_writeln(message: impl AsRef<str>) {
    stderr_write(message);
    stderr_write("\n");
}

pub fn stderr_write(message: impl AsRef<str>) {
    fputs(wasi::FD_STDERR, message.as_ref().as_bytes());
}

pub fn fputs(fd: u32, message: &[u8]) {
    let mut nwritten = 0u32;
    let out_vec = wasi::IOVec {
        base: message.as_ptr(),
        size: message.len() as u32,
    };
    let err = unsafe {
        wasi::fd_write(
            fd,
            &out_vec as *const wasi::IOVec,
            1,
            &mut nwritten as *mut u32,
        )
    };
    if err != wasi::ERR_SUCCESS {
        unreachable!()
    };
}

pub mod wasi {
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
}
