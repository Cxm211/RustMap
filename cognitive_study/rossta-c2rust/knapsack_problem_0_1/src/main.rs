#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    fn free(_: *mut std::os::raw::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct item_t {
    pub name: *mut std::os::raw::c_char,
    pub weight: std::os::raw::c_int,
    pub value: std::os::raw::c_int,
}
#[no_mangle]
pub static mut items: [item_t; 22] = [
    {
        let mut init = item_t {
            name: b"map\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            weight: 9 as std::os::raw::c_int,
            value: 150 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"compass\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            weight: 13 as std::os::raw::c_int,
            value: 35 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"water\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            weight: 153 as std::os::raw::c_int,
            value: 200 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"sandwich\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            weight: 50 as std::os::raw::c_int,
            value: 160 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"glucose\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            weight: 15 as std::os::raw::c_int,
            value: 60 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"tin\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            weight: 68 as std::os::raw::c_int,
            value: 45 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"banana\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            weight: 27 as std::os::raw::c_int,
            value: 60 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"apple\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            weight: 39 as std::os::raw::c_int,
            value: 40 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"cheese\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            weight: 23 as std::os::raw::c_int,
            value: 30 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"beer\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            weight: 52 as std::os::raw::c_int,
            value: 10 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"suntan cream\0" as *const u8 as *const std::os::raw::c_char
                as *mut std::os::raw::c_char,
            weight: 11 as std::os::raw::c_int,
            value: 70 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"camera\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            weight: 32 as std::os::raw::c_int,
            value: 30 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"T-shirt\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            weight: 24 as std::os::raw::c_int,
            value: 15 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"trousers\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            weight: 48 as std::os::raw::c_int,
            value: 10 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"umbrella\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            weight: 73 as std::os::raw::c_int,
            value: 40 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"waterproof trousers\0" as *const u8 as *const std::os::raw::c_char
                as *mut std::os::raw::c_char,
            weight: 42 as std::os::raw::c_int,
            value: 70 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"waterproof overclothes\0" as *const u8 as *const std::os::raw::c_char
                as *mut std::os::raw::c_char,
            weight: 43 as std::os::raw::c_int,
            value: 75 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"note-case\0" as *const u8 as *const std::os::raw::c_char
                as *mut std::os::raw::c_char,
            weight: 22 as std::os::raw::c_int,
            value: 80 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"sunglasses\0" as *const u8 as *const std::os::raw::c_char
                as *mut std::os::raw::c_char,
            weight: 7 as std::os::raw::c_int,
            value: 20 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"towel\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            weight: 18 as std::os::raw::c_int,
            value: 12 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"socks\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            weight: 4 as std::os::raw::c_int,
            value: 50 as std::os::raw::c_int,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"book\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            weight: 30 as std::os::raw::c_int,
            value: 10 as std::os::raw::c_int,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn knapsack(
    mut items_0: *mut item_t,
    mut n: std::os::raw::c_int,
    mut w: std::os::raw::c_int,
) -> *mut std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut a: std::os::raw::c_int = 0;
    let mut b: std::os::raw::c_int = 0;
    let mut mm: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    let mut m: *mut *mut std::os::raw::c_int = 0 as *mut *mut std::os::raw::c_int;
    let mut s: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    mm = calloc(
        ((n + 1 as std::os::raw::c_int) * (w + 1 as std::os::raw::c_int)) as std::os::raw::c_ulong,
        ::core::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_int;
    m = malloc(
        ((n + 1 as std::os::raw::c_int) as std::os::raw::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut *mut std::os::raw::c_int;
    let ref mut fresh0 = *m.offset(0 as std::os::raw::c_int as isize);
    *fresh0 = mm;
    i = 1 as std::os::raw::c_int;
    while i <= n {
        let ref mut fresh1 = *m.offset(i as isize);
        *fresh1 = &mut *mm.offset((i * (w + 1 as std::os::raw::c_int)) as isize)
            as *mut std::os::raw::c_int;
        j = 0 as std::os::raw::c_int;
        while j <= w {
            if (*items_0.offset((i - 1 as std::os::raw::c_int) as isize)).weight > j {
                *(*m.offset(i as isize))
                    .offset(
                        j as isize,
                    ) = *(*m.offset((i - 1 as std::os::raw::c_int) as isize)).offset(j as isize);
            } else {
                a = *(*m.offset((i - 1 as std::os::raw::c_int) as isize)).offset(j as isize);
                b = *(*m.offset((i - 1 as std::os::raw::c_int) as isize))
                    .offset(
                        (j - (*items_0.offset((i - 1 as std::os::raw::c_int) as isize)).weight)
                            as isize,
                    ) + (*items_0.offset((i - 1 as std::os::raw::c_int) as isize)).value;
                *(*m.offset(i as isize)).offset(j as isize) = if a > b { a } else { b };
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    s = calloc(
        n as std::os::raw::c_ulong,
        ::core::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_int;
    i = n;
    j = w;
    while i > 0 as std::os::raw::c_int {
        if *(*m.offset(i as isize)).offset(j as isize)
            > *(*m.offset((i - 1 as std::os::raw::c_int) as isize)).offset(j as isize)
        {
            *s.offset((i - 1 as std::os::raw::c_int) as isize) = 1 as std::os::raw::c_int;
            j -= (*items_0.offset((i - 1 as std::os::raw::c_int) as isize)).weight;
        }
        i -= 1;
        i;
    }
    free(mm as *mut std::os::raw::c_void);
    free(m as *mut std::os::raw::c_void);
    return s;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0;
    let mut tw: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut tv: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut s: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    n = (::core::mem::size_of::<[item_t; 22]>() as std::os::raw::c_ulong)
        .wrapping_div(::core::mem::size_of::<item_t>() as std::os::raw::c_ulong) as std::os::raw::c_int;
    s = knapsack(items.as_mut_ptr(), n, 400 as std::os::raw::c_int);
    i = 0 as std::os::raw::c_int;
    while i < n {
        if *s.offset(i as isize) != 0 {
            printf(
                b"%-22s %5d %5d\n\0" as *const u8 as *const std::os::raw::c_char,
                items[i as usize].name,
                items[i as usize].weight,
                items[i as usize].value,
            );
            tw += items[i as usize].weight;
            tv += items[i as usize].value;
        }
        i += 1;
        i;
    }
    printf(
        b"%-22s %5d %5d\n\0" as *const u8 as *const std::os::raw::c_char,
        b"totals:\0" as *const u8 as *const std::os::raw::c_char,
        tw,
        tv,
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
