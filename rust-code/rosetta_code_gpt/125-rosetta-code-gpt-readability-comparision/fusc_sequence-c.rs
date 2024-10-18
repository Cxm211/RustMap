fn fusc(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    } else if n % 2 == 0 {
        return fusc(n / 2);
    } else {
        return fusc((n - 1) / 2) + fusc((n + 1) / 2);
    }
}

fn num_len(n: i32) -> i32 {
    let mut sum = 1;
    let mut n = n;

    while n > 9 {
        n = n / 10;
        sum += 1;
    }

    sum
}

fn print_large_fuscs(limit: i32) {
    let mut max_len = 1;

    println!("\n\nPrinting all largest Fusc numbers up to {}", limit);
    println!("Index-------Value");

    for i in 0..=limit {
        let f = fusc(i);
        let len = num_len(f);

        if len > max_len {
            max_len = len;
            println!("{:5} {:12}", i, f);
        }
    }
}

fn main() {
    println!("Index-------Value");
    for i in 0..61 {
        println!("{:5} {:12}", i, fusc(i));
    }
}