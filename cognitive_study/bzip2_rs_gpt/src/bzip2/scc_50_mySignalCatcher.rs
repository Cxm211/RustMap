use crate::bzip2::scc_80_cleanUpAndFail::*;
use crate::global_vars::bzip2_c2::*;
fn my_signal_catcher() {
    // 使用 unsafe 块访问全局变量
    unsafe {
        let prog_name_str = String::from_utf8_lossy(&PROG_NAME);
        eprintln!("\n{}: Control-C or similar caught, quitting.", prog_name_str);
    }
    clean_up_and_fail(1); // 假设 clean_up_and_fail 已定义
}