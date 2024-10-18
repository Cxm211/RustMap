fn quib(strs: &[&str]) -> String {
    let size = strs.len();
    let mut len = 3 + if size > 1 { 2 * size + 1 } else { 0 };

    for s in strs {
        len += s.len();
    }

    let mut result = String::with_capacity(len);
    result.push('{');

    match size {
        0 => {}
        1 => result.push_str(strs[0]),
        _ => {
            for i in 0..size - 1 {
                result.push_str(strs[i]);
                if i < size - 2 {
                    result.push_str(", ");
                } else {
                    result.push_str(" and ");
                }
            }
            result.push_str(strs[size - 1]);
        }
    }

    result.push('}');
    result
}

fn main() {
    let test = ["ABC", "DEF", "G", "H"];
    for i in 0..=4 {
        let s = quib(&test[0..i]);
        println!("{}", s);
    }
}