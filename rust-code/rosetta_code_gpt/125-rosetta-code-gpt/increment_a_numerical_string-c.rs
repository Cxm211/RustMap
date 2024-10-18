fn incr(s: &mut String) -> &str {
    let mut len = s.len();
    let is_negative = s.starts_with('-');
    let target = if is_negative { '0' } else { '9' };
    
    if s == "-1" {
        *s = "0".to_string();
        return s;
    }
    
    let start = if is_negative || s.starts_with('+') { 1 } else { 0 };

    let mut tail = len - 1;
    while tail >= start && s.as_bytes()[tail] == target as u8 {
        if tail == 0 { break }
        tail -= 1;
    }

    if tail < start && !is_negative {
        s.clear();
        s.push('1');
        s.push_str(&"0".repeat(len - start));
    } else if tail == start && is_negative && &s[1..2] == "1" {
        *s = format!("-{}", "9".repeat(len - start - 1));
    } else {
        let prev_digit = s.as_bytes()[tail] as char;
        s.replace_range(tail..tail+1, &((prev_digit as u8 + if is_negative { -1 } else { 1 }) as char).to_string());
        for i in tail + 1..len {
            s.replace_range(i..i+1, &if is_negative { "9" } else { "0" });
        }
    }

    s
}

fn string_test(s: &str) {
    let mut input = s.to_string();
    println!("text: {}", input);
    println!("  ->: {}", incr(&mut input));
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