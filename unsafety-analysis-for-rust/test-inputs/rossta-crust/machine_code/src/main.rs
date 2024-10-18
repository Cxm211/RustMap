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
    fn mmap(
        __addr: *mut libc::c_void,
        __len: u64,
        __prot: i32,
        __flags: i32,
        __fd: i32,
        __offset: i64,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: u64) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
#[no_mangle]
pub extern "C" fn test(mut a: i32, mut b: i32) -> i32 {
    unsafe {
        let mut code: [i8; 9] = [0x8b, 0x44, 0x24, 0x4, 0x3, 0x44, 0x24, 0x8, 0xc3];
        let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut c: i32 = 0;
        buf = mmap(
            0 as *mut libc::c_void,
            ::core::mem::size_of::<[i8; 9]>() as u64,
            0x1 | 0x2 | 0x4,
            0x2 | 0x20,
            -1,
            0,
        );
        memcpy(
            buf,
            code.as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[i8; 9]>() as u64,
        );
        c = (::core::mem::transmute::<
            *mut libc::c_void,
            Option<unsafe extern "C" fn(i32, i32) -> i32>,
        >(buf))
        .expect("non-null function pointer")(a, b);
        munmap(buf, ::core::mem::size_of::<[i8; 9]>() as u64);
        return c;
    }
}

fn main_0() -> i32 {
    print!("{}\n", test(7, 12));
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
