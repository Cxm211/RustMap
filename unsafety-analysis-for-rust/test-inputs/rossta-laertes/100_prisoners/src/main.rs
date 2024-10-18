#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn atoi(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn strtoull(
        _: *const std::os::raw::c_char,
        _: *mut *mut std::os::raw::c_char,
        _: std::os::raw::c_int,
    ) -> std::os::raw::c_ulonglong;
    fn rand() -> std::os::raw::c_int;
    fn srand(__seed: std::os::raw::c_uint);
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type __time_t = std::os::raw::c_long;
pub type time_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drawer {
    pub cardNum: std::os::raw::c_int,
    pub hasBeenOpened: bool,
}
impl std::default::Default for drawer {
    fn default() -> Self {
        drawer {
        cardNum: std::os::raw::c_int::default(),
        hasBeenOpened: false
        }
    }
}

#[no_mangle]
pub static mut drawerSet: *mut drawer = 0 as *const drawer as *mut drawer;
#[no_mangle]
pub unsafe extern "C" fn initialize(mut prisoners: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut card: std::os::raw::c_int = 0;
    let mut unique: bool = false;
    drawerSet = (malloc(
        (prisoners as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<drawer>() as std::os::raw::c_ulong),
    ) as *mut drawer)
        .offset(-(1 as std::os::raw::c_int as isize));
    card = rand() % prisoners + 1 as std::os::raw::c_int;
    *drawerSet
        .offset(
            1 as std::os::raw::c_int as isize,
        ) = {
        let mut init = drawer {
            cardNum: card,
            hasBeenOpened: 0 as std::os::raw::c_int != 0,
        };
        init
    };
    i = 1 as std::os::raw::c_int + 1 as std::os::raw::c_int;
    while i < prisoners + 1 as std::os::raw::c_int {
        unique = 0 as std::os::raw::c_int != 0;
        while unique as std::os::raw::c_int == 0 as std::os::raw::c_int {
            j = 0 as std::os::raw::c_int;
            while j < i {
                if (*drawerSet.offset(j as isize)).cardNum == card {
                    card = rand() % prisoners + 1 as std::os::raw::c_int;
                    break;
                } else {
                    j += 1;
                    j;
                }
            }
            if j == i {
                unique = 1 as std::os::raw::c_int != 0;
            }
        }
        *drawerSet
            .offset(
                i as isize,
            ) = {
            let mut init = drawer {
                cardNum: card,
                hasBeenOpened: 0 as std::os::raw::c_int != 0,
            };
            init
        };
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn closeAllDrawers(mut prisoners: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i < prisoners + 1 as std::os::raw::c_int {
        (*drawerSet.offset(i as isize)).hasBeenOpened = 0 as std::os::raw::c_int != 0;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn libertyOrDeathAtRandom(
    mut prisoners: std::os::raw::c_int,
    mut chances: std::os::raw::c_int,
) -> bool {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut chosenDrawer: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i < prisoners + 1 as std::os::raw::c_int {
        let mut foundCard: bool = 0 as std::os::raw::c_int != 0;
        j = 0 as std::os::raw::c_int;
        while j < chances {
            loop {
                chosenDrawer = rand() % prisoners + 1 as std::os::raw::c_int;
                if !((*drawerSet.offset(chosenDrawer as isize)).hasBeenOpened
                    as std::os::raw::c_int == 1 as std::os::raw::c_int)
                {
                    break;
                }
            }
            if (*drawerSet.offset(chosenDrawer as isize)).cardNum == i {
                foundCard = 1 as std::os::raw::c_int != 0;
                break;
            } else {
                (*drawerSet.offset(chosenDrawer as isize))
                    .hasBeenOpened = 1 as std::os::raw::c_int != 0;
                j += 1;
                j;
            }
        }
        closeAllDrawers(prisoners);
        if foundCard as std::os::raw::c_int == 0 as std::os::raw::c_int {
            return 1 as std::os::raw::c_int != 0;
        }
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn libertyOrDeathPlanned(
    mut prisoners: std::os::raw::c_int,
    mut chances: std::os::raw::c_int,
) -> bool {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut chosenDrawer: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i < prisoners + 1 as std::os::raw::c_int {
        chosenDrawer = i;
        let mut foundCard: bool = 0 as std::os::raw::c_int != 0;
        j = 0 as std::os::raw::c_int;
        while j < chances {
            (*drawerSet.offset(chosenDrawer as isize))
                .hasBeenOpened = 1 as std::os::raw::c_int != 0;
            if (*drawerSet.offset(chosenDrawer as isize)).cardNum == i {
                foundCard = 1 as std::os::raw::c_int != 0;
                break;
            } else {
                if chosenDrawer == (*drawerSet.offset(chosenDrawer as isize)).cardNum {
                    loop {
                        chosenDrawer = rand() % prisoners + 1 as std::os::raw::c_int;
                        if !((*drawerSet.offset(chosenDrawer as isize)).hasBeenOpened
                            as std::os::raw::c_int == 1 as std::os::raw::c_int)
                        {
                            break;
                        }
                    }
                } else {
                    chosenDrawer = (*drawerSet.offset(chosenDrawer as isize)).cardNum;
                }
                j += 1;
                j;
            }
        }
        closeAllDrawers(prisoners);
        if foundCard as std::os::raw::c_int == 0 as std::os::raw::c_int {
            return 1 as std::os::raw::c_int != 0;
        }
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int != 0;
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut prisoners: std::os::raw::c_int = 0;
    let mut chances: std::os::raw::c_int = 0;
    let mut trials: std::os::raw::c_ulonglong = 0;
    let mut i: std::os::raw::c_ulonglong = 0;
    let mut count: std::os::raw::c_ulonglong = 0 as std::os::raw::c_int as std::os::raw::c_ulonglong;
    let mut end: *mut std::os::raw::c_char = core::ptr::null_mut();
    if argc != 4 as std::os::raw::c_int {
        return printf(
            b"Usage : %s <Number of prisoners> <Number of chances> <Number of trials>\0"
                as *const u8 as *const std::os::raw::c_char,
            *argv.offset(0 as std::os::raw::c_int as isize),
        );
    }
    prisoners = atoi(*argv.offset(1 as std::os::raw::c_int as isize));
    chances = atoi(*argv.offset(2 as std::os::raw::c_int as isize));
    trials = strtoull(
        *argv.offset(3 as std::os::raw::c_int as isize),
        &mut end,
        10 as std::os::raw::c_int,
    );
    srand(time(0 as *mut time_t) as std::os::raw::c_uint);
    printf(b"Running random trials...\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int as std::os::raw::c_ulonglong;
    while i < trials {
        initialize(prisoners);
        count = count
            .wrapping_add(
                (if libertyOrDeathAtRandom(prisoners, chances) as std::os::raw::c_int
                    == 1 as std::os::raw::c_int
                {
                    0 as std::os::raw::c_int
                } else {
                    1 as std::os::raw::c_int
                }) as std::os::raw::c_ulonglong,
            );
        i = i.wrapping_add(1 as std::os::raw::c_long as std::os::raw::c_ulonglong);
    }
    printf(
        b"\n\nGames Played : %llu\nGames Won : %llu\nChances : %lf %% \n\n\0"
            as *const u8 as *const std::os::raw::c_char,
        trials,
        count,
        100.0f64 * count as std::os::raw::c_double / trials as std::os::raw::c_double,
    );
    count = 0 as std::os::raw::c_int as std::os::raw::c_ulonglong;
    printf(b"Running strategic trials...\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int as std::os::raw::c_ulonglong;
    while i < trials {
        initialize(prisoners);
        count = count
            .wrapping_add(
                (if libertyOrDeathPlanned(prisoners, chances) as std::os::raw::c_int
                    == 1 as std::os::raw::c_int
                {
                    0 as std::os::raw::c_int
                } else {
                    1 as std::os::raw::c_int
                }) as std::os::raw::c_ulonglong,
            );
        i = i.wrapping_add(1 as std::os::raw::c_long as std::os::raw::c_ulonglong);
    }
    printf(
        b"\n\nGames Played : %llu\nGames Won : %llu\nChances : %lf %% \n\n\0"
            as *const u8 as *const std::os::raw::c_char,
        trials,
        count,
        100.0f64 * count as std::os::raw::c_double / trials as std::os::raw::c_double,
    );
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
