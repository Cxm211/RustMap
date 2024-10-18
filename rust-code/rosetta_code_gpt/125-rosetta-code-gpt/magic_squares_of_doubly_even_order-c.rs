fn doubly_even_magic_square(n: usize) -> Option<Vec<Vec<usize>>> {
    if n < 4 || n % 4 != 0 {
        return None;
    }

    let bits = 38505;
    let size = n * n;
    let mult = n / 4;
    let mut result = vec![vec![0; n]; n];

    for r in 0..n {
        for c in 0..n {
            let i = r * n + c;
            let bit_pos = c / mult + (r / mult) * 4;
            result[r][c] = if (bits & (1 << bit_pos)) != 0 {
                i + 1
            } else {
                size - i
            };
        }
    }

    Some(result)
}

fn num_digits(mut n: usize) -> usize {
    let mut count = 1;
    while n >= 10 {
        n /= 10;
        count += 1;
    }
    count
}

fn print_magic_square(square: Vec<Vec<usize>>) {
    let rows = square.len();
    let base_width = num_digits(rows * rows) + 3;

    println!("Doubly Magic Square of Order : {} and Magic Constant : {}\n", rows, (rows * rows + 1) * rows / 2);

    for row in square {
        for &num in &row {
            print!("{:width$}", num, width = base_width - num_digits(num));
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 || !args[1].parse::<usize>().is_ok() {
        println!("Usage : {} <integer specifying rows in magic square>", args[0]);
    } else {
        let n: usize = args[1].parse().unwrap();
        if let Some(square) = doubly_even_magic_square(n) {
            print_magic_square(square);
        }
    }
}