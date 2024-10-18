// 该函数获取三个u8中的中间值。
// 为了与C语言的UChar保持一致，这里我们使用u8。
pub fn mmed3(a: u8, b: u8, c: u8) -> u8 {
    let mut a = a;
    let mut b = b;
    if a > b {
        std::mem::swap(&mut a, &mut b); // 使用std::mem::swap进行交换
    }
    if b > c {
        b = c;
        if a > b {
            b = a;
        }
    }
    b // 返回中间值
}

#[cfg(test)]
mod tests {
    use super::mmed3;

    #[test]
    fn test_mmed3() {
        assert_eq!(mmed3(1, 2, 3), 2); // 中间值是2
        assert_eq!(mmed3(3, 1, 2), 2); // 中间值是2
        assert_eq!(mmed3(2, 3, 1), 2); // 中间值是2
        assert_eq!(mmed3(5, 5, 5), 5); // 所有值都相同
        assert_eq!(mmed3(0, 255, 128), 128); // 测试极限值
    }
}

fn main() {
    // Rust的测试通常在测试模式下通过cargo test命令运行，所以main函数可以留空或用于其他目的
}
