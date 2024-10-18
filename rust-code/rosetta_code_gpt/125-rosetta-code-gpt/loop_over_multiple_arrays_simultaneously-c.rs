fn main() {
    let a1 = ['a', 'b', 'c'];
    let a2 = ['A', 'B', 'C'];
    let a3 = [1, 2, 3];

    for i in 0..3 {
        println!("{}{}{}", a1[i], a2[i], a3[i]);
    }
}