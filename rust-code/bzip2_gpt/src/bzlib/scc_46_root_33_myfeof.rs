use std::io::{Read, Write, Seek, SeekFrom};
use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;

pub fn myfeof(f: &mut File) -> bool {
    let mut buf = [0u8; 1]; // 用于读取的缓冲区
    match f.read(&mut buf) {
        Ok(0) => true, // 读取到0字节表示文件结束
        Ok(_) => {
            // 将字节移回文件流
            f.seek(SeekFrom::Current(-1)).unwrap();
            false
        },
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::fs::OpenOptions;

    // 测试`myfeof`函数
    #[test]
    fn test_myfeof() {
        // 创建一个临时文件
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("temp.txt")
            .unwrap();

        // 写入一些数据
        f.write_all(b"test").unwrap();

        // 回到文件的开头
        f.seek(SeekFrom::Start(0)).unwrap();

        // 确保文件没有到结尾
        assert_eq!(myfeof(&mut f), false);

        // 读取文件的所有内容
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        // 确保文件已经到结尾
        assert_eq!(myfeof(&mut f), true);

        // 关闭文件
        // 在Rust中, 当`f`离开作用域时，文件会自动关闭。
    }
}
