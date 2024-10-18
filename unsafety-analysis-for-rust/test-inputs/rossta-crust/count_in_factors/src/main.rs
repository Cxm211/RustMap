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
//use c2rust_out::*;
extern "C" {
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
}
#[no_mangle]
pub extern "C" fn get_prime(mut idx: i32) -> u64 {
    unsafe {
        static mut n_primes: i64 = 0;
        static mut alloc: i64 = 0;
        static mut primes: *mut u64 = 0 as *const u64 as *mut u64;
        let mut last: u64 = 0;
        let mut p: u64 = 0;
        let mut i: i32 = 0;
        if idx as i64 >= n_primes {
            if n_primes >= alloc {
                alloc += 16;
                primes = realloc(
                    primes as *mut libc::c_void,
                    (::core::mem::size_of::<u64>() as u64).wrapping_mul(alloc as u64),
                ) as *mut u64;
            }
            if n_primes == 0 {
                *primes.offset(0 as isize) = 2;
                *primes.offset(1 as isize) = 3;
                n_primes = 2;
            }
            last = *primes.offset((n_primes - 1i64) as isize);
            while idx as i64 >= n_primes {
                last = (last).wrapping_add(2) as u64;
                i = 0;
                while (i as i64) < n_primes {
                    p = *primes.offset(i as isize);
                    if p.wrapping_mul(p) > last {
                        let fresh0 = n_primes;
                        n_primes = n_primes + 1;
                        *primes.offset(fresh0 as isize) = last;
                        break;
                    } else {
                        if last.wrapping_rem(p) == 0 {
                            break;
                        }
                        i += 1;
                        i;
                    }
                }
            }
        }
        return *primes.offset(idx as isize);
    }
}

fn main_0() -> i32 {
    let mut n: u64 = 0;
    let mut x: u64 = 0;
    let mut p: u64 = 0;
    let mut i: i32 = 0;
    let mut first: i32 = 0;
    x = 1;
    while x < 1000 {
        n = x;
        print!("{} = ", n);
        i = 0;
        first = 1;
        loop {
            p = get_prime(i);
            while n.wrapping_rem(p) == 0 {
                n = (n).wrapping_div(p) as u64;
                if first == 0 {
                    print!(" x ");
                }
                first = 0;
                print!("{}", p);
            }
            if n <= p.wrapping_mul(p) {
                break;
            }
            i += 1;
            i;
        }
        if first != 0 {
            print!("{}\n", n);
        } else if n > 1 {
            print!(" x {}\n", n);
        } else {
            print!("\n");
        }
        x = x.wrapping_add(1);
        x;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
