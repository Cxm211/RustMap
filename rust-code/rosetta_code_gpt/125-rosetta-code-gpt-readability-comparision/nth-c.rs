fn add_suffix(num: i32) -> String {
    let suffixes = ["th", "st", "nd", "rd"];
    let i = match num % 10 {
        1 => if num % 100 == 11 { 0 } else { 1 },
        2 => if num % 100 == 12 { 0 } else { 2 },
        3 => if num % 100 == 13 { 0 } else { 3 },
        _ => 0,
    };

    format!("{}{}", num, suffixes[i])
}

fn main() {
    println!("Set [0,25]:");
    for i in 0..26 {
        print!("{} ", add_suffix(i));
    }
    println!();

    println!("Set [250,265]:");
    for i in 250..266 {
        print!("{} ", add_suffix(i));
    }
    println!();

    println!("Set [1000,1025]:");
    for i in 1000..1026 {
        print!("{} ", add_suffix(i));
    }
}