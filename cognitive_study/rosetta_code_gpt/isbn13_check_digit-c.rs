fn check_isbn13(isbn: &str) -> bool {
    let mut count = 0;
    let mut sum = 0;

    for (idx, ch) in isbn.chars().enumerate() {
        match ch {
            ' ' | '-' => continue,
            '0'..='9' => {
                let digit = ch.to_digit(10).unwrap();
                if idx % 2 == 0 {
                    sum += digit;
                } else {
                    sum += 3 * digit;
                }
                count += 1;
            }
            _ => return false,
        }
    }

    if count != 13 {
        return false;
    }
    sum % 10 == 0
}

fn main() {
    let isbns = [
        "978-1734314502",
        "978-1734314509",
        "978-1788399081",
        "978-1788399083",
    ];

    for &isbn in &isbns {
        println!("{}: {}", isbn, if check_isbn13(isbn) { "good" } else { "bad" });
    }
}