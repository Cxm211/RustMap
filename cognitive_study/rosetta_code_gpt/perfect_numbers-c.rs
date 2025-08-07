fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Enter an argument. Example 1234 or dcba:");
        return;
    }
    
    let mut arg1 = args[1].as_bytes().to_owned();
    let x = arg1.len();

    for f in 0..x {
        for v in (f + 1..x).rev() {
            if arg1[v - 1] > arg1[v] {
                arg1.swap(v - 1, v);
            }
        }
    }

    let mut k = 0;
    let mut fact = 1;
    while k < x {
        fact *= k + 1;
        k += 1;
    }
    
    let mut y = 0;
    while y != fact {
        println!("{}", String::from_utf8_lossy(&arg1));
        let mut i = x - 2;
        while arg1[i] >= arg1[i + 1] {
            i -= 1;
        }
        let mut j = x - 1;
        while arg1[j] <= arg1[i] {
            j -= 1;
        }
        arg1.swap(i, j);
        arg1[i + 1..].reverse();
        y += 1;
    }
}