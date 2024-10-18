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
pub extern "C" fn hpo2(mut n: u32) -> u32 {
    return n & n.wrapping_neg();
}

#[no_mangle]
pub extern "C" fn lhpo2(mut n: u32) -> u32 {
    let mut q: u32 = 0;
    let mut m: u32 = hpo2(n);
    while m.wrapping_rem(2) == 0 {
        m >>= 1;
        q = q.wrapping_add(1);
        q;
    }
    return q;
}

#[no_mangle]
pub extern "C" fn nimsum(mut x: u32, mut y: u32) -> u32 {
    return x ^ y;
}

#[no_mangle]
pub extern "C" fn nimprod(mut x: u32, mut y: u32) -> u32 {
    if x < 2 || y < 2 {
        return x.wrapping_mul(y);
    }
    let mut h: u32 = hpo2(x);
    if x > h {
        return nimprod(h, y) ^ nimprod(x ^ h, y);
    }
    if hpo2(y) < y {
        return nimprod(y, x);
    }
    let mut xp: u32 = lhpo2(x);
    let mut yp: u32 = lhpo2(y);
    let mut comp: u32 = xp & yp;
    if comp == 0 {
        return x.wrapping_mul(y);
    }
    h = hpo2(comp);
    return nimprod(
        nimprod(x >> h, y >> h),
        (3i32 << h.wrapping_sub(1u32)) as u32,
    );
}

#[no_mangle]
pub extern "C" fn print_table(
    mut n: u32,
    mut op: i8,
    mut func: Option<unsafe extern "C" fn(u32, u32) -> u32>,
) {
    print!(" {} |", op as i32);
    let mut a: u32 = 0;
    while a <= n {
        print!("{:3}", a);
        a = a.wrapping_add(1);
        a;
    }
    print!("\n--- -");
    let mut a_0: u32 = 0;
    while a_0 <= n {
        print!("---");
        a_0 = a_0.wrapping_add(1);
        a_0;
    }
    print!("\n");
    let mut b: u32 = 0;
    unsafe {
        while b <= n {
            print!("{:2} |", b);
            let mut a_1: u32 = 0;
            while a_1 <= n {
                print!("{:3}", func.expect("non-null function pointer")(a_1, b));
                a_1 = a_1.wrapping_add(1);
                a_1;
            }
            print!("\n");
            b = b.wrapping_add(1);
            b;
        }
    }
}

fn main_0() -> i32 {
    print_table(
        15,
        '+' as i8,
        Some(nimsum as unsafe extern "C" fn(u32, u32) -> u32),
    );
    print!("\n");
    print_table(
        15,
        '*' as i8,
        Some(nimprod as unsafe extern "C" fn(u32, u32) -> u32),
    );
    let a: u32 = 21508;
    let b: u32 = 42689;
    print!("\n{} + {} = {}\n", a, b, nimsum(a, b));
    print!("{} * {} = {}\n", a, b, nimprod(a, b));
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
