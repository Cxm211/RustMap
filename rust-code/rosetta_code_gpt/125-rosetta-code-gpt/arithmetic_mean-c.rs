fn mean(v: &[f64]) -> f64 {
    let sum: f64 = v.iter().sum();
    sum / (v.len() as f64)
}

fn main() {
    let v = vec![1.0, 2.0, 2.718, 3.0, 3.142];
    let mut len = 5;

    while len >= 0 {
        print!("mean[");
        for (i, &item) in v[0..len].iter().enumerate() {
            print!("{}", if i == 0 { format!("{}", item) } else { format!(", {}", item) });
        }
        println!("] = {}", mean(&v[0..len]));
        
        if len == 0 { break; }
        len -= 1;
    }
}