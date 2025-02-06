use crate::global_vars::bzip2_c1::{UInt64};



pub fn uInt64_qrm10(n: &mut UInt64) -> i32 {
    let mut rem: u32 = 0;
    for byte in &mut n.b.iter_mut().rev() {  // 使用iter_mut()和rev()迭代数组
        let tmp = rem * 256 + (*byte as u32);
        *byte = (tmp / 10) as u8;
        rem = tmp % 10;
    }
    rem as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uInt64_qrm10_zero_remainder() {
        let mut n = UInt64 { b: [10, 0, 0, 0, 0, 0, 0, 0] };
        let rem = uInt64_qrm10(&mut n);
        assert_eq!(rem, 0);
        assert_eq!(n.b, [1, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_uInt64_qrm10_non_zero_remainder() {
        let mut n = UInt64 { b: [255, 0, 0, 0, 0, 0, 0, 0] };
        let rem = uInt64_qrm10(&mut n);
        assert_eq!(rem, 5);
        assert_eq!(n.b, [25, 0, 0, 0, 0, 0, 0, 0]);
    }
    
}

