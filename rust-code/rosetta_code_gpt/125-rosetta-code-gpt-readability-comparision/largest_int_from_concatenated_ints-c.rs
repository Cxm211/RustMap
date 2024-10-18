fn catcmp(a: &i32, b: &i32) -> std::cmp::Ordering {
    let ab = format!("{}{}", a, b);
    let ba = format!("{}{}", b, a);
    ba.cmp(&ab)
}

fn maxcat(arr: &mut [i32]) {
    arr.sort_by(catcmp);
    let result: String = arr.iter().map(|&x| x.to_string()).collect();
    println!("{}", result);
}

fn main() {
    let mut x = vec![1, 34, 3, 98, 9, 76, 45, 4];
    let mut y = vec![54, 546, 548, 60];
    
    maxcat(&mut x);
    maxcat(&mut y);
}