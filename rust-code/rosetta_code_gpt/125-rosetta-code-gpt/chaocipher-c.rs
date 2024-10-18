use std::collections::VecDeque;

#[derive(Clone, Copy)]
enum CMode {
    Encrypt,
    Decrypt,
}

fn chao(input: &str, output: &mut VecDeque<char>, mode: CMode, show_steps: bool) {
    let mut left: VecDeque<char> = "HXUCZVAMDSLKPEFJRIGTWOBNYQ".chars().collect();
    let mut right: VecDeque<char> = "PTLNBQDEOYSFAVZKGJRIHWXUMC".chars().collect();
    let mut temp: VecDeque<char> = VecDeque::with_capacity(26);

    for ch in input.chars() {
        if show_steps {
            println!("{:?}  {:?}", left, right);
        }

        let index = match mode {
            CMode::Encrypt => right.iter().position(|&r| r == ch).unwrap(),
            CMode::Decrypt => left.iter().position(|&l| l == ch).unwrap(),
        };

        let out_char = match mode {
            CMode::Encrypt => left[index],
            CMode::Decrypt => right[index],
        };

        output.push_back(out_char);

        temp.extend(left.split_off(index));
        temp.append(&mut left);
        temp.rotate_left(1);
        temp.swap(1, 12);
        left.append(&mut temp);

        temp.extend(right.split_off(index));
        temp.append(&mut right);
        temp.swap(0, 24);
        temp.swap(2, 12);
        right.append(&mut temp);
    }
}

fn main() {
    let plain_text = "WELLDONEISBETTERTHANWELLSAID";
    let mut cipher_text = VecDeque::new();
    let mut recovered_text = VecDeque::new();

    println!("The original plaintext is: {}", plain_text);
    println!("The left and right alphabets after each permutation during encryption are:");
    chao(plain_text, &mut cipher_text, CMode::Encrypt, true);
    let cipher_str: String = cipher_text.iter().collect();
    println!("\nThe ciphertext is: {}", cipher_str);

    chao(&cipher_str, &mut recovered_text, CMode::Decrypt, false);
    let recovered_str: String = recovered_text.iter().collect();
    println!("\nThe recovered plaintext is: {}", recovered_str);
}