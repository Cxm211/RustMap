fn main() {
    const N: i32 = 2;
    let base = 10;
    let mut c1 = 0;
    let mut c2 = 0;

    for k in 1..base.pow(N as u32) {
        c1 += 1;
        if k % (base - 1) == (k * k) % (base - 1) {
            c2 += 1;
            print!("{} ", k);
        }
    }

    println!("\nTrying {} numbers instead of {} numbers saves {:.2}% ",
             c2, c1, 100.0 - 100.0 * c2 as f64 / c1 as f64);
}