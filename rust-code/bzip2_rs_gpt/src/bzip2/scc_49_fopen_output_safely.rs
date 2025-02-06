

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::process::Command;

pub fn fopen_output_safely(name: &str, mode: &str) -> Option<std::fs::File> {
    if mode != "w" {
        return None;
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(name);

    match file {
        Ok(f) => Some(f),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_fopen_output_safely() {
        // 使用命令行真的创建一个test.txt
        let _output = Command::new("touch").arg("test.txt").output().expect("Failed to create test.txt using touch");

        // 使用fopen_output_safely打开文件
        let file = fopen_output_safely("test.txt", "w");
        assert!(file.is_some()); // 断言文件指针不为None，表示文件成功打开
        drop(file); // 明确关闭文件

        // 确保文件确实被创建，我们再次使用open来打开它
        let file = fs::File::open("test.txt");
        assert!(file.is_ok()); // 断言文件指针不为None，表示文件确实存在并被成功打开

        // 使用命令行真的删除test.txt
        let _output = Command::new("rm").arg("test.txt").output().expect("Failed to remove test.txt using rm");
    }
}
