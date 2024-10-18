fn modulo(n: i32, m: i32) -> i32 {
    ((n % m) + m) % m
}

fn is_prime(n: u32) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    } else {
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        return true;
    }
}

fn carmichael3(p1: u32) {
    if !is_prime(p1) {
        return;
    }

    for h3 in 1..p1 {
        for d in 1..(h3 + p1) {
            if ((h3 + p1) * (p1 - 1)) as i32 % d == 0 && modulo(-((p1 * p1) as i32), h3 as i32) == d % h3 {
                let p2 = 1 + (p1 - 1) * (h3 + p1) / d;
                if !is_prime(p2) {
                    continue;
                }
                let p3 = 1 + p1 * p2 / h3;
                if !is_prime(p3) || (p2 * p3) % (p1 - 1) != 1 {
                    continue;
                }
                println!("{} {} {}", p1, p2, p3);
            }
        }
    }
}

fn main() {
    for p1 in 2..62 {
        carmichael3(p1);
    }
}