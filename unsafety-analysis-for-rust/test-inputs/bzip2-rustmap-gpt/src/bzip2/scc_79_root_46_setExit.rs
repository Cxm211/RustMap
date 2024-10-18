use crate::global_vars::bzip2_c5::{exitValue};
// 定义一个全局可变变量 exitValue
// 在 Rust 中，全局变量必须使用静态关键字，并且必须有一个生命周期。
// 为了修改这样的变量，我们还需要使用 unsafe 代码块。
// 不过全局变量并不是 Rust 中的最佳实践。如果可能的话，最好避免使用全局变量。


// 定义函数 setExit
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
