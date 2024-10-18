use std::fs;
use std::os::unix::fs::MetadataExt;
// 统计一个文件或目录的硬链接数量
// 参数: name - 需要统计的文件或目录的名字
// 返回值: 硬链接数量（减去文件本身的一个链接）
pub fn count_hard_links(name: &str) -> i32 {
    match fs::metadata(name) {
        Ok(metadata) => (metadata.nlink() as i32) - 1,
        Err(_) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_count_hard_links() {
        // 清理现有的文件
        let _ = fs::remove_file("temp_file");
        let _ = fs::remove_file("temp_file_link");

        // 创建一个临时文件并测试其硬链接数
        File::create("temp_file").unwrap();

        // 确保新创建的文件只有一个硬链接
        assert_eq!(count_hard_links("temp_file"), 0);

        // 创建硬链接
        fs::hard_link("temp_file", "temp_file_link").unwrap();  // 这里使用hard_link

        // 检查硬链接是否真的被创建
        assert!(fs::metadata("temp_file_link").is_ok());
        assert_eq!(fs::metadata("temp_file_link").unwrap().nlink() as i32, 2);

        // 检查硬链接数量
        assert_eq!(count_hard_links("temp_file"), 1);

        // 清理
        fs::remove_file("temp_file_link").unwrap();
        fs::remove_file("temp_file").unwrap();
    }
}
