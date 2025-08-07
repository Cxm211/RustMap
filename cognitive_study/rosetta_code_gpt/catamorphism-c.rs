type IntFn = fn(i32, i32) -> i32;

fn reduce(func: IntFn, size: usize, elms: &[i32]) -> i32 {
    let mut val = elms[0];
    for i in 1..size {
        val = func(val, elms[i]);
    }
    val
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    let nums = [1, 2, 3, 4, 5];
    println!("{}", reduce(add, 5, &nums));
    println!("{}", reduce(sub, 5, &nums));
    println!("{}", reduce(mul, 5, &nums));
}