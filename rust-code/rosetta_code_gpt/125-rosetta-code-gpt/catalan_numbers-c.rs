type Ull = u64;

fn binomial(mut m: Ull, mut n: Ull) -> Ull {
    let mut r = 1;
    let mut d = m - n;
    if d > n {
        n = d;
        d = m - n;
    }

    while m > n {
        r *= m;
        m -= 1;
        while d > 1 && r % d == 0 {
            r /= d;
            d -= 1;
        }
    }

    r
}

fn catalan1(n: i32) -> Ull {
    binomial(2 * n as Ull, n as Ull) / (1 + n) as Ull
}

fn catalan2(n: i32) -> Ull {
    let mut r = if n == 0 { 1 } else { 0 };

    for i in 0..n {
        r += catalan2(i) * catalan2(n - 1 - i);
    }
    r
}

fn catalan3(n: i32) -> Ull {
    if n == 0 {
        1
    } else {
        2 * (2 * n - 1) as Ull * catalan3(n - 1) / (1 + n) as Ull
    }
}

fn main() {
    println!("\tdirect\tsumming\tfrac");
    for i in 0..16 {
        println!("{}\t{}\t{}\t{}", i, catalan1(i), catalan2(i), catalan3(i));
    }
}