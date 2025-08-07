fn add_number(x: i32) {
    print!("{} ", x);
}

fn add_range(x: i32, y: i32) -> bool {
    if y <= x {
        return false;
    }
    for i in x..=y {
        print!("{} ", i);
    }
    true
}

fn get_number(s: &str) -> Option<(i32, usize)> {
    s.trim_start()
        .parse::<i32>()
        .ok()
        .map(|x| (x, s.trim_start().len() - s.trim_start().trim_start_matches(x).len()))
}

fn get_rnge(s: &str) -> Option<usize> {
    let mut split = s.splitn(2, '-');
    if let Some(start) = split.next() {
        if let Some((x, len)) = get_number(start) {
            if let Some(end) = split.next() {
                if let Some((y, _)) = get_number(end) {
                    if add_range(x, y) {
                        return Some(len + 1 + end.len() - end.trim_start_matches(y).len());
                    }
                }
            }
        }
    }
    None
}

fn get_list(s: &str) -> bool {
    let mut index = 0;
    loop {
        let s = &s[index..];
        if let Some(len) = get_rnge(s) {
            index += len;
        } else if let Some((x, len)) = get_number(s) {
            add_number(x);
            index += len;
        } else {
            break;
        }

        let s = &s[index..].trim_start();
        index = s.len() - s.trim_start_matches(',').len();
        if s.starts_with(',') {
            index += 1;
            continue;
        }
        if s.is_empty() {
            println!();
            return true;
        }
        break;
    }
    println!("\nSyntax error at {}", &s[index..]);
    false
}

fn main() {
    if get_list("-6,-3--1,3-5,7-11,14,15,17-20") {
        println!("Ok");
    }
    get_list("-6 -3--1,3-5,7-11,14,15,17-20");
}