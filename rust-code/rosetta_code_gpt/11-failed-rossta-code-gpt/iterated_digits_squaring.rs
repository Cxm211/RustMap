type Ull = u64;

fn is_89(mut x: i32) -> bool {
    loop {
        let mut s = 0;
        while x != 0 {
            s += (x % 10) * (x % 10);
            x /= 10;
        }
        
        if s == 89 {
            return true;
        }
        if s == 1 {
            return false;
        }
        x = s;
    }
}

fn main() {
    let mut sums = vec![0; 32 * 81 + 1];
    sums[0] = 1;
    
    let mut n = 1;
    loop {
        for i in 1..=(n * 81) {
            for j in 1..10 {
                let s = j * j;
                if s > i {
                    break;
                }
                sums[i] += sums[i - s];
            }
        }

        let mut count_89 = 0;
        for i in 1..=(n * 81) {
            if !is_89(i) {
                continue;
            }

            if sums[i] > Ull::MAX - count_89 {
                println!("counter overflow for 10^{}", n);
                return;
            }

            count_89 += sums[i];
        }

        println!("1->10^{}: {}", n, count_89);
        n += 1;
    }
}