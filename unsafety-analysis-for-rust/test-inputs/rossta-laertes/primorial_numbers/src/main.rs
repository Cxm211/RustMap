#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn log(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
}
pub type __uint32_t = std::os::raw::c_uint;
pub type __uint64_t = std::os::raw::c_ulong;
pub type uint32_t = std::os::raw::c_uint;
pub type uint64_t = std::os::raw::c_ulong;
pub type size_t = std::os::raw::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn es_check(
    mut sieve: *mut uint32_t,
    mut n: uint64_t,
) -> std::os::raw::c_int {
    if n != 2 as std::os::raw::c_int as std::os::raw::c_ulong
        && n & 1 as std::os::raw::c_int as std::os::raw::c_ulong == 0
        || n < 2 as std::os::raw::c_int as std::os::raw::c_ulong
    {
        return 0 as std::os::raw::c_int
    } else {
        return (*sieve.offset((n >> 6 as std::os::raw::c_int) as isize)
            & ((1 as std::os::raw::c_int)
                << (n >> 1 as std::os::raw::c_int & 31 as std::os::raw::c_int as std::os::raw::c_ulong))
                as std::os::raw::c_uint == 0) as std::os::raw::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn es_sieve<'a1>(
    nth: std::os::raw::c_ulong,
    mut es_size: Option<&'a1 mut std::os::raw::c_ulong>,
) -> * mut std::os::raw::c_uint {
    *borrow_mut(&mut es_size).unwrap() = (nth as std::os::raw::c_double * log(nth as std::os::raw::c_double)
        + nth as std::os::raw::c_double
            * (log(log(nth as std::os::raw::c_double)) - 0.9385f32 as std::os::raw::c_double)
        + 1 as std::os::raw::c_int as std::os::raw::c_double) as uint64_t;
    let mut sieve: *mut uint32_t = calloc(
        (*borrow(& es_size).unwrap() >> 6 as std::os::raw::c_int).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
        std::mem::size_of::<uint32_t>() as std::os::raw::c_ulong,
    ) as *mut uint32_t;
    let mut i: uint64_t = 3 as std::os::raw::c_int as uint64_t;
    while (i as std::os::raw::c_double)
        < sqrt(*borrow_mut(&mut es_size).unwrap() as std::os::raw::c_double) + 1 as std::os::raw::c_int as std::os::raw::c_double
    {
        if *sieve.offset((i >> 6 as std::os::raw::c_int) as isize)
            & ((1 as std::os::raw::c_int)
                << (i >> 1 as std::os::raw::c_int & 31 as std::os::raw::c_int as std::os::raw::c_ulong))
                as std::os::raw::c_uint == 0
        {
            let mut j: uint64_t = i.wrapping_mul(i);
            while j < *borrow(& es_size).unwrap() {
                let ref mut fresh0 = *sieve.offset((j >> 6 as std::os::raw::c_int) as isize);
                *fresh0
                    |= ((1 as std::os::raw::c_int)
                        << (j >> 1 as std::os::raw::c_int & 31 as std::os::raw::c_int as std::os::raw::c_ulong))
                        as std::os::raw::c_uint;
                j = (j as std::os::raw::c_ulong).wrapping_add(i << 1 as std::os::raw::c_int) as uint64_t
                    as uint64_t;
            }
        }
        i = (i as std::os::raw::c_ulong).wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
            as uint64_t as uint64_t;
    }
    return sieve;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_number_of_digits(op: std::os::raw::c_int) -> size_t {
    let mut opstr: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let oplen: size_t = strlen(opstr);
    free(opstr as *mut u8);
    return oplen;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut sieve_size: uint64_t = 0;
    let mut sieve: *mut uint32_t = es_sieve(
        100000 as std::os::raw::c_int as uint64_t,
        Some(&mut sieve_size),
    );
    let mut prime_count: uint64_t = 0 as std::os::raw::c_int as uint64_t;
    let mut print: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut unused: std::os::raw::c_double = 0.;
    let mut i: uint64_t = 2 as std::os::raw::c_int as uint64_t;
    while i < sieve_size && prime_count <= 100000 as std::os::raw::c_int as std::os::raw::c_ulong {
        if print != 0 {
            print = 0 as std::os::raw::c_int;
        }
        if es_check(sieve, i) != 0 {
            prime_count = prime_count.wrapping_add(1);
            prime_count;
            print = 1 as std::os::raw::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    free(sieve as *mut u8);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
