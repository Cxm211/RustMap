fn decode(roman: &str) -> i32 {
    let digits = [0, 0, 100, 500, 0, 0, 0, 0, 1, 1, 0, 50, 1000, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 10, 0, 0];
    
    let value = |x: char| -> i32 { digits[(x.to_ascii_uppercase() as u8 - b'A') as usize] };

    let mut arabic = 0;
    let mut chars = roman.chars().peekable();
    while let Some(current_char) = chars.next() {
        let mut current = value(current_char);
        
        while let Some(&next_char) = chars.peek() {
            let next = value(next_char);
            if next <= current {
                break;
            }
            current = next;
            arabic -= value(chars.next().unwrap());
        }

        arabic += current;
    }

    arabic
}

fn main() {
    let romans = ["MCmxC", "MMVIII", "MDClXVI", "MCXLUJ"];
    
    for roman in &romans {
        println!("{}\t{}", roman, decode(roman));
    }
}