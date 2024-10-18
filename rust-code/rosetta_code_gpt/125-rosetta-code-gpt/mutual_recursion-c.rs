fn f(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }
    n - m(f(n - 1))
}

fn m(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    n - f(m(n - 1))
}

fn main() {
    for i in 0..20 {
        print!("{:2} ", f(i));
    }
    println!();
    
    for i in 0..20 {
        print!("{:2} ", m(i));
    }
    println!();
}