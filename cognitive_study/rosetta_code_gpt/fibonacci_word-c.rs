use std::f64;

fn print_headings() {
    println!("{:<2} {:<10} {:<20} {:<40}", "N", "Length", "Entropy", "Word");
}

fn calculate_entropy(ones: usize, zeros: usize) -> f64 {
    let total = ones + zeros;
    let mut result = 0.0;

    if total == 0 {
        return 0.0;
    }

    result -= (ones as f64 / total as f64) * f64::log2(ones as f64 / total as f64);
    result -= (zeros as f64 / total as f64) * f64::log2(zeros as f64 / total as f64);

    if result.is_nan() {
        return 0.0;
    }

    result
}

fn print_entropy(word: &str) {
    let ones = word.chars().filter(|&c| c == '1').count();
    let zeros = word.chars().filter(|&c| c == '0').count();
    let entropy = calculate_entropy(ones, zeros);
    print!("{:<20.18} ", entropy);
}

fn print_word(n: usize, word: &str) {
    print!("{:<2} {:<10} ", n, word.len());
    print_entropy(word);
    if n < 10 {
        println!("{:<40}", word);
    } else {
        println!("...");
    }
}

fn main() {
    print_headings();

    let mut last_word = "1".to_string();
    let mut current_word = "0".to_string();

    print_word(1, &last_word);

    for i in 2..=37 {
        print_word(i, &current_word);
        let next_word = format!("{}{}", current_word, last_word);
        last_word = current_word;
        current_word = next_word;
    }
}