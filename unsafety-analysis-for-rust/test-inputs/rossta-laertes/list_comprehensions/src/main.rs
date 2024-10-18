#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
// //use ::c2rust_out::*;
extern "C" {
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn memcpy(
        _: *mut u8,
        _: *const u8,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct List {
    pub nx: *mut List,
    pub val: [std::os::raw::c_char; 0],
}
impl std::default::Default for List {
    fn default() -> Self {
        List {
        nx: core::ptr::null_mut(),
        val: [std::os::raw::c_char::default(); 0]
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Triple {
    pub _1: std::os::raw::c_int,
    pub _2: std::os::raw::c_int,
    pub _3: std::os::raw::c_int,
}
impl std::default::Default for Triple {
    fn default() -> Self {
        Triple {
        _1: std::os::raw::c_int::default(),
        _2: std::os::raw::c_int::default(),
        _3: std::os::raw::c_int::default()
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ITERATOR {
    pub l: * mut crate::List,
    pub old: * mut crate::List,
    pub p: * mut crate::ITERATOR,
}
impl std::default::Default for ITERATOR {
    fn default() -> Self {
        ITERATOR {
        l: core::ptr::null_mut(),
        old: core::ptr::null_mut(),
        p: core::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub static mut FE_var: *mut ITERATOR = 0 as *const ITERATOR as *mut ITERATOR;
#[no_mangle]
pub static mut SEQ_base: ITERATOR = ITERATOR {
    l: 0 as *const List as *mut List,
    old: 0 as *const List as *mut List,
    p: 0 as *const ITERATOR as *mut ITERATOR,
};
#[no_mangle]
pub static mut SEQ_var: *mut ITERATOR = unsafe {
    &SEQ_base as *const ITERATOR as *mut ITERATOR
};
#[no_mangle]
pub unsafe extern "C" fn listNew(
    mut sz: std::os::raw::c_int,
    mut val: *mut u8,
) -> *mut List {
    let mut l: *mut List = malloc(
        (std::mem::size_of::<List>() as std::os::raw::c_ulong)
            .wrapping_add(sz as std::os::raw::c_ulong),
    ) as *mut List;
    (*l).nx = 0 as *mut List;
    memcpy(((*l).val).as_mut_ptr() as *mut u8, val, sz as std::os::raw::c_ulong);
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn listAppend(
    mut l: *mut List,
    mut sz: std::os::raw::c_int,
    mut val: *mut u8,
) -> *mut List {
    while !((*l).nx).is_null() {
        l = (*l).nx;
    }
    (*l).nx = listNew(sz, val);
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn intRangeList(
    mut f: std::os::raw::c_int,
    mut t: std::os::raw::c_int,
) -> *mut List {
    let mut l: *mut List = listNew(
        std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong as std::os::raw::c_int,
        &mut f as *mut std::os::raw::c_int as *mut u8,
    );
    let mut e: *mut List = l;
    let mut i: std::os::raw::c_int = f + 1 as std::os::raw::c_int;
    while i <= t {
        (*e)
            .nx = listNew(
            std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong as std::os::raw::c_int,
            &mut i as *mut std::os::raw::c_int as *mut u8,
        );
        e = (*e).nx;
        i += 1;
        i;
    }
    return l;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut x: std::os::raw::c_int = 0;
    let mut y: std::os::raw::c_int = 0;
    let mut z: std::os::raw::c_int = 0;
    let n: std::os::raw::c_int = 20 as std::os::raw::c_int;
    SEQ_var = &mut {
        let mut init = ITERATOR {
            l: 0 as *mut List,
            old: 0 as *mut List,
            p: SEQ_var,
        };
        init
    } as *mut ITERATOR;
    FE_var = &mut {
        let mut init = ITERATOR {
            l: (intRangeList
                as unsafe extern "C" fn(
                    std::os::raw::c_int,
                    std::os::raw::c_int,
                ) -> *mut List)(1 as std::os::raw::c_int, n),
            old: 0 as *mut List,
            p: FE_var,
        };
        init
    } as *mut ITERATOR;
    std::ptr::write_volatile(
        &mut x as *mut std::os::raw::c_int,
        *(&mut (*(*FE_var).l).val as *mut [std::os::raw::c_char; 0] as *mut std::os::raw::c_int),
    );
    while (if !((*FE_var).l).is_null() {
        std::ptr::write_volatile(
            &mut x as *mut std::os::raw::c_int,
            *(&mut (*(*FE_var).l).val as *mut [std::os::raw::c_char; 0] as *mut std::os::raw::c_int),
        );
        1 as std::os::raw::c_int
    } else {
        0 as std::os::raw::c_int
    }) != 0
    {
        FE_var = &mut {
            let mut init = ITERATOR {
                l: (intRangeList
                    as unsafe extern "C" fn(
                        std::os::raw::c_int,
                        std::os::raw::c_int,
                    ) -> *mut List)(x, n),
                old: 0 as *mut List,
                p: FE_var,
            };
            init
        } as *mut ITERATOR;
        std::ptr::write_volatile(
            &mut y as *mut std::os::raw::c_int,
            *(&mut (*(*FE_var).l).val as *mut [std::os::raw::c_char; 0] as *mut std::os::raw::c_int),
        );
        while (if !((*FE_var).l).is_null() {
            std::ptr::write_volatile(
                &mut y as *mut std::os::raw::c_int,
                *(&mut (*(*FE_var).l).val as *mut [std::os::raw::c_char; 0] as *mut std::os::raw::c_int),
            );
            1 as std::os::raw::c_int
        } else {
            0 as std::os::raw::c_int
        }) != 0
        {
            FE_var = &mut {
                let mut init = ITERATOR {
                    l: (intRangeList
                        as unsafe extern "C" fn(
                            std::os::raw::c_int,
                            std::os::raw::c_int,
                        ) -> *mut List)(y, n),
                    old: 0 as *mut List,
                    p: FE_var,
                };
                init
            } as *mut ITERATOR;
            std::ptr::write_volatile(
                &mut z as *mut std::os::raw::c_int,
                *(&mut (*(*FE_var).l).val as *mut [std::os::raw::c_char; 0] as *mut std::os::raw::c_int),
            );
            while (if !((*FE_var).l).is_null() {
                std::ptr::write_volatile(
                    &mut z as *mut std::os::raw::c_int,
                    *(&mut (*(*FE_var).l).val as *mut [std::os::raw::c_char; 0]
                        as *mut std::os::raw::c_int),
                );
                1 as std::os::raw::c_int
            } else {
                0 as std::os::raw::c_int
            }) != 0
            {
                if x * x + y * y == z * z {
                    if !((*SEQ_var).l).is_null() {
                        listAppend(
                            (*SEQ_var).l,
                            std::mem::size_of::<Triple>() as std::os::raw::c_ulong
                                as std::os::raw::c_int,
                            &mut {
                                let mut init = Triple { _1: x, _2: y, _3: z };
                                init
                            } as *mut Triple as *mut u8,
                        );
                    } else {
                        (*SEQ_var)
                            .l = listNew(
                            std::mem::size_of::<Triple>() as std::os::raw::c_ulong
                                as std::os::raw::c_int,
                            &mut {
                                let mut init = Triple { _1: x, _2: y, _3: z };
                                init
                            } as *mut Triple as *mut u8,
                        );
                    };
                } else {};
                (*FE_var).l = (*(*FE_var).l).nx;
            }
            FE_var = (*FE_var).p;
            (*FE_var).l = (*(*FE_var).l).nx;
        }
        FE_var = (*FE_var).p;
        (*FE_var).l = (*(*FE_var).l).nx;
    }
    FE_var = (*FE_var).p;
    (*(*SEQ_var).p).old = (*SEQ_var).l;
    SEQ_var = (*SEQ_var).p;
    let mut pTriples: *mut List = (*SEQ_var).old;
    let mut t: Triple = Triple { _1: 0, _2: 0, _3: 0 };
    FE_var = &mut {
        let mut init = ITERATOR {
            l: pTriples,
            old: 0 as *mut List,
            p: FE_var,
        };
        init
    } as *mut ITERATOR;
    std::ptr::write_volatile(
        &mut t as *mut Triple,
        *(&mut (*(*FE_var).l).val as *mut [std::os::raw::c_char; 0] as *mut Triple),
    );
    while (if !((*FE_var).l).is_null() {
        std::ptr::write_volatile(
            &mut t as *mut Triple,
            *(&mut (*(*FE_var).l).val as *mut [std::os::raw::c_char; 0] as *mut Triple),
        );
        1 as std::os::raw::c_int
    } else {
        0 as std::os::raw::c_int
    }) != 0
    {
        printf(b"%d, %d, %d\n\0" as *const u8 as *const std::os::raw::c_char, t._1, t._2, t._3);
        (*FE_var).l = (*(*FE_var).l).nx;
    }
    FE_var = (*FE_var).p;
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
