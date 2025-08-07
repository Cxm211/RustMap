use crate::bzip2::scc_76_root_43_showFileNames::*;
use crate::bzip2::scc_80_cleanUpAndFail::*;
use crate::global_vars::bzip2_c5::*;
use std::io::{self, Write};
use std::alloc::{alloc, dealloc, Layout};

pub fn out_of_memory(in_name: &str, out_name: &str, prog_name: &str, output_handle_just_in_case: Option<&mut std::fs::File>) {
    // 向标准错误输出流打印错误信息，其中包含程序名
    writeln!(io::stderr(), "\n{}: couldn't allocate enough memory", prog_name).unwrap();

    // 显示相关文件名
    show_file_names();

    // 清理并以错误代码1退出
    clean_up_and_fail(1);
}

pub fn my_malloc(n: i32, in_name: &str, out_name: &str, prog_name: &str, output_handle_just_in_case: Option<&mut std::fs::File>) -> *mut u8 {
    // 尝试分配n个字节的内存
    let layout = Layout::from_size_align(n as usize, 1).expect("Failed to create layout");
    let p = unsafe { alloc(layout) };

    // 如果分配失败，调用out_of_memory函数
    if p.is_null() {
        out_of_memory(in_name, out_name, prog_name, output_handle_just_in_case);
    }

    p
}

use std::fs::{self, File};
use std::process::Command;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "couldn't allocate enough memory")] // 期望这个测试会 panic，因为我们要测试的是 out_of_memory 函数
    fn test_out_of_memory() {
        // 创建临时环境
        let in_name = "test_in_file.txt";
        let out_name = "test_out_file.txt";
        let prog_name = "test_program";
        
        File::create(in_name).unwrap();
        File::create(out_name).unwrap();
        File::create(prog_name).unwrap();

        // 测试 out_of_memory
        out_of_memory(in_name, out_name, prog_name, None);

        // 清理临时环境
        fs::remove_file(in_name).unwrap();
        fs::remove_file(out_name).unwrap();
        fs::remove_file(prog_name).unwrap();
    }

    #[test]
    fn test_my_malloc() {
        // 创建临时环境
        let in_name = "test_in_file.txt";
        let out_name = "test_out_file.txt";
        let prog_name = "test_program";
        
        File::create(in_name).unwrap();
        File::create(out_name).unwrap();
        File::create(prog_name).unwrap();

        // 测试 my_malloc
        let ptr = my_malloc(10, in_name, out_name, prog_name, None);
        assert!(!ptr.is_null());
        
        // 注意：这里没有释放内存，你应该在合适的时机释放它，或者增加一个 my_free 函数来进行释放

        // 清理临时环境
        fs::remove_file(in_name).unwrap();
        fs::remove_file(out_name).unwrap();
        fs::remove_file(prog_name).unwrap();
    }
}
