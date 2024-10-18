use std::f64::consts::PI;

fn main() {
    let pi2 = 2.0 * PI;
    for n in 1..10 {
        for i in 0..n {
            let mut c = 0.0;
            let mut s = 0.0;
            if i == 0 {
                c = 1.0;
            } else if n == 4 * i {
                s = 1.0;
            } else if n == 2 * i {
                c = -1.0;
            } else if 3 * n == 4 * i {
                s = -1.0;
            } else {
                let a = i as f64 * pi2 / n as f64;
                c = a.cos();
                s = a.sin();
            }
            
            if c != 0.0 {
                print!("{:.2}", c);
            }
            
            print!("{}", match s {
                1.0 => "i",
                -1.0 => "-i",
                _ if s != 0.0 => format!("{:+.2}i", s),
                _ => "".to_string()
            });
            
            if i == n - 1 {
                println!();
            } else {
                print!(",  ");
            }
        }
    }
}