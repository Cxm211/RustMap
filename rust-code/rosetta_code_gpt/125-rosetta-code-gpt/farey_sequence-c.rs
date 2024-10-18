use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Frac {
    d: i32,
    n: i32,
}

fn farey(n: i32) {
    let mut f1 = Frac { d: 0, n: 1 };
    let mut f2 = Frac { d: 1, n };
    let mut t: Frac;
    let mut k: i32;

    print!("{}/{} {}/{}", 0, 1, 1, n);
    while f2.n > 1 {
        k = (n + f1.n) / f2.n;
        t = f1;
        f1 = f2;
        f2 = Frac {
            d: f2.d * k - t.d,
            n: f2.n * k - t.n,
        };
        print!(" {}/{}", f2.d, f2.n);
    }

    println!();
}

fn farey_len(n: i32, cache: &mut HashMap<i32, u64>) -> u64 {
    if let Some(&result) = cache.get(&n) {
        return result;
    }

    let mut len: u64 = (n as u64 * (n as u64 + 3)) / 2;
    let mut p = 2;
    let mut q = 0;
    while p <= n {
        q = n / (n / p) + 1;
        len -= farey_len(n / p, cache) * ((q - p) as u64);
        p = q;
    }

    cache.insert(n, len);
    len
}

fn main() {
    for n in 1..=11 {
        print!("{}: ", n);
        farey(n);
    }

    let mut cache: HashMap<i32, u64> = HashMap::new();
    for n in (100..=1000).step_by(100) {
        println!("{}: {} items", n, farey_len(n, &mut cache));
    }

    let n = 10_000_000;
    println!("\n{}: {} items", n, farey_len(n, &mut cache));
}