fn hailstone(n: i32, arry: &mut Vec<i32>) -> i32 {
    let mut hs = 1; 
    let mut n = n; 

    while n != 1 { 
        hs += 1;
        if let Some(arry) = arry.as_mut() { 
            arry.push(n);
        }
        n = if n & 1 == 1 { 
            3 * n + 1 
        } else {
            n / 2 
        };
    }
    if let Some(arry) = arry.as_mut() {
        arry.push(n);
    }
    hs 
}

fn main() {
    let mut hmax = 0; 
    let mut jatmax = 0; 

    for j in 1..100000 { 
        let n = hailstone(j, None);
        if hmax < n {
            hmax = n;
            jatmax = j;
        }
    }
    let mut arry: Vec<i32> = Vec::new(); 
    let n = hailstone(27, Some(&mut arry)); 


    println!(
        "[{}, {}, {}, {}, ...., {}, {}, {}, {}] len={}",
        arry[0], arry[1], arry[2], arry[3], arry[n as usize - 4],
        arry[n as usize - 3], arry[n as usize - 2], arry[n as usize - 1], n
    );
    println!("Max {} at j= {}", hmax, jatmax);
}