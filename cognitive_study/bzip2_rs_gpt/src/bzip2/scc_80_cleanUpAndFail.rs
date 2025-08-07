use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::MetadataExt;
use crate::bzip2::scc_79_root_46_setExit::*;
use crate::global_vars::bzip2_c5::*;
use crate::global_vars::bzip2_c1::*;
use crate::global_vars::bzip2_c2::*;


pub fn clean_up_and_fail(ec: i32) {
    // unsafe {
    //     // 访问全局变量
    //     let ret_val = fs::metadata(String::from_utf8_lossy(&IN_NAME)).is_ok() as i32;

    //     if SRC_MODE == 3 && OP_MODE != 3 && DELETE_OUTPUT_ON_INTERRUPT {
    //         if ret_val == 0 {
    //             if VERBOSITY != 0 {
    //                 eprintln!("{}: Deleting output file {}, if it exists.", 
    //                     String::from_utf8_lossy(&PROG_NAME), String::from_utf8_lossy(&OUT_NAME));
    //             }

    //             // 假设 outputHandleJustInCase 是全局的 File 类型变量
    //             if let Ok(_) = fs::remove_file(String::from_utf8_lossy(&OUT_NAME)) {
    //                 // 成功删除
    //             } else {
    //                 eprintln!("{}: WARNING: deletion of output file (apparently) failed.", 
    //                     String::from_utf8_lossy(&PROG_NAME));
    //             }
    //         } else {
    //             eprintln!("{}: WARNING: deletion of output file suppressed", String::from_utf8_lossy(&PROG_NAME));
    //             eprintln!("{}:    since input file no longer exists.  Output file", String::from_utf8_lossy(&PROG_NAME));
    //             eprintln!("{}:    `{}` may be incomplete.", String::from_utf8_lossy(&PROG_NAME), String::from_utf8_lossy(&OUT_NAME));
    //             eprintln!("{}:    I suggest doing an integrity test (bzip2 -tv) of it.", String::from_utf8_lossy(&PROG_NAME));
    //         }
    //     }

    //     if VERBOSITY != 0 && NUM_FILE_NAMES > 0 && NUM_FILES_PROCESSED < NUM_FILE_NAMES {
    //         eprintln!("{}: WARNING: some files have not been processed:", String::from_utf8_lossy(&PROG_NAME));
    //         eprintln!("{}:    {} specified on command line, {} not processed yet.\n", 
    //             String::from_utf8_lossy(&PROG_NAME), NUM_FILE_NAMES, NUM_FILE_NAMES - NUM_FILES_PROCESSED);
    //     }

    //     set_exit(EXIT_VALUE);
    //     std::process::exit(EXIT_VALUE);
    // }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_output_on_interrupt() {
        let input_filename = "test_input.txt";
        let output_filename = "test_output.txt";

        // 创建一个假的输入文件和输出文件
        std::fs::File::create(input_filename).unwrap();
        std::fs::File::create(output_filename).unwrap();

        // 设置全局变量
        unsafe {
            DELETE_OUTPUT_ON_INTERRUPT = true;
            OP_MODE = 1;
            SRC_MODE = 3;
            IN_NAME.copy_from_slice(input_filename.as_bytes());
            OUT_NAME.copy_from_slice(output_filename.as_bytes());
            PROG_NAME.copy_from_slice(b"test_prog");
        }

        // 调用函数并忽略返回值（因为该函数不返回）
        let _ = std::panic::catch_unwind(|| {
            clean_up_and_fail(1);
        });

        // 确保输出文件已被删除
        assert!(!std::path::Path::new(output_filename).exists());
        std::fs::remove_file(input_filename).unwrap(); // 清除测试文件
    }

    #[test]
    #[should_panic(expected = "process::exit")]
    fn test_missing_input_file_warning() {
        let input_filename = "nonexistent_input.txt";
        let output_filename = "test_output.txt";

        // 设置全局变量
        unsafe {
            DELETE_OUTPUT_ON_INTERRUPT = false;
            OP_MODE = 1;
            SRC_MODE = 1;
            VERBOSITY = 1;
            IN_NAME.copy_from_slice(input_filename.as_bytes());
            OUT_NAME.copy_from_slice(output_filename.as_bytes());
            PROG_NAME.copy_from_slice(b"test_prog");
        }

        // 调用函数并期待它调用 process::exit
        let _ = std::panic::catch_unwind(|| {
            clean_up_and_fail(3);
        });
    }
}
