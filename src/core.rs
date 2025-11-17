use crate::wasi;
use alloc::{format, string::String, vec, vec::Vec};
use core::{cell::RefCell, ffi::CStr, fmt::Write, str};

#[derive(PartialEq, Clone)]
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
    pub fn to_string(&self, fm: &FileManager) -> String {
        let mut out = String::new();
        self.loc.format(&mut out, fm);
        out.push_str(" - ");
        out.push_str(&self.message);
        out
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct LoPosition {
    pub offset: usize,
    pub line: usize,
    pub col: usize,
}

#[derive(PartialEq, Clone)]
pub struct LoLocation {
    pub file_index: usize,
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

    pub fn read_span(&self, source: &'static [u8]) -> &str {
        return unsafe { str::from_utf8_unchecked(&source[self.pos.offset..self.end_pos.offset]) };
    }

    pub fn format(&self, out: &mut String, fm: &FileManager) {
        let file_path = &fm.files[self.file_index].absolute_path;
        write!(out, "{}:{}:{}", file_path, self.pos.line, self.pos.col).unwrap();
    }

    pub fn to_string(&self, fm: &FileManager) -> String {
        let mut out = String::new();
        self.format(&mut out, fm);
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
}

/// Hack for https://github.com/microsoft/vscode-wasm/issues/161
pub fn unlock_fs() -> Result<(), u32> {
    let mut prestat = wasi::Prestat::default();
    let err = unsafe { wasi::fd_prestat_get(CWD_PREOPEN_FD, &mut prestat as *mut wasi::Prestat) };
    if err != wasi::ERR_SUCCESS {
        return Err(err);
    }

    let path_len = prestat.pr_name_len;
    let mut path_buf = vec![0u8; path_len as usize];
    let err = unsafe { wasi::fd_prestat_dir_name(CWD_PREOPEN_FD, path_buf.as_mut_ptr(), path_len) };
    if err != wasi::ERR_SUCCESS {
        return Err(err);
    }

    let mut fdstat = wasi::Fdstat::default();
    let _err = unsafe { wasi::fd_fdstat_get(CWD_PREOPEN_FD, &mut fdstat as *mut wasi::Fdstat) };

    let mut prestat2 = wasi::Prestat::default();
    let _err =
        unsafe { wasi::fd_prestat_get(CWD_PREOPEN_FD + 1, &mut prestat2 as *mut wasi::Prestat) };

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

#[allow(dead_code)]
pub fn debug(msg: impl AsRef<str>) {
    stderr_write(format!("{}\n", msg.as_ref()));
}

pub struct ListFmt<'a, T: core::fmt::Display>(pub &'a [T]);

impl<'a, T: core::fmt::Display> core::fmt::Display for ListFmt<'a, T> {
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

pub struct RangeFmt<'a>(pub &'a LoLocation);

impl<'a> core::fmt::Display for RangeFmt<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let sl = self.0.pos.line;
        let sc = self.0.pos.col;
        let el = self.0.end_pos.line;
        let ec = self.0.end_pos.col;

        write!(f, "{sl}:{sc}-{el}:{ec}")?;
        Ok(())
    }
}

pub struct FileInfo {
    pub index: usize,
    pub included_times: usize,
    pub absolute_path: String,
    pub source: String,
}

pub struct FileManager {
    pub files: Vec<FileInfo>,
}

impl FileManager {
    pub fn new() -> Self {
        let mut files = Vec::new();
        files.push(FileInfo {
            index: 0,
            included_times: 0,
            absolute_path: String::from("<internal>"),
            source: String::from(""),
        });
        Self { files }
    }

    pub fn include_file(
        &mut self,
        relative_path: &str,
        loc: &LoLocation,
    ) -> Result<usize, LoError> {
        let parent_path = &self.files[loc.file_index].absolute_path;
        let absolute_path = resolve_path(relative_path, parent_path);

        for file in &mut self.files {
            if file.absolute_path == absolute_path {
                file.included_times += 1;
                return Ok(file.index);
            }
        }

        let file_contents = file_read_utf8(&absolute_path).map_err(|message| LoError {
            message,
            loc: loc.clone(),
        })?;

        let file_index = self.files.len();
        self.files.push(FileInfo {
            index: file_index,
            included_times: 1,
            absolute_path: absolute_path.into(),
            source: file_contents,
        });

        Ok(file_index)
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

pub(crate) trait UnsafeGoodies {
    fn be_mut(&self) -> &mut Self;
    fn relax(&self) -> &'static Self;
    fn relax_mut(&mut self) -> &'static mut Self;
}

impl<T: ?Sized> UnsafeGoodies for T {
    #[allow(invalid_reference_casting)]
    fn be_mut(&self) -> &mut Self {
        unsafe { &mut *(self as *const Self as *mut Self) }
    }

    fn relax(&self) -> &'static T {
        unsafe { &*(self as *const T) }
    }

    fn relax_mut(&mut self) -> &'static mut T {
        unsafe { &mut *(self as *mut T) }
    }
}
