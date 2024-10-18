use std::fs::File;
use std::path::Path;
use std::io::Write;

// 检查给定文件名的文件是否存在
pub fn file_exists(name: &str) -> bool {
    // 尝试打开文件
    let file = File::open(name);
    
    // 根据结果判断文件是否存在
    match file {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_file_exists() {
        // 确保在当前目录下创建一个名为"testfile.txt"的文件
        let mut file = File::create("testfile.txt").unwrap();
        writeln!(file, "Test content.").unwrap();

        // 使用Path获取testfile.txt的完整路径
        let testfile_path = Path::new("testfile.txt").display().to_string();
        
        assert!(file_exists(&testfile_path));

        // 假设当前目录下没有一个名为"nonexistent.txt"的文件
        assert!(!file_exists("nonexistent.txt"));

        // 删除创建的测试文件
        fs::remove_file("testfile.txt").unwrap();
    }
}

fn main() {
    // 可以在这里写主函数的内容，但目前C代码的主函数只是运行测试，而Rust的测试通过`cargo test`运行，所以main函数可以保持空白。
}
