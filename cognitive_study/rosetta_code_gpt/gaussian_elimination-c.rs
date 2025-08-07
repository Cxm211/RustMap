fn swap_row(a: &mut [f64], b: &mut [f64], r1: usize, r2: usize, n: usize) {
    if r1 == r2 {
        return;
    }
    for i in 0..n {
        a.swap(r1 * n + i, r2 * n + i);
    }
    b.swap(r1, r2);
}

fn gauss_eliminate(a: &mut [f64], b: &mut [f64], x: &mut [f64], n: usize) {
    for dia in 0..n {
        let mut max_row = dia;
        let mut max = a[dia * n + dia];
        
        for row in (dia + 1)..n {
            let tmp = a[row * n + dia].abs();
            if tmp > max {
                max_row = row;
                max = tmp;
            }
        }
        
        swap_row(a, b, dia, max_row, n);
        
        for row in (dia + 1)..n {
            let tmp = a[row * n + dia] / a[dia * n + dia];
            for col in (dia + 1)..n {
                a[row * n + col] -= tmp * a[dia * n + col];
            }
            a[row * n + dia] = 0.0;
            b[row] -= tmp * b[dia];
        }
    }
    
    for row in (0..n).rev() {
        let mut tmp = b[row];
        for j in ((row + 1)..n).rev() {
            tmp -= x[j] * a[row * n + j];
        }
        x[row] = tmp / a[row * n + row];
    }
}

fn main() {
    let mut a: [f64; 36] = [
        1.00, 0.00, 0.00,  0.00,  0.00, 0.00,
        1.00, 0.63, 0.39,  0.25,  0.16, 0.10,
        1.00, 1.26, 1.58,  1.98,  2.49, 3.13,
        1.00, 1.88, 3.55,  6.70, 12.62, 23.80,
        1.00, 2.51, 6.32, 15.88, 39.90, 100.28,
        1.00, 3.14, 9.87, 31.01, 97.41, 306.02,
    ];
    let mut b: [f64; 6] = [-0.01, 0.61, 0.91, 0.99, 0.60, 0.02];
    let mut x = [0.0; 6];
    
    gauss_eliminate(&mut a, &mut b, &mut x, 6);
    
    for &xi in x.iter() {
        println!("{:?}", xi);
    }
}