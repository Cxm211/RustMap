fn gcd_ui(mut x: u64, mut y: u64) -> u64 {
    let mut t;
    if y < x {
        t = x;
        x = y;
        y = t;
    }
    while y > 0 {
        t = y;
        y = x % y;
        x = t;
    }
    x
}

fn binomial(mut n: u64, mut k: u64) -> u64 {
    let mut d;
    let mut g;
    let mut r = 1;
    if k == 0 {
        return 1;
    }
    if k == 1 {
        return n;
    }
    if k >= n {
        return if k == n { 1 } else { 0 };
    }
    if k > n / 2 {
        k = n - k;
    }
    for d in 1..=k {
        if r >= u64::MAX / n {
            let (nr, dr);
            g = gcd_ui(n, d);
            nr = n / g;
            dr = d / g;
            g = gcd_ui(r, dr);
            r /= g;
            if r >= u64::MAX / nr {
                return 0;
            }
            r *= nr;
            r /= dr;
            n -= 1;
        } else {
            r *= n;
            r /= d;
            n -= 1;
        }
    }
    r
}

fn main() {
    println!("{}", binomial(5, 3));
    println!("{}", binomial(40, 19));
    println!("{}", binomial(67, 31));
}