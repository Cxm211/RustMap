fn main() {
    let a = [1, 3, -5];
    let b = [4, -2, -1];
    
    println!("{}", dot_product(&a, &b));
}

fn dot_product(a: &[i32], b: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    sum
}