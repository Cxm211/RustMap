fn odd_magic_square(n: usize) -> Option<Vec<Vec<i32>>> {
    if n < 3 || n % 2 == 0 {
        return None;
    }

    let mut value = 0;
    let square_size = n * n;
    let mut c = n / 2;
    let mut r = 0;

    let mut result = vec![vec![0; n]; n];

    while value < square_size {
        value += 1;
        result[r][c] = value as i32;
        if r == 0 {
            if c == n - 1 {
                r += 1;
            } else {
                r = n - 1;
                c += 1;
            }
        } else if c == n - 1 {
            r -= 1;
            c = 0;
        } else if result[r - 1][c + 1] == 0 {
            r -= 1;
            c += 1;
        } else {
            r += 1;
        }
    }
    Some(result)
}

fn singly_even_magic_square(n: usize) -> Option<Vec<Vec<i32>>> {
    if n < 6 || (n - 2) % 4 != 0 {
        return None;
    }

    let size = n * n;
    let half_n = n / 2;
    let sub_grid_size = size / 4;

    let sub_grid = odd_magic_square(half_n)?;

    let grid_factors = [0, 2, 3, 1];
    let mut result = vec![vec![0; n]; n];

    for r in 0..n {
        for c in 0..n {
            let grid = (r / half_n) * 2 + (c / half_n);
            result[r][c] = sub_grid[r % half_n][c % half_n];
            result[r][c] += grid_factors[grid] * sub_grid_size as i32;
        }
    }

    let n_cols_left = half_n / 2;
    let n_cols_right = n_cols_left - 1;

    for r in 0..half_n {
        for c in 0..n {
            if c < n_cols_left || c >= n - n_cols_right || (c == n_cols_left && r == n_cols_left) {
                if c == 0 && r == n_cols_left {
                    continue;
                }

                result.swap(r, c, r + half_n, c);
            }
        }
    }

    Some(result)
}

fn num_digits(mut n: i32) -> usize {
    let mut count = 1;

    while n >= 10 {
        n /= 10;
        count += 1;
    }

    count
}

fn print_magic_square(square: &Vec<Vec<i32>>) {
    let rows = square.len();

    for i in 0..rows {
        for j in 0..rows {
            print!("{:width$}{}", "", square[i][j], width = rows - num_digits(square[i][j]));
        }
        println!();
    }
    println!("\nMagic constant: {}", (rows * rows + 1) * rows / 2);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || !args[1].chars().all(char::is_numeric) {
        println!("Usage: {} <integer specifying rows in magic square>", args[0]);
    } else {
        let n = args[1].parse::<usize>().unwrap();
        if let Some(square) = singly_even_magic_square(n) {
            print_magic_square(&square);
        }
    }
}
