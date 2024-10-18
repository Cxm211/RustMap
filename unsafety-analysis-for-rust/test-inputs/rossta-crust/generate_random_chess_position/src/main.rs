#![no_std]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

use std::time::SystemTime;
pub fn rust_time(ref_result: Option<&mut i64>) -> i64 {
    let result = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    match ref_result {
        Some(r) => *r = result,
        None => {}
    }
    return result as i64;
}

//use c2rust_out::*;
extern "C" {
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn rand() -> i32;
    fn srand(__seed: u32);
    fn abs(_: i32) -> i32;
}
#[no_mangle]
pub static mut grid: [[i8; 8]; 8] = [[0; 8]; 8];
#[no_mangle]
pub extern "C" fn placeKings() {
    let mut r1: i32 = 0;
    let mut r2: i32 = 0;
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    unsafe {
        loop {
            r1 = rand() % 8;
            c1 = rand() % 8;
            r2 = rand() % 8;
            c2 = rand() % 8;
            if r1 != r2 && abs(r1 - r2) > 1 && abs(c1 - c2) > 1 {
                grid[r1 as usize][c1 as usize] = 'K' as i8;
                grid[r2 as usize][c2 as usize] = 'k' as i8;
                return;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn placePieces(mut pieces: *const i8, mut isPawn: i32) {
    unsafe {
        let mut n: i32 = 0;
        let mut r: i32 = 0;
        let mut c: i32 = 0;
        let mut numToPlace: i32 = (rand() as u64).wrapping_rem(strlen(pieces)) as i32;
        n = 0;
        while n < numToPlace {
            loop {
                r = rand() % 8;
                c = rand() % 8;
                if !(grid[r as usize][c as usize] as i32 != 0 || isPawn != 0 && (r == 7 || r == 0))
                {
                    break;
                }
            }
            grid[r as usize][c as usize] = *pieces.offset(n as isize);
            n += 1;
            n;
        }
    }
}

#[no_mangle]
pub extern "C" fn toFen() {
    let mut fen: [i8; 80] = [0; 80];
    let mut ch: i8 = 0;
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    let mut countEmpty: i32 = 0;
    let mut index: i32 = 0;
    r = 0;
    unsafe {
        while r < 8 {
            c = 0;
            while c < 8 {
                ch = grid[r as usize][c as usize];
                if ch as i32 == 0 {
                    print!("{:2} ", '.' as i32)
                } else {
                    print!("{:2} ", ch as i32)
                };
                if ch as i32 == 0 {
                    countEmpty += 1;
                    countEmpty;
                } else {
                    if countEmpty > 0 {
                        let fresh0 = index;
                        index = index + 1;
                        fen[fresh0 as usize] = (countEmpty + 48i32) as i8;
                        countEmpty = 0;
                    }
                    let fresh1 = index;
                    index = index + 1;
                    fen[fresh1 as usize] = ch;
                }
                c += 1;
                c;
            }
            if countEmpty > 0 {
                let fresh2 = index;
                index = index + 1;
                fen[fresh2 as usize] = (countEmpty + 48i32) as i8;
                countEmpty = 0;
            }
            let fresh3 = index;
            index = index + 1;
            fen[fresh3 as usize] = '/' as i8;
            print!("\n");
            r += 1;
            r;
        }
        strcpy(
            fen.as_mut_ptr().offset(index as isize),
            b" w - - 0 1\0" as *const u8 as *const i8,
        );
        print!("{}\n", build_str_from_raw_ptr(fen.as_mut_ptr() as *mut u8));
    }
}

#[no_mangle]
pub extern "C" fn createFen() -> *mut i8 {
    placeKings();
    placePieces(b"PPPPPPPP\0" as *const u8 as *const i8, 1);
    placePieces(b"pppppppp\0" as *const u8 as *const i8, 1);
    placePieces(b"RNBQBNR\0" as *const u8 as *const i8, 0);
    placePieces(b"rnbqbnr\0" as *const u8 as *const i8, 0);
    toFen();
    panic!("Reached end of non-void function without returning");
}

fn main_0() -> i32 {
    unsafe {
        srand(rust_time(None) as u32);
    }
    createFen();
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
