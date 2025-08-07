const N: usize = 100000;

fn main() {
    let mut q = vec![0; N + 1];  // Rust 中数组的下标从 0 开始

    q[1] = 1;
    q[2] = 1;

    for i in 3..=N {
        q[i] = q[i - q[i - 1]] + q[i - q[i - 2]];
    }

    for i in 1..=10 {
        print!("{}{}", q[i], if i == 10 { "\n" } else { " " });
    }

    println!("{}", q[1000]);

    let mut flip = 0;
    for i in 1..N {
        flip += (q[i] > q[i + 1]) as i32;
    }

    println!("flips: {}", flip);
}