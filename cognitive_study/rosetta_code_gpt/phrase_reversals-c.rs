fn reverse_section(s: &mut [u8], length: usize) {
    if length == 0 {
        return;
    }
    let mid = length / 2;
    for i in 0..=mid {
        s.swap(i, length - i);
    }
}

fn reverse_words_in_order(s: &mut [u8], delim: u8) {
    if s.is_empty() {
        return;
    }
    let mut i = 0;
    while i < s.len() {
        let mut j = 0;
        while s.get(i + j).unwrap_or(&0) != &0 && s.get(i + j).unwrap_or(&delim) != &delim {
            j += 1;
        }
        reverse_section(&mut s[i..i + j], j - 1);
        i += j + 1;
    }
}

fn reverse_string(s: &mut [u8]) {
    if !s.is_empty() {
        reverse_section(s, s.len() - 1);
    }
}

fn reverse_order_of_words(s: &mut [u8], delim: u8) {
    reverse_string(s);
    reverse_words_in_order(s, delim);
}

fn main() {
    let mut str = b"rosetta code phrase reversal".to_vec();
    let delim = b' ';

    println!("Original:       \"{}\"", String::from_utf8_lossy(&str));

    let mut scopy = str.clone();
    reverse_string(&mut scopy);
    println!("Reversed:       \"{}\"", String::from_utf8_lossy(&scopy));

    let mut scopy = str.clone();
    reverse_words_in_order(&mut scopy, delim);
    println!("Reversed words: \"{}\"", String::from_utf8_lossy(&scopy));

    let mut scopy = str.clone();
    reverse_order_of_words(&mut scopy, delim);
    println!("Reversed order: \"{}\"", String::from_utf8_lossy(&scopy));
}