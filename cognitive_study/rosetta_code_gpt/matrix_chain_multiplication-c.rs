fn optimal_matrix_chain_order(dims: &[usize]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let n = dims.len() - 1;
    let mut m = vec![vec![usize::MAX; n]; n];
    let mut s = vec![vec![0; n]; n];

    for len in 1..n {
        for i in 0..n - len {
            let j = i + len;
            m[i][j] = usize::MAX;
            for k in i..j {
                let temp = dims[i] * dims[k + 1] * dims[j + 1];
                let cost = m[i][k] + m[k + 1][j] + temp;
                if cost < m[i][j] {
                    m[i][j] = cost;
                    s[i][j] = k;
                }
            }
        }
    }

    (m, s)
}

fn print_optimal_chain_order(i: usize, j: usize, s: &[Vec<usize>]) {
    if i == j {
        print!("{}", (i as u8 + b'A') as char);
    } else {
        print!("(");
        print_optimal_chain_order(i, s[i][j], s);
        print_optimal_chain_order(s[i][j] + 1, j, s);
        print!(")");
    }
}

fn main() {
    let test_cases = vec![
        vec![5, 6, 3, 1],
        vec![1, 5, 25, 30, 100, 70, 2, 1, 100, 250, 1, 1000, 2],
        vec![1000, 1, 500, 12, 1, 700, 2500, 3, 2, 5, 14, 10],
    ];

    for dims in test_cases.iter() {
        print!("Dims  : [{:?}]\n", dims);
        let (m, s) = optimal_matrix_chain_order(dims);
        print!("Order : ");
        print_optimal_chain_order(0, dims.len() - 2, &s);
        println!("\nCost  : {}", m[0][dims.len() - 2]);
    }
}