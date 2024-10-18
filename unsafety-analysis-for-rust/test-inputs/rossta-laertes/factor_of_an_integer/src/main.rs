#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn realloc(_: *mut u8, _: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Factors {
    pub list: * mut std::os::raw::c_int,
    pub count: std::os::raw::c_short,
}
impl std::default::Default for Factors {
    fn default() -> Self {
        Factors {
        list: 0 as * mut std::os::raw::c_int,
        count: std::os::raw::c_short::default()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn xferFactors<'a1>(
    mut fctrs: Option<&'a1 mut crate::Factors>,
    mut flist: * mut std::os::raw::c_int,
    mut flix: std::os::raw::c_int,
) {
    let mut ix: std::os::raw::c_int = 0;
    let mut ij: std::os::raw::c_int = 0;
    let mut newSize: std::os::raw::c_int = (*borrow(& fctrs).unwrap()).count as std::os::raw::c_int + flix;
    if newSize > flix {
        (*borrow_mut(&mut fctrs).unwrap())
            .list = realloc(
            (*borrow_mut(&mut fctrs).unwrap()).list as *mut u8,
            (newSize as std::os::raw::c_ulong)
                .wrapping_mul(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
        ) as *mut std::os::raw::c_int;
    } else {
        (*borrow_mut(&mut fctrs).unwrap())
            .list = malloc(
            (newSize as std::os::raw::c_ulong)
                .wrapping_mul(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
        ) as *mut std::os::raw::c_int;
    }
    ij = 0 as std::os::raw::c_int;
    ix = (*borrow_mut(&mut fctrs).unwrap()).count as std::os::raw::c_int;
    while ix < newSize {
        *((*borrow_mut(&mut fctrs).unwrap()).list).offset(ix as isize) = *flist.offset(ij as isize);
        ij += 1;
        ij;
        ix += 1;
        ix;
    }
    (*borrow_mut(&mut fctrs).unwrap()).count = newSize as std::os::raw::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn factor<'a1, 'a2>(
    mut num: std::os::raw::c_int,
    mut fctrs: Option<&'a1 mut crate::Factors>,
) -> Option<&'a2 mut crate::Factors> {
    let mut flist: [std::os::raw::c_int; 301] = [0; 301];
    let mut flix: std::os::raw::c_int = 0;
    let mut dvsr: std::os::raw::c_int = 0;
    flix = 0 as std::os::raw::c_int;
    (*borrow_mut(&mut fctrs).unwrap()).count = 0 as std::os::raw::c_int as std::os::raw::c_short;
    free((*borrow_mut(&mut fctrs).unwrap()).list as *mut u8);
    (*borrow_mut(&mut fctrs).unwrap()).list = 0 as *mut std::os::raw::c_int;
    dvsr = 1 as std::os::raw::c_int;
    while dvsr * dvsr < num {
        if !(num % dvsr != 0 as std::os::raw::c_int) {
            if flix == 300 as std::os::raw::c_int {
                xferFactors(borrow_mut(&mut fctrs), flist.as_mut_ptr(), flix);
                flix = 0 as std::os::raw::c_int;
            }
            let fresh0 = flix;
            flix = flix + 1;
            flist[fresh0 as usize] = dvsr;
            let fresh1 = flix;
            flix = flix + 1;
            flist[fresh1 as usize] = num / dvsr;
        }
        dvsr += 1;
        dvsr;
    }
    if dvsr * dvsr == num {
        let fresh2 = flix;
        flix = flix + 1;
        flist[fresh2 as usize] = dvsr;
    }
    if flix > 0 as std::os::raw::c_int {
        xferFactors(borrow_mut(&mut fctrs), flist.as_mut_ptr(), flix);
    }
    return fctrs;
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut nums2factor: [std::os::raw::c_int; 4] = [
        2059 as std::os::raw::c_int,
        223092870 as std::os::raw::c_int,
        3135 as std::os::raw::c_int,
        45 as std::os::raw::c_int,
    ];
    let mut ftors: Factors = {
        let mut init = Factors {
            list: 0 as *mut std::os::raw::c_int,
            count: 0 as std::os::raw::c_int as std::os::raw::c_short,
        };
        init
    };
    let mut sep: std::os::raw::c_char = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 4 as std::os::raw::c_int {
        factor(nums2factor[i as usize], Some(&mut ftors));
        printf(
            b"\nfactors of %d are:\n  \0" as *const u8 as *const std::os::raw::c_char,
            nums2factor[i as usize],
        );
        sep = ' ' as i32 as std::os::raw::c_char;
        j = 0 as std::os::raw::c_int;
        while j < ftors.count as std::os::raw::c_int {
            printf(
                b"%c %d\0" as *const u8 as *const std::os::raw::c_char,
                sep as std::os::raw::c_int,
                *(ftors.list).offset(j as isize),
            );
            sep = ',' as i32 as std::os::raw::c_char;
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as std::os::raw::c_int,
                args.as_mut_ptr() as *mut *mut std::os::raw::c_char,
            ) as i32,
        )
    }
}
