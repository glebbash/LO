use alloc::{format, rc::Rc, string::String, vec, vec::Vec};
use core::{cell::RefCell, ffi::CStr, str};

#[derive(Default, PartialEq)]
pub enum CompilerMode {
    #[default]
    Compile,
    CompileV2,
    Inspect,
    PrettyPrint,
    Eval,
    EvalWasm,
}

#[derive(PartialEq)]
pub struct LoError {
    pub message: String,
    pub loc: LoLocation,
}

impl LoError {
    pub fn unreachable(file: &str, line: u32) -> LoError {
        LoError {
            message: format!("{}:{} - unreachable", file, line),
            loc: LoLocation::internal(),
        }
    }

    pub fn todo(file: &str, line: u32) -> LoError {
        LoError {
            message: format!("{}:{} - not implemented", file, line),
            loc: LoLocation::internal(),
        }
    }
}

impl core::fmt::Display for LoError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{loc} - {msg}", loc = self.loc, msg = self.message)
    }
}

impl From<LoError> for String {
    fn from(err: LoError) -> Self {
        format!("{err}")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoPosition {
    pub offset: usize,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoLocation {
    pub file_name: Rc<str>,

    pub pos: LoPosition,
    pub end_pos: LoPosition,
}

impl LoLocation {
    pub fn internal() -> Self {
        LoLocation {
            file_name: "<internal>".into(),
            pos: LoPosition {
                offset: 0,
                line: 1,
                col: 1,
            },
            end_pos: LoPosition {
                offset: 0,
                line: 1,
                col: 1,
            },
        }
    }
}

impl core::fmt::Display for LoLocation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}:{}", self.file_name, self.pos.line, self.pos.col)
    }
}

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
        if argv_size != 0 {
            unsafe { wasi::args_get(argv.as_mut_ptr() as *mut *mut u8, _argv_buf.as_mut_ptr()) }?;
        }

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

/// Hack for https://github.com/microsoft/vscode-wasm/issues/161
pub fn unlock_fs() -> Result<(), wasi::Errno> {
    use alloc::alloc::*;

    let prestat = unsafe { wasi::fd_prestat_get(CWD_PREOPEN_FD) }?;
    let path_len = unsafe { prestat.u.dir.pr_name_len };
    let path_buf = unsafe { alloc_zeroed(Layout::from_size_align(path_len, 8).unwrap()) };
    let _ = unsafe { wasi::fd_prestat_dir_name(CWD_PREOPEN_FD, path_buf, path_len) }?;
    let _ = unsafe { wasi::fd_prestat_get(CWD_PREOPEN_FD + 1) };
    let _ = unsafe { wasi::fd_fdstat_get(CWD_PREOPEN_FD) };
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

    let bytes = fd_read_all(fd).map_err(|err| format!("Cannot read file {file_path}: {err}"))?;

    if let Err(err) = unsafe { wasi::fd_close(fd) } {
        return Err(format!("Cannot close file {file_path}: error code = {err}"));
    }

    return Ok(bytes);
}

fn fd_open(file_path: &str) -> Result<u32, wasi::Errno> {
    unsafe { wasi::path_open(CWD_PREOPEN_FD, 1, &file_path, 0, 264240830, 268435455, 0) }
}

fn fd_read_all(fd: u32) -> Result<Vec<u8>, String> {
    let mut output = Vec::<u8>::new();
    let mut chunk = [0; 256];

    let in_vec = [wasi::Iovec {
        buf: chunk.as_mut_ptr(),
        buf_len: chunk.len(),
    }];

    loop {
        let nread = match unsafe { wasi::fd_read(fd, &in_vec) } {
            Ok(nread) => nread,
            Err(err) => {
                // stdin is empty
                if fd == 0 && err == wasi::ERRNO_AGAIN {
                    break;
                }

                return Err(alloc::format!("Error reading file: fd={fd}, err={err}\n"));
            }
        };

        if nread == 0 {
            break;
        }

        output.extend(&chunk[0..nread]);
    }

    Ok(output)
}

pub fn stdout_writeln(message: impl AsRef<str>) {
    stdout_write(message);
    stdout_write("\n");
}

#[thread_local]
static STDOUT_BUFFER: RefCell<Option<Vec<u8>>> = RefCell::new(None);
const STDOUT_BUFFER_SIZE: usize = 4096;

pub fn stdout_enable_bufferring() {
    *STDOUT_BUFFER.borrow_mut() = Some(Vec::with_capacity(STDOUT_BUFFER_SIZE));
}

pub fn stdout_disable_bufferring() {
    if let Some(buffer) = &mut *STDOUT_BUFFER.borrow_mut() {
        if !buffer.is_empty() {
            fputs(wasi::FD_STDOUT, &buffer);
            buffer.clear();
        }
    }
    *STDOUT_BUFFER.borrow_mut() = None;
}

pub fn stdout_write(message: impl AsRef<str>) {
    let message_bytes = message.as_ref().as_bytes();

    let Some(buffer) = &mut *STDOUT_BUFFER.borrow_mut() else {
        fputs(wasi::FD_STDOUT, message_bytes);
        return;
    };

    if buffer.len() + message_bytes.len() > STDOUT_BUFFER_SIZE {
        if !buffer.is_empty() {
            fputs(wasi::FD_STDOUT, &buffer);
            buffer.clear();
        }
    }

    if message_bytes.len() >= STDOUT_BUFFER_SIZE {
        fputs(wasi::FD_STDOUT, message_bytes);
    } else {
        buffer.extend_from_slice(message_bytes);
    }
}

pub fn stderr_write(message: impl AsRef<str>) {
    fputs(wasi::FD_STDERR, message.as_ref().as_bytes());
}

pub fn fputs(fd: u32, message: &[u8]) {
    let out_vec = [wasi::Ciovec {
        buf: message.as_ptr(),
        buf_len: message.len(),
    }];

    unsafe { wasi::fd_write(fd, &out_vec) }.unwrap();
}

#[allow(dead_code)]
pub fn debug(msg: String) {
    stderr_write(format!("{msg}\n"));
}

pub fn resolve_path(file_path: &str, relative_to: &str) -> String {
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

pub struct ListDisplay<'a, T: core::fmt::Display>(pub &'a Vec<T>);

impl<'a, T: core::fmt::Display> core::fmt::Display for ListDisplay<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut iter = self.0.iter();
        if let Some(item) = iter.next() {
            write!(f, "{item}")?;
        }
        for item in iter {
            write!(f, ", {item}")?;
        }
        Ok(())
    }
}

pub struct RangeDisplay<'a>(pub &'a LoLocation);

impl<'a> core::fmt::Display for RangeDisplay<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let sl = self.0.pos.line;
        let sc = self.0.pos.col;
        let el = self.0.end_pos.line;
        let ec = self.0.end_pos.col;

        write!(f, "{sl}:{sc}-{el}:{ec}")?;
        Ok(())
    }
}

#[derive(Default)]
pub struct LoErrorManager {
    errors: Vec<LoError>,
}

impl LoErrorManager {
    pub fn report(&mut self, error: LoError) {
        self.errors.push(error);
    }

    pub fn print_all(&self) -> Result<(), String> {
        if self.errors.len() == 0 {
            return Ok(());
        }

        for error in &self.errors {
            stderr_write(format!("{error}\n"));
        }

        Err(format!(""))
    }
}
