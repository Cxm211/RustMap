#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn sscanf(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
    fn fgets(
        __s: *mut std::os::raw::c_char,
        __n: std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut std::os::raw::c_char;
    fn puts(__s: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn rand() -> std::os::raw::c_int;
    fn srand(__seed: std::os::raw::c_uint);
    fn exit(_: std::os::raw::c_int) -> !;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __time_t = std::os::raw::c_long;
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
pub type time_t = std::os::raw::c_long;
static mut DESCRIPTION: [std::os::raw::c_char; 398] = unsafe {
    *std::mem::transmute::<
        &[u8; 398],
        &mut [std::os::raw::c_char; 398],
    >(
        b"21 Game                                                          \n                                                                 \n21 is a two player game, the game is played by choosing a number \n(1, 2, or 3) to be added to the running total. The game is won by\nthe player whose chosen number causes the running total to reach \nexactly 21. The running total starts at zero.                    \n\n\0",
    )
};
static mut total: std::os::raw::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn update(mut player: *mut std::os::raw::c_char, mut move_0: std::os::raw::c_int) {
    printf(
        b"%8s:  %d = %d + %d\n\n\0" as *const u8 as *const std::os::raw::c_char,
        player,
        total + move_0,
        total,
        move_0,
    );
    total += move_0;
    if total == 21 as std::os::raw::c_int {
        printf(b"The winner is %s.\n\n\0" as *const u8 as *const std::os::raw::c_char, player);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ai() -> std::os::raw::c_int {
    static mut precomputed: [std::os::raw::c_int; 31] = [
        1 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
    ];
    update(
        b"ai\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        precomputed[total as usize],
    );
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn human() {
    let mut buffer: [std::os::raw::c_char; 256] = [0; 256];
    let mut move_0: std::os::raw::c_int = 0;
    loop {
        printf(
            b"enter your move to play (or enter 0 to exit game): \0" as *const u8
                as *const std::os::raw::c_char,
        );
        fgets(buffer.as_mut_ptr(), 256 as std::os::raw::c_int, stdin);
        if !(sscanf(
            buffer.as_mut_ptr(),
            b"%d\0" as *const u8 as *const std::os::raw::c_char,
            &mut move_0 as *mut std::os::raw::c_int,
        ) != 1 as std::os::raw::c_int
            || move_0 != 0
                && (move_0 < 1 as std::os::raw::c_int || move_0 > 3 as std::os::raw::c_int
                    || total + move_0 > 21 as std::os::raw::c_int))
        {
            break;
        }
        puts(
            b"\nYour answer is not a valid choice.\n\0" as *const u8
                as *const std::os::raw::c_char,
        );
    }
    putchar('\n' as i32);
    if move_0 == 0 {
        exit(0 as std::os::raw::c_int);
    }
    update(b"human\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char, move_0);
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    srand(time(0 as *mut time_t) as std::os::raw::c_uint);
    puts(DESCRIPTION.as_mut_ptr());
    loop {
        puts(b"\n---- NEW GAME ----\n\0" as *const u8 as *const std::os::raw::c_char);
        puts(
            b"\nThe running total is currently zero.\n\0" as *const u8
                as *const std::os::raw::c_char,
        );
        total = 0 as std::os::raw::c_int;
        if rand() % 2 as std::os::raw::c_int != 0 {
            puts(b"The first move is AI move.\n\0" as *const u8 as *const std::os::raw::c_char);
            ai();
        } else {
            puts(
                b"The first move is human move.\n\0" as *const u8 as *const std::os::raw::c_char,
            );
        }
        while total < 21 as std::os::raw::c_int {
            human();
            ai();
        }
    };
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
