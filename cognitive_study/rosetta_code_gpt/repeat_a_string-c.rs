fn string_repeat(n: usize, s: &str) -> String {
    s.repeat(n)
}

fn main() {
    let result = string_repeat(5, "ha");
    println!("{}", result);
}