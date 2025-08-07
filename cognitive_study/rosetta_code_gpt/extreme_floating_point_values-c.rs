fn main() {
    let inf = f64::INFINITY;
    let minus_inf = f64::NEG_INFINITY;
    let minus_zero = -0.0;
    let nan = f64::NAN;

    println!("positive infinity: {}", inf);
    println!("negative infinity: {}", minus_inf);
    println!("negative zero: {}", minus_zero);
    println!("not a number: {}", nan);

    println!("+inf + 2.0 = {}", inf + 2.0);
    println!("+inf - 10.1 = {}", inf - 10.1);
    println!("+inf + -inf = {}", inf + minus_inf);
    println!("0.0 * +inf = {}", 0.0 * inf);
    println!("1.0/-0.0 = {}", 1.0 / minus_zero);
    println!("NaN + 1.0 = {}", nan + 1.0);
    println!("NaN + NaN = {}", nan + nan);

    println!("NaN == NaN = {}", if nan == nan { "true" } else { "false" });
    println!("0.0 == -0.0 = {}", if 0.0 == minus_zero { "true" } else { "false" });
}