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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct item_t {
    pub name: *mut i8,
    pub weight: i32,
    pub value: i32,
}
#[no_mangle]
pub static mut items: [item_t; 22] = [
    {
        let mut init = item_t {
            name: b"map\0" as *const u8 as *const i8 as *mut i8,
            weight: 9,
            value: 150,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"compass\0" as *const u8 as *const i8 as *mut i8,
            weight: 13,
            value: 35,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"water\0" as *const u8 as *const i8 as *mut i8,
            weight: 153,
            value: 200,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"sandwich\0" as *const u8 as *const i8 as *mut i8,
            weight: 50,
            value: 160,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"glucose\0" as *const u8 as *const i8 as *mut i8,
            weight: 15,
            value: 60,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"tin\0" as *const u8 as *const i8 as *mut i8,
            weight: 68,
            value: 45,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"banana\0" as *const u8 as *const i8 as *mut i8,
            weight: 27,
            value: 60,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"apple\0" as *const u8 as *const i8 as *mut i8,
            weight: 39,
            value: 40,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"cheese\0" as *const u8 as *const i8 as *mut i8,
            weight: 23,
            value: 30,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"beer\0" as *const u8 as *const i8 as *mut i8,
            weight: 52,
            value: 10,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"suntan cream\0" as *const u8 as *const i8 as *mut i8,
            weight: 11,
            value: 70,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"camera\0" as *const u8 as *const i8 as *mut i8,
            weight: 32,
            value: 30,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"T-shirt\0" as *const u8 as *const i8 as *mut i8,
            weight: 24,
            value: 15,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"trousers\0" as *const u8 as *const i8 as *mut i8,
            weight: 48,
            value: 10,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"umbrella\0" as *const u8 as *const i8 as *mut i8,
            weight: 73,
            value: 40,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"waterproof trousers\0" as *const u8 as *const i8 as *mut i8,
            weight: 42,
            value: 70,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"waterproof overclothes\0" as *const u8 as *const i8 as *mut i8,
            weight: 43,
            value: 75,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"note-case\0" as *const u8 as *const i8 as *mut i8,
            weight: 22,
            value: 80,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"sunglasses\0" as *const u8 as *const i8 as *mut i8,
            weight: 7,
            value: 20,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"towel\0" as *const u8 as *const i8 as *mut i8,
            weight: 18,
            value: 12,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"socks\0" as *const u8 as *const i8 as *mut i8,
            weight: 4,
            value: 50,
        };
        init
    },
    {
        let mut init = item_t {
            name: b"book\0" as *const u8 as *const i8 as *mut i8,
            weight: 30,
            value: 10,
        };
        init
    },
];
#[no_mangle]
pub extern "C" fn knapsack(mut items_0: *mut item_t, mut n: i32, mut w: i32) -> *mut i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut mm: *mut i32 = 0 as *mut i32;
        let mut m: *mut *mut i32 = 0 as *mut *mut i32;
        let mut s: *mut i32 = 0 as *mut i32;
        mm = calloc(
            ((n + 1i32) * (w + 1)) as u64,
            ::core::mem::size_of::<i32>() as u64,
        ) as *mut i32;
        m = malloc(((n + 1i32) as u64).wrapping_mul(::core::mem::size_of::<*mut i32>() as u64))
            as *mut *mut i32;
        let ref mut fresh0 = *m.offset(0 as isize);
        *fresh0 = mm;
        i = 1;
        while i <= n {
            let ref mut fresh1 = *m.offset(i as isize);
            *fresh1 = &mut *mm.offset((i * (w + 1i32)) as isize) as *mut i32;
            j = 0;
            while j <= w {
                if (*items_0.offset((i - 1i32) as isize)).weight > j {
                    *(*m.offset(i as isize)).offset(j as isize) =
                        *(*m.offset((i - 1i32) as isize)).offset(j as isize);
                } else {
                    a = *(*m.offset((i - 1i32) as isize)).offset(j as isize);
                    b = *(*m.offset((i - 1i32) as isize))
                        .offset((j - (*items_0.offset((i - 1i32) as isize)).weight) as isize)
                        + (*items_0.offset((i - 1i32) as isize)).value;
                    *(*m.offset(i as isize)).offset(j as isize) = if a > b { a } else { b };
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        s = calloc(n as u64, ::core::mem::size_of::<i32>() as u64) as *mut i32;
        i = n;
        j = w;
        while i > 0 {
            if *(*m.offset(i as isize)).offset(j as isize)
                > *(*m.offset((i - 1i32) as isize)).offset(j as isize)
            {
                *s.offset((i - 1i32) as isize) = 1;
                j -= (*items_0.offset((i - 1i32) as isize)).weight;
            }
            i -= 1;
            i;
        }
        free(mm as *mut libc::c_void);
        free(m as *mut libc::c_void);
        return s;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut n: i32 = 0;
        let mut tw: i32 = 0;
        let mut tv: i32 = 0;
        let mut s: *mut i32 = 0 as *mut i32;
        n = (::core::mem::size_of::<[item_t; 22]>() as u64)
            .wrapping_div(::core::mem::size_of::<item_t>() as u64) as i32;
        s = knapsack(items.as_mut_ptr(), n, 400);
        i = 0;
        while i < n {
            if *s.offset(i as isize) != 0 {
                print!(
                    "{:-22} {:5} {:5}\n",
                    build_str_from_raw_ptr(items[i as usize].name as *mut u8),
                    items[i as usize].weight,
                    items[i as usize].value
                );
                tw += items[i as usize].weight;
                tv += items[i as usize].value;
            }
            i += 1;
            i;
        }
        print!("{:-22} {:5} {:5}\n", "totals:\0", tw, tv);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
