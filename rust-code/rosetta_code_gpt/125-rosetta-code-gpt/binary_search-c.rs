use std::f64;

fn bin(x: u32) -> String {
    let bits = if x == 0 {
        1
    } else {
        (f64::from(x).log(2.0) + 1.0).floor() as usize
    };

    let mut ret = String::with_capacity(bits);
    let mut x = x;

    for _ in 0..bits {
        ret.push(if x & 1 == 1 { '1' } else { '0' });
        x >>= 1;
    }

    ret.chars().rev().collect()
}

fn main() {
    for i in 0..20 {
        let binstr = bin(i);
        println!("{}", binstr);
    }
}