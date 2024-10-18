fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    println!("This program is named {}.", args[0]);

    for (i, arg) in args.iter().enumerate().skip(1) {
        println!("the argument #{} is {}", i, arg);
    }
}