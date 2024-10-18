fn rprint(s: Option<&mut String>, x: &[i32]) -> usize {
    let mut length = 0;
    let mut i = 0;
    while i < x.len() {
        let mut j = i;
        while j + 1 < x.len() && x[j + 1] == x[j] + 1 {
            j += 1;
        }

        if i + 1 < j {
            if let Some(ref mut s) = s {
                if !s.is_empty() {
                    s.push(',');
                }
                let part = format!("{}-{}", x[i], x[j]);
                length += part.len();
                s.push_str(&part);
            }
        } else {
            while i <= j {
                if let Some(ref mut s) = s {
                    if !s.is_empty() {
                        s.push(',');
                    }
                    s.push_str(&x[i].to_string());
                    length += x[i].to_string().len();
                }
                i += 1;
            }
            continue;
        }
        i = j + 1;
    }
    length
}

fn main() {
    let x = [0, 1, 2, 4, 6, 7, 8, 11, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 27, 28, 29, 30, 31, 32, 33, 35, 36, 37, 38, 39];
    let s = Some(&mut String::new());
    let _ = rprint(s, &x);
}
