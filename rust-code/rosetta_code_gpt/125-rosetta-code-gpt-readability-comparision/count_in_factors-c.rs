type ULong = u64;

fn get_prime(idx: usize, primes: &mut Vec<ULong>) -> ULong {
    if idx >= primes.len() {
        let mut last = *primes.last().unwrap_or(&3);
        while idx >= primes.len() {
            last += 2;
            let mut is_prime = true;
            for &p in primes.iter() {
                if p * p > last {
                    primes.push(last);
                    break;
                }
                if last % p == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime && !primes.contains(&last) {
                primes.push(last);
            }
        }
    }
    primes[idx]
}

fn main() {
    let mut primes = vec![2, 3];
    for x in 1..1000 {
        let mut n = x;
        let mut first = true;
        print!("{} = ", n);
        let mut i = 0;
        loop {
            let p = get_prime(i, &mut primes);
            while n % p == 0 {
                n /= p;
                if !first {
                    print!(" x ");
                }
                first = false;
                print!("{}", p);
            }
            if n <= p * p {
                break;
            }
            i += 1;
        }
        if first {
            println!("{}", n);
        } else if n > 1 {
            println!(" x {}", n);
        } else {
            println!();
        }
    }
}