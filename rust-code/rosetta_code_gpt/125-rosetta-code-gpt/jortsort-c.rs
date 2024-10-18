use std::env;
use std::str::FromStr;

// 计算数字的位数
fn number_of_digits(x: i32) -> i32 {
    let mut num = x;
    let mut number_of_digits = 0;
    while num != 0 {
        num /= 10;
        number_of_digits += 1;
    }
    number_of_digits
}

// 将字符数组转换为整数数组
fn convert_array(array: &str, number_of_elements: usize) -> Vec<i32> {
    let mut converted_array = Vec::with_capacity(number_of_elements);
    let mut original_element = 0;
    let chars: Vec<char> = array.chars().collect();

    for _ in 0..number_of_elements {
        let slice = &array[original_element..];
        if let Ok(converted_element) = i32::from_str(slice) {
            converted_array.push(converted_element);
            original_element += number_of_digits(converted_element) as usize + 1;
        } else {
            break;
        }
    }
    converted_array
}

// 判断数组是否已排序
fn is_sorted(array: &[i32]) -> bool {
    for i in 1..array.len() {
        if array[i - 1] > array[i] {
            return false;
        }
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide the array as an argument.");
        return;
    }

    let converted_array = convert_array(&args[1], args.len() - 1);

    if is_sorted(&converted_array) {
        println!("Did you forgot to turn on your brain?! This array is already sorted!");
    } else if args.len() - 1 <= 10 {
        println!("Am I really supposed to sort this? Sort it by yourself!");
    } else {
        println!("Am I really supposed to sort this? Bhahahaha!");
    }
}
