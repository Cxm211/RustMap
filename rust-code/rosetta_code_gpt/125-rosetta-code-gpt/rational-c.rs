#[derive(Clone, Copy)]
struct Frac {
    num: i64,
    den: i64,
}

fn gcd(m: i64, mut n: i64) -> i64 {
    let mut t;
    while n != 0 {
        t = n;
        n = m % n;
        n = t;
    }
    m
}

fn frac_new(num: i64, den: i64) -> Frac {
    if den == 0 {
        panic!("Divide by zero: {}/{}", num, den);
    }
    let mut num = num;
    let mut den = den;
    let g = gcd(num, den);
    if g != 0 {
        num /= g;
        den /= g;
    } else {
        num = 0;
        den = 1;
    }
    if den < 0 {
        den = -den;
        num = -num;
    }
    Frac { num, den }
}

fn frac_add(a: Frac, b: Frac) -> Frac {
    frac_new(a.num * b.den + b.num * a.den, a.den * b.den)
}

fn frac_cmp_int(a: Frac, b: i64) -> i64 {
    let l = a.num * 1;
    let r = a.den * b;
    if l < r {
        -1
    } else if l > r {
        1
    } else {
        0
    }
}

fn main() {
    let mut sum: Frac;
    let mut kf: Frac;

    for n in 2..1 << 19 {
        sum = frac_new(1, n as i64);
        for k in 2..(n as f64).sqrt() as i64 {
            if n % k as usize != 0 {
                continue;
            }
            kf = frac_new(1, k);
            sum = frac_add(sum, kf);

            kf = frac_new(1, (n / k as usize) as i64);
            sum = frac_add(sum, kf);
        }
        if frac_cmp_int(sum, 1) == 0 {
            println!("{}", n);
        }
    }
}