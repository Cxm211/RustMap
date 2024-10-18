fn approx_equals(value: f64, other: f64, epsilon: f64) -> bool {
    (value - other).abs() < epsilon
}

fn test(a: f64, b: f64) {
    let epsilon = 1e-18;
    println!("{}, {} => {}", a, b, approx_equals(a, b, epsilon));
}

fn main() {
    test(100000000000000.01, 100000000000000.011);
    test(100.01, 100.011);
    test(10000000000000.001 / 10000.0, 1000000000.0000001000);
    test(0.001, 0.0010000001);
    test(0.000000000000000000000101, 0.0);
    test((2.0 as f64).sqrt() * (2.0 as f64).sqrt(), 2.0);
    test((-2.0 as f64).sqrt() * (-2.0 as f64).sqrt(), -2.0);
    test(3.14159265358979323846, 3.14159265358979324);
}