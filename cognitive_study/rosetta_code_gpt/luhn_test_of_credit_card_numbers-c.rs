fn main() {
    let credit_cards = [
        "49927398716",
        "49927398717",
        "1234567812345678",
        "1234567812345670",
    ];

    for &cc in credit_cards.iter() {
        println!("{:16}\t{}", cc, if luhn(cc) { "ok" } else { "not ok" });
    }
}

fn luhn(cc: &str) -> bool {
    let m = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
    let mut odd = true;
    let mut sum = 0;

    for digit_char in cc.chars().rev() {
        let digit = digit_char.to_digit(10).unwrap() as usize;
        sum += if odd { digit } else { m[digit] };
        odd = !odd;
    }

    sum % 10 == 0
}