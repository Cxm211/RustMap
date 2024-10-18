#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn perror(__s: *const std::os::raw::c_char);
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn memcpy(
        _: *mut u8,
        _: *const u8,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
    fn exit(_: std::os::raw::c_int) -> !;
    fn mmap(
        __addr: *mut u8,
        __len: size_t,
        __prot: std::os::raw::c_int,
        __flags: std::os::raw::c_int,
        __fd: std::os::raw::c_int,
        __offset: __off_t,
    ) -> *mut u8;
}
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type f_int = Option::<unsafe extern "C" fn() -> std::os::raw::c_int>;
#[no_mangle]
pub extern "C" fn _tmpl() -> std::os::raw::c_int {
    let mut x: std::os::raw::c_int = 0xdeadbeef as std::os::raw::c_uint as std::os::raw::c_int;
    return x * x;
}
#[no_mangle]
pub unsafe extern "C" fn dupf(mut v: std::os::raw::c_int) -> f_int {
    let mut len: size_t = (std::mem::transmute::<
        Option::<unsafe extern "C" fn(std::os::raw::c_int) -> f_int>,
        *mut u8,
    >(Some(dupf as unsafe extern "C" fn(std::os::raw::c_int) -> f_int)))
        .offset_from(
            std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> std::os::raw::c_int>,
                *mut u8,
            >(
                Some(
                    std::mem::transmute::<
                        unsafe extern "C" fn() -> std::os::raw::c_int,
                        unsafe extern "C" fn() -> std::os::raw::c_int,
                    >(_tmpl),
                ),
            ),
        ) as std::os::raw::c_long as size_t;
    let mut ret: f_int = std::mem::transmute::<
        *mut u8,
        f_int,
    >(
        mmap(
            0 as *mut u8,
            len,
            0x4 as std::os::raw::c_int | 0x2 as std::os::raw::c_int,
            0x2 as std::os::raw::c_int | 0x20 as std::os::raw::c_int,
            0 as std::os::raw::c_int,
            0 as std::os::raw::c_int as __off_t,
        ),
    );
    let mut p: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    if ret
        == std::mem::transmute::<
            *mut u8,
            f_int,
        >(-(1 as std::os::raw::c_int) as *mut u8)
    {
        perror(b"mmap\0" as *const u8 as *const std::os::raw::c_char);
        exit(-(1 as std::os::raw::c_int));
    }
    memcpy(
        std::mem::transmute::<f_int, *mut u8>(ret),
        std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> std::os::raw::c_int>,
            *const u8,
        >(
            Some(
                std::mem::transmute::<
                    unsafe extern "C" fn() -> std::os::raw::c_int,
                    unsafe extern "C" fn() -> std::os::raw::c_int,
                >(_tmpl),
            ),
        ),
        len,
    );
    p = std::mem::transmute::<f_int, *mut std::os::raw::c_char>(ret);
    while p
        < (std::mem::transmute::<f_int, *mut std::os::raw::c_char>(ret))
            .offset(len as isize)
            .offset(-(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong as isize))
    {
        if *(p as *mut std::os::raw::c_int) as std::os::raw::c_uint == 0xdeadbeef as std::os::raw::c_uint {
            *(p as *mut std::os::raw::c_int) = v;
        }
        p = p.offset(1);
        p;
    }
    return ret;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut funcs: [f_int; 10] = [None; 10];
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 10 as std::os::raw::c_int {
        funcs[i as usize] = dupf(i);
        i += 1;
        i;
    }
    i = 0 as std::os::raw::c_int;
    while i < 9 as std::os::raw::c_int {
        printf(
            b"func[%d]: %d\n\0" as *const u8 as *const std::os::raw::c_char,
            i,
            std::mem::transmute::<
                _,
                fn() -> std::os::raw::c_int,
            >((funcs[i as usize]).expect("non-null function pointer"))(),
        );
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
