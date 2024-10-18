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
pub extern "C" fn jos(mut n: i32, mut k: i32, mut m: i32) -> i32 {
    let mut a: i32 = 0;
    a = m + 1;
    while a <= n {
        m = (m + k) % a;
        a += 1;
        a;
    }
    return m;
}

#[no_mangle]
pub extern "C" fn jos_large(mut n: u64, mut k: u64, mut m: u64) -> u64 {
    if k <= 1 {
        return n.wrapping_sub(m).wrapping_sub(1);
    }
    let mut a: u64 = m;
    while a < n {
        let mut q: u64 = a
            .wrapping_sub(m)
            .wrapping_add(k)
            .wrapping_sub(2)
            .wrapping_div(k.wrapping_sub(1));
        if a.wrapping_add(q) > n {
            q = n.wrapping_sub(a);
        } else if q == 0 {
            q = 1;
        }
        a = (a).wrapping_add(q) as u64;
        m = m.wrapping_add(q.wrapping_mul(k)).wrapping_rem(a);
    }
    return m;
}

fn main_0() -> i32 {
    let mut n: u64 = 0;
    let mut k: u64 = 0;
    let mut i: u64 = 0;
    n = 41;
    k = 3;
    print!(
        "n = {}, k = {}, final survivor: {}\n",
        n,
        k,
        jos(n as i32, k as i32, 0)
    );
    n = 9876543210987654321;
    k = 12031;
    print!("n = {}, k = {}, three survivors:", n, k);
    i = 3;
    loop {
        let fresh0 = i;
        i = i.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        print!(" {}", jos_large(n, k, i));
    }
    print!("{}", '\n' as i32);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
