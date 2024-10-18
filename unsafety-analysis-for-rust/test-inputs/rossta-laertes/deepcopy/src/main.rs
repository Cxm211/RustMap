#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layer1 {
    pub a: std::os::raw::c_int,
}
impl std::default::Default for layer1 {
    fn default() -> Self {
        layer1 {
        a: std::os::raw::c_int::default()
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct layer2 {
    pub l1: crate::layer1,
    pub b: std::os::raw::c_float,
    pub c: std::os::raw::c_float,
}
impl std::default::Default for layer2 {
    fn default() -> Self {
        layer2 {
        l1: crate::layer1::default(),
        b: std::os::raw::c_float::default(),
        c: std::os::raw::c_float::default()
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct layer3 {
    pub l2: crate::layer2,
    pub l1: crate::layer1,
    pub d: std::os::raw::c_int,
    pub e: std::os::raw::c_int,
}
impl std::default::Default for layer3 {
    fn default() -> Self {
        layer3 {
        l2: crate::layer2::default(),
        l1: crate::layer1::default(),
        d: std::os::raw::c_int::default(),
        e: std::os::raw::c_int::default()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn showCake(mut cake: layer3) {
    printf(b"\ncake.d = %d\0" as *const u8 as *const std::os::raw::c_char, cake.d);
    printf(b"\ncake.e = %d\0" as *const u8 as *const std::os::raw::c_char, cake.e);
    printf(b"\ncake.l1.a = %d\0" as *const u8 as *const std::os::raw::c_char, cake.l1.a);
    printf(
        b"\ncake.l2.b = %f\0" as *const u8 as *const std::os::raw::c_char,
        cake.l2.b as std::os::raw::c_double,
    );
    printf(b"\ncake.l2.l1.a = %d\0" as *const u8 as *const std::os::raw::c_char, cake.l2.l1.a);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut cake1: layer3 = layer3 {
        l2: layer2 {
            l1: layer1 { a: 0 },
            b: 0.,
            c: 0.,
        },
        l1: layer1 { a: 0 },
        d: 0,
        e: 0,
    };
    let mut cake2: layer3 = layer3 {
        l2: layer2 {
            l1: layer1 { a: 0 },
            b: 0.,
            c: 0.,
        },
        l1: layer1 { a: 0 },
        d: 0,
        e: 0,
    };
    cake1.d = 1 as std::os::raw::c_int;
    cake1.e = 2 as std::os::raw::c_int;
    cake1.l1.a = 3 as std::os::raw::c_int;
    cake1.l2.b = 4 as std::os::raw::c_int as std::os::raw::c_float;
    cake1.l2.l1.a = 5 as std::os::raw::c_int;
    printf(b"Cake 1 is : \0" as *const u8 as *const std::os::raw::c_char);
    showCake(cake1);
    cake2 = cake1;
    cake2.l2.b += cake2.l2.l1.a as std::os::raw::c_float;
    printf(b"\nCake 2 is : \0" as *const u8 as *const std::os::raw::c_char);
    showCake(cake2);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
