fn hpo2(n: u32) -> u32 {
    n & (!n).wrapping_add(1)
}

fn lhpo2(n: u32) -> u32 {
    let mut q = 0;
    let mut m = hpo2(n);
    while m % 2 == 0 {
        m >>= 1;
        q += 1;
    }
    q
}

fn nimsum(x: u32, y: u32) -> u32 {
    x ^ y
}

fn nimprod(x: u32, y: u32) -> u32 {
    if x < 2 || y < 2 {
        return x * y;
    }
    let h = hpo2(x);
    if x > h {
        return nimprod(h, y) ^ nimprod(x ^ h, y);
    }
    if hpo2(y) < y {
        return nimprod(y, x);
    }
    let xp = lhpo2(x);
    let yp = lhpo2(y);
    let comp = xp & yp;
    if comp == 0 {
        return x * y;
    }
    let h = hpo2(comp);
    nimprod(nimprod(x >> h, y >> h), 3 << (h - 1))
}

fn print_table(n: u32, op: char, func: fn(u32, u32) -> u32) {
    print!(" {} |", op);
    for a in 0..=n {
        print!("{:3}", a);
    }
    println!();
    print!("----");
    for _ in 0..=n {
        print!("---");
    }
    println!();
    for b in 0..=n {
        print!("{:2} |", b);
        for a in 0..=n {
            print!("{:3}", func(a, b));
        }
        println!();
    }
}

fn main() {
    print_table(15, '+', nimsum);
    println!();
    print_table(15, '*', nimprod);
    let a = 21508;
    let b = 42689;
    println!();
    println!("{} + {} = {}", a, b, nimsum(a, b));
    println!("{} * {} = {}", a, b, nimprod(a, b));
}