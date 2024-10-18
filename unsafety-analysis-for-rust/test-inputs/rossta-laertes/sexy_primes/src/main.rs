#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn setlocale(
        __category: std::os::raw::c_int,
        __locale: *const std::os::raw::c_char,
    ) -> *mut std::os::raw::c_char;
}
pub type bool_0 = std::os::raw::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn sieve(mut c: *mut bool_0, mut limit: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut p: std::os::raw::c_int = 3 as std::os::raw::c_int;
    let mut p2: std::os::raw::c_int = 0;
    *c.offset(0 as std::os::raw::c_int as isize) = 1 as std::os::raw::c_int as bool_0;
    *c.offset(1 as std::os::raw::c_int as isize) = 1 as std::os::raw::c_int as bool_0;
    loop {
        p2 = p * p;
        if p2 >= limit {
            break;
        }
        i = p2;
        while i < limit {
            *c.offset(i as isize) = 1 as std::os::raw::c_int as bool_0;
            i += 2 as std::os::raw::c_int * p;
        }
        loop {
            p += 2 as std::os::raw::c_int;
            if *c.offset(p as isize) == 0 {
                break;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn printHelper(
    mut cat: *const std::os::raw::c_char,
    mut len: std::os::raw::c_int,
    mut lim: std::os::raw::c_int,
    mut n: std::os::raw::c_int,
) {
    let mut sp: *const std::os::raw::c_char = if strcmp(
        cat,
        b"unsexy primes\0" as *const u8 as *const std::os::raw::c_char,
    ) != 0
    {
        b"sexy prime \0" as *const u8 as *const std::os::raw::c_char
    } else {
        b"\0" as *const u8 as *const std::os::raw::c_char
    };
    let mut verb: *const std::os::raw::c_char = if len == 1 as std::os::raw::c_int {
        b"is\0" as *const u8 as *const std::os::raw::c_char
    } else {
        b"are\0" as *const u8 as *const std::os::raw::c_char
    };
    printf(
        b"Number of %s%s less than %'d = %'d\n\0" as *const u8 as *const std::os::raw::c_char,
        sp,
        cat,
        lim,
        len,
    );
    printf(b"The last %d %s:\n\0" as *const u8 as *const std::os::raw::c_char, n, verb);
}
#[no_mangle]
pub unsafe extern "C" fn printArray(mut a: *mut std::os::raw::c_int, mut len: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    printf(b"[\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < len {
        printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, *a.offset(i as isize));
        i += 1;
        i;
    }
    printf(b"\x08]\0" as *const u8 as *const std::os::raw::c_char);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut ix: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0;
    let mut lim: std::os::raw::c_int = 1000035 as std::os::raw::c_int;
    let mut pairs: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut trips: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut quads: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut quins: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut unsexy: std::os::raw::c_int = 2 as std::os::raw::c_int;
    let mut pr: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut tr: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut qd: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut qn: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut un: std::os::raw::c_int = 2 as std::os::raw::c_int;
    let mut lpr: std::os::raw::c_int = 5 as std::os::raw::c_int;
    let mut ltr: std::os::raw::c_int = 5 as std::os::raw::c_int;
    let mut lqd: std::os::raw::c_int = 5 as std::os::raw::c_int;
    let mut lqn: std::os::raw::c_int = 5 as std::os::raw::c_int;
    let mut lun: std::os::raw::c_int = 10 as std::os::raw::c_int;
    let mut last_pr: [[std::os::raw::c_int; 2]; 5] = [[0; 2]; 5];
    let mut last_tr: [[std::os::raw::c_int; 3]; 5] = [[0; 3]; 5];
    let mut last_qd: [[std::os::raw::c_int; 4]; 5] = [[0; 4]; 5];
    let mut last_qn: [[std::os::raw::c_int; 5]; 5] = [[0; 5]; 5];
    let mut last_un: [std::os::raw::c_int; 10] = [0; 10];
    let mut sv: *mut bool_0 = calloc(
        (lim - 1 as std::os::raw::c_int) as std::os::raw::c_ulong,
        std::mem::size_of::<bool_0>() as std::os::raw::c_ulong,
    ) as *mut bool_0;
    setlocale(1 as std::os::raw::c_int, b"\0" as *const u8 as *const std::os::raw::c_char);
    sieve(sv, lim);
    i = 3 as std::os::raw::c_int;
    while i < lim {
        if i > 5 as std::os::raw::c_int && i < lim - 6 as std::os::raw::c_int
            && *sv.offset(i as isize) == 0
            && *sv.offset((i - 6 as std::os::raw::c_int) as isize) as std::os::raw::c_int != 0
            && *sv.offset((i + 6 as std::os::raw::c_int) as isize) as std::os::raw::c_int != 0
        {
            unsexy += 1;
            unsexy;
        } else if i < lim - 6 as std::os::raw::c_int && *sv.offset(i as isize) == 0
            && *sv.offset((i + 6 as std::os::raw::c_int) as isize) == 0
        {
            pairs += 1;
            pairs;
            if i < lim - 12 as std::os::raw::c_int
                && *sv.offset((i + 12 as std::os::raw::c_int) as isize) == 0
            {
                trips += 1;
                trips;
                if i < lim - 18 as std::os::raw::c_int
                    && *sv.offset((i + 18 as std::os::raw::c_int) as isize) == 0
                {
                    quads += 1;
                    quads;
                    if i < lim - 24 as std::os::raw::c_int
                        && *sv.offset((i + 24 as std::os::raw::c_int) as isize) == 0
                    {
                        quins += 1;
                        quins;
                    }
                }
            }
        }
        i += 2 as std::os::raw::c_int;
    }
    if pairs < lpr {
        lpr = pairs;
    }
    if trips < ltr {
        ltr = trips;
    }
    if quads < lqd {
        lqd = quads;
    }
    if quins < lqn {
        lqn = quins;
    }
    if unsexy < lun {
        lun = unsexy;
    }
    i = 3 as std::os::raw::c_int;
    while i < lim {
        if i > 5 as std::os::raw::c_int && i < lim - 6 as std::os::raw::c_int
            && *sv.offset(i as isize) == 0
            && *sv.offset((i - 6 as std::os::raw::c_int) as isize) as std::os::raw::c_int != 0
            && *sv.offset((i + 6 as std::os::raw::c_int) as isize) as std::os::raw::c_int != 0
        {
            un += 1;
            un;
            if un > unsexy - lun {
                last_un[(un + lun - 1 as std::os::raw::c_int - unsexy) as usize] = i;
            }
        } else if i < lim - 6 as std::os::raw::c_int && *sv.offset(i as isize) == 0
            && *sv.offset((i + 6 as std::os::raw::c_int) as isize) == 0
        {
            pr += 1;
            pr;
            if pr > pairs - lpr {
                ix = pr + lpr - 1 as std::os::raw::c_int - pairs;
                last_pr[ix as usize][0 as std::os::raw::c_int as usize] = i;
                last_pr[ix as usize][1 as std::os::raw::c_int as usize] = i + 6 as std::os::raw::c_int;
            }
            if i < lim - 12 as std::os::raw::c_int
                && *sv.offset((i + 12 as std::os::raw::c_int) as isize) == 0
            {
                tr += 1;
                tr;
                if tr > trips - ltr {
                    ix = tr + ltr - 1 as std::os::raw::c_int - trips;
                    last_tr[ix as usize][0 as std::os::raw::c_int as usize] = i;
                    last_tr[ix
                        as usize][1 as std::os::raw::c_int as usize] = i + 6 as std::os::raw::c_int;
                    last_tr[ix
                        as usize][2 as std::os::raw::c_int as usize] = i + 12 as std::os::raw::c_int;
                }
                if i < lim - 18 as std::os::raw::c_int
                    && *sv.offset((i + 18 as std::os::raw::c_int) as isize) == 0
                {
                    qd += 1;
                    qd;
                    if qd > quads - lqd {
                        ix = qd + lqd - 1 as std::os::raw::c_int - quads;
                        last_qd[ix as usize][0 as std::os::raw::c_int as usize] = i;
                        last_qd[ix
                            as usize][1 as std::os::raw::c_int as usize] = i + 6 as std::os::raw::c_int;
                        last_qd[ix
                            as usize][2 as std::os::raw::c_int as usize] = i + 12 as std::os::raw::c_int;
                        last_qd[ix
                            as usize][3 as std::os::raw::c_int as usize] = i + 18 as std::os::raw::c_int;
                    }
                    if i < lim - 24 as std::os::raw::c_int
                        && *sv.offset((i + 24 as std::os::raw::c_int) as isize) == 0
                    {
                        qn += 1;
                        qn;
                        if qn > quins - lqn {
                            ix = qn + lqn - 1 as std::os::raw::c_int - quins;
                            last_qn[ix as usize][0 as std::os::raw::c_int as usize] = i;
                            last_qn[ix
                                as usize][1 as std::os::raw::c_int as usize] = i + 6 as std::os::raw::c_int;
                            last_qn[ix
                                as usize][2 as std::os::raw::c_int
                                as usize] = i + 12 as std::os::raw::c_int;
                            last_qn[ix
                                as usize][3 as std::os::raw::c_int
                                as usize] = i + 18 as std::os::raw::c_int;
                            last_qn[ix
                                as usize][4 as std::os::raw::c_int
                                as usize] = i + 24 as std::os::raw::c_int;
                        }
                    }
                }
            }
        }
        i += 2 as std::os::raw::c_int;
    }
    printHelper(b"pairs\0" as *const u8 as *const std::os::raw::c_char, pairs, lim, lpr);
    printf(b"  [\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < lpr {
        printArray((last_pr[i as usize]).as_mut_ptr(), 2 as std::os::raw::c_int);
        printf(b"\x08] \0" as *const u8 as *const std::os::raw::c_char);
        i += 1;
        i;
    }
    printf(b"\x08]\n\n\0" as *const u8 as *const std::os::raw::c_char);
    printHelper(b"triplets\0" as *const u8 as *const std::os::raw::c_char, trips, lim, ltr);
    printf(b"  [\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < ltr {
        printArray((last_tr[i as usize]).as_mut_ptr(), 3 as std::os::raw::c_int);
        printf(b"\x08] \0" as *const u8 as *const std::os::raw::c_char);
        i += 1;
        i;
    }
    printf(b"\x08]\n\n\0" as *const u8 as *const std::os::raw::c_char);
    printHelper(b"quadruplets\0" as *const u8 as *const std::os::raw::c_char, quads, lim, lqd);
    printf(b"  [\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < lqd {
        printArray((last_qd[i as usize]).as_mut_ptr(), 4 as std::os::raw::c_int);
        printf(b"\x08] \0" as *const u8 as *const std::os::raw::c_char);
        i += 1;
        i;
    }
    printf(b"\x08]\n\n\0" as *const u8 as *const std::os::raw::c_char);
    printHelper(b"quintuplets\0" as *const u8 as *const std::os::raw::c_char, quins, lim, lqn);
    printf(b"  [\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < lqn {
        printArray((last_qn[i as usize]).as_mut_ptr(), 5 as std::os::raw::c_int);
        printf(b"\x08] \0" as *const u8 as *const std::os::raw::c_char);
        i += 1;
        i;
    }
    printf(b"\x08]\n\n\0" as *const u8 as *const std::os::raw::c_char);
    printHelper(
        b"unsexy primes\0" as *const u8 as *const std::os::raw::c_char,
        unsexy,
        lim,
        lun,
    );
    printf(b"  [\0" as *const u8 as *const std::os::raw::c_char);
    printArray(last_un.as_mut_ptr(), lun);
    printf(b"\x08]\n\0" as *const u8 as *const std::os::raw::c_char);
    free(sv as *mut u8);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
