#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn memcpy(
        _: *mut u8,
        _: *const u8,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
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
#[no_mangle]
pub static mut kDecks: [std::os::raw::c_int; 7] = [
    8 as std::os::raw::c_int,
    24 as std::os::raw::c_int,
    52 as std::os::raw::c_int,
    100 as std::os::raw::c_int,
    1020 as std::os::raw::c_int,
    1024 as std::os::raw::c_int,
    10000 as std::os::raw::c_int,
];
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut nCards: std::os::raw::c_int = 0;
    let mut nShuffles: std::os::raw::c_int = 0;
    let mut deck: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < 7 as std::os::raw::c_int {
        nCards = kDecks[i as usize];
        if CreateDeck(Some(&mut deck), nCards) == 0 {
            fprintf(
                stderr,
                b"Error: malloc() failed!\n\0" as *const u8 as *const std::os::raw::c_char,
            );
            return 1 as std::os::raw::c_int;
        }
        InitDeck(deck, nCards);
        nShuffles = 0 as std::os::raw::c_int;
        loop {
            ShuffleDeck(deck, nCards);
            nShuffles += 1;
            nShuffles;
            if !(InitedDeck(deck, nCards) == 0) {
                break;
            }
        }
        printf(
            b"Cards count: %d, shuffles required: %d.\n\0" as *const u8
                as *const std::os::raw::c_char,
            nCards,
            nShuffles,
        );
        FreeDeck(Some(&mut deck));
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CreateDeck<'a1>(
    mut deck: Option<&'a1 mut * mut std::os::raw::c_int>,
    mut nCards: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut tmp: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    if !borrow(& deck).is_none() {
        tmp = malloc(
            (nCards as std::os::raw::c_ulong)
                .wrapping_mul(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
        ) as *mut std::os::raw::c_int;
    }
    return if !tmp.is_null() {
        *borrow_mut(&mut deck).unwrap() = tmp;
        (*borrow(& deck).unwrap() != 0 as *mut u8 as *mut std::os::raw::c_int) as std::os::raw::c_int
    } else {
        0 as std::os::raw::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn InitDeck(mut deck: *mut std::os::raw::c_int, mut nCards: std::os::raw::c_int) {
    if !deck.is_null() {
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < nCards {
            *deck.offset(i as isize) = i;
            i += 1;
            i;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn DuplicateDeck<'a1>(
    mut dest: Option<&'a1 mut * mut std::os::raw::c_int>,
    mut orig: * const std::os::raw::c_int,
    mut nCards: std::os::raw::c_int,
) -> std::os::raw::c_int {
    if !orig.is_null() && CreateDeck(borrow_mut(&mut dest), nCards) != 0 {
        memcpy(
            *borrow_mut(&mut dest).unwrap() as *mut u8,
            orig as *const u8,
            (nCards as std::os::raw::c_ulong)
                .wrapping_mul(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
        );
        return 1 as std::os::raw::c_int;
    } else {
        return 0 as std::os::raw::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn InitedDeck(
    mut deck: *mut std::os::raw::c_int,
    mut nCards: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < nCards {
        if *deck.offset(i as isize) != i {
            return 0 as std::os::raw::c_int;
        }
        i += 1;
        i;
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ShuffleDeck(
    mut deck: *mut std::os::raw::c_int,
    mut nCards: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut copy: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    if DuplicateDeck(Some(&mut copy), deck, nCards) != 0 {
        let mut i: std::os::raw::c_int = 0;
        let mut j: std::os::raw::c_int = 0;
        j = 0 as std::os::raw::c_int;
        i = j;
        while i < nCards / 2 as std::os::raw::c_int {
            *deck.offset(j as isize) = *copy.offset(i as isize);
            *deck
                .offset(
                    (j + 1 as std::os::raw::c_int) as isize,
                ) = *copy.offset((i + nCards / 2 as std::os::raw::c_int) as isize);
            i += 1;
            i;
            j += 2 as std::os::raw::c_int;
        }
        FreeDeck(Some(&mut copy));
        return 1 as std::os::raw::c_int;
    } else {
        return 0 as std::os::raw::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn FreeDeck<'a1>(mut deck: Option<&'a1 mut * mut std::os::raw::c_int>) {
    if !(*borrow_mut(&mut deck).unwrap()).is_null() {
        free(*borrow_mut(&mut deck).unwrap() as *mut u8);
        *borrow_mut(&mut deck).unwrap() = 0 as *mut std::os::raw::c_int;
    }
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
