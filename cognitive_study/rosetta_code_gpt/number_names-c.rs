fn say_hundred(s: &str, len: usize, _depth: usize, mut has_lead: bool) -> bool {
    let mut c = [0; 3];
    for i in 0..3 {
        if len + i >= 3 {
            c[2 - i] = s.chars().nth(len - 3 + i).unwrap() as i32 - '0' as i32;
        }
    }
    if c.iter().sum::<i32>() == 0 {
        return false;
    }

    if c[0] > 0 {
        print!("{} hundred", ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"][c[0] as usize]);
        has_lead = true;
    }

    if has_lead && (c[1] > 0 || c[2] > 0) {
        if c[0] > 0 {
            print!(" and ");
        }
    }

    if c[1] < 2 {
        if c[1] > 0 || c[2] > 0 {
            print!("{}", ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"][((c[1] * 10) + c[2]) as usize]);
        }
    } else {
        if c[1] > 0 {
            print!("{}", ["zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"][c[1] as usize]);
            if c[2] > 0 {
                print!("-");
            }
        }
        if c[2] > 0 {
            print!("{}", ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"][c[2] as usize]);
        }
    }

    true
}

fn main() {
    say_hundred("123", 3, 0, false);
    println!();
}