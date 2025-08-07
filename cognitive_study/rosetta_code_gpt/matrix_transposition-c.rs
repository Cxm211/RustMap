fn transpose(dest: &mut [[f64; 3]; 5], src: [[f64; 5]; 3]) {
    for i in 0..3 {
        for j in 0..5 {
            dest[j][i] = src[i][j];
        }
    }
}

fn main() {
    let a: [[f64; 5]; 3] = [
        [0.0, 1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0, 9.0],
        [1.0, 0.0, 0.0, 0.0, 42.0],
    ];
    let mut b: [[f64; 3]; 5] = [[0.0; 3]; 5];

    transpose(&mut b, a);

    for i in 0..5 {
        for j in 0..3 {
            print!("{}{}", b[i][j], if j == 2 { "\n" } else { " " });
        }
    }
}