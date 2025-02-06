
#[cfg(unix)]
pub fn contains_dubious_chars(name: &str) -> bool {
    // 在Unix平台上，文件名可以包含任何字符
    false
}

#[cfg(not(unix))]
pub fn contains_dubious_chars(name: &str) -> bool {
    // 在非Unix平台（如Windows）上，不允许文件名中包含通配符
    name.chars().any(|c| c == '?' || c == '*')
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_dubious_chars() {
        // 因为函数总是返回false，所以任何输入都应该返回false
        assert_eq!(contains_dubious_chars("someName"), false);
        assert_eq!(contains_dubious_chars("anotherName"), false);
    }
}
