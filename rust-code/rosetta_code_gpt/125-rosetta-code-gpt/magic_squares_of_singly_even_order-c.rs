fn num_digits(mut n: i32) -> i32 {
    let mut count = 1;
    while n >= 10 {
        n /= 10;
        count += 1;
    }
    count
}

fn print_magic_square(square: Vec<Vec<i32>>) {
    let rows = square.len();
    for row in &square {
        for &num in row {
            print!("{:width$}", num, width = (rows as i32 - num_digits(num)) as usize);
        }
        println!();
    }
    println!("\nMagic constant: {}", (rows * rows + 1) * rows / 2);
}

fn odd_magic_square(n: usize) -> Option<Vec<Vec<i32>>> {
    if n < 3 || n % 2 == 0 {
        return None;
    }

    let mut result = vec![vec![0; n]; n];
    let mut value = 0;
    let mut c = n / 2;
    let mut r = 0;

    for _ in 1..=(n * n) {
        value += 1;
        result[r][c] = value;
        let new_r = if r == 0 { n - 1 } else { r - 1 };
        let new_c = if c == n - 1 { 0 } else { c + 1 };

        if result[new_r][new_c] == 0 {
            r = new_r;
            c = new_c;
        } else {
            r = (r + 1) % n;
        }
    }
    Some(result)
}

fn singly_even_magic_square(n: usize) -> Option<Vec<Vec<i32>>> {
    if n < 6 || (n - 2) % 4 != 0 {
        return None;
    }

    let sub_grid = odd_magic_square(n / 2)?;
    let mut result = vec![vec![0; n]; n];
    let grid_factors = [0, 2, 3, 1];
    let sub_grid_size = (n * n) as i32 / 4;

    for r in 0..n {
        for c in 0..n {
            let grid = (r / (n / 2)) * 2 + (c