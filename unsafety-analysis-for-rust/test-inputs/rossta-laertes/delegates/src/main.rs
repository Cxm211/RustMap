#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
}
pub type Responder = Option<unsafe extern "C"  fn(_: std::os::raw::c_int,) -> * const std::os::raw::c_char>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sDelegate {
    pub operation: Option<unsafe extern "C"  fn(_: std::os::raw::c_int,) -> * const std::os::raw::c_char>,
}
impl std::default::Default for sDelegate {
    fn default() -> Self {
        sDelegate {
        operation: None
        }
    }
}

pub type Delegate = * mut crate::sDelegate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sDelegator {
    pub param: std::os::raw::c_int,
    pub phrase: * mut std::os::raw::c_char,
    pub delegate: * mut crate::sDelegate,
}
impl std::default::Default for sDelegator {
    fn default() -> Self {
        sDelegator {
        param: std::os::raw::c_int::default(),
        phrase: 0 as * mut std::os::raw::c_char,
        delegate: core::ptr::null_mut()
        }
    }
}

pub type Delegator = * mut crate::sDelegator;
#[no_mangle]
pub unsafe extern "C" fn NewDelegate(mut rspndr: Responder) -> Delegate {
    let mut dl: Delegate = malloc(std::mem::size_of::<sDelegate>() as std::os::raw::c_ulong)
        as Delegate;
    (*dl).operation = rspndr;
    return dl;
}
#[no_mangle]
pub unsafe extern "C" fn DelegateThing(
    mut dl: Delegate,
    mut p1: std::os::raw::c_int,
) -> *const std::os::raw::c_char {
    return if ((*dl).operation).is_some() {
        (Some(((*dl).operation).expect("non-null function pointer")))
            .expect("non-null function pointer")(p1)
    } else {
        0 as *const std::os::raw::c_char
    };
}
#[no_mangle]
pub extern "C" fn defaultResponse(mut p1: std::os::raw::c_int) -> *const std::os::raw::c_char {
    return b"default implementation\0" as *const u8 as *const std::os::raw::c_char;
}
static mut defaultDel: sDelegate = {
    let mut init = sDelegate {
        operation: Some(
            defaultResponse as unsafe extern "C" fn(std::os::raw::c_int) -> *const std::os::raw::c_char,
        ),
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn NewDelegator(
    mut p: std::os::raw::c_int,
    mut phrase: *mut std::os::raw::c_char,
) -> Delegator {
    let mut d: Delegator = malloc(std::mem::size_of::<sDelegator>() as std::os::raw::c_ulong)
        as Delegator;
    (*d).param = p;
    (*d).phrase = phrase;
    (*d).delegate = &mut defaultDel;
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn Delegator_Operation(
    mut theDelegator: Delegator,
    mut p1: std::os::raw::c_int,
    mut delroy: Delegate,
) -> *const std::os::raw::c_char {
    let mut rtn: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if !delroy.is_null() {
        rtn = DelegateThing(delroy, p1);
        if rtn.is_null() {
            rtn = DelegateThing((*theDelegator).delegate, p1);
        }
    } else {
        rtn = DelegateThing((*theDelegator).delegate, p1);
    }
    printf(b"%s\n\0" as *const u8 as *const std::os::raw::c_char, (*theDelegator).phrase);
    return rtn;
}
#[no_mangle]
pub unsafe extern "C" fn thing1(mut p1: std::os::raw::c_int) -> *const std::os::raw::c_char {
    printf(b"We're in thing1 with value %d\n\0" as *const u8 as *const std::os::raw::c_char, p1);
    return b"delegate implementation\0" as *const u8 as *const std::os::raw::c_char;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut del1: Delegate = NewDelegate(
        Some(thing1 as unsafe extern "C" fn(std::os::raw::c_int) -> *const std::os::raw::c_char),
    );
    let mut del2: Delegate = NewDelegate(None);
    let mut theDelegator: Delegator = NewDelegator(
        14 as std::os::raw::c_int,
        b"A stellar vista, Baby.\0" as *const u8 as *const std::os::raw::c_char
            as *mut std::os::raw::c_char,
    );
    printf(
        b"Delegator returns %s\n\n\0" as *const u8 as *const std::os::raw::c_char,
        Delegator_Operation(theDelegator, 3 as std::os::raw::c_int, core::ptr::null_mut()),
    );
    printf(
        b"Delegator returns %s\n\n\0" as *const u8 as *const std::os::raw::c_char,
        Delegator_Operation(theDelegator, 3 as std::os::raw::c_int, del1),
    );
    printf(
        b"Delegator returns %s\n\n\0" as *const u8 as *const std::os::raw::c_char,
        Delegator_Operation(theDelegator, 3 as std::os::raw::c_int, del2),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
