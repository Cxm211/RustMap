use crate::bzip2::scc_80_cleanUpAndFail::*;
use crate::bzip2::scc_76_root_43_showFileNames::show_file_names;
use crate::bzip2::scc_1_root_1_cadvise::*;
use crate::global_vars::bzip2_c2::{PROG_NAME};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::process;

pub fn crc_error() {
    unsafe {
        // 从 PROG_NAME 创建C风格的字符串，但不转移所有权
        let prog_name_cstr = CStr::from_ptr(PROG_NAME.as_ptr() as *const c_char);

        // 输出错误信息
        eprintln!("\n{}: 数据完整性在解压缩时出错。\n", prog_name_cstr.to_str().unwrap());
        
        // 调用其他函数
        show_file_names();
        cadvise();
        clean_up_and_fail(2);
    }
}

pub fn io_error() {
    unsafe {
        // 从 PROG_NAME 创建C风格的字符串，但不转移所有权
        let prog_name_cstr = CStr::from_ptr(PROG_NAME.as_ptr() as *const c_char);

        // 输出错误信息
        eprintln!("\n{}: I/O或其他错误，退出程序。下面可能是具体原因。\n", prog_name_cstr.to_str().unwrap());
        
        // 输出具体的系统错误信息
        eprintln!("Error: {}", std::io::Error::last_os_error());
        
        // 调用其他函数
        show_file_names();
        clean_up_and_fail(1);
    }
}
