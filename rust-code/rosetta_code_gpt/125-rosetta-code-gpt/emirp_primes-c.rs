type uint = u32;

fn is_prime(n: uint) -> bool {
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut p = 1;
    while p * p < n {
        if n % (p += 4) == 0 || n % (p += 2) == 0 {
            return false;
        }
    }
    true
}

fn reverse(n: uint) -> uint {
    let mut r = 0;
    let mut n = n;
    while n > 0 {
        r = r * 10 + (n % 10);
        n /= 10;
    }
    r
}

fn is_emirp(n: uint) -> bool {
    let r = reverse(n);
    r != n && is_prime(n) && is_prime(r)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut x;
    let mut c = 0;

    match args.len() {
        1 => {
            x = 11;
            while c < 20 {
                if is_emirp(x) {
                    print!(" {}", x);
                    c += 1;
                }
                x += 2;
            }
        }
        2 => {
            for x in (7701..8000).step_by(2) {
                if is_emirp(x) {
                    print!(" {}", x);
                }
            }
        }
        _ => {
            x = 11;
            loop {
                if is_emirp(x) && {
                    c += 1;
                    c == 10000
                } {
                    print!("{}", x);
                    break;
                }
                x += 2;
            }
        }
    }

    println!();
}
