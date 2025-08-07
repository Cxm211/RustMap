fn mul_inv(a: i32, b: i32) -> i32 {
    let mut b0 = b;
    let mut x0 = 0;
    let mut x1 = 1;
    let mut t;
    let mut q;

    if b == 1 {
        return 1;
    }

    while a > 1 {
        q = a / b;
        t = b;
        b = a % b;
        a = t;

        t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }

    if x1 < 0 {
        x1 += b0;
    }

    x1
}

fn chinese_remainder(n: &[i32], a: &[i32]) -> i32 {
    let mut prod = 1;
    let mut sum = 0;
    let mut p;
    
    for &ni in n.iter() {
        prod *= ni;
    }

    for (ni, ai) in n.iter().zip(a.iter()) {
        p = prod / ni;
        sum += ai * mul_inv(p, *ni) * p;
    }

    sum % prod
}

fn main() {
    let n = [3, 5, 7];
    let a = [2, 3, 2];

    println!("{}", chinese_remainder(&n, &a));
}