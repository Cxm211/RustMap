#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut police: std::os::raw::c_int = 0;
    let mut sanitation: std::os::raw::c_int = 0;
    let mut fire: std::os::raw::c_int = 0;
    printf(
        b"Police     Sanitation         Fire\n\0" as *const u8 as *const std::os::raw::c_char,
    );
    printf(b"----------------------------------\0" as *const u8 as *const std::os::raw::c_char);
    police = 2 as std::os::raw::c_int;
    while police <= 6 as std::os::raw::c_int {
        sanitation = 1 as std::os::raw::c_int;
        while sanitation <= 7 as std::os::raw::c_int {
            fire = 1 as std::os::raw::c_int;
            while fire <= 7 as std::os::raw::c_int {
                if police != sanitation && sanitation != fire && fire != police
                    && police + fire + sanitation == 12 as std::os::raw::c_int
                {
                    printf(
                        b"\n%d\t\t%d\t\t%d\0" as *const u8 as *const std::os::raw::c_char,
                        police,
                        sanitation,
                        fire,
                    );
                }
                fire += 1;
                fire;
            }
            sanitation += 1;
            sanitation;
        }
        police += 2 as std::os::raw::c_int;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
