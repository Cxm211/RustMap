fn semiprime(n: i32) -> bool {
    let mut p = 2;
    let mut f = 0;
    let mut n = n;
    
    while f < 2 && p * p <= n {
        while n % p == 0 {
            n /= p;
            f += 1;
        }
        p += 1;
    }
    
    f + (if n > 1 { 1 } else { 0 }) == 2
}

fn main() {
    for i in 2..100 {
        if semiprime(i) {
            print!(" {} ", i);
        }
    }
    println!();
}