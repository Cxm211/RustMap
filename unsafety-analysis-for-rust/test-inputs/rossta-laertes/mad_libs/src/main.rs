#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fgets(
        __s: *mut std::os::raw::c_char,
        __n: std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut std::os::raw::c_char;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn realloc(_: *mut u8, _: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn exit(_: std::os::raw::c_int) -> !;
    fn memmove(
        _: *mut u8,
        _: *const u8,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
    fn strncpy(
        _: *mut std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_char;
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    fn strstr(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dstr {
    pub data: * mut std::os::raw::c_char,
    pub alloc: std::os::raw::c_ulong,
    pub length: std::os::raw::c_ulong,
}
impl std::default::Default for dstr {
    fn default() -> Self {
        dstr {
        data: 0 as * mut std::os::raw::c_char,
        alloc: std::os::raw::c_ulong::default(),
        length: std::os::raw::c_ulong::default()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn dstr_space(
    mut s: *mut dstr,
    mut grow_amount: size_t,
) -> std::os::raw::c_int {
    return (((*s).length).wrapping_add(grow_amount) < (*s).alloc) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dstr_grow(mut s: *mut dstr) -> std::os::raw::c_int {
    (*s)
        .alloc = ((*s).alloc as std::os::raw::c_ulong)
        .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
    let mut attempt: *mut std::os::raw::c_char = realloc(
        (*s).data as *mut u8,
        (*s).alloc,
    ) as *mut std::os::raw::c_char;
    if attempt.is_null() {
        return 0 as std::os::raw::c_int
    } else {
        (*s).data = attempt;
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dstr_init(to_allocate: size_t) -> *mut dstr {
    let mut s: *mut dstr = malloc(std::mem::size_of::<dstr>() as std::os::raw::c_ulong)
        as *mut dstr;
    if !s.is_null() {
        (*s).length = 0 as std::os::raw::c_int as size_t;
        (*s).alloc = to_allocate;
        (*s).data = malloc((*s).alloc) as *mut std::os::raw::c_char;
        if !((*s).data).is_null() {
            return s;
        }
    }
    if !((*s).data).is_null() {
        free((*s).data as *mut u8);
    }
    if !s.is_null() {
        free(s as *mut u8);
    }
    return 0 as *mut dstr;
}
#[no_mangle]
pub unsafe extern "C" fn dstr_delete(mut s: *mut dstr) {
    if !((*s).data).is_null() {
        free((*s).data as *mut u8);
    }
    if !s.is_null() {
        free(s as *mut u8);
    }
}
#[no_mangle]
pub unsafe extern "C" fn readinput(mut fd: *mut FILE) -> *mut dstr {
    let mut current_block: u64;
    static mut buffer_size: size_t = 4096 as std::os::raw::c_int as size_t;
    let vla = buffer_size as usize;
    let mut buffer: Vec::<std::os::raw::c_char> = ::std::vec::from_elem(0, vla);
    let mut s: *mut dstr = dstr_init(buffer_size);
    if s.is_null() {
        current_block = 17156671859070965445;
    } else {
        current_block = 8258075665625361029;
    }
    '_failure: loop {
        match current_block {
            17156671859070965445 => {
                dstr_delete(s);
                return 0 as *mut dstr;
            }
            _ => {
                if !(fgets(buffer.as_mut_ptr(), buffer_size as std::os::raw::c_int, fd))
                    .is_null()
                {
                    while dstr_space(s, buffer_size) == 0 {
                        if dstr_grow(s) == 0 {
                            current_block = 17156671859070965445;
                            continue '_failure;
                        }
                    }
                    strncpy(
                        ((*s).data).offset((*s).length as isize),
                        buffer.as_mut_ptr(),
                        buffer_size,
                    );
                    (*s)
                        .length = ((*s).length as std::os::raw::c_ulong)
                        .wrapping_add(strlen(buffer.as_mut_ptr())) as size_t as size_t;
                    current_block = 8258075665625361029;
                } else {
                    return s
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dstr_replace_all(
    mut story: *mut dstr,
    mut replace: *const std::os::raw::c_char,
    mut insert: *const std::os::raw::c_char,
) {
    let replace_l: size_t = strlen(replace);
    let insert_l: size_t = strlen(insert);
    let mut start: *mut std::os::raw::c_char = (*story).data;
    loop {
        start = strstr(start, replace);
        if start.is_null() {
            break;
        }
        if dstr_space(story, insert_l.wrapping_sub(replace_l)) == 0 {
            if dstr_grow(story) == 0 {
                fprintf(
                    stderr,
                    b"Failed to allocate memory\0" as *const u8 as *const std::os::raw::c_char,
                );
                exit(1 as std::os::raw::c_int);
            }
        }
        if insert_l != replace_l {
            memmove(
                start.offset(insert_l as isize) as *mut u8,
                start.offset(replace_l as isize) as *const u8,
                ((*story).length)
                    .wrapping_sub(
                        start.offset(replace_l as isize).offset_from((*story).data)
                            as std::os::raw::c_long as std::os::raw::c_ulong,
                    ),
            );
            (*story)
                .length = ((*story).length as std::os::raw::c_ulong)
                .wrapping_add(insert_l.wrapping_sub(replace_l)) as size_t as size_t;
            *((*story).data)
                .offset((*story).length as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
        }
        memmove(start as *mut u8, insert as *const u8, insert_l);
    };
}
#[no_mangle]
pub unsafe extern "C" fn madlibs(mut story: *mut dstr) {
    static mut buffer_size: size_t = 128 as std::os::raw::c_int as size_t;
    let vla = buffer_size as usize;
    let mut insert: Vec::<std::os::raw::c_char> = ::std::vec::from_elem(0, vla);
    let vla_0 = buffer_size as usize;
    let mut replace: Vec::<std::os::raw::c_char> = ::std::vec::from_elem(0, vla_0);
    let mut start: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut end: *mut std::os::raw::c_char = (*story).data;
    loop {
        start = strchr(end, '<' as i32);
        if start.is_null() {
            break;
        }
        end = strchr(start, '>' as i32);
        if end.is_null() {
            fprintf(
                stderr,
                b"Malformed brackets in input\0" as *const u8 as *const std::os::raw::c_char,
            );
            exit(1 as std::os::raw::c_int);
        }
        strncpy(
            replace.as_mut_ptr(),
            start,
            (end.offset_from(start) as std::os::raw::c_long + 1 as std::os::raw::c_int as std::os::raw::c_long)
                as std::os::raw::c_ulong,
        );
        *replace
            .as_mut_ptr()
            .offset(
                (end.offset_from(start) as std::os::raw::c_long
                    + 1 as std::os::raw::c_int as std::os::raw::c_long) as isize,
            ) = '\0' as i32 as std::os::raw::c_char;
        printf(
            b"Enter value for field %s: \0" as *const u8 as *const std::os::raw::c_char,
            replace.as_mut_ptr(),
        );
        fgets(insert.as_mut_ptr(), buffer_size as std::os::raw::c_int, stdin);
        let il: size_t = (strlen(insert.as_mut_ptr()))
            .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
        if *insert.as_mut_ptr().offset(il as isize) as std::os::raw::c_int == '\n' as i32 {
            *insert.as_mut_ptr().offset(il as isize) = '\0' as i32 as std::os::raw::c_char;
        }
        dstr_replace_all(story, replace.as_mut_ptr(), insert.as_mut_ptr());
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    if argc < 2 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    let mut fd: *mut FILE = fopen(
        *argv.offset(1 as std::os::raw::c_int as isize),
        b"r\0" as *const u8 as *const std::os::raw::c_char,
    );
    if fd.is_null() {
        fprintf(
            stderr,
            b"Could not open file: '%s\n\0" as *const u8 as *const std::os::raw::c_char,
            *argv.offset(1 as std::os::raw::c_int as isize),
        );
        exit(1 as std::os::raw::c_int);
    }
    let mut story: *mut dstr = readinput(fd);
    fclose(fd);
    if story.is_null() {
        fprintf(
            stderr,
            b"Failed to allocate memory\0" as *const u8 as *const std::os::raw::c_char,
        );
        exit(1 as std::os::raw::c_int);
    }
    madlibs(story);
    printf(b"%s\n\0" as *const u8 as *const std::os::raw::c_char, (*story).data);
    dstr_delete(story);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as std::os::raw::c_int,
                args.as_mut_ptr() as *mut *mut std::os::raw::c_char,
            ) as i32,
        )
    }
}
