#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn sleep(__seconds: std::os::raw::c_uint) -> std::os::raw::c_uint;
    fn usleep(__useconds: __useconds_t) -> std::os::raw::c_int;
    fn atan2(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sin(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut u8) -> std::os::raw::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut u8) -> *mut u8,
        >,
        __arg: *mut u8,
    ) -> std::os::raw::c_int;
}
pub type __time_t = std::os::raw::c_long;
pub type __useconds_t = std::os::raw::c_uint;
pub type __suseconds_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
impl std::default::Default for timeval {
    fn default() -> Self {
        timeval {
        tv_sec: std::os::raw::c_long::default(),
        tv_usec: std::os::raw::c_long::default()
        }
    }
}

pub type pthread_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [std::os::raw::c_char; 56],
    pub __align: std::os::raw::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct integ_t {
    pub func: Option<unsafe extern "C"  fn(_: std::os::raw::c_double,) -> std::os::raw::c_double>,
    pub start: crate::timeval,
    pub v: std::os::raw::c_double,
    pub last_v: std::os::raw::c_double,
    pub last_t: std::os::raw::c_double,
    pub id: std::os::raw::c_ulong,
}
impl std::default::Default for integ_t {
    fn default() -> Self {
        integ_t {
        func: None,
        start: crate::timeval::default(),
        v: std::os::raw::c_double::default(),
        last_v: std::os::raw::c_double::default(),
        last_t: std::os::raw::c_double::default(),
        id: std::os::raw::c_ulong::default()
        }
    }
}

pub type integ = * mut crate::integ_t;
#[no_mangle]
pub unsafe extern "C" fn update(mut x: integ) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut t: std::os::raw::c_double = 0.;
    let mut v: std::os::raw::c_double = 0.;
    let mut f: Option::<unsafe extern "C" fn(std::os::raw::c_double) -> std::os::raw::c_double> = None;
    f = (*x).func;
    gettimeofday(&mut tv, 0 as *mut u8);
    t = ((tv.tv_sec - (*x).start.tv_sec) * 1000000 as std::os::raw::c_int as std::os::raw::c_long
        + tv.tv_usec - (*x).start.tv_usec) as std::os::raw::c_double * 1e-6f64;
    v = if f.is_some() {
        f.expect("non-null function pointer")(t)
    } else {
        0 as std::os::raw::c_int as std::os::raw::c_double
    };
    (*x).v += ((*x).last_v + v) * (t - (*x).last_t) / 2 as std::os::raw::c_int as std::os::raw::c_double;
    (*x).last_t = t;
}
#[no_mangle]
pub unsafe extern "C" fn tick(mut a: *mut u8) -> *mut u8 {
    let mut x: integ = a as integ;
    loop {
        usleep(100000 as std::os::raw::c_int as __useconds_t);
        update(x);
    };
}
#[no_mangle]
pub unsafe extern "C" fn set_input(
    mut x: integ,
    mut func: Option::<unsafe extern "C" fn(std::os::raw::c_double) -> std::os::raw::c_double>,
) {
    update(x);
    (*x).func = func;
    (*x).last_t = 0 as std::os::raw::c_int as std::os::raw::c_double;
    (*x)
        .last_v = if func.is_some() {
        func.expect("non-null function pointer")(0 as std::os::raw::c_int as std::os::raw::c_double)
    } else {
        0 as std::os::raw::c_int as std::os::raw::c_double
    };
}
#[no_mangle]
pub unsafe extern "C" fn new_integ(
    mut func: Option::<unsafe extern "C" fn(std::os::raw::c_double) -> std::os::raw::c_double>,
) -> integ {
    let mut x: integ = malloc(std::mem::size_of::<integ_t>() as std::os::raw::c_ulong)
        as integ;
    (*x).last_v = 0 as std::os::raw::c_int as std::os::raw::c_double;
    (*x).v = (*x).last_v;
    (*x).func = None;
    gettimeofday(&mut (*x).start, 0 as *mut u8);
    set_input(x, func);
    pthread_create(
        &mut (*x).id,
        0 as *const pthread_attr_t,
        Some(tick as unsafe extern "C" fn(*mut u8) -> *mut u8),
        x as *mut u8,
    );
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn sine(mut t: std::os::raw::c_double) -> std::os::raw::c_double {
    return sin(
        4 as std::os::raw::c_int as std::os::raw::c_double
            * atan2(
                1 as std::os::raw::c_int as std::os::raw::c_double,
                1 as std::os::raw::c_int as std::os::raw::c_double,
            ) * t,
    );
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut x: integ = new_integ(
        Some(sine as unsafe extern "C" fn(std::os::raw::c_double) -> std::os::raw::c_double),
    );
    sleep(2 as std::os::raw::c_int as std::os::raw::c_uint);
    set_input(x, None);
    usleep(500000 as std::os::raw::c_int as __useconds_t);
    printf(b"%g\n\0" as *const u8 as *const std::os::raw::c_char, (*x).v);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
