#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fcntl(__fd: std::os::raw::c_int, __cmd: std::os::raw::c_int, _: ...) -> std::os::raw::c_int;
    fn open(__file: *const std::os::raw::c_char, __oflag: std::os::raw::c_int, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> std::os::raw::c_int;
    fn exit(_: std::os::raw::c_int) -> !;
    fn getenv(__name: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fputs(__s: *const std::os::raw::c_char, __stream: *mut FILE) -> std::os::raw::c_int;
    fn puts(__s: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn perror(__s: *const std::os::raw::c_char);
    fn memcpy(
        _: *mut u8,
        _: *const u8,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn sleep(__seconds: std::os::raw::c_uint) -> std::os::raw::c_uint;
    fn unlink(__name: *const std::os::raw::c_char) -> std::os::raw::c_int;
}
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __pid_t = std::os::raw::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_type: std::os::raw::c_short,
    pub l_whence: std::os::raw::c_short,
    pub l_start: std::os::raw::c_long,
    pub l_len: std::os::raw::c_long,
    pub l_pid: std::os::raw::c_int,
}
impl std::default::Default for flock {
    fn default() -> Self {
        flock {
        l_type: std::os::raw::c_short::default(),
        l_whence: std::os::raw::c_short::default(),
        l_start: std::os::raw::c_long::default(),
        l_len: std::os::raw::c_long::default(),
        l_pid: std::os::raw::c_int::default()
        }
    }
}

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
#[no_mangle]
pub unsafe extern "C" fn fail(mut message: *const std::os::raw::c_char) {
    perror(message);
    exit(1 as std::os::raw::c_int);
}
static mut ooi_path: *mut std::os::raw::c_char = 0 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
#[no_mangle]
pub unsafe extern "C" fn ooi_unlink() {
    unlink(ooi_path);
}
#[no_mangle]
pub unsafe extern "C" fn only_one_instance() {
    let mut fl: flock = flock {
        l_type: 0,
        l_whence: 0,
        l_start: 0,
        l_len: 0,
        l_pid: 0,
    };
    let mut dirlen: size_t = 0;
    let mut fd: std::os::raw::c_int = 0;
    let mut dir: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    dir = getenv(b"HOME\0" as *const u8 as *const std::os::raw::c_char);
    if dir.is_null()
        || *dir.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int != '/' as i32
    {
        fputs(b"Bad home directory.\n\0" as *const u8 as *const std::os::raw::c_char, stderr);
        exit(1 as std::os::raw::c_int);
    }
    dirlen = strlen(dir);
    ooi_path = malloc(
        dirlen
            .wrapping_add(std::mem::size_of::<[std::os::raw::c_char; 19]>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_char;
    if ooi_path.is_null() {
        fail(b"malloc\0" as *const u8 as *const std::os::raw::c_char);
    }
    memcpy(ooi_path as *mut u8, dir as *const u8, dirlen);
    memcpy(
        ooi_path.offset(dirlen as isize) as *mut u8,
        b"/rosetta-code-lock\0" as *const u8 as *const std::os::raw::c_char
            as *const u8,
        std::mem::size_of::<[std::os::raw::c_char; 19]>() as std::os::raw::c_ulong,
    );
    fd = open(ooi_path, 0o2 as std::os::raw::c_int | 0o100 as std::os::raw::c_int, 0o600 as std::os::raw::c_int);
    if fd < 0 as std::os::raw::c_int {
        fail(ooi_path);
    }
    fl.l_start = 0 as std::os::raw::c_int as __off_t;
    fl.l_len = 0 as std::os::raw::c_int as __off_t;
    fl.l_type = 1 as std::os::raw::c_int as std::os::raw::c_short;
    fl.l_whence = 0 as std::os::raw::c_int as std::os::raw::c_short;
    if fcntl(fd, 6 as std::os::raw::c_int, &mut fl as *mut flock) < 0 as std::os::raw::c_int {
        fputs(
            b"Another instance of this program is running.\n\0" as *const u8
                as *const std::os::raw::c_char,
            stderr,
        );
        exit(1 as std::os::raw::c_int);
    }
    atexit(Some(ooi_unlink as unsafe extern "C" fn() -> ()));
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    only_one_instance();
    i = 10 as std::os::raw::c_int;
    while i > 0 as std::os::raw::c_int {
        printf(
            b"%d...%s\0" as *const u8 as *const std::os::raw::c_char,
            i,
            if i % 5 as std::os::raw::c_int == 1 as std::os::raw::c_int {
                b"\n\0" as *const u8 as *const std::os::raw::c_char
            } else {
                b" \0" as *const u8 as *const std::os::raw::c_char
            },
        );
        fflush(stdout);
        sleep(1 as std::os::raw::c_int as std::os::raw::c_uint);
        i -= 1;
        i;
    }
    puts(b"Fin!\0" as *const u8 as *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
