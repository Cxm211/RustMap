fn main() {
    let r = 7.125_f32;
    
    println!("{: >9.3}", -r);
    println!("{: >9.3}", r);
    println!("{: <9.3}", r);
    println!("{:0>9.3}", -r);
    println!("{:0>9.3}", r);
    println!("{:0<9.3}", r);
}