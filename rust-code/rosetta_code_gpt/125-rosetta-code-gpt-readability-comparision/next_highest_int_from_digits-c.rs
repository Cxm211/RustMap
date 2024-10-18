fn swap(str: &mut [char], i: usize, j: usize) {
    str.swap(i, j);
}

fn reverse(str: &mut [char], i: usize, j: usize) {
    str[i..=j].reverse();
}

fn next_permutation(str: &mut [char]) -> bool {
    let len = str.len();
    if len < 2 {
        return false;
    }
    for i in (1..len).rev() {
        let j = i;
        if str[i - 1] < str[j] {
            let mut k = len;
            while str[i - 1] >= str[k - 1] {
                k -= 1;
            }
            swap(str, i - 1, k - 1);
            reverse(str, j, len - 1);
            return true;
        }
    }
    false
}

fn next_highest_int(n: u32) -> Option<u32> {
    let mut str: Vec<char> = n.to_string().chars().collect();
    if !next_permutation(&mut str) {
        return None;
    }
    str.iter().collect::<String>().parse::<u32>().ok()
}

fn main() {
    let numbers = [0, 9, 12, 21, 12453, 738440, 45072010, 95322020];
    for &n in numbers.iter() {
        match next_highest_int(n) {
            Some(next) => println!("{} -> {}", n, next),
            None => println!("{} -> None", n),
        }
    }

    let big = "9589776899767587796600".to_string();
    let mut next = big.clone();
    let next_chars: &mut [char] = &mut next.chars().collect::<Vec<_>>();
    if next_permutation(next_chars) {
        println!("{} -> {}", big, next_chars.iter().collect::<String>());
    } else {
        println!("{} -> None", big);
    }
}