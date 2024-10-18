const DIGITS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const DIGITS_LEN: usize = 64;

fn encode_negative_base(n: i64, base: i64) -> Option<String> {
    if base > -1 || base < -62 {
        return None;
    }
    if n == 0 {
        return Some("0".to_string());
    }

    let mut result = String::new();
    let mut n = n;
    
    while n != 0 {
        let mut rem = n % base;
        n /= base;
        if rem < 0 {
            n += 1;
            rem -= base;
        }
        result.push(DIGITS.chars().nth(rem as usize).unwrap());
    }
    
    result = result.chars().rev().collect();

    Some(result)
}

fn decode_negative_base(s: &str, base: i64) -> Option<i64> {
    if base < -62 || base > -1 {
        return None;
    }

    if s.is_empty() || (s == "0") {
        return Some(0);
    }

    let mut value = 0;
    let mut bb = 1;
    for c in s.chars().rev() {
        if let Some(i) = DIGITS.find(c) {
            value += i as i64 * bb;
            bb *= base;
        } else {
            return None;
        }
    }

    Some(value)
}

fn driver(n: i64, b: i64) {
    match encode_negative_base(n, b) {
        Some(encoded) => {
            println!("{:12} encoded in base {:3} = {:12}", n, b, encoded);
            if let Some(decoded) = decode_negative_base(&encoded, b) {
                println!("{:12} decoded in base {:3} = {:12}", encoded, b, decoded);
            }
        },
        None => println!("Error encoding or decoding."),
    }
}

fn main() {
    driver(10, -2);
    driver(146, -3);
    driver(15, -10);
    driver(12, -62);
}