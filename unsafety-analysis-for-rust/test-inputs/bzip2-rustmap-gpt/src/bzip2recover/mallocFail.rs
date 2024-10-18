// use crate::global_vars::bzip2_c2::{progName};


// mallocFail ( Int32 n )

use std::io::{self, Write};

// 之前已经给出的定义
pub static mut PROG_NAME: [Char; 2000] = [0; 2000];
type Char = u8;
type Int32 = i32;

pub fn malloc_fail(n: Int32) {
    let prog_name_str = unsafe {
        // 将Char数组转换为字符串
        // 注意：这假设PROG_NAME是一个有效的UTF-8字符串。
        // 如果不是，你可能需要使用其他方式来进行转换。
        std::str::from_utf8_unchecked(&PROG_NAME)
    };

    // 输出错误信息
    writeln!(io::stderr(), "{}: malloc failed on request for {} bytes.", prog_name_str, n).unwrap();
    writeln!(io::stderr(), "{}: warning: output file(s) may be incomplete.", prog_name_str).unwrap();

    // 退出程序
    std::process::exit(1);
}
