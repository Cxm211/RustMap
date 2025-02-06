use crate::global_vars::bzip2_c1::{UInt64};



fn u_int64_is_zero(n: &UInt64) -> bool {
    // 遍历这8个字节
    for &byte in n.b.iter() {
        // 如果任意一个字节不为0，返回false
        if byte != 0 {
            return false;
        }
    }
    // 所有字节都是0，返回true
    true
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u_int64_is_zero_true() {
        let n = UInt64 {
            b: [0, 0, 0, 0, 0, 0, 0, 0],
        };
        assert!(u_int64_is_zero(&n));
    }

    #[test]
    fn test_u_int64_is_zero_false() {
        let n = UInt64 {
            b: [0, 0, 0, 0, 0, 1, 0, 0],
        };
        assert!(!u_int64_is_zero(&n));
    }
}
