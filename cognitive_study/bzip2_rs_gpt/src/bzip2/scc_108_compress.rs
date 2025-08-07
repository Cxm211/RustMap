use crate::global_vars::bzip2_c2::*;
use crate::global_vars::bzip2_c5::*;
use crate::global_vars::bzip2_c1::*;
use crate::global_vars::conversion::*;
use crate::bzip2::scc_84_applySavedTimeInfoToOutputFile_85_applySavedFileAttrToOutputFile_86_saveInputFileMetaInfo::*;
use crate::bzip2::scc_82_crcError_83_ioError::io_error;
use crate::bzip2::scc_79_root_46_setExit::set_exit;
use crate::bzip2::scc_49_fopen_output_safely::*;
use crate::bzip2::scc_87_fileExists::*;
use crate::bzip2::scc_105_hasSuffix_106_mapSuffix::*;
use crate::bzip2::scc_94_copyFileName::*;
use crate::bzip2::scc_99_compressStream::*;
use crate::bzip2::scc_55_root_40_containsDubiousChars::contains_dubious_chars;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process;
use libc::{isatty, fileno};
use std::io::stdout;  // 导入 stdout
use std::os::unix::io::AsRawFd;  // 导入 Unix-specific trait 和函数
use std::os::fd::FromRawFd;
 
pub static mut SRC_MODE: i32 = 1;


pub fn compress(name: Option<&str>) -> Result<(), io::Error> {
    // println!("Compress🔥");
     if let Some(n) = name {
        // println!("Compressing file: {}", n);
    } else {
        // println!("No file name provided");
    }

    unsafe {
        // println!("0Here💪");
        DELETE_OUTPUT_ON_INTERRUPT = false;
    
        if name.is_none() && SRC_MODE != 1 {
            panic!("compress: bad modes\n");
        }

        match SRC_MODE {
            1 => {
                IN_NAME = string_to_u8_array_1034("(stdin)".to_string());
                OUT_NAME = string_to_u8_array_1034("(stdout)".to_string());
                // stdin and stdout handling
            }
            3 => {
                IN_NAME = string_to_u8_array_1034(name.clone().unwrap_or_default().to_string());
                OUT_NAME = string_to_u8_array_1034(format!("{}.bz2", name.unwrap_or_default()));
                // normal file handling
            }
            2 => {
                IN_NAME = string_to_u8_array_1034(name.clone().unwrap_or_default().to_string());
                OUT_NAME = string_to_u8_array_1034("(stdout)".to_string());
                // stdout handling
            }
            _ => panic!("compress: bad srcMode"),
        }
    
        // Check if input file exists
        // 通过调用 as_str() 方法将这个 String 转换为字符串切片
        if SRC_MODE != 1 && !Path::new(u8_array_to_string(&IN_NAME).as_str()).exists() {
            eprintln!("{}: Can't open input file {}: No such file or directory", (u8_array_to_string(&PROG_NAME)), u8_array_to_string(&IN_NAME));
            process::exit(1);
        }

        // Open input and output files
        // let in_file = match SRC_MODE {
        //     1 => io::stdin(),
        //     _ => fs::File::open(&IN_NAME)?,
        // };
        // let out_file = match SRC_MODE {
        //     1 | 2 => io::stdout(),
        //     _ => fs::File::create(&OUT_NAME)?,
        // };

        let mut in_file: Option<File> = None;
        let mut out_file: Option<File> = None;
        if let Some(n) = name {
            // println!("Compressing file: {}", n);
            // 尝试打开文件，并将结果赋值给 in_file
            in_file = Some(File::open(n)?);
        } else {
            // println!("No file name provided");
        }
    



        if name.is_none() && SRC_MODE != 1 {
            panic!("compress: bad modes");
        }
        // println!("Here💪");
        // 这里，IN_NAME 被转换成一个 Rust 字符串，
        // 但如果 IN_NAME 是基于 C 风格的字符串（以 NUL 字节结尾），
        // 转换后的字符串也会包含这个 NUL 字节。
        let in_name_str = IN_NAME
            .iter()
            .take_while(|&&c| c != 0) // 只取直到 NUL 字节之前的部分
            .map(|&c| c as char)
            .collect::<String>();

        let out_name_str = format!("{}.bz2", in_name_str); // 构建输出文件名
        // println!("🐦 in_name_str: {}; out_name_str: {}",in_name_str, out_name_str);

        // 处理文件打开操作
        match File::open(Path::new(&in_name_str)) {
            Ok(file) => in_file = Some(file),
            Err(e) => {
                eprintln!("Error opening input file: {}", e);
                return Err(e);
            }
        }

        match File::create(Path::new(&out_name_str)) {
            Ok(file) => out_file = Some(file),
            Err(e) => {
                eprintln!("Error creating output file: {}", e);
                return Err(e);
            }
        }

        // println!("🌲");
        if let (Some(mut in_file), Some(mut out_file)) = (in_file, out_file) {
            // println!("🚗");
            if let Err(e) = compress_stream(&mut in_file, &mut out_file) {
                eprintln!("compress_stream error: {}", e);
                return Err(io::Error::new(io::ErrorKind::Other, "compress_stream failed"));
            }
        }

        DELETE_OUTPUT_ON_INTERRUPT = false;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};
    use std::fs::File;
    use std::path::Path;

    // 测试compress函数处理有效输入的能力
    #[test]
    fn test_compress_valid_input() {
        // let input_filename = "/root/crown-rust/bzip2/sample1.ref";
        // let output_filename = "output.txt";
        let input_filename = "/root/crown-rust/bzip2-backup/11Nov.txt";
                // 创建一个输入文件用于测试
        let mut file = File::create(input_filename).unwrap();
        writeln!(file, "1").unwrap();
        // 设置测试环境.
        unsafe{
            SRC_MODE = 3; // 假设 3 代表正常模式
        }
        let mut in_name_array = [0; 1034];
        let in_bytes = input_filename.as_bytes();
        in_name_array[..in_bytes.len()].copy_from_slice(in_bytes);
        unsafe{
            IN_NAME = in_name_array; // 将Vec转换为固定大小数组
        }

        // let mut out_name_array = [0; 1034];
        // let out_bytes = output_filename.as_bytes();
        // out_name_array[..out_bytes.len()].copy_from_slice(out_bytes);
        // unsafe{
        //     OUT_NAME = out_name_array; // 将Vec转换为固定大小数组
        // }

        // 执行compress函数

        let result = compress(Some(input_filename));
        // assert!(result.is_ok());

        // 验证输出文件是否存在
        // assert!(Path::new(output_filename).exists());

        // 清理测试文件
        // std::fs::remove_file(input_filename).unwrap();
        // std::fs::remove_file(output_filename).unwrap();
    }

    // 测试compress函数处理无效输入的能力
    #[test]
    fn test_compress_invalid_input() {
        let result = compress(None);
        assert!(result.is_err());
    }

    // 更多测试用例...
}


