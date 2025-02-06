use crate::global_vars::bzip2_c1::*;

pub fn ends_in_bz2(name: &str) -> bool {
    // 获取name的长度
    let n: usize = name.len();

    // 如果name的长度小于等于4，则直接返回假（False）
    if n <= 4 {
        return false;
    }

    // 检查name是否以“.bz2”结尾，如果是则返回真（True），否则返回假（False）
    return &name[(n - 4)..] == ".bz2";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ends_in_bz2() {
        assert_eq!(ends_in_bz2("file.bz2"), true);         // 期望为真
        assert_eq!(ends_in_bz2("file.bz"), false);         // 期望为假
        assert_eq!(ends_in_bz2("bz2"), false);             // 期望为假
        assert_eq!(ends_in_bz2(".bz2"), false);             // 期望为真
        assert_eq!(ends_in_bz2("example.txt"), false);     // 期望为假
    }
}
