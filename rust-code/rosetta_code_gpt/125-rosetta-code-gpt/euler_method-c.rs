use std::f64::consts::E;

type DerivF = fn(f64, f64) -> f64;

fn ivp_euler(f: DerivF, mut y: f64, step: i32, end_t: i32) {
    print!(" Step {:2}: ", step);
    let mut t = 0;
    while t <= end_t {
        if t % 10 == 0 {
            print!(" {:7.3}", y);
        }
        y += (step as f64) * f(t as f64, y);
        t += step;
    }
    println!();
}

fn analytic() {
    print!("    Time: ");
    for t in (0..=100).step_by(10) {
        print!(" {:7}", t);
    }
    println!("\nAnalytic: ");

    for t in (0..=100).step_by(10) {
        print!(" {:7.3}", 20.0 + 80.0 * E.powf(-0.07 * t as f64));
    }
    println!();
}

fn cooling(_t: f64, temp: f64) -> f64 {
    -0.07 * (temp - 20.0)
}

fn main() {
    analytic();
    ivp_euler(cooling, 100.0, 2, 100);
    ivp_euler(cooling, 100.0, 5, 100);
    ivp_euler(cooling, 100.0, 10, 100);
}