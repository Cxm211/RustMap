#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic, extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut u8) -> *mut u8,
        >,
        __arg: *mut u8,
    ) -> std::os::raw::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut u8,
    ) -> std::os::raw::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> std::os::raw::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> std::os::raw::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> std::os::raw::c_int;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn sprintf(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn vprintf(_: *const std::os::raw::c_char, _: std::ffi::VaList) -> std::os::raw::c_int;
    fn rand() -> std::os::raw::c_int;
    fn sleep(__seconds: std::os::raw::c_uint) -> std::os::raw::c_uint;
    fn usleep(__useconds: __useconds_t) -> std::os::raw::c_int;
}
pub type __builtin_va_list = [crate::__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: std::os::raw::c_uint,
    pub fp_offset: std::os::raw::c_uint,
    pub overflow_arg_area: * mut u8,
    pub reg_save_area: * mut u8,
}
impl std::default::Default for __va_list_tag {
    fn default() -> Self {
        __va_list_tag {
        gp_offset: std::os::raw::c_uint::default(),
        fp_offset: std::os::raw::c_uint::default(),
        overflow_arg_area: 0 as * mut u8,
        reg_save_area: 0 as * mut u8
        }
    }
}

pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __useconds_t = std::os::raw::c_uint;
pub type size_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: * mut crate::__pthread_internal_list,
    pub __next: * mut crate::__pthread_internal_list,
}
impl std::default::Default for __pthread_internal_list {
    fn default() -> Self {
        __pthread_internal_list {
        __prev: core::ptr::null_mut(),
        __next: core::ptr::null_mut()
        }
    }
}

pub type __pthread_list_t = crate::__pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: std::os::raw::c_int,
    pub __count: std::os::raw::c_uint,
    pub __owner: std::os::raw::c_int,
    pub __nusers: std::os::raw::c_uint,
    pub __kind: std::os::raw::c_int,
    pub __spins: std::os::raw::c_short,
    pub __elision: std::os::raw::c_short,
    pub __list: crate::__pthread_internal_list,
}
impl std::default::Default for __pthread_mutex_s {
    fn default() -> Self {
        __pthread_mutex_s {
        __lock: std::os::raw::c_int::default(),
        __count: std::os::raw::c_uint::default(),
        __owner: std::os::raw::c_int::default(),
        __nusers: std::os::raw::c_uint::default(),
        __kind: std::os::raw::c_int::default(),
        __spins: std::os::raw::c_short::default(),
        __elision: std::os::raw::c_short::default(),
        __list: crate::__pthread_internal_list::default()
        }
    }
}

pub type pthread_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [std::os::raw::c_char; 4],
    pub __align: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [std::os::raw::c_char; 56],
    pub __align: std::os::raw::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [std::os::raw::c_char; 40],
    pub __align: std::os::raw::c_long,
}
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub type va_list = [crate::__va_list_tag; 1];
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
pub static mut names: [*const std::os::raw::c_char; 5] = [
    b"Aristotle\0" as *const u8 as *const std::os::raw::c_char,
    b"Kant\0" as *const u8 as *const std::os::raw::c_char,
    b"Spinoza\0" as *const u8 as *const std::os::raw::c_char,
    b"Marx\0" as *const u8 as *const std::os::raw::c_char,
    b"Russell\0" as *const u8 as *const std::os::raw::c_char,
];
#[no_mangle]
pub static mut forks: [pthread_mutex_t; 5] = [pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __elision: 0,
        __list: __pthread_list_t {
            __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
            __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
        },
    },
}; 5];
#[no_mangle]
pub static mut topic: [*const std::os::raw::c_char; 5] = [
    b"Spaghetti!\0" as *const u8 as *const std::os::raw::c_char,
    b"Life\0" as *const u8 as *const std::os::raw::c_char,
    b"Universe\0" as *const u8 as *const std::os::raw::c_char,
    b"Everything\0" as *const u8 as *const std::os::raw::c_char,
    b"Bathroom\0" as *const u8 as *const std::os::raw::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn print(
    mut y: std::os::raw::c_int,
    mut x: std::os::raw::c_int,
    mut fmt: *const std::os::raw::c_char,
    mut args: ...
) {
    static mut screen: pthread_mutex_t = pthread_mutex_t {
        __data: {
            let mut init = __pthread_mutex_s {
                __lock: 0 as std::os::raw::c_int,
                __count: 0 as std::os::raw::c_int as std::os::raw::c_uint,
                __owner: 0 as std::os::raw::c_int,
                __nusers: 0 as std::os::raw::c_int as std::os::raw::c_uint,
                __kind: PTHREAD_MUTEX_TIMED_NP as std::os::raw::c_int,
                __spins: 0 as std::os::raw::c_int as std::os::raw::c_short,
                __elision: 0 as std::os::raw::c_int as std::os::raw::c_short,
                __list: {
                    let mut init = __pthread_internal_list {
                        __prev: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                        __next: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                    };
                    init
                },
            };
            init
        },
    };
    let mut ap: std::ffi::VaListImpl;
    ap = args.clone();
    pthread_mutex_lock(&mut screen);
    printf(
        b"\x1B[%d;%dH\0" as *const u8 as *const std::os::raw::c_char,
        y + 1 as std::os::raw::c_int,
        x,
    );
    vprintf(fmt, ap.as_va_list());
    printf(
        b"\x1B[%d;%dH\0" as *const u8 as *const std::os::raw::c_char,
        5 as std::os::raw::c_int + 1 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
    );
    fflush(stdout);
    pthread_mutex_unlock(&mut screen);
}
#[no_mangle]
pub unsafe extern "C" fn eat(mut id: std::os::raw::c_int) {
    let mut f: [std::os::raw::c_int; 2] = [0; 2];
    let mut ration: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    f[1 as std::os::raw::c_int as usize] = id;
    f[0 as std::os::raw::c_int as usize] = f[1 as std::os::raw::c_int as usize];
    f[(id & 1 as std::os::raw::c_int) as usize] = (id + 1 as std::os::raw::c_int) % 5 as std::os::raw::c_int;
    print(id, 12 as std::os::raw::c_int, b"\x1B[K\0" as *const u8 as *const std::os::raw::c_char);
    print(
        id,
        12 as std::os::raw::c_int,
        b"..oO (forks, need forks)\0" as *const u8 as *const std::os::raw::c_char,
    );
    i = 0 as std::os::raw::c_int;
    while i < 2 as std::os::raw::c_int {
        pthread_mutex_lock(forks.as_mut_ptr().offset(f[i as usize] as isize));
        if i == 0 {
            print(
                id,
                12 as std::os::raw::c_int,
                b"\x1B[K\0" as *const u8 as *const std::os::raw::c_char,
            );
        }
        print(
            id,
            12 as std::os::raw::c_int + (f[i as usize] != id) as std::os::raw::c_int * 6 as std::os::raw::c_int,
            b"fork%d\0" as *const u8 as *const std::os::raw::c_char,
            f[i as usize],
        );
        sleep(1 as std::os::raw::c_int as std::os::raw::c_uint);
        i += 1;
        i;
    }
    i = 0 as std::os::raw::c_int;
    ration = 3 as std::os::raw::c_int + rand() % 8 as std::os::raw::c_int;
    while i < ration {
        print(
            id,
            24 as std::os::raw::c_int + i * 4 as std::os::raw::c_int,
            b"nom\0" as *const u8 as *const std::os::raw::c_char,
        );
        sleep(1 as std::os::raw::c_int as std::os::raw::c_uint);
        i += 1;
        i;
    }
    i = 0 as std::os::raw::c_int;
    while i < 2 as std::os::raw::c_int {
        pthread_mutex_unlock(forks.as_mut_ptr().offset(f[i as usize] as isize));
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn think(mut id: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut t: std::os::raw::c_int = 0;
    let mut buf: [std::os::raw::c_char; 64] = [
        0 as std::os::raw::c_int as std::os::raw::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    loop {
        print(id, 12 as std::os::raw::c_int, b"\x1B[K\0" as *const u8 as *const std::os::raw::c_char);
        t = rand() % 5 as std::os::raw::c_int;
        sprintf(
            buf.as_mut_ptr(),
            b"..oO (%s)\0" as *const u8 as *const std::os::raw::c_char,
            topic[t as usize],
        );
        i = 0 as std::os::raw::c_int;
        while buf[i as usize] != 0 {
            print(
                id,
                i + 12 as std::os::raw::c_int,
                b"%c\0" as *const u8 as *const std::os::raw::c_char,
                buf[i as usize] as std::os::raw::c_int,
            );
            if i < 5 as std::os::raw::c_int {
                usleep(200000 as std::os::raw::c_int as __useconds_t);
            }
            i += 1;
            i;
        }
        usleep(
            (500000 as std::os::raw::c_int + rand() % 1000000 as std::os::raw::c_int) as __useconds_t,
        );
        if !(t != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn philosophize(mut a: *mut u8) -> *mut u8 {
    let mut id: std::os::raw::c_int = *(a as *mut std::os::raw::c_int);
    print(
        id,
        1 as std::os::raw::c_int,
        b"%10s\0" as *const u8 as *const std::os::raw::c_char,
        names[id as usize],
    );
    loop {
        think(id);
        eat(id);
    };
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut id: [std::os::raw::c_int; 5] = [0; 5];
    let mut tid: [pthread_t; 5] = [0; 5];
    i = 0 as std::os::raw::c_int;
    while i < 5 as std::os::raw::c_int {
        id[i as usize] = i;
        pthread_mutex_init(
            forks.as_mut_ptr().offset(id[i as usize] as isize),
            0 as *const pthread_mutexattr_t,
        );
        i += 1;
        i;
    }
    i = 0 as std::os::raw::c_int;
    while i < 5 as std::os::raw::c_int {
        pthread_create(
            tid.as_mut_ptr().offset(i as isize),
            0 as *const pthread_attr_t,
            Some(
                philosophize
                    as unsafe extern "C" fn(*mut u8) -> *mut u8,
            ),
            id.as_mut_ptr().offset(i as isize) as *mut u8,
        );
        i += 1;
        i;
    }
    return pthread_join(tid[0 as std::os::raw::c_int as usize], 0 as *mut *mut u8);
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
