
use crate::bzip2::scc_10_root_12_uInt64_qrm10::*;
use crate::global_vars::bzip2_c1::{UInt64};



fn uInt64_isZero(n: &UInt64) -> bool {
    for &byte in &n.b {
        if byte != 0 {
            return false;
        }
    }
    true
}

fn uInt64_toAscii(outbuf: &mut [char; 32], n: &UInt64) {
    let mut q;
    let mut buf = ['\0'; 32]; // 初始化为null字符
    let mut nBuf = 0usize;
    let mut n_copy = *n;

    // 循环直到n_copy为0
    while !uInt64_isZero(&n_copy) {
        q = uInt64_qrm10(&mut n_copy);
        buf[nBuf] = (q + '0' as i32) as u8 as char;
        nBuf += 1;
    }

    outbuf[nBuf] = '\0'; // 设置字符串结束标志
    for i in 0..nBuf {
        outbuf[i] = buf[nBuf - i - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uInt64_toAscii() {
        let number = UInt64 {
            b: [12, 0, 0, 0, 0, 0, 0, 0],
        };
        let mut output = ['\0'; 32];
        uInt64_toAscii(&mut output, &number);

        let output_string: String = output.iter().take_while(|&&c| c != '\0').collect();
        assert_eq!(output_string, "12");
    }
}

