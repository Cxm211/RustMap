
use std::io::Write;
use std::fs::File;
use std::io::prelude::*;
use crate::global_vars::bzip2_c1::*;
pub fn pad(s: &str, output: &mut dyn Write) {
    let s_len = s.len() as i32; 

    unsafe {
        if s_len >= LONGEST_FILE_NAME {
            return;
        }
        let spaces_to_print = LONGEST_FILE_NAME - s_len;
        let spaces = " ".repeat(spaces_to_print as usize);
        output.write_all(spaces.as_bytes()).unwrap();
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad() {
        // 假设longestFileName为10用于测试
        unsafe { LONGEST_FILE_NAME = 10; }
    
        // 打开一个临时文件用于写入
        let mut tmp_file = File::create("tmp.txt").expect("无法打开临时文件");
    
        // 调用pad函数来输出空格到临时文件
        pad("test", &mut tmp_file);
    
        // 读取临时文件的内容
        let mut tmp = File::open("tmp.txt").expect("无法读取临时文件");
        let mut buffer = String::new();
        tmp.read_to_string(&mut buffer).unwrap();
    
        // 断言：检查读取的内容是否是6个空格
        assert_eq!(buffer, "      ");
    
        // 删除临时文件
        std::fs::remove_file("tmp.txt").unwrap();
    }
}







