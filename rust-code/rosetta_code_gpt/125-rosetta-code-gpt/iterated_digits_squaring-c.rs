type Ull = u64;

fn is89(mut x: i32) -> bool {
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
    let mut sums = vec![1u64, 0];
    sums.resize(32 * 81 + 1, 0);

    let mut n = 1;
    loop {
        for i in 1..=n * 81 {
            for j in 1..10 {
                let s = j * j;
                if s > i {
                    break;
                }
                sums[i] += sums[i - s];
            }
        }

        let mut count89 = 0u64;
        for i in 1..=n * 81 {
            if !is89(i as i32) {
                continue;
            }

            if let Some(new_count89) = count89.checked_add(sums[i]) {
                count89 = new_count89;
            } else {
                println!("counter overflow for 10^{}", n);
                return;
            }
        }

        println!("1->10^{}: {}", n, count89);
        n += 1;
    }
}