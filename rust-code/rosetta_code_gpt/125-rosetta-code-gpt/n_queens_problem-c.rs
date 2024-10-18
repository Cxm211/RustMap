fn main() {
    for i in 1..=10 { // 使用1到10的范围
        print!("{}", i);
        if i == 10 {
            println!(); // 如果是最后一个整数，打印换行符
        } else {
            print!(", "); // 否则，打印逗号和空格
        }
    }
}