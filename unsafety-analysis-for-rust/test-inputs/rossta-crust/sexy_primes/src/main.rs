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

//use c2rust_out::*;
extern "C" {
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
}
#[no_mangle]
pub extern "C" fn sieve(mut c: *mut u8, mut limit: i32) {
    unsafe {
        let mut i: i32 = 0;
        let mut p: i32 = 3;
        let mut p2: i32 = 0;
        *c.offset(0 as isize) = 1;
        *c.offset(1 as isize) = 1;
        loop {
            p2 = p * p;
            if p2 >= limit {
                break;
            }
            i = p2;
            while i < limit {
                *c.offset(i as isize) = 1;
                i += 2 * p;
            }
            loop {
                p += 2;
                if *c.offset(p as isize) == 0 {
                    break;
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn printHelper(mut cat: *const i8, mut len: i32, mut lim: i32, mut n: i32) {
    unsafe {
        let mut sp: *const i8 = if strcmp(cat, b"unsexy primes\0" as *const u8 as *const i8) != 0 {
            b"sexy prime \0" as *const u8 as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        };
        let mut verb: *const i8 = if len == 1 {
            b"is\0" as *const u8 as *const i8
        } else {
            b"are\0" as *const u8 as *const i8
        };
        print!(
            "Number of {}{} less than {} = {}\n",
            build_str_from_raw_ptr(sp as *mut u8),
            build_str_from_raw_ptr(cat as *mut u8),
            lim,
            len
        );
        print!(
            "The last {} {}:\n",
            n,
            build_str_from_raw_ptr(verb as *mut u8)
        );
    }
}

#[no_mangle]
pub extern "C" fn printArray(mut a: *mut i32, mut len: i32) {
    unsafe {
        let mut i: i32 = 0;
        print!("[");
        i = 0;
        while i < len {
            print!("{} ", *a.offset(i as isize));
            i += 1;
            i;
        }
        print!("\x08]");
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut ix: i32 = 0;
        let mut n: i32 = 0;
        let mut lim: i32 = 1000035;
        let mut pairs: i32 = 0;
        let mut trips: i32 = 0;
        let mut quads: i32 = 0;
        let mut quins: i32 = 0;
        let mut unsexy: i32 = 2;
        let mut pr: i32 = 0;
        let mut tr: i32 = 0;
        let mut qd: i32 = 0;
        let mut qn: i32 = 0;
        let mut un: i32 = 2;
        let mut lpr: i32 = 5;
        let mut ltr: i32 = 5;
        let mut lqd: i32 = 5;
        let mut lqn: i32 = 5;
        let mut lun: i32 = 10;
        let mut last_pr: [[i32; 2]; 5] = [[0; 2]; 5];
        let mut last_tr: [[i32; 3]; 5] = [[0; 3]; 5];
        let mut last_qd: [[i32; 4]; 5] = [[0; 4]; 5];
        let mut last_qn: [[i32; 5]; 5] = [[0; 5]; 5];
        let mut last_un: [i32; 10] = [0; 10];
        let mut sv: *mut u8 =
            calloc((lim - 1i32) as u64, ::core::mem::size_of::<u8>() as u64) as *mut u8;
        setlocale(1, b"\0" as *const u8 as *const i8);
        sieve(sv, lim);
        i = 3;
        while i < lim {
            if i > 5
                && i < lim - 6
                && *sv.offset(i as isize) == 0
                && *sv.offset((i - 6i32) as isize) as i32 != 0
                && *sv.offset((i + 6i32) as isize) as i32 != 0
            {
                unsexy += 1;
                unsexy;
            } else if i < lim - 6
                && *sv.offset(i as isize) == 0
                && *sv.offset((i + 6i32) as isize) == 0
            {
                pairs += 1;
                pairs;
                if i < lim - 12 && *sv.offset((i + 12i32) as isize) == 0 {
                    trips += 1;
                    trips;
                    if i < lim - 18 && *sv.offset((i + 18i32) as isize) == 0 {
                        quads += 1;
                        quads;
                        if i < lim - 24 && *sv.offset((i + 24i32) as isize) == 0 {
                            quins += 1;
                            quins;
                        }
                    }
                }
            }
            i += 2;
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
        i = 3;
        while i < lim {
            if i > 5
                && i < lim - 6
                && *sv.offset(i as isize) == 0
                && *sv.offset((i - 6i32) as isize) as i32 != 0
                && *sv.offset((i + 6i32) as isize) as i32 != 0
            {
                un += 1;
                un;
                if un > unsexy - lun {
                    last_un[(un + lun - 1 - unsexy) as usize] = i;
                }
            } else if i < lim - 6
                && *sv.offset(i as isize) == 0
                && *sv.offset((i + 6i32) as isize) == 0
            {
                pr += 1;
                pr;
                if pr > pairs - lpr {
                    ix = pr + lpr - 1 - pairs;
                    last_pr[ix as usize][0 as usize] = i;
                    last_pr[ix as usize][1 as usize] = i + 6;
                }
                if i < lim - 12 && *sv.offset((i + 12i32) as isize) == 0 {
                    tr += 1;
                    tr;
                    if tr > trips - ltr {
                        ix = tr + ltr - 1 - trips;
                        last_tr[ix as usize][0 as usize] = i;
                        last_tr[ix as usize][1 as usize] = i + 6;
                        last_tr[ix as usize][2 as usize] = i + 12;
                    }
                    if i < lim - 18 && *sv.offset((i + 18i32) as isize) == 0 {
                        qd += 1;
                        qd;
                        if qd > quads - lqd {
                            ix = qd + lqd - 1 - quads;
                            last_qd[ix as usize][0 as usize] = i;
                            last_qd[ix as usize][1 as usize] = i + 6;
                            last_qd[ix as usize][2 as usize] = i + 12;
                            last_qd[ix as usize][3 as usize] = i + 18;
                        }
                        if i < lim - 24 && *sv.offset((i + 24i32) as isize) == 0 {
                            qn += 1;
                            qn;
                            if qn > quins - lqn {
                                ix = qn + lqn - 1 - quins;
                                last_qn[ix as usize][0 as usize] = i;
                                last_qn[ix as usize][1 as usize] = i + 6;
                                last_qn[ix as usize][2 as usize] = i + 12;
                                last_qn[ix as usize][3 as usize] = i + 18;
                                last_qn[ix as usize][4 as usize] = i + 24;
                            }
                        }
                    }
                }
            }
            i += 2;
        }
        printHelper(b"pairs\0" as *const u8 as *const i8, pairs, lim, lpr);
        print!("  [");
        i = 0;
        while i < lpr {
            printArray((last_pr[i as usize]).as_mut_ptr(), 2);
            print!("\x08] ");
            i += 1;
            i;
        }
        print!("\x08]\n\n");
        printHelper(b"triplets\0" as *const u8 as *const i8, trips, lim, ltr);
        print!("  [");
        i = 0;
        while i < ltr {
            printArray((last_tr[i as usize]).as_mut_ptr(), 3);
            print!("\x08] ");
            i += 1;
            i;
        }
        print!("\x08]\n\n");
        printHelper(b"quadruplets\0" as *const u8 as *const i8, quads, lim, lqd);
        print!("  [");
        i = 0;
        while i < lqd {
            printArray((last_qd[i as usize]).as_mut_ptr(), 4);
            print!("\x08] ");
            i += 1;
            i;
        }
        print!("\x08]\n\n");
        printHelper(b"quintuplets\0" as *const u8 as *const i8, quins, lim, lqn);
        print!("  [");
        i = 0;
        while i < lqn {
            printArray((last_qn[i as usize]).as_mut_ptr(), 5);
            print!("\x08] ");
            i += 1;
            i;
        }
        print!("\x08]\n\n");
        printHelper(
            b"unsexy primes\0" as *const u8 as *const i8,
            unsexy,
            lim,
            lun,
        );
        print!("  [");
        printArray(last_un.as_mut_ptr(), lun);
        print!("\x08]\n");
        free(sv as *mut libc::c_void);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
