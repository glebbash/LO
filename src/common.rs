pub(crate) use crate::io::*;
use crate::registry::Registry;
pub(crate) use alloc::{
    boxed::Box, collections::BTreeMap, fmt::Write, format, string::String, vec, vec::Vec,
};
pub(crate) use core::{cell::UnsafeCell, ffi::CStr, str};

#[derive(PartialEq, Clone)]
pub struct Error {
    pub message: String,
    pub loc: Loc,
}

impl Error {
    pub fn to_string(&self, registry: &Registry) -> String {
        let mut out = String::new();
        self.loc.format(&mut out, registry);
        out.push_str(" - ");
        out.push_str(&self.message);
        out
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Pos {
    pub offset: usize,
    pub line: usize,
    pub col: usize,
}

#[derive(PartialEq, Clone, Copy)]
pub struct Loc {
    pub file_id: usize,
    pub pos: Pos,
    pub end_pos: Pos,
}

impl Loc {
    pub fn internal() -> Self {
        Loc {
            file_id: 0, // internal
            pos: Pos {
                offset: 0,
                line: 1,
                col: 1,
            },
            end_pos: Pos {
                offset: 0,
                line: 1,
                col: 1,
            },
        }
    }

    pub fn read_span(&self, source: &'static [u8]) -> &'static str {
        return unsafe { str::from_utf8_unchecked(&source[self.pos.offset..self.end_pos.offset]) };
    }

    pub fn format(&self, out: &mut String, registry: &Registry) {
        let file_path = &registry.files[self.file_id].absolute_path;
        write!(out, "{}:{}:{}", file_path, self.pos.line, self.pos.col).unwrap();
    }

    pub fn to_string(&self, registry: &Registry) -> String {
        let mut out = String::new();
        self.format(&mut out, registry);
        out
    }
}

pub struct InspectInfo {
    pub message: String,
    pub loc: Loc,
    pub linked_loc: Option<Loc>,
}

#[derive(Default)]
pub struct Reporter {
    pub registry: UBRef<Registry>,

    pub in_inspection_mode: bool,

    pub error_count: UBCell<u32>,
    pub warning_count: UBCell<u32>,
}

impl Reporter {
    pub fn begin_inspection(&self) {
        self.be_mut().in_inspection_mode = true;
        stdout_enable_buffering();
        stdout_writeln("[");
    }

    pub fn end_inspection(&self) {
        self.be_mut().in_inspection_mode = false;
        // this item is a stub to make json array valid
        //   as last inspection ended with a comma
        stdout_writeln("{ \"type\": \"end\" }");
        stdout_writeln("]");
        stdout_disable_buffering();
    }

    pub fn error(&self, err: &Error) {
        *self.error_count.be_mut() += 1;

        if self.in_inspection_mode {
            let source_index = err.loc.file_id;
            let source_range = RangeFmt(&err.loc);
            let content = json_str_escape(&err.message);
            stdout_writeln(format!(
                "{{ \"type\": \"message\", \
                    \"content\": \"{content}\", \
                    \"severity\": \"error\", \
                    \"loc\": \"{source_index}/{source_range}\" }},",
            ));
            return;
        }

        stderr_write("ERROR: ");
        stderr_write(err.to_string(&*self.registry));
        stderr_write("\n");
    }

    pub fn warning(&self, err: &Error) {
        *self.warning_count.be_mut() += 1;

        if self.in_inspection_mode {
            let source_index = err.loc.file_id;
            let source_range = RangeFmt(&err.loc);
            let content = json_str_escape(&err.message);
            stdout_writeln(format!(
                "{{ \"type\": \"message\", \
                    \"content\": \"{content}\", \
                    \"severity\": \"warning\", \
                    \"loc\": \"{source_index}/{source_range}\" }},",
            ));
            return;
        }

        stderr_write("WARNING: ");
        stderr_write(err.to_string(&self.registry));
        stderr_write("\n");
    }

    pub fn print_inspection(&self, inspect_info: InspectInfo) {
        let source_index = inspect_info.loc.file_id;
        let source_range = RangeFmt(&inspect_info.loc);
        let message = json_str_escape(&inspect_info.message);

        if let Some(linked_loc) = &inspect_info.linked_loc {
            if linked_loc.file_id != 0 {
                let target_index = linked_loc.file_id;
                let target_range = RangeFmt(&linked_loc);
                stdout_writeln(format!(
                    "{{ \"type\": \"info\", \
                        \"link\": \"{target_index}/{target_range}\", \
                        \"hover\": \"{message}\", \
                        \"loc\": \"{source_index}/{source_range}\" }},",
                ));
                return;
            }
        };

        stdout_writeln(format!(
            "{{ \"type\": \"info\", \
                \"hover\": \"{message}\", \
                \"loc\": \"{source_index}/{source_range}\" }},",
        ));
    }

    pub fn print_include_info(&self, file_is_newly_added: bool, file_id: usize, loc: &Loc) {
        if file_is_newly_added {
            let file_id = file_id;
            let file_path = &self.registry.files[file_id].absolute_path;
            stdout_writeln(format!(
                "{{ \"type\": \"file\", \
                    \"index\": {file_id}, \
                    \"path\": \"{file_path}\" }},",
            ));
        }

        if loc.file_id != 0 {
            let source_index = loc.file_id;
            let source_range = RangeFmt(loc);
            let target_index = file_id;
            let target_range = "1:1-1:1";

            stdout_writeln(format!(
                "{{ \"type\": \"info\", \
                    \"link\": \"{target_index}/{target_range}\", \
                    \"loc\": \"{source_index}/{source_range}\" }},",
            ));
        }
    }

    pub fn abort_due_to_compiler_bug(&self, message: &str, loc: Loc) -> ! {
        self.error(&Error {
            message: format!("Compiler bug: {message}"),
            loc,
        });
        self.end_inspection();
        proc_exit(1)
    }
}

fn json_str_escape(value: &str) -> String {
    value
        .replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
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

pub struct RangeFmt<'a>(pub &'a Loc);

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
    #[inline(always)]
    fn be_mut(&self) -> &mut Self {
        unsafe {
            #[allow(invalid_reference_casting)]
            &mut *(self as *const Self as *mut Self)
        }
    }

    #[inline(always)]
    fn relax(&self) -> &'static T {
        unsafe { &*(self as *const T) }
    }

    #[inline(always)]
    fn relax_mut(&mut self) -> &'static mut T {
        unsafe { &mut *(self as *mut T) }
    }
}

#[derive(Default)]
pub struct UBCell<T>(UnsafeCell<T>);

impl<T> UBCell<T> {
    pub const fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }

    #[inline(always)]
    pub fn be_mut(&self) -> &mut T {
        unsafe { &mut *self.0.get() }
    }
}

impl<T> core::ops::Deref for UBCell<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0.get() }
    }
}

impl<T> core::ops::DerefMut for UBCell<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0.get() }
    }
}

#[derive(Default)]
pub struct UBRef<T: ?Sized>(pub *const T);

impl<T: ?Sized> UBRef<T> {
    pub const fn new(value: *const T) -> Self {
        Self(value)
    }
}

impl<T> Copy for UBRef<T> {}

impl<T> Clone for UBRef<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> core::ops::Deref for UBRef<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<T> core::ops::DerefMut for UBRef<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.0 as *mut T) }
    }
}
