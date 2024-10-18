fn hailstone(mut n: i32, arry: &mut Vec<i32>) -> usize {
    let mut hs = 1;
    
    while n != 1 {
        hs += 1;
        if !arry.is_empty() {
            arry.push(n);
        }
        n = if n & 1 == 1 { 3 * n + 1 } else { n / 2 };
    }
    if !arry.is_empty() {
        arry.push(n);
    }
    hs
}

fn main() {
    let mut hmax = 0;
    let mut jatmax = 0;
    let mut n;

    for j in 1..100_000 {
        n = hailstone(j, &mut vec![]);
        if hmax < n {
            hmax = n;
            jatmax = j;
        }
    }
    let mut arry = Vec::new();
    n = hailstone(27, &mut arry);

    println!("[ {}, {}, {}, {}, ...., {}, {}, {}, {}] len={}",
        arry[0], arry[1], arry[2], arry[3],
        arry[n - 4], arry[n - 3], arry[n - 2], arry[n - 1], n);

    println!("Max {} at j= {}", hmax, jatmax);
}