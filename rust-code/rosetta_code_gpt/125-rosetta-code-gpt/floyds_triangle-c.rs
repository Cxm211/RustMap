fn print_triangle(n: usize) {
    let mut i = n * (n - 1) / 2;
    let mut len = 1;
    let mut c = 1;
    while c < i {
        c *= 10;
        len += 1;
    }
    c -= i;

    /*
    let mut s = String::new();
    let mut num = i;
    for i in 1..=n {
        for j in 1..=i {
            num += 1;
            let width = if j < c { len - 1 } else { len };
            s += &format!("{:width$}", num, width = width);
            s.push(if i == j { '\n' } else { ' ' });
        }
    }
    print!("{}", s);
    */

    let mut num = i;
    for i in 1..=n {
        for j in 1..=i {
            num += 1;
            let width = if j < c { len - 1 } else { len };
            print!("{:width$}", num, width = width);
            print!("{}", if i == j { '\n' } else { ' ' });
        }
    }
}

fn main() {
    print_triangle(5);
    print_triangle(14);
}