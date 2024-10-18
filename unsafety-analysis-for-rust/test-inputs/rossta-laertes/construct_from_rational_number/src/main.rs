#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fraction {
    pub num: std::os::raw::c_int,
    pub den: std::os::raw::c_int,
}
impl std::default::Default for fraction {
    fn default() -> Self {
        fraction {
        num: std::os::raw::c_int::default(),
        den: std::os::raw::c_int::default()
        }
    }
}

#[no_mangle]
pub static mut examples: [fraction; 6] = [
    {
        let mut init = fraction {
            num: 1 as std::os::raw::c_int,
            den: 2 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 3 as std::os::raw::c_int,
            den: 1 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 23 as std::os::raw::c_int,
            den: 8 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 13 as std::os::raw::c_int,
            den: 11 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 22 as std::os::raw::c_int,
            den: 7 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: -(151 as std::os::raw::c_int),
            den: 77 as std::os::raw::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut sqrt2: [fraction; 4] = [
    {
        let mut init = fraction {
            num: 14142 as std::os::raw::c_int,
            den: 10000 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 141421 as std::os::raw::c_int,
            den: 100000 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 1414214 as std::os::raw::c_int,
            den: 1000000 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 14142136 as std::os::raw::c_int,
            den: 10000000 as std::os::raw::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut pi: [fraction; 8] = [
    {
        let mut init = fraction {
            num: 31 as std::os::raw::c_int,
            den: 10 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 314 as std::os::raw::c_int,
            den: 100 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 3142 as std::os::raw::c_int,
            den: 1000 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 31428 as std::os::raw::c_int,
            den: 10000 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 314285 as std::os::raw::c_int,
            den: 100000 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 3142857 as std::os::raw::c_int,
            den: 1000000 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 31428571 as std::os::raw::c_int,
            den: 10000000 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 314285714 as std::os::raw::c_int,
            den: 100000000 as std::os::raw::c_int,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn r2cf<'a1, 'a2>(
    mut numerator: Option<&'a1 mut std::os::raw::c_int>,
    mut denominator: Option<&'a2 mut std::os::raw::c_int>,
) -> std::os::raw::c_int {
    let mut quotient: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut temp: std::os::raw::c_int = 0;
    if !borrow(& denominator).is_none() {
        quotient = *borrow(& numerator).unwrap() / *borrow(& denominator).unwrap();
        temp = *borrow_mut(&mut numerator).unwrap();
        *borrow_mut(&mut numerator).unwrap() = *borrow_mut(&mut denominator).unwrap();
        *borrow_mut(&mut denominator).unwrap() = temp % *borrow(& denominator).unwrap();
    }
    return quotient;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    printf(b"Running the examples :\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while (i as std::os::raw::c_ulong)
        < (std::mem::size_of::<[fraction; 6]>() as std::os::raw::c_ulong)
            .wrapping_div(std::mem::size_of::<fraction>() as std::os::raw::c_ulong)
    {
        printf(
            b"\nFor N = %d, D = %d :\0" as *const u8 as *const std::os::raw::c_char,
            examples[i as usize].num,
            examples[i as usize].den,
        );
        while examples[i as usize].den != 0 as std::os::raw::c_int {
            printf(
                b" %d \0" as *const u8 as *const std::os::raw::c_char,
                r2cf(
                    Some(&mut (*examples.as_mut_ptr().offset(i as isize)).num),
                    Some(&mut (*examples.as_mut_ptr().offset(i as isize)).den),
                ),
            );
        }
        i += 1;
        i;
    }
    printf(
        b"\n\nRunning for %c2 :\0" as *const u8 as *const std::os::raw::c_char,
        251 as std::os::raw::c_int,
    );
    i = 0 as std::os::raw::c_int;
    while (i as std::os::raw::c_ulong)
        < (std::mem::size_of::<[fraction; 4]>() as std::os::raw::c_ulong)
            .wrapping_div(std::mem::size_of::<fraction>() as std::os::raw::c_ulong)
    {
        printf(
            b"\nFor N = %d, D = %d :\0" as *const u8 as *const std::os::raw::c_char,
            sqrt2[i as usize].num,
            sqrt2[i as usize].den,
        );
        while sqrt2[i as usize].den != 0 as std::os::raw::c_int {
            printf(
                b" %d \0" as *const u8 as *const std::os::raw::c_char,
                r2cf(
                    Some(&mut (*sqrt2.as_mut_ptr().offset(i as isize)).num),
                    Some(&mut (*sqrt2.as_mut_ptr().offset(i as isize)).den),
                ),
            );
        }
        i += 1;
        i;
    }
    printf(
        b"\n\nRunning for %c :\0" as *const u8 as *const std::os::raw::c_char,
        227 as std::os::raw::c_int,
    );
    i = 0 as std::os::raw::c_int;
    while (i as std::os::raw::c_ulong)
        < (std::mem::size_of::<[fraction; 8]>() as std::os::raw::c_ulong)
            .wrapping_div(std::mem::size_of::<fraction>() as std::os::raw::c_ulong)
    {
        printf(
            b"\nFor N = %d, D = %d :\0" as *const u8 as *const std::os::raw::c_char,
            pi[i as usize].num,
            pi[i as usize].den,
        );
        while pi[i as usize].den != 0 as std::os::raw::c_int {
            printf(
                b" %d \0" as *const u8 as *const std::os::raw::c_char,
                r2cf(
                    Some(&mut (*pi.as_mut_ptr().offset(i as isize)).num),
                    Some(&mut (*pi.as_mut_ptr().offset(i as isize)).den),
                ),
            );
        }
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
