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
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

//use c2rust_out::*;
extern "C" {
    fn log(_: f64) -> f64;
}
pub type pfunc =
    Option<unsafe extern "C" fn(f64, f64, f64, Option<unsafe extern "C" fn() -> f64>) -> f64>;
pub type rfunc = Option<unsafe extern "C" fn(f64) -> f64>;
#[no_mangle]
pub extern "C" fn int_leftrect(
    mut from: f64,
    mut to: f64,
    mut n: f64,
    mut func: Option<unsafe extern "C" fn() -> f64>,
) -> f64 {
    let mut h: f64 = (to - from) / n;
    let mut sum: f64 = 0.0f64;
    let mut x: f64 = 0.;
    x = from;
    unsafe {
        while x <= to - h {
            sum += ::core::mem::transmute::<_, fn(_) -> f64>(
                func.expect("non-null function pointer"),
            )(x);
            x += h;
        }
    }
    return h * sum;
}

#[no_mangle]
pub extern "C" fn int_rightrect(
    mut from: f64,
    mut to: f64,
    mut n: f64,
    mut func: Option<unsafe extern "C" fn() -> f64>,
) -> f64 {
    let mut h: f64 = (to - from) / n;
    let mut sum: f64 = 0.0f64;
    let mut x: f64 = 0.;
    x = from;
    unsafe {
        while x <= to - h {
            sum += ::core::mem::transmute::<_, fn(_) -> f64>(
                func.expect("non-null function pointer"),
            )(x + h);
            x += h;
        }
    }
    return h * sum;
}

#[no_mangle]
pub extern "C" fn int_midrect(
    mut from: f64,
    mut to: f64,
    mut n: f64,
    mut func: Option<unsafe extern "C" fn() -> f64>,
) -> f64 {
    let mut h: f64 = (to - from) / n;
    let mut sum: f64 = 0.0f64;
    let mut x: f64 = 0.;
    x = from;
    unsafe {
        while x <= to - h {
            sum += ::core::mem::transmute::<_, fn(_) -> f64>(
                func.expect("non-null function pointer"),
            )(x + h / 2.0f64);
            x += h;
        }
    }
    return h * sum;
}

#[no_mangle]
pub extern "C" fn int_trapezium(
    mut from: f64,
    mut to: f64,
    mut n: f64,
    mut func: Option<unsafe extern "C" fn() -> f64>,
) -> f64 {
    let mut h: f64 = (to - from) / n;
    unsafe {
        let mut sum: f64 = ::core::mem::transmute::<_, fn(_) -> f64>(
            func.expect("non-null function pointer"),
        )(from)
            + ::core::mem::transmute::<_, fn(_) -> f64>(func.expect("non-null function pointer"))(
                to,
            );
        let mut i: i32 = 0;
        i = 1;
        while (i as f64) < n {
            sum += 2.0f64
                * ::core::mem::transmute::<_, fn(_) -> f64>(
                    func.expect("non-null function pointer"),
                )(from + i as f64 * h);
            i += 1;
            i;
        }
        return h * sum / 2.0f64;
    }
}

#[no_mangle]
pub extern "C" fn int_simpson(
    mut from: f64,
    mut to: f64,
    mut n: f64,
    mut func: Option<unsafe extern "C" fn() -> f64>,
) -> f64 {
    let mut h: f64 = (to - from) / n;
    let mut sum1: f64 = 0.0f64;
    let mut sum2: f64 = 0.0f64;
    let mut i: i32 = 0;
    let mut x: f64 = 0.;
    i = 0;
    unsafe {
        while (i as f64) < n {
            sum1 += ::core::mem::transmute::<_, fn(_) -> f64>(
                func.expect("non-null function pointer"),
            )(from + h * i as f64 + h / 2.0f64);
            i += 1;
            i;
        }
    }
    i = 1;
    unsafe {
        while (i as f64) < n {
            sum2 += ::core::mem::transmute::<_, fn(_) -> f64>(
                func.expect("non-null function pointer"),
            )(from + h * i as f64);
            i += 1;
            i;
        }
        return h / 6.0f64
            * (::core::mem::transmute::<_, fn(_) -> f64>(
                func.expect("non-null function pointer"),
            )(from)
                + ::core::mem::transmute::<_, fn(_) -> f64>(
                    func.expect("non-null function pointer"),
                )(to)
                + 4.0f64 * sum1
                + 2.0f64 * sum2);
    }
}

#[no_mangle]
pub extern "C" fn f3(mut x: f64) -> f64 {
    return x;
}

#[no_mangle]
pub extern "C" fn f3a(mut x: f64) -> f64 {
    return x * x / 2.0f64;
}

#[no_mangle]
pub extern "C" fn f2(mut x: f64) -> f64 {
    return 1.0f64 / x;
}

#[no_mangle]
pub extern "C" fn f2a(mut x: f64) -> f64 {
    unsafe {
        return log(x);
    }
}

#[no_mangle]
pub extern "C" fn f1(mut x: f64) -> f64 {
    return x * x * x;
}

#[no_mangle]
pub extern "C" fn f1a(mut x: f64) -> f64 {
    return x * x * x * x / 4.0f64;
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut ic: f64 = 0.;
    unsafe {
        let mut f: [pfunc; 5] = [
            Some(
                int_leftrect
                    as unsafe extern "C" fn(
                        f64,
                        f64,
                        f64,
                        Option<unsafe extern "C" fn() -> f64>,
                    ) -> f64,
            ),
            Some(
                int_rightrect
                    as unsafe extern "C" fn(
                        f64,
                        f64,
                        f64,
                        Option<unsafe extern "C" fn() -> f64>,
                    ) -> f64,
            ),
            Some(
                int_midrect
                    as unsafe extern "C" fn(
                        f64,
                        f64,
                        f64,
                        Option<unsafe extern "C" fn() -> f64>,
                    ) -> f64,
            ),
            Some(
                int_trapezium
                    as unsafe extern "C" fn(
                        f64,
                        f64,
                        f64,
                        Option<unsafe extern "C" fn() -> f64>,
                    ) -> f64,
            ),
            Some(
                int_simpson
                    as unsafe extern "C" fn(
                        f64,
                        f64,
                        f64,
                        Option<unsafe extern "C" fn() -> f64>,
                    ) -> f64,
            ),
        ];
        let mut names: [*const i8; 5] = [
            b"leftrect\0" as *const u8 as *const i8,
            b"rightrect\0" as *const u8 as *const i8,
            b"midrect\0" as *const u8 as *const i8,
            b"trapezium\0" as *const u8 as *const i8,
            b"simpson\0" as *const u8 as *const i8,
        ];
        let mut rf: [rfunc; 4] = [
            Some(f1 as unsafe extern "C" fn(f64) -> f64),
            Some(f2 as unsafe extern "C" fn(f64) -> f64),
            Some(f3 as unsafe extern "C" fn(f64) -> f64),
            Some(f3 as unsafe extern "C" fn(f64) -> f64),
        ];
        let mut If: [rfunc; 4] = [
            Some(f1a as unsafe extern "C" fn(f64) -> f64),
            Some(f2a as unsafe extern "C" fn(f64) -> f64),
            Some(f3a as unsafe extern "C" fn(f64) -> f64),
            Some(f3a as unsafe extern "C" fn(f64) -> f64),
        ];
        let mut ivals: [f64; 8] = [
            0.0f64, 1.0f64, 1.0f64, 100.0f64, 0.0f64, 5000.0f64, 0.0f64, 6000.0f64,
        ];
        let mut approx: [f64; 4] = [100.0f64, 1000.0f64, 5000000.0f64, 6000000.0f64];
        j = 0;
        while (j as u64)
            < (::core::mem::size_of::<[rfunc; 4]>() as u64)
                .wrapping_div(::core::mem::size_of::<rfunc>() as u64)
        {
            i = 0;
            while i < 5 {
                ic = (Some(
                    (*f.as_mut_ptr().offset(i as isize)).expect("non-null function pointer"),
                ))
                .expect("non-null function pointer")(
                    ivals[(2 * j) as usize],
                    ivals[(2 * j + 1i32) as usize],
                    approx[j as usize],
                    ::core::mem::transmute::<rfunc, Option<unsafe extern "C" fn() -> f64>>(
                        rf[j as usize],
                    ),
                );
                print!(
                    "{:10} [ 0,1] num: {:+}, an: {}\n",
                    build_str_from_raw_ptr(names[i as usize] as *mut u8),
                    ic,
                    (Some(
                        (*If.as_mut_ptr().offset(j as isize)).expect("non-null function pointer")
                    ))
                    .expect("non-null function pointer")(
                        ivals[(2 * j + 1i32) as usize]
                    ) - (Some(
                        (*If.as_mut_ptr().offset(j as isize)).expect("non-null function pointer"),
                    ))
                    .expect("non-null function pointer")(
                        ivals[(2 * j) as usize]
                    )
                );
                i += 1;
                i;
            }
            print!("\n");
            j += 1;
            j;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
