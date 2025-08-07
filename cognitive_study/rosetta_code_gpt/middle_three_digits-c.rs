fn mid3(n: i32) -> Option<String> {
    let s = n.abs().to_string();
    let len = s.len();
    if len < 3 || len % 2 == 0 {
        return None;
    }
    let start = len / 2 - 1;
    let end = start + 3;
    Some(s[start..end].to_string())
}

fn main() {
    let x = [
        123, 12345, 1234567, 987654321, 10001, -10001, -123, -100, 100, -12345,
        1, 2, -1, -10, 2002, -2002, 0, 1234567890,
    ];

    for &n in x.iter() {
        match mid3(n) {
            Some(m) => println!("{}: {}", n, m),
            None => println!("{}: error", n),
        }
    }
}