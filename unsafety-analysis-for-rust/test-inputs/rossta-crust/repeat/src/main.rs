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
extern "C" {}
#[no_mangle]
pub extern "C" fn repeat(mut f: Option<unsafe extern "C" fn() -> ()>, mut n: u32) {
    unsafe {
        loop {
            let fresh0 = n;
            n = n.wrapping_sub(1);
            if !(fresh0 > 0) {
                break;
            };
            (Some(f.expect("non-null function pointer"))).expect("non-null function pointer")();
        }
    }
}

#[no_mangle]
pub extern "C" fn example() {
    print!("Example\n");
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        repeat(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(::core::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(example))),
            4,
        );
        return 0;
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32);
}
