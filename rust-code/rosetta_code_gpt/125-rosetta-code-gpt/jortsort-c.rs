fn number_of_digits(mut x: i32) -> i32 {
    let mut number_of_digits = 0;
    while x != 0 {
        number_of_digits += 1;
        x /= 10;
    }
    number_of_digits
}

fn convert_array(array: Vec<&str>) -> Vec<i32> {
    let mut converted_array = vec![];
    for &element in &array {
        let converted_element: i32 = element.parse().expect("Failed to parse integer");
        converted_array.push(converted_element);
    }
    converted_array
}

fn is_sorted(array: &[i32]) -> bool {
    for i in