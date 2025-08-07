macro_rules! test {
    ($x:expr, $y:expr, $op:tt) => {
        let x = a($x) $op b($y);
        println!(concat!(stringify!($x), " ", stringify!($op), " ", stringify!($y), " = {:?}\n"), x);
    };
}

fn a(in_val: bool) -> bool {
    println!("I am a");
    in_val
}

fn b(in_val: bool) -> bool {
    println!("I am b");
    in_val
}

fn main() {
    test!(false, true, &&); // b is not evaluated
    test!(true, false, ||); // b is not evaluated
    test!(true, false, &&); // b is evaluated
    test!(false, false, ||); // b is evaluated
}