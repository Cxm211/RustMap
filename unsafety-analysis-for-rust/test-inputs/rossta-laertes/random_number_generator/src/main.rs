#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
}
pub type ull = std::os::raw::c_ulonglong;
#[no_mangle]
pub unsafe extern "C" fn evolve(mut state: ull, mut rule: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut p: std::os::raw::c_int = 0;
    let mut q: std::os::raw::c_int = 0;
    let mut b: std::os::raw::c_int = 0;
    p = 0 as std::os::raw::c_int;
    while p < 10 as std::os::raw::c_int {
        b = 0 as std::os::raw::c_int;
        q = 8 as std::os::raw::c_int;
        loop {
            let fresh0 = q;
            q = q - 1;
            if !(fresh0 != 0) {
                break;
            }
            let mut st: ull = state;
            b = (b as std::os::raw::c_ulonglong
                | (st & 1 as std::os::raw::c_int as std::os::raw::c_ulonglong) << q) as std::os::raw::c_int;
            i = 0 as std::os::raw::c_int;
            state = i as ull;
            while (i as std::os::raw::c_ulong)
                < (std::mem::size_of::<ull>() as std::os::raw::c_ulong)
                    .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
            {
                if rule as std::os::raw::c_ulonglong
                    & (1 as std::os::raw::c_ulonglong)
                        << (7 as std::os::raw::c_int as std::os::raw::c_ulonglong
                            & (st >> i - 1 as std::os::raw::c_int
                                | st
                                    << (std::mem::size_of::<ull>() as std::os::raw::c_ulong)
                                        .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                        .wrapping_sub(i as std::os::raw::c_ulong))) != 0
                {
                    state |= (1 as std::os::raw::c_ulonglong) << i;
                }
                i += 1;
                i;
            }
        }
        printf(b" %d\0" as *const u8 as *const std::os::raw::c_char, b);
        p += 1;
        p;
    }
    putchar('\n' as i32);
}
unsafe fn main_0() -> std::os::raw::c_int {
    evolve(1 as std::os::raw::c_int as ull, 30 as std::os::raw::c_int);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
