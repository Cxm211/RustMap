#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn rand() -> std::os::raw::c_int;
    fn srand(__seed: std::os::raw::c_uint);
    fn abs(_: std::os::raw::c_int) -> std::os::raw::c_int;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type __time_t = std::os::raw::c_long;
pub type time_t = __time_t;
pub type bool_0 = std::os::raw::c_int;
#[no_mangle]
pub static mut grid: [[std::os::raw::c_char; 8]; 8] = [[0; 8]; 8];
#[no_mangle]
pub unsafe extern "C" fn placeKings() {
    let mut r1: std::os::raw::c_int = 0;
    let mut r2: std::os::raw::c_int = 0;
    let mut c1: std::os::raw::c_int = 0;
    let mut c2: std::os::raw::c_int = 0;
    loop {
        r1 = rand() % 8 as std::os::raw::c_int;
        c1 = rand() % 8 as std::os::raw::c_int;
        r2 = rand() % 8 as std::os::raw::c_int;
        c2 = rand() % 8 as std::os::raw::c_int;
        if r1 != r2 && abs(r1 - r2) > 1 as std::os::raw::c_int && abs(c1 - c2) > 1 as std::os::raw::c_int
        {
            grid[r1 as usize][c1 as usize] = 'K' as i32 as std::os::raw::c_char;
            grid[r2 as usize][c2 as usize] = 'k' as i32 as std::os::raw::c_char;
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn placePieces(
    mut pieces: *const std::os::raw::c_char,
    mut isPawn: bool_0,
) {
    let mut n: std::os::raw::c_int = 0;
    let mut r: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut numToPlace: std::os::raw::c_int = (rand() as std::os::raw::c_ulong)
        .wrapping_rem(strlen(pieces)) as std::os::raw::c_int;
    n = 0 as std::os::raw::c_int;
    while n < numToPlace {
        loop {
            r = rand() % 8 as std::os::raw::c_int;
            c = rand() % 8 as std::os::raw::c_int;
            if !(grid[r as usize][c as usize] as std::os::raw::c_int != 0 as std::os::raw::c_int
                || isPawn != 0 && (r == 7 as std::os::raw::c_int || r == 0 as std::os::raw::c_int))
            {
                break;
            }
        }
        grid[r as usize][c as usize] = *pieces.offset(n as isize);
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn toFen() {
    let mut fen: [std::os::raw::c_char; 80] = [0; 80];
    let mut ch: std::os::raw::c_char = 0;
    let mut r: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut countEmpty: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut index: std::os::raw::c_int = 0 as std::os::raw::c_int;
    r = 0 as std::os::raw::c_int;
    while r < 8 as std::os::raw::c_int {
        c = 0 as std::os::raw::c_int;
        while c < 8 as std::os::raw::c_int {
            ch = grid[r as usize][c as usize];
            printf(
                b"%2c \0" as *const u8 as *const std::os::raw::c_char,
                if ch as std::os::raw::c_int == 0 as std::os::raw::c_int {
                    '.' as i32
                } else {
                    ch as std::os::raw::c_int
                },
            );
            if ch as std::os::raw::c_int == 0 as std::os::raw::c_int {
                countEmpty += 1;
                countEmpty;
            } else {
                if countEmpty > 0 as std::os::raw::c_int {
                    let fresh0 = index;
                    index = index + 1;
                    fen[fresh0
                        as usize] = (countEmpty + 48 as std::os::raw::c_int) as std::os::raw::c_char;
                    countEmpty = 0 as std::os::raw::c_int;
                }
                let fresh1 = index;
                index = index + 1;
                fen[fresh1 as usize] = ch;
            }
            c += 1;
            c;
        }
        if countEmpty > 0 as std::os::raw::c_int {
            let fresh2 = index;
            index = index + 1;
            fen[fresh2 as usize] = (countEmpty + 48 as std::os::raw::c_int) as std::os::raw::c_char;
            countEmpty = 0 as std::os::raw::c_int;
        }
        let fresh3 = index;
        index = index + 1;
        fen[fresh3 as usize] = '/' as i32 as std::os::raw::c_char;
        printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
        r += 1;
        r;
    }
    strcpy(
        fen.as_mut_ptr().offset(index as isize),
        b" w - - 0 1\0" as *const u8 as *const std::os::raw::c_char,
    );
    printf(b"%s\n\0" as *const u8 as *const std::os::raw::c_char, fen.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn createFen() -> *mut std::os::raw::c_char {
    placeKings();
    placePieces(b"PPPPPPPP\0" as *const u8 as *const std::os::raw::c_char, 1 as std::os::raw::c_int);
    placePieces(b"pppppppp\0" as *const u8 as *const std::os::raw::c_char, 1 as std::os::raw::c_int);
    placePieces(b"RNBQBNR\0" as *const u8 as *const std::os::raw::c_char, 0 as std::os::raw::c_int);
    placePieces(b"rnbqbnr\0" as *const u8 as *const std::os::raw::c_char, 0 as std::os::raw::c_int);
    toFen();
    panic!("Reached end of non-void function without returning");
}
unsafe fn main_0() -> std::os::raw::c_int {
    srand(time(0 as *mut time_t) as std::os::raw::c_uint);
    createFen();
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
