fn incr(mut s: String) -> String {
    let len = s.len();
    let neg = s.starts_with('-');
    let tgt = if neg { '0' } else { '9' };

    // Special case: "-1"
    if s == "-1" {
        return "0".to_string();
    }

    let begin = if s.starts_with('-') || s.starts_with('+') {
        1
    } else {
        0
    };

    // Find out how many digits need to be changed
    let mut tail = len - 1;
    while tail >= begin && s.chars().nth(tail).unwrap() == tgt {
        tail -= 1;
    }

    if tail < begin && !neg {
        // Special case: all 9s, string will grow
        if begin == 0 {
            s = "1".to_string() + &"0".repeat(len);
        } else {
            s = "+1".to_string() + &"0".repeat(len - 1);
        }
    } else if tail == begin && neg && s.chars().nth(1).unwrap() == '1' {
        // Special case: -1000..., so string will shrink
        s = "-".to_string() + &"9".repeat(len - begin - 1);
    } else {
        // Normal case; change tail to all 0 or 9, change prev digit by 1
        let tail_char = s.chars().nth(tail).unwrap();
        let updated_tail_char = ((tail_char as u8) + if neg { -1 } else { 1 }) as char;
        s.replace_range(tail..len, &"0".repeat(len - tail));
        s.replace_range(tail..=tail, &updated_tail_char.to_string());
    }

    s
}

fn string_test(s: &str) {
    let mut ret = s.to_string();
    println!("text: {}", ret);
    ret = incr(ret);
    println!("  ->: {}", ret);
}

fn main() {
    string_test("+0");
    string_test("-1");
    string_test("-41");
    string_test("+41");
    string_test("999");
    string_test("+999");
    string_test("109999999999999999999999999999999999999999");
    string_test("-100000000000000000000000000000000000000000000");
}