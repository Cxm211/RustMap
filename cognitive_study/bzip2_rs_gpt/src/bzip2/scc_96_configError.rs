use crate::global_vars::bzip2_c5::{exitValue};
use crate::bzip2::scc_79_root_46_setExit::*;

pub fn config_error() {
    eprintln!("bzip2: I'm not configured correctly for this platform!\n\
               \tI require Int32, Int16 and Char to have sizes\n\
               \tof 4, 2 and 1 bytes to run properly, and they don't.\n\
               \tProbably you can fix this by defining them correctly,\n\
               \tand recompiling.  Bye!");
    
    set_exit(3);  // 调用您之前提供的 set_exit 函数
    std::process::exit(unsafe { exitValue });
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_config_error_behavior() {
        // 由于config_error函数会导致程序退出，所以这里标记为应该panic。
        config_error();
    }
}
