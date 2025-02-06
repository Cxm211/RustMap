use crate::global_vars::bzip2_c5::{exitValue};


pub fn set_exit(v: i32) {  // 明确注释数字类型
    // 使用 unsafe 代码块来访问和修改全局变量
    unsafe {
        if v > exitValue {
            exitValue = v;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_exit_no_update() {
        // 使用 unsafe 代码块来修改全局变量
        unsafe {
            exitValue = 10;  // 设置初始值
        }
        set_exit(5);       // 尝试更新

        // 使用 assert_eq! 宏来检查值仍然是10
        unsafe {
            assert_eq!(exitValue, 10);
        }
    }
}
