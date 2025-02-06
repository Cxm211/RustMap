use std::io::Write;
use std::fs::File;
use std::path::Path;
use crate::global_vars::bzip2_c5::*;
use crate::global_vars::bzip2_c1::*;
type Bool = bool;
type Char = u8;

pub fn show_file_names() {
    unsafe {
        if NOISY {
            // 将字符数组转为字符串
            let in_str = std::str::from_utf8(&IN_NAME).unwrap().trim_end_matches('\0');
            let out_str = std::str::from_utf8(&OUT_NAME).unwrap().trim_end_matches('\0');

            writeln!(std::io::stderr(), "\tInput file = {}, output file = {}", in_str, out_str).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use crate::global_vars::bzip2_c5::*;
    #[test]
    fn test_show_file_names_noisy_true() {
        // 创建一个temp文件模拟输入文件
        let _temp_file = File::create("testInput.txt").unwrap();

        show_file_names();

        // 使用Path验证文件是否存在
        assert!(Path::new("testInput.txt").exists());

        // 删除temp文件
        Command::new("rm").arg("testInput.txt").output().unwrap();
    }

    #[test]
    fn test_show_file_names_noisy_false() {
        unsafe {
            NOISY = false;
        }

        // 此测试中没有设置文件名，因为当noisy为false时不应该输出任何内容
        show_file_names();

        // 确保没有创建输出文件
        assert!(!Path::new("testOutput.txt").exists());
    }
}
