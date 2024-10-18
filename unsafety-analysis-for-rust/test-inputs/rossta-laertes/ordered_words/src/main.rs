#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn exit(_: std::os::raw::c_int) -> !;
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fscanf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn __assert_fail(
        __assertion: *const std::os::raw::c_char,
        __file: *const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: *const std::os::raw::c_char,
    ) -> !;
}
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
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
pub type TWord = [std::os::raw::c_char; 100];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub word: [std::os::raw::c_char; 100],
    pub next: * mut crate::Node,
}
impl std::default::Default for Node {
    fn default() -> Self {
        Node {
        word: [std::os::raw::c_char::default(); 100],
        next: core::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn is_ordered_word(mut word: *const std::os::raw::c_char) -> std::os::raw::c_int {
    if !word.is_null() {} else {
        __assert_fail(
            b"word != NULL\0" as *const u8 as *const std::os::raw::c_char,
            b"main.c\0" as *const u8 as *const std::os::raw::c_char,
            18 as std::os::raw::c_int as std::os::raw::c_uint,
            (*std::mem::transmute::<
                &[u8; 34],
                &[std::os::raw::c_char; 34],
            >(b"int is_ordered_word(const char *)\0"))
                .as_ptr(),
        );
    }
    'c_1788: {
        if !word.is_null() {} else {
            __assert_fail(
                b"word != NULL\0" as *const u8 as *const std::os::raw::c_char,
                b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                18 as std::os::raw::c_int as std::os::raw::c_uint,
                (*std::mem::transmute::<
                    &[u8; 34],
                    &[std::os::raw::c_char; 34],
                >(b"int is_ordered_word(const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while *word.offset(i as isize) as std::os::raw::c_int != '\0' as i32 {
        if *word.offset(i as isize) as std::os::raw::c_int
            > *word.offset((i + 1 as std::os::raw::c_int) as isize) as std::os::raw::c_int
            && *word.offset((i + 1 as std::os::raw::c_int) as isize) as std::os::raw::c_int
                != '\0' as i32
        {
            return 0 as std::os::raw::c_int;
        }
        i += 1;
        i;
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn list_prepend(
    mut words_list: *mut Node,
    mut new_word: *const std::os::raw::c_char,
) -> *mut Node {
    if !new_word.is_null() {} else {
        __assert_fail(
            b"new_word != NULL\0" as *const u8 as *const std::os::raw::c_char,
            b"main.c\0" as *const u8 as *const std::os::raw::c_char,
            30 as std::os::raw::c_int as std::os::raw::c_uint,
            (*std::mem::transmute::<
                &[u8; 41],
                &[std::os::raw::c_char; 41],
            >(b"Node *list_prepend(Node *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_1888: {
        if !new_word.is_null() {} else {
            __assert_fail(
                b"new_word != NULL\0" as *const u8 as *const std::os::raw::c_char,
                b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                30 as std::os::raw::c_int as std::os::raw::c_uint,
                (*std::mem::transmute::<
                    &[u8; 41],
                    &[std::os::raw::c_char; 41],
                >(b"Node *list_prepend(Node *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut new_node: *mut Node = malloc(std::mem::size_of::<Node>() as std::os::raw::c_ulong)
        as *mut Node;
    if new_node.is_null() {
        exit(1 as std::os::raw::c_int);
    }
    strcpy(((*new_node).word).as_mut_ptr(), new_word);
    (*new_node).next = words_list;
    return new_node;
}
#[no_mangle]
pub unsafe extern "C" fn list_destroy(mut words_list: *mut Node) -> *mut Node {
    while !words_list.is_null() {
        let mut temp: *mut Node = words_list;
        words_list = (*words_list).next;
        free(temp as *mut u8);
    }
    return words_list;
}
#[no_mangle]
pub unsafe extern "C" fn list_print(mut words_list: *mut Node) {
    while !words_list.is_null() {
        printf(
            b"\n%s\0" as *const u8 as *const std::os::raw::c_char,
            ((*words_list).word).as_mut_ptr(),
        );
        words_list = (*words_list).next;
    }
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut fp: *mut FILE = fopen(
        b"unixdict.txt\0" as *const u8 as *const std::os::raw::c_char,
        b"r\0" as *const u8 as *const std::os::raw::c_char,
    );
    if fp.is_null() {
        return 1 as std::os::raw::c_int;
    }
    let mut words: *mut Node = core::ptr::null_mut();
    let mut line: TWord = [0; 100];
    let mut max_len: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while fscanf(fp, b"%99s\n\0" as *const u8 as *const std::os::raw::c_char, line.as_mut_ptr())
        != -(1 as std::os::raw::c_int)
    {
        if strlen(line.as_mut_ptr()) > max_len as std::os::raw::c_ulong
            && is_ordered_word(line.as_mut_ptr() as *const std::os::raw::c_char) != 0
        {
            max_len = strlen(line.as_mut_ptr()) as std::os::raw::c_uint;
            words = list_destroy(words);
            words = list_prepend(words, line.as_mut_ptr() as *const std::os::raw::c_char);
        } else if strlen(line.as_mut_ptr()) == max_len as std::os::raw::c_ulong
            && is_ordered_word(line.as_mut_ptr() as *const std::os::raw::c_char) != 0
        {
            words = list_prepend(words, line.as_mut_ptr() as *const std::os::raw::c_char);
        }
    }
    fclose(fp);
    list_print(words);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
