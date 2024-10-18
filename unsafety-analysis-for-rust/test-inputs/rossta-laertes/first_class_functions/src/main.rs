#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn acos(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn asin(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn atan(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn cos(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sin(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn tan(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn exp(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn log(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
pub type Class2Func = Option<unsafe extern "C"  fn(_: std::os::raw::c_double,) -> std::os::raw::c_double>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sComposition {
    pub f1: Option<unsafe extern "C"  fn(_: std::os::raw::c_double,) -> std::os::raw::c_double>,
    pub f2: Option<unsafe extern "C"  fn(_: std::os::raw::c_double,) -> std::os::raw::c_double>,
}
impl std::default::Default for sComposition {
    fn default() -> Self {
        sComposition {
        f1: None,
        f2: None
        }
    }
}

pub type Composition = * mut crate::sComposition;
#[no_mangle]
pub extern "C" fn functionA(mut v: std::os::raw::c_double) -> std::os::raw::c_double {
    return v * v * v;
}
#[no_mangle]
pub unsafe extern "C" fn functionB(mut v: std::os::raw::c_double) -> std::os::raw::c_double {
    return exp(log(v) / 3 as std::os::raw::c_int as std::os::raw::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn Function1(
    mut f2: Class2Func,
    mut val: std::os::raw::c_double,
) -> std::os::raw::c_double {
    return f2.expect("non-null function pointer")(val);
}
#[no_mangle]
pub extern "C" fn WhichFunc(mut idx: std::os::raw::c_int) -> Class2Func {
    return if idx < 4 as std::os::raw::c_int {
        Some(functionA as unsafe extern "C" fn(std::os::raw::c_double) -> std::os::raw::c_double)
    } else {
        Some(functionB as unsafe extern "C" fn(std::os::raw::c_double) -> std::os::raw::c_double)
    };
}
#[no_mangle]
pub static mut funcListA: [Class2Func; 4] = [
    Some(functionA as unsafe extern "C" fn(std::os::raw::c_double) -> std::os::raw::c_double),
    Some(sin as unsafe extern "C" fn(std::os::raw::c_double) -> std::os::raw::c_double),
    Some(cos as unsafe extern "C" fn(std::os::raw::c_double) -> std::os::raw::c_double),
    Some(tan as unsafe extern "C" fn(std::os::raw::c_double) -> std::os::raw::c_double),
];
#[no_mangle]
pub static mut funcListB: [Class2Func; 4] = [
    Some(functionB as unsafe extern "C" fn(std::os::raw::c_double) -> std::os::raw::c_double),
    Some(asin as unsafe extern "C" fn(std::os::raw::c_double) -> std::os::raw::c_double),
    Some(acos as unsafe extern "C" fn(std::os::raw::c_double) -> std::os::raw::c_double),
    Some(atan as unsafe extern "C" fn(std::os::raw::c_double) -> std::os::raw::c_double),
];
#[no_mangle]
pub unsafe extern "C" fn InvokeComposed(
    mut f1: Class2Func,
    mut f2: Class2Func,
    mut val: std::os::raw::c_double,
) -> std::os::raw::c_double {
    return f1
        .expect(
            "non-null function pointer",
        )(f2.expect("non-null function pointer")(val));
}
#[no_mangle]
pub unsafe extern "C" fn Compose(mut f1: Class2Func, mut f2: Class2Func) -> Composition {
    let mut comp: Composition = malloc(
        std::mem::size_of::<sComposition>() as std::os::raw::c_ulong,
    ) as Composition;
    (*comp).f1 = f1;
    (*comp).f2 = f2;
    return comp;
}
#[no_mangle]
pub unsafe extern "C" fn CallComposed(
    mut comp: Composition,
    mut val: std::os::raw::c_double,
) -> std::os::raw::c_double {
    return ((*comp).f1)
        .expect(
            "non-null function pointer",
        )(((*comp).f2).expect("non-null function pointer")(val));
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut ix: std::os::raw::c_int = 0;
    let mut c: Composition = core::ptr::null_mut();
    printf(
        b"Function1(functionA, 3.0) = %f\n\0" as *const u8 as *const std::os::raw::c_char,
        Function1(WhichFunc(0 as std::os::raw::c_int), 3.0f64),
    );
    ix = 0 as std::os::raw::c_int;
    while ix < 4 as std::os::raw::c_int {
        c = Compose(funcListA[ix as usize], funcListB[ix as usize]);
        printf(
            b"Compostion %d(0.9) = %f\n\0" as *const u8 as *const std::os::raw::c_char,
            ix,
            CallComposed(c, 0.9f64),
        );
        ix += 1;
        ix;
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
