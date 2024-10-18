#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn memset(
        _: *mut u8,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
    fn close(__fd: std::os::raw::c_int) -> std::os::raw::c_int;
    fn read(__fd: std::os::raw::c_int, __buf: *mut u8, __nbytes: size_t) -> ssize_t;
    fn open(__file: *const std::os::raw::c_char, __oflag: std::os::raw::c_int, _: ...) -> std::os::raw::c_int;
    fn fstat(__fd: std::os::raw::c_int, __buf: *mut stat) -> std::os::raw::c_int;
}
pub type size_t = std::os::raw::c_ulong;
pub type __dev_t = std::os::raw::c_ulong;
pub type __uid_t = std::os::raw::c_uint;
pub type __gid_t = std::os::raw::c_uint;
pub type __ino_t = std::os::raw::c_ulong;
pub type __mode_t = std::os::raw::c_uint;
pub type __nlink_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __time_t = std::os::raw::c_long;
pub type __blksize_t = std::os::raw::c_long;
pub type __blkcnt_t = std::os::raw::c_long;
pub type __ssize_t = std::os::raw::c_long;
pub type __syscall_slong_t = std::os::raw::c_long;
pub type ssize_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
impl std::default::Default for timespec {
    fn default() -> Self {
        timespec {
        tv_sec: std::os::raw::c_long::default(),
        tv_nsec: std::os::raw::c_long::default()
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: std::os::raw::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
impl std::default::Default for stat {
    fn default() -> Self {
        stat {
        st_dev: std::os::raw::c_ulong::default(),
        st_ino: std::os::raw::c_ulong::default(),
        st_nlink: std::os::raw::c_ulong::default(),
        st_mode: std::os::raw::c_uint::default(),
        st_uid: std::os::raw::c_uint::default(),
        st_gid: std::os::raw::c_uint::default(),
        __pad0: std::os::raw::c_int::default(),
        st_rdev: std::os::raw::c_ulong::default(),
        st_size: std::os::raw::c_long::default(),
        st_blksize: std::os::raw::c_long::default(),
        st_blocks: std::os::raw::c_long::default(),
        st_atim: crate::timespec::default(),
        st_mtim: crate::timespec::default(),
        st_ctim: crate::timespec::default(),
        __glibc_reserved: [std::os::raw::c_long::default(); 3]
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct word {
    pub w: * const std::os::raw::c_char,
    pub next: * mut crate::word,
}
impl std::default::Default for word {
    fn default() -> Self {
        word {
        w: 0 as * const std::os::raw::c_char,
        next: core::ptr::null_mut()
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union node {
    pub down: [*mut node; 10],
    pub list: [*mut word; 10],
}
#[no_mangle]
pub static mut freq: *const std::os::raw::c_char = b"zqxjkvbpygfwmucldrhsnioate\0" as *const u8
    as *const std::os::raw::c_char;
#[no_mangle]
pub static mut char_to_idx: [std::os::raw::c_int; 128] = [0; 128];
#[no_mangle]
pub unsafe extern "C" fn deranged(
    mut s1: *const std::os::raw::c_char,
    mut s2: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while *s1.offset(i as isize) != 0 {
        if *s1.offset(i as isize) as std::os::raw::c_int == *s2.offset(i as isize) as std::os::raw::c_int
        {
            return 0 as std::os::raw::c_int;
        }
        i += 1;
        i;
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn count_letters(
    mut s: *const std::os::raw::c_char,
    mut c: *mut std::os::raw::c_uchar,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    memset(c as *mut u8, 0 as std::os::raw::c_int, 26 as std::os::raw::c_int as std::os::raw::c_ulong);
    i = 0 as std::os::raw::c_int;
    len = i;
    while *s.offset(i as isize) != 0 {
        if (*s.offset(i as isize) as std::os::raw::c_int) < 'a' as i32
            || *s.offset(i as isize) as std::os::raw::c_int > 'z' as i32
        {
            return 0 as std::os::raw::c_int;
        }
        len += 1;
        len;
        let ref mut fresh0 = *c
            .offset(
                char_to_idx[*s.offset(i as isize) as std::os::raw::c_uchar as usize] as isize,
            );
        *fresh0 = (*fresh0).wrapping_add(1);
        *fresh0;
        i += 1;
        i;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn insert(
    mut root: *mut node,
    mut s: *const std::os::raw::c_char,
    mut cnt: *mut std::os::raw::c_uchar,
) -> *const std::os::raw::c_char {
    let mut i: std::os::raw::c_int = 0;
    let mut n: *mut node = core::ptr::null_mut();
    let mut v: *mut word = core::ptr::null_mut();
    let mut w: *mut word = core::ptr::null_mut();
    i = 0 as std::os::raw::c_int;
    while i < 25 as std::os::raw::c_int {
        n = (*root).down[*cnt.offset(i as isize) as usize];
        if n.is_null() {
            n = calloc(
                1 as std::os::raw::c_int as std::os::raw::c_ulong,
                std::mem::size_of::<node>() as std::os::raw::c_ulong,
            ) as *mut node;
            (*root).down[*cnt.offset(i as isize) as usize] = n;
        }
        i += 1;
        i;
        root = n;
    }
    w = malloc(std::mem::size_of::<word>() as std::os::raw::c_ulong) as *mut word;
    (*w).w = s;
    (*w).next = (*root).list[*cnt.offset(25 as std::os::raw::c_int as isize) as usize];
    (*root).list[*cnt.offset(25 as std::os::raw::c_int as isize) as usize] = w;
    v = (*w).next;
    while !v.is_null() {
        if deranged((*w).w, (*v).w) != 0 {
            return (*v).w;
        }
        v = (*v).next;
    }
    return 0 as *const std::os::raw::c_char;
}
unsafe fn main_0(mut c: std::os::raw::c_int, mut v: *mut *mut std::os::raw::c_char) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut words: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut fd: std::os::raw::c_int = open(
        if c < 2 as std::os::raw::c_int {
            b"unixdict.txt\0" as *const u8 as *const std::os::raw::c_char
        } else {
            *v.offset(1 as std::os::raw::c_int as isize) as *const std::os::raw::c_char
        },
        0 as std::os::raw::c_int,
    );
    if fstat(fd, &mut st) < 0 as std::os::raw::c_int {
        return 1 as std::os::raw::c_int;
    }
    words = malloc(st.st_size as std::os::raw::c_ulong) as *mut std::os::raw::c_char;
    read(fd, words as *mut u8, st.st_size as size_t);
    close(fd);
    let mut root: node = node {
        down: [
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        ],
    };
    let mut cnt: [std::os::raw::c_uchar; 26] = [0; 26];
    let mut best_len: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut b1: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut b2: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    i = 0 as std::os::raw::c_int;
    while *freq.offset(i as isize) != 0 {
        char_to_idx[*freq.offset(i as isize) as std::os::raw::c_uchar as usize] = i;
        i += 1;
        i;
    }
    j = 0 as std::os::raw::c_int;
    i = j;
    while (i as std::os::raw::c_long) < st.st_size {
        if !(*words.offset(i as isize) as std::os::raw::c_int != '\n' as i32) {
            *words.offset(i as isize) = '\0' as i32 as std::os::raw::c_char;
            if i - j > best_len {
                count_letters(words.offset(j as isize), cnt.as_mut_ptr());
                let mut match_0: *const std::os::raw::c_char = insert(
                    &mut root,
                    words.offset(j as isize),
                    cnt.as_mut_ptr(),
                );
                if !match_0.is_null() {
                    best_len = i - j;
                    b1 = words.offset(j as isize);
                    b2 = match_0;
                }
            }
            i += 1;
            j = i;
        }
        i += 1;
        i;
    }
    if best_len != 0 {
        printf(
            b"longest derangement: %s %s\n\0" as *const u8 as *const std::os::raw::c_char,
            b1,
            b2,
        );
    }
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
