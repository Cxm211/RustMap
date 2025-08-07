fn gray_encode(n: i32) -> i32 {
    n ^ (n >> 1)
}

fn gray_decode(n: i32) -> i32 {
    let mut p = n;
    let mut n = n;
    while n >>= 1 {
        p ^= n;
    }
    p
}

fn fmtbool(n: i32) -> String {
    let mut n = n;
    let mut buf = vec!['0'; 5];
    for b in buf.iter_mut().rev() {
        *b = std::char::from_digit((n & 1) as u32, 10).unwrap();
        n >>= 1;
    }
    buf.into_iter().collect()
}

fn main() {
    for i in 0..32 {
        let g = gray_encode(i);
        let b = gray_decode(g);
        let bi = fmtbool(i);
        let bg = fmtbool(g);
        let bb = fmtbool(b);
        println!("{:2} : {:5} => {:5} => {:5} : {:2}", i, bi, bg, bb, b);
    }
}