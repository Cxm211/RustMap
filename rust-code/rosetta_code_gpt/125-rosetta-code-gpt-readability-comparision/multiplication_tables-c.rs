fn main() {
    let n = 12;
    
    for j in 1..=n {
        print!("{:3}{}", j, if j != n { " " } else { "\n" });
    }
    
    for j in 0..=n {
        print!("{}", if j != n { "----" } else { "+\n" });
    }
    
    for i in 1..=n {
        for j in 1..=n {
            print!("{}", if j < i { "    " } else { format!("{:3} ", i * j) });
        }
        println!("| {}", i);
    }
}