#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fgets(
        __s: *mut std::os::raw::c_char,
        __n: std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut std::os::raw::c_char;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn strdup(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
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
pub struct sDictWord {
    pub word: * const std::os::raw::c_char,
    pub next: * mut crate::sDictWord,
}
impl std::default::Default for sDictWord {
    fn default() -> Self {
        sDictWord {
        word: 0 as * const std::os::raw::c_char,
        next: core::ptr::null_mut()
        }
    }
}

pub type DictWord = * mut crate::sDictWord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sHashEntry {
    pub key: * const std::os::raw::c_char,
    pub next: * mut crate::sHashEntry,
    pub words: * mut crate::sDictWord,
    pub link: * mut crate::sHashEntry,
    pub wordCount: std::os::raw::c_short,
}
impl std::default::Default for sHashEntry {
    fn default() -> Self {
        sHashEntry {
        key: 0 as * const std::os::raw::c_char,
        next: core::ptr::null_mut(),
        words: core::ptr::null_mut(),
        link: core::ptr::null_mut(),
        wordCount: std::os::raw::c_short::default()
        }
    }
}

pub type HashEntry = * mut crate::sHashEntry;
#[no_mangle]
pub unsafe extern "C" fn sortedWord(
    mut word: *const std::os::raw::c_char,
    mut wbuf: *mut std::os::raw::c_char,
) -> *mut std::os::raw::c_char {
    let mut p1: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut p2: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut endwrd: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut t: std::os::raw::c_char = 0;
    let mut swaps: std::os::raw::c_int = 0;
    strcpy(wbuf, word);
    endwrd = wbuf.offset(strlen(wbuf) as isize);
    loop {
        swaps = 0 as std::os::raw::c_int;
        p1 = wbuf;
        p2 = endwrd.offset(-(1 as std::os::raw::c_int as isize));
        while p1 < p2 {
            if *p2 as std::os::raw::c_int > *p1 as std::os::raw::c_int {
                t = *p2;
                *p2 = *p1;
                *p1 = t;
                swaps = 1 as std::os::raw::c_int;
            }
            p1 = p1.offset(1);
            p1;
            p2 = p2.offset(-1);
            p2;
        }
        p1 = wbuf;
        p2 = p1.offset(1 as std::os::raw::c_int as isize);
        while p2 < endwrd {
            if *p2 as std::os::raw::c_int > *p1 as std::os::raw::c_int {
                t = *p2;
                *p2 = *p1;
                *p1 = t;
                swaps = 1 as std::os::raw::c_int;
            }
            p1 = p1.offset(1);
            p1;
            p2 = p2.offset(1);
            p2;
        }
        if !(swaps != 0) {
            break;
        }
    }
    return wbuf;
}
static mut cxmap: [std::os::raw::c_short; 96] = [
    0x6 as std::os::raw::c_int as std::os::raw::c_short,
    0x1f as std::os::raw::c_int as std::os::raw::c_short,
    0x4d as std::os::raw::c_int as std::os::raw::c_short,
    0xc as std::os::raw::c_int as std::os::raw::c_short,
    0x5c as std::os::raw::c_int as std::os::raw::c_short,
    0x28 as std::os::raw::c_int as std::os::raw::c_short,
    0x5d as std::os::raw::c_int as std::os::raw::c_short,
    0xe as std::os::raw::c_int as std::os::raw::c_short,
    0x9 as std::os::raw::c_int as std::os::raw::c_short,
    0x33 as std::os::raw::c_int as std::os::raw::c_short,
    0x31 as std::os::raw::c_int as std::os::raw::c_short,
    0x56 as std::os::raw::c_int as std::os::raw::c_short,
    0x52 as std::os::raw::c_int as std::os::raw::c_short,
    0x19 as std::os::raw::c_int as std::os::raw::c_short,
    0x29 as std::os::raw::c_int as std::os::raw::c_short,
    0x53 as std::os::raw::c_int as std::os::raw::c_short,
    0x32 as std::os::raw::c_int as std::os::raw::c_short,
    0x48 as std::os::raw::c_int as std::os::raw::c_short,
    0x35 as std::os::raw::c_int as std::os::raw::c_short,
    0x55 as std::os::raw::c_int as std::os::raw::c_short,
    0x5e as std::os::raw::c_int as std::os::raw::c_short,
    0x14 as std::os::raw::c_int as std::os::raw::c_short,
    0x27 as std::os::raw::c_int as std::os::raw::c_short,
    0x24 as std::os::raw::c_int as std::os::raw::c_short,
    0x2 as std::os::raw::c_int as std::os::raw::c_short,
    0x3e as std::os::raw::c_int as std::os::raw::c_short,
    0x18 as std::os::raw::c_int as std::os::raw::c_short,
    0x4a as std::os::raw::c_int as std::os::raw::c_short,
    0x3f as std::os::raw::c_int as std::os::raw::c_short,
    0x4c as std::os::raw::c_int as std::os::raw::c_short,
    0x45 as std::os::raw::c_int as std::os::raw::c_short,
    0x30 as std::os::raw::c_int as std::os::raw::c_short,
    0x8 as std::os::raw::c_int as std::os::raw::c_short,
    0x2c as std::os::raw::c_int as std::os::raw::c_short,
    0x1a as std::os::raw::c_int as std::os::raw::c_short,
    0x3 as std::os::raw::c_int as std::os::raw::c_short,
    0xb as std::os::raw::c_int as std::os::raw::c_short,
    0xd as std::os::raw::c_int as std::os::raw::c_short,
    0x4f as std::os::raw::c_int as std::os::raw::c_short,
    0x7 as std::os::raw::c_int as std::os::raw::c_short,
    0x20 as std::os::raw::c_int as std::os::raw::c_short,
    0x1d as std::os::raw::c_int as std::os::raw::c_short,
    0x51 as std::os::raw::c_int as std::os::raw::c_short,
    0x3b as std::os::raw::c_int as std::os::raw::c_short,
    0x11 as std::os::raw::c_int as std::os::raw::c_short,
    0x58 as std::os::raw::c_int as std::os::raw::c_short,
    0 as std::os::raw::c_int as std::os::raw::c_short,
    0x49 as std::os::raw::c_int as std::os::raw::c_short,
    0x15 as std::os::raw::c_int as std::os::raw::c_short,
    0x2d as std::os::raw::c_int as std::os::raw::c_short,
    0x41 as std::os::raw::c_int as std::os::raw::c_short,
    0x17 as std::os::raw::c_int as std::os::raw::c_short,
    0x5f as std::os::raw::c_int as std::os::raw::c_short,
    0x39 as std::os::raw::c_int as std::os::raw::c_short,
    0x16 as std::os::raw::c_int as std::os::raw::c_short,
    0x42 as std::os::raw::c_int as std::os::raw::c_short,
    0x37 as std::os::raw::c_int as std::os::raw::c_short,
    0x22 as std::os::raw::c_int as std::os::raw::c_short,
    0x1c as std::os::raw::c_int as std::os::raw::c_short,
    0xf as std::os::raw::c_int as std::os::raw::c_short,
    0x43 as std::os::raw::c_int as std::os::raw::c_short,
    0x5b as std::os::raw::c_int as std::os::raw::c_short,
    0x46 as std::os::raw::c_int as std::os::raw::c_short,
    0x4b as std::os::raw::c_int as std::os::raw::c_short,
    0xa as std::os::raw::c_int as std::os::raw::c_short,
    0x26 as std::os::raw::c_int as std::os::raw::c_short,
    0x2e as std::os::raw::c_int as std::os::raw::c_short,
    0x40 as std::os::raw::c_int as std::os::raw::c_short,
    0x12 as std::os::raw::c_int as std::os::raw::c_short,
    0x21 as std::os::raw::c_int as std::os::raw::c_short,
    0x3c as std::os::raw::c_int as std::os::raw::c_short,
    0x36 as std::os::raw::c_int as std::os::raw::c_short,
    0x38 as std::os::raw::c_int as std::os::raw::c_short,
    0x1e as std::os::raw::c_int as std::os::raw::c_short,
    0x1 as std::os::raw::c_int as std::os::raw::c_short,
    0x1b as std::os::raw::c_int as std::os::raw::c_short,
    0x5 as std::os::raw::c_int as std::os::raw::c_short,
    0x4e as std::os::raw::c_int as std::os::raw::c_short,
    0x44 as std::os::raw::c_int as std::os::raw::c_short,
    0x3d as std::os::raw::c_int as std::os::raw::c_short,
    0x4 as std::os::raw::c_int as std::os::raw::c_short,
    0x10 as std::os::raw::c_int as std::os::raw::c_short,
    0x5a as std::os::raw::c_int as std::os::raw::c_short,
    0x2a as std::os::raw::c_int as std::os::raw::c_short,
    0x23 as std::os::raw::c_int as std::os::raw::c_short,
    0x34 as std::os::raw::c_int as std::os::raw::c_short,
    0x25 as std::os::raw::c_int as std::os::raw::c_short,
    0x2f as std::os::raw::c_int as std::os::raw::c_short,
    0x2b as std::os::raw::c_int as std::os::raw::c_short,
    0x50 as std::os::raw::c_int as std::os::raw::c_short,
    0x3a as std::os::raw::c_int as std::os::raw::c_short,
    0x54 as std::os::raw::c_int as std::os::raw::c_short,
    0x47 as std::os::raw::c_int as std::os::raw::c_short,
    0x59 as std::os::raw::c_int as std::os::raw::c_short,
    0x13 as std::os::raw::c_int as std::os::raw::c_short,
    0x57 as std::os::raw::c_int as std::os::raw::c_short,
];
#[no_mangle]
pub unsafe extern "C" fn Str_Hash(
    mut key: *const std::os::raw::c_char,
    mut ix_max: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut cp: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut mash: std::os::raw::c_short = 0;
    let mut hash: std::os::raw::c_int = 33501551 as std::os::raw::c_int;
    cp = key;
    while *cp != 0 {
        mash = cxmap[(*cp as std::os::raw::c_ulong)
            .wrapping_rem(
                (std::mem::size_of::<[std::os::raw::c_short; 96]>() as std::os::raw::c_ulong)
                    .wrapping_div(
                        std::mem::size_of::<std::os::raw::c_short>() as std::os::raw::c_ulong,
                    ),
            ) as usize];
        hash = hash >> 4 as std::os::raw::c_int ^ 0x5c5cf5c as std::os::raw::c_int
            ^ (hash << 1 as std::os::raw::c_int) + ((mash as std::os::raw::c_int) << 5 as std::os::raw::c_int);
        hash &= 0x3fffffff as std::os::raw::c_int;
        cp = cp.offset(1);
        cp;
    }
    return hash % ix_max;
}
#[no_mangle]
pub static mut hashTable: [HashEntry; 8192] = [0 as *const sHashEntry
    as *mut sHashEntry; 8192];
#[no_mangle]
pub static mut mostPerms: HashEntry = 0 as *const sHashEntry as HashEntry;
#[no_mangle]
pub unsafe extern "C" fn buildAnagrams(mut fin: *mut FILE) -> std::os::raw::c_int {
    let mut buffer: [std::os::raw::c_char; 40] = [0; 40];
    let mut bufr2: [std::os::raw::c_char; 40] = [0; 40];
    let mut hkey: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut hix: std::os::raw::c_int = 0;
    let mut he: HashEntry = core::ptr::null_mut();
    let mut hep: *mut HashEntry = core::ptr::null_mut();
    let mut we: DictWord = core::ptr::null_mut();
    let mut maxPC: std::os::raw::c_int = 2 as std::os::raw::c_int;
    let mut numWords: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while !(fgets(buffer.as_mut_ptr(), 40 as std::os::raw::c_int, fin)).is_null() {
        hkey = buffer.as_mut_ptr();
        while *hkey as std::os::raw::c_int != 0 && *hkey as std::os::raw::c_int != '\n' as i32 {
            hkey = hkey.offset(1);
            hkey;
        }
        *hkey = 0 as std::os::raw::c_int as std::os::raw::c_char;
        hkey = sortedWord(buffer.as_mut_ptr(), bufr2.as_mut_ptr());
        hix = Str_Hash(hkey, 8192 as std::os::raw::c_int);
        he = hashTable[hix as usize];
        hep = &mut *hashTable.as_mut_ptr().offset(hix as isize) as *mut HashEntry;
        while !he.is_null() && strcmp((*he).key, hkey) != 0 {
            hep = &mut (*he).next;
            he = (*he).next;
        }
        if he.is_null() {
            he = malloc(std::mem::size_of::<sHashEntry>() as std::os::raw::c_ulong)
                as HashEntry;
            (*he).next = core::ptr::null_mut();
            (*he).key = strdup(hkey);
            (*he).wordCount = 0 as std::os::raw::c_int as std::os::raw::c_short;
            (*he).words = core::ptr::null_mut();
            (*he).link = core::ptr::null_mut();
            *hep = he;
        }
        we = malloc(std::mem::size_of::<sDictWord>() as std::os::raw::c_ulong) as DictWord;
        (*we).word = strdup(buffer.as_mut_ptr());
        (*we).next = (*he).words;
        (*he).words = we;
        (*he).wordCount += 1;
        (*he).wordCount;
        if maxPC < (*he).wordCount as std::os::raw::c_int {
            maxPC = (*he).wordCount as std::os::raw::c_int;
            mostPerms = he;
            (*he).link = core::ptr::null_mut();
        } else if maxPC == (*he).wordCount as std::os::raw::c_int {
            (*he).link = mostPerms;
            mostPerms = he;
        }
        numWords += 1;
        numWords;
    }
    printf(
        b"%d words in dictionary max ana=%d\n\0" as *const u8 as *const std::os::raw::c_char,
        numWords,
        maxPC,
    );
    return maxPC;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut he: HashEntry = core::ptr::null_mut();
    let mut we: DictWord = core::ptr::null_mut();
    let mut f1: *mut FILE = 0 as *mut FILE;
    f1 = fopen(
        b"unixdict.txt\0" as *const u8 as *const std::os::raw::c_char,
        b"r\0" as *const u8 as *const std::os::raw::c_char,
    );
    buildAnagrams(f1);
    fclose(f1);
    f1 = fopen(
        b"anaout.txt\0" as *const u8 as *const std::os::raw::c_char,
        b"w\0" as *const u8 as *const std::os::raw::c_char,
    );
    he = mostPerms;
    while !he.is_null() {
        fprintf(
            f1,
            b"%d:\0" as *const u8 as *const std::os::raw::c_char,
            (*he).wordCount as std::os::raw::c_int,
        );
        we = (*he).words;
        while !we.is_null() {
            fprintf(f1, b"%s, \0" as *const u8 as *const std::os::raw::c_char, (*we).word);
            we = (*we).next;
        }
        fprintf(f1, b"\n\0" as *const u8 as *const std::os::raw::c_char);
        he = (*he).link;
    }
    fclose(f1);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
