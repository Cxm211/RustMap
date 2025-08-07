#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ReplaceInfo {
    n: i32,
    text: String,
}

fn generic_fizz_buzz(max: i32, mut info: Vec<ReplaceInfo>) {
    info.sort();
    
    for i in 1..max {
        let mut found_word = false;
        
        for it in &info {
            if i % it.n == 0 {
                print!("{}", it.text);
                found_word = true;
            }
        }
        
        if !found_word {
            print!("{}", i);
        }
        
        println!();
    }
}

fn main() {
    let info = vec![
        ReplaceInfo { n: 5, text: "Buzz".to_string() },
        ReplaceInfo { n: 7, text: "Baxx".to_string() },
        ReplaceInfo { n: 3, text: "Fizz".to_string() },
    ];
    
    generic_fizz_buzz(20, info);
}