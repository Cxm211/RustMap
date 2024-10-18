#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_b_loc() -> *mut *const std::os::raw::c_ushort;
    fn toupper(_: std::os::raw::c_int) -> std::os::raw::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn realloc(_: *mut u8, _: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn exit(_: std::os::raw::c_int) -> !;
    fn memcpy(
        _: *mut u8,
        _: *const u8,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
    fn strncmp(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: std::os::raw::c_ulong,
    ) -> std::os::raw::c_int;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
}
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::os::raw::c_int,
    pub _IO_read_ptr: *mut std::os::raw::c_char,
    pub _IO_read_end: *mut std::os::raw::c_char,
    pub _IO_read_base: *mut std::os::raw::c_char,
    pub _IO_write_base: *mut std::os::raw::c_char,
    pub _IO_write_ptr: *mut std::os::raw::c_char,
    pub _IO_write_end: *mut std::os::raw::c_char,
    pub _IO_buf_base: *mut std::os::raw::c_char,
    pub _IO_buf_end: *mut std::os::raw::c_char,
    pub _IO_save_base: *mut std::os::raw::c_char,
    pub _IO_backup_base: *mut std::os::raw::c_char,
    pub _IO_save_end: *mut std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::os::raw::c_int,
    pub _flags2: std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: std::os::raw::c_ushort,
    pub _vtable_offset: std::os::raw::c_schar,
    pub _shortbuf: [std::os::raw::c_char; 1],
    pub _lock: *mut u8,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut u8,
    pub __pad5: size_t,
    pub _mode: std::os::raw::c_int,
    pub _unused2: [std::os::raw::c_char; 20],
}
impl std::default::Default for _IO_FILE {
    fn default() -> Self {
        _IO_FILE {
        _flags: std::os::raw::c_int::default(),
        _IO_read_ptr: 0 as * mut std::os::raw::c_char,
        _IO_read_end: 0 as * mut std::os::raw::c_char,
        _IO_read_base: 0 as * mut std::os::raw::c_char,
        _IO_write_base: 0 as * mut std::os::raw::c_char,
        _IO_write_ptr: 0 as * mut std::os::raw::c_char,
        _IO_write_end: 0 as * mut std::os::raw::c_char,
        _IO_buf_base: 0 as * mut std::os::raw::c_char,
        _IO_buf_end: 0 as * mut std::os::raw::c_char,
        _IO_save_base: 0 as * mut std::os::raw::c_char,
        _IO_backup_base: 0 as * mut std::os::raw::c_char,
        _IO_save_end: 0 as * mut std::os::raw::c_char,
        _markers: 0 as * mut crate::_IO_marker,
        _chain: core::ptr::null_mut(),
        _fileno: std::os::raw::c_int::default(),
        _flags2: std::os::raw::c_int::default(),
        _old_offset: std::os::raw::c_long::default(),
        _cur_column: std::os::raw::c_ushort::default(),
        _vtable_offset: std::os::raw::c_schar::default(),
        _shortbuf: [std::os::raw::c_char::default(); 1],
        _lock: 0 as * mut u8,
        _offset: std::os::raw::c_long::default(),
        _codecvt: 0 as * mut crate::_IO_codecvt,
        _wide_data: 0 as * mut crate::_IO_wide_data,
        _freeres_list: core::ptr::null_mut(),
        _freeres_buf: 0 as * mut u8,
        __pad5: std::os::raw::c_ulong::default(),
        _mode: std::os::raw::c_int::default(),
        _unused2: [std::os::raw::c_char::default(); 20]
        }
    }
}

pub type _IO_lock_t = ();
pub type FILE = crate::_IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct command_tag {
    pub cmd: * mut std::os::raw::c_char,
    pub length: std::os::raw::c_ulong,
    pub min_len: std::os::raw::c_ulong,
    pub next: * mut crate::command_tag,
}
impl std::default::Default for command_tag {
    fn default() -> Self {
        command_tag {
        cmd: 0 as * mut std::os::raw::c_char,
        length: std::os::raw::c_ulong::default(),
        min_len: std::os::raw::c_ulong::default(),
        next: core::ptr::null_mut()
        }
    }
}

pub type command_t = crate::command_tag;
#[no_mangle]
pub static mut command_table: *const std::os::raw::c_char = b"Add ALTer  BAckup Bottom  CAppend Change SCHANGE  CInsert CLAst COMPress COpy COUnt COVerlay CURsor DELete CDelete Down DUPlicate Xedit EXPand EXTract Find NFind NFINDUp NFUp CFind FINdup FUp FOrward GET Help HEXType Input POWerinput Join SPlit SPLTJOIN  LOAD  Locate CLocate  LOWercase UPPercase  LPrefix MACRO MErge MODify MOve MSG Next Overlay PARSE PREServe PURge PUT PUTD  Query  QUIT READ  RECover REFRESH RENum REPeat  Replace CReplace  RESet  RESTore  RGTLEFT RIght LEft  SAVE  SET SHift SI  SORT  SOS  STAck STATus  TOP TRAnsfer Type Up\0"
    as *const u8 as *const std::os::raw::c_char;
#[no_mangle]
pub unsafe extern "C" fn command_match(
    mut command: *const command_t,
    mut str: *const std::os::raw::c_char,
) -> bool {
    let mut olen: size_t = strlen(str);
    return olen >= (*command).min_len && olen <= (*command).length
        && strncmp(str, (*command).cmd, olen) == 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uppercase(
    mut str: *mut std::os::raw::c_char,
    mut n: size_t,
) -> *mut std::os::raw::c_char {
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i < n {
        *str
            .offset(
                i as isize,
            ) = toupper(*str.offset(i as isize) as std::os::raw::c_uchar as std::os::raw::c_int)
            as std::os::raw::c_char;
        i = i.wrapping_add(1);
        i;
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn get_min_length(
    mut str: *const std::os::raw::c_char,
    mut n: size_t,
) -> size_t {
    let mut len: size_t = 0 as std::os::raw::c_int as size_t;
    while len < n
        && *(*__ctype_b_loc())
            .offset(*str.offset(len as isize) as std::os::raw::c_uchar as std::os::raw::c_int as isize)
            as std::os::raw::c_int & _ISupper as std::os::raw::c_int as std::os::raw::c_ushort as std::os::raw::c_int
            != 0
    {
        len = len.wrapping_add(1);
        len;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn fatal(mut message: *const std::os::raw::c_char) {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const std::os::raw::c_char, message);
    exit(1 as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmalloc(mut n: size_t) -> *mut u8 {
    let mut ptr: *mut u8 = malloc(n);
    if ptr.is_null() {
        fatal(b"Out of memory\0" as *const u8 as *const std::os::raw::c_char);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn xrealloc(
    mut p: *mut u8,
    mut n: size_t,
) -> *mut u8 {
    let mut ptr: *mut u8 = realloc(p, n);
    if ptr.is_null() {
        fatal(b"Out of memory\0" as *const u8 as *const std::os::raw::c_char);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn split_into_words<'a1>(
    mut str: * const std::os::raw::c_char,
    mut count: Option<&'a1 mut std::os::raw::c_ulong>,
) -> * mut * mut std::os::raw::c_char {
    let mut size: size_t = 0 as std::os::raw::c_int as size_t;
    let mut capacity: size_t = 16 as std::os::raw::c_int as size_t;
    let mut words: *mut *mut std::os::raw::c_char = xmalloc(
        capacity
            .wrapping_mul(std::mem::size_of::<*mut std::os::raw::c_char>() as std::os::raw::c_ulong),
    ) as *mut *mut std::os::raw::c_char;
    let mut len: size_t = strlen(str);
    let mut begin: size_t = 0 as std::os::raw::c_int as size_t;
    while begin < len {
        let mut i: size_t = begin;
        while i < len
            && *(*__ctype_b_loc())
                .offset(*str.offset(i as isize) as std::os::raw::c_uchar as std::os::raw::c_int as isize)
                as std::os::raw::c_int & _ISspace as std::os::raw::c_int as std::os::raw::c_ushort as std::os::raw::c_int
                != 0
        {
            i = i.wrapping_add(1);
            i;
        }
        begin = i;
        while i < len
            && *(*__ctype_b_loc())
                .offset(*str.offset(i as isize) as std::os::raw::c_uchar as std::os::raw::c_int as isize)
                as std::os::raw::c_int & _ISspace as std::os::raw::c_int as std::os::raw::c_ushort as std::os::raw::c_int
                == 0
        {
            i = i.wrapping_add(1);
            i;
        }
        let mut word_len: size_t = i.wrapping_sub(begin);
        if word_len == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
            break;
        }
        let mut word: *mut std::os::raw::c_char = xmalloc(
            word_len.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
        ) as *mut std::os::raw::c_char;
        memcpy(
            word as *mut u8,
            str.offset(begin as isize) as *const u8,
            word_len,
        );
        *word.offset(word_len as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
        begin = (begin as std::os::raw::c_ulong).wrapping_add(word_len) as size_t as size_t;
        if capacity == size {
            capacity = (capacity as std::os::raw::c_ulong)
                .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
            words = xrealloc(
                words as *mut u8,
                capacity
                    .wrapping_mul(
                        std::mem::size_of::<*mut std::os::raw::c_char>() as std::os::raw::c_ulong,
                    ),
            ) as *mut *mut std::os::raw::c_char;
        }
        let fresh0 = size;
        size = size.wrapping_add(1);
        let ref mut fresh1 = *words.offset(fresh0 as isize);
        *fresh1 = word;
    }
    *borrow_mut(&mut count).unwrap() = size;
    return words;
}
#[no_mangle]
pub unsafe extern "C" fn make_command_list(
    mut table: *const std::os::raw::c_char,
) -> *mut command_t {
    let mut cmd: *mut command_t = 0 as *mut command_t;
    let mut count: size_t = 0 as std::os::raw::c_int as size_t;
    let mut words: *mut *mut std::os::raw::c_char = split_into_words(table, Some(&mut count));
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i < count {
        let mut word: *mut std::os::raw::c_char = *words.offset(i as isize);
        let mut new_cmd: *mut command_t = xmalloc(
            std::mem::size_of::<command_t>() as std::os::raw::c_ulong,
        ) as *mut command_t;
        let mut word_len: size_t = strlen(word);
        (*new_cmd).length = word_len;
        (*new_cmd).min_len = get_min_length(word, word_len);
        (*new_cmd).cmd = uppercase(word, word_len);
        (*new_cmd).next = cmd;
        cmd = new_cmd;
        i = i.wrapping_add(1);
        i;
    }
    free(words as *mut u8);
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn free_command_list(mut cmd: *mut command_t) {
    while !cmd.is_null() {
        let mut next: *mut command_t = (*cmd).next;
        free((*cmd).cmd as *mut u8);
        free(cmd as *mut u8);
        cmd = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn find_command(
    mut commands: *const command_t,
    mut word: *const std::os::raw::c_char,
) -> *const command_t {
    let mut cmd: *const command_t = commands;
    while !cmd.is_null() {
        if command_match(cmd, word) {
            return cmd;
        }
        cmd = (*cmd).next;
    }
    return 0 as *const command_t;
}
#[no_mangle]
pub unsafe extern "C" fn test(
    mut commands: *const command_t,
    mut input: *const std::os::raw::c_char,
) {
    printf(b" input: %s\n\0" as *const u8 as *const std::os::raw::c_char, input);
    printf(b"output:\0" as *const u8 as *const std::os::raw::c_char);
    let mut count: size_t = 0 as std::os::raw::c_int as size_t;
    let mut words: *mut *mut std::os::raw::c_char = split_into_words(input, Some(&mut count));
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i < count {
        let mut word: *mut std::os::raw::c_char = *words.offset(i as isize);
        uppercase(word, strlen(word));
        let mut cmd_ptr: *const command_t = find_command(commands, word);
        printf(
            b" %s\0" as *const u8 as *const std::os::raw::c_char,
            if !cmd_ptr.is_null() {
                (*cmd_ptr).cmd as *const std::os::raw::c_char
            } else {
                b"*error*\0" as *const u8 as *const std::os::raw::c_char
            },
        );
        free(word as *mut u8);
        i = i.wrapping_add(1);
        i;
    }
    free(words as *mut u8);
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut commands: *mut command_t = make_command_list(command_table);
    let mut input: *const std::os::raw::c_char = b"riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin\0"
        as *const u8 as *const std::os::raw::c_char;
    test(commands, input);
    free_command_list(commands);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
