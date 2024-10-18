fn gcd(m: i32, n: i32) -> i32 {
    let mut m = m;
    let mut n = n;
    let mut tmp;
    while m != 0 {
        tmp = m;
        m = n % m;
        n = tmp;
    }
    n
}

fn lcm(m: i32, n: i32) -> i32 {
    m / gcd(m, n) * n
}

fn main() {
    println!("lcm(35, 21) = {}", lcm(21, 35));
}