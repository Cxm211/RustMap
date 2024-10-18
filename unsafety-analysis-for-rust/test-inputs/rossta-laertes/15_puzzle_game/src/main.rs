#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn getchar() -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
    fn rand() -> std::os::raw::c_int;
    fn srand(__seed: std::os::raw::c_uint);
    fn exit(_: std::os::raw::c_int) -> !;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type __time_t = std::os::raw::c_long;
pub type time_t = std::os::raw::c_long;
pub type Move = std::os::raw::c_uint;
pub const MOVE_RIGHT: Move = 3;
pub const MOVE_LEFT: Move = 2;
pub const MOVE_DOWN: Move = 1;
pub const MOVE_UP: Move = 0;
#[no_mangle]
pub static mut holeRow: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut holeCollumn: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut cells: [[std::os::raw::c_int; 4]; 4] = [[0; 4]; 4];
#[no_mangle]
pub static mut nShuffles: std::os::raw::c_int = 100 as std::os::raw::c_int;
#[no_mangle]
pub unsafe extern "C" fn Game_update(mut move_0: Move) -> std::os::raw::c_int {
    let dx: [std::os::raw::c_int; 4] = [
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        -(1 as std::os::raw::c_int),
        1 as std::os::raw::c_int,
    ];
    let dy: [std::os::raw::c_int; 4] = [
        -(1 as std::os::raw::c_int),
        1 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
    ];
    let mut i: std::os::raw::c_int = holeRow + dy[move_0 as usize];
    let mut j: std::os::raw::c_int = holeCollumn + dx[move_0 as usize];
    if i >= 0 as std::os::raw::c_int && i < 4 as std::os::raw::c_int && j >= 0 as std::os::raw::c_int
        && j < 4 as std::os::raw::c_int
    {
        cells[holeRow as usize][holeCollumn as usize] = cells[i as usize][j as usize];
        cells[i as usize][j as usize] = 0 as std::os::raw::c_int;
        holeRow = i;
        holeCollumn = j;
        return 1 as std::os::raw::c_int;
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Game_setup() {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 4 as std::os::raw::c_int {
        j = 0 as std::os::raw::c_int;
        while j < 4 as std::os::raw::c_int {
            cells[i as usize][j as usize] = i * 4 as std::os::raw::c_int + j + 1 as std::os::raw::c_int;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    cells[(4 as std::os::raw::c_int - 1 as std::os::raw::c_int)
        as usize][(4 as std::os::raw::c_int - 1 as std::os::raw::c_int) as usize] = 0 as std::os::raw::c_int;
    holeRow = 4 as std::os::raw::c_int - 1 as std::os::raw::c_int;
    holeCollumn = 4 as std::os::raw::c_int - 1 as std::os::raw::c_int;
    k = 0 as std::os::raw::c_int;
    while k < nShuffles {
        k += Game_update((rand() % 4 as std::os::raw::c_int) as Move);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Game_isFinished() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 1 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < 4 as std::os::raw::c_int {
        j = 0 as std::os::raw::c_int;
        while j < 4 as std::os::raw::c_int {
            if k < 4 as std::os::raw::c_int * 4 as std::os::raw::c_int
                && {
                    let fresh0 = k;
                    k = k + 1;
                    cells[i as usize][j as usize] != fresh0
                }
            {
                return 0 as std::os::raw::c_int;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn View_showBoard() {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    putchar('\n' as i32);
    i = 0 as std::os::raw::c_int;
    while i < 4 as std::os::raw::c_int {
        j = 0 as std::os::raw::c_int;
        while j < 4 as std::os::raw::c_int {
            if cells[i as usize][j as usize] != 0 {
                printf(
                    if j != 4 as std::os::raw::c_int - 1 as std::os::raw::c_int {
                        b" %2d \0" as *const u8 as *const std::os::raw::c_char
                    } else {
                        b" %2d \n\0" as *const u8 as *const std::os::raw::c_char
                    },
                    cells[i as usize][j as usize],
                );
            } else {
                printf(
                    if j != 4 as std::os::raw::c_int - 1 as std::os::raw::c_int {
                        b" %2s \0" as *const u8 as *const std::os::raw::c_char
                    } else {
                        b" %2s \n\0" as *const u8 as *const std::os::raw::c_char
                    },
                    b"\0" as *const u8 as *const std::os::raw::c_char,
                );
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    putchar('\n' as i32);
}
#[no_mangle]
pub unsafe extern "C" fn View_displayMessage(mut text: *mut std::os::raw::c_char) {
    printf(b"\n%s\n\0" as *const u8 as *const std::os::raw::c_char, text);
}
#[no_mangle]
pub unsafe extern "C" fn Controller_getMove() -> Move {
    let mut c: std::os::raw::c_int = 0;
    loop {
        printf(
            b"%s\0" as *const u8 as *const std::os::raw::c_char,
            b"enter u/d/l/r : \0" as *const u8 as *const std::os::raw::c_char,
        );
        c = getchar();
        while getchar() != '\n' as i32 {}
        match c {
            27 => {
                exit(0 as std::os::raw::c_int);
            }
            100 => return MOVE_UP,
            117 => return MOVE_DOWN,
            114 => return MOVE_LEFT,
            108 => return MOVE_RIGHT,
            _ => {}
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Controller_pause() {
    getchar();
}
unsafe fn main_0() -> std::os::raw::c_int {
    srand(time(0 as *mut time_t) as std::os::raw::c_uint);
    loop {
        Game_setup();
        if !(Game_isFinished() != 0) {
            break;
        }
    }
    View_showBoard();
    while Game_isFinished() == 0 {
        Game_update(Controller_getMove());
        View_showBoard();
    }
    View_displayMessage(
        b"You win\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
    );
    Controller_pause();
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
