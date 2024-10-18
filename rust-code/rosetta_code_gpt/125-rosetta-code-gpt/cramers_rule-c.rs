use std::f64::NAN;

#[derive(Debug, Clone)]
struct SquareMatrix {
    n: usize,
    elems: Vec<Vec<f64>>,
}

impl SquareMatrix {
    fn new(n: usize, elems: Vec<Vec<f64>>) -> Self {
        Self { n, elems }
    }

    fn det(&self) -> f64 {
        let mut det = 1.0;
        let mut a = self.elems.clone();

        for j in 0..self.n {
            let mut i_max = j;
            for i in j..self.n {
                if a[i][j].abs() > a[i_max][j].abs() {
                    i_max = i;
                }
            }

            if i_max != j {
                a.swap(i_max, j);
                det *= -1.0;
            }

            if a[j][j].abs() < 1e-12 {
                println!("Singular matrix!");
                return NAN;
            }

            for i in j + 1..self.n {
                let mult = -a[i][j] / a[j][j];
                for k in 0..self.n {
                    a[i][k] += mult * a[j][k];
                }
            }
        }

        for i in 0..self.n {
            det *= a[i][i];
        }

        det
    }

    fn cramer_solve(&self, det_a: f64, b: &[f64], var: usize) -> f64 {
        let mut tmp = self.clone();
        for i in 0..tmp.n {
            tmp.elems[i][var] = b[i];
        }
        let det_tmp = tmp.det();
        det_tmp / det_a
    }
}

fn main() {
    let n = 4;
    let elems = vec![
        vec![2.0, -1.0, 5.0, 1.0],
        vec![3.0, 2.0, 2.0, -6.0],
        vec![1.0, 3.0, 3.0, -1.0],
        vec![5.0, -2.0, -3.0, 3.0],
    ];
    let a = SquareMatrix::new(n, elems);

    let det_a = a.det();
    let b = vec![-3.0, -32.0, -47.0, 49.0];

    for i in 0..n {
        println!("{:7.3}", a.cramer_solve(det_a, &b, i));
    }
}