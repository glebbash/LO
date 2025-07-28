use alloc::{fmt, format, string::String, vec, vec::Vec};
use core::{cell::RefCell, default, ffi::CStr, str};

#[derive(Default, PartialEq, Clone, Copy)]
pub enum LoCommand {
    #[default]
    Compile,
    Inspect,
    Format,
    Eval,
    Wasi,
}

#[derive(PartialEq)]
pub struct LoError {
    pub message: String,
    pub loc: LoLocation,
}

#[macro_export]
macro_rules! lo_todo {
    ($loc:expr) => {
        LoError {
            message: format!("Not implemented feature at {}:{}", file!(), line!()),
            loc: $loc,
        }
    };
}

impl LoError {
    pub fn format(&self, out: &mut impl fmt::Write, fm: &FileManager) -> core::fmt::Result {
        self.loc.format(out, &fm)?;
        write!(out, " - {}", self.message)
    }

    pub fn to_string(&self, fm: &FileManager) -> String {
        let mut out = String::new();
        self.format(&mut out, fm).unwrap();
        out
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LoPosition {
    pub offset: usize,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoLocation {
    pub file_index: u32,
    pub pos: LoPosition,
    pub end_pos: LoPosition,
}

impl LoLocation {
    pub fn internal() -> Self {
        LoLocation {
            file_index: 0, // internal
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

    pub fn format(&self, out: &mut impl fmt::Write, fm: &FileManager) -> core::fmt::Result {
        let file_path = fm.get_file_path(self.file_index);
        write!(out, "{}:{}:{}", file_path, self.pos.line, self.pos.col)
    }

    pub fn to_string(&self, fm: &FileManager) -> String {
        let mut out = String::new();
        self.format(&mut out, fm).unwrap();
        out
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

#[thread_local]
static STDOUT_BUFFER: RefCell<Option<Vec<u8>>> = RefCell::new(None);
const STDOUT_BUFFER_SIZE: usize = 4096;

pub fn stdout_enable_buffering() {
    *STDOUT_BUFFER.borrow_mut() = Some(Vec::with_capacity(STDOUT_BUFFER_SIZE));
}

pub fn stdout_disable_buffering() {
    if let Some(buffer) = &mut *STDOUT_BUFFER.borrow_mut() {
        if !buffer.is_empty() {
            fputs(wasi::FD_STDOUT, &buffer);
            buffer.clear();
        }
    }
    *STDOUT_BUFFER.borrow_mut() = None;
}

pub fn stdout_writeln(message: impl AsRef<[u8]>) {
    stdout_write(message);
    stdout_write("\n");
}

pub fn stdout_write(message: impl AsRef<[u8]>) {
    let message_bytes = message.as_ref();

    let Some(buffer) = &mut *STDOUT_BUFFER.borrow_mut() else {
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

    buffer.extend_from_slice(message_bytes);
}

pub fn stderr_writeln(message: impl AsRef<str>) {
    stderr_write(message);
    stderr_write("\n");
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
pub fn debug(msg: impl AsRef<str>) {
    stderr_write(format!("{}\n", msg.as_ref()));
}

pub struct ListDisplay<'a, T: core::fmt::Display>(pub &'a [T]);

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

#[derive(Debug)]
struct FileInfo {
    index: u32,
    path: String,
    contents: String,
}

pub struct FileManager {
    files: Vec<FileInfo>,
}

impl default::Default for FileManager {
    fn default() -> Self {
        let mut files = Vec::new();
        files.push(FileInfo {
            index: 0,
            path: String::from("<internal>"),
            contents: String::from(""),
        });
        Self { files }
    }
}

impl FileManager {
    pub fn include_file(
        &mut self,
        file_name: &str,
        is_newly_added: Option<&mut bool>,
        loc: &LoLocation,
    ) -> Result<u32, LoError> {
        let parent_path = self.get_file_path(loc.file_index);
        let absolute_file_path = resolve_path(file_name, parent_path);

        for parsed_file in &self.files {
            if parsed_file.path == absolute_file_path {
                return Ok(parsed_file.index);
            }
        }

        let file_contents = file_read_utf8(&absolute_file_path).map_err(|message| LoError {
            message,
            loc: loc.clone(),
        })?;

        let file_index = self.files.len() as u32;
        self.files.push(FileInfo {
            index: file_index,
            path: absolute_file_path.into(),
            contents: file_contents,
        });

        if let Some(is_newly_added) = is_newly_added {
            *is_newly_added = true;
        }

        Ok(file_index)
    }

    pub fn get_file_path(&self, file_index: u32) -> &str {
        &self.files[file_index as usize].path
    }

    pub fn get_file_contents(&self, file_index: u32) -> &str {
        &self.files[file_index as usize].contents
    }
}

fn resolve_path(file_path: &str, relative_to: &str) -> String {
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

macro_rules! catch {
    ($result:expr, $err_var:ident, $err_block:block) => {
        match $result {
            Ok(val) => val,
            Err($err_var) => $err_block,
        }
    };
}
pub(crate) use catch;

pub fn unsafe_borrow<T>(x: &T) -> &'static T {
    unsafe { &*(x as *const T) }
}
