fn map_range(a1: f64, a2: f64, b1: f64, b2: f64, s: f64) -> f64 {
    b1 + (s - a1) * (b2 - b1) / (a2 - a1)
}

fn main() {
    println!("Mapping [0,10] to [-1,0] at intervals of 1:");

    for i in 0..=10 {
        println!("f({}) = {}", i, map_range(0.0, 10.0, -1.0, 0.0, i as f64));
    }
}