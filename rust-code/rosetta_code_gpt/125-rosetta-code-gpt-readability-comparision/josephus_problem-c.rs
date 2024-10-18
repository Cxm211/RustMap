type XInt = u64;

fn jos(n: i32, k: i32, mut m: i32) -> i32 {
    for a in m+1..=n {
        m = (m + k) % a;
    }
    m
}

fn jos_large(n: XInt, k: XInt, mut m: XInt) -> XInt {
    if k <= 1 { return n - m - 1; }

    let mut a = m;
    while a < n {
        let mut q = (a - m + k - 2) / (k - 1);

        if a + q > n { q = n - a; }
        else if q == 0 { q = 1; }

        m = (m + q * k) % (a += q);
    }

    m
}

fn main() {
    let (mut n, mut k, mut i): (XInt, XInt, XInt);

    n = 41;
    k = 3;
    println!("n = {}, k = {}, final survivor: {}", n, k, jos(n as i32, k as i32, 0));

    n = 9876543210987654321;
    k = 12031;
    print!("n = {}, k = {}, three survivors:", n, k);

    for _ in 0..3 {
        print!(" {}", jos_large(n, k, i));
        i -= 1;
    }
    println!();
}