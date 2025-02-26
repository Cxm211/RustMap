use crate::bzip2::scc_79_root_46_setExit::*;
use crate::global_vars::bzip2_c5::{exitValue, IN_NAME, OP_MODE, OUT_NAME};
use crate::global_vars::bzip2_c2::{PROG_NAME};
use std::io::{self, Write};
use std::process;

pub fn my_sigsegv_or_sigbus_catcher(op_mode: i32) {
    let msg: &str;
    unsafe {
        // 根据操作模式判断错误消息
        msg = if op_mode == 1 {
            "Compression error message" // 压缩时的错误消息
        } else {
            "Decompression error message" // 解压缩时的错误消息
        };

        // 输出错误消息到标准错误输出
        let _ = io::stderr().write_all(b"\n");
        let _ = io::stderr().write_all(&PROG_NAME);
        let _ = io::stderr().write_all(msg.as_bytes());

        // 输出当前输入和输出文件的名字
        let _ = io::stderr().write_all(b"\tInput file = ");
        let _ = io::stderr().write_all(&IN_NAME);
        let _ = io::stderr().write_all(b"\n");

        let _ = io::stderr().write_all(b"\tOutput file = ");
        let _ = io::stderr().write_all(&OUT_NAME);
        let _ = io::stderr().write_all(b"\n");

        // 设置退出状态并退出程序
        if op_mode == 1 { set_exit(3); } else { set_exit(2); }
    }

    // process::exit(unsafe { exitValue });
}







