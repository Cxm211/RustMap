fn mul_inv(a: i32, b: i32) -> i32 {
    let mut b0 = b;
    let (mut t, mut q) = (0, 0);
    let (mut x0, mut x1) = (0, 1);

    if b == 1 {
        return 1;
    }

    while a > 1 {
        q = a / b;
        t = b;
        b = a % b;
        let tmp_a = t;

        t = x0;
        x0 = x1 - q * x0;
        x1 = t;

        let tmp_b = b;
        b = tmp_a % tmp_b;
    }

    if x1 < 0 {
        x1 += b0;
    }
    x1
}

fn main() {
    println!("{}", mul_inv(42, 2017));
}