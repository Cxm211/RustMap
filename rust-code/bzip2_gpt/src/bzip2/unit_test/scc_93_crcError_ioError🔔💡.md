progName 变量: 这似乎是一个全局变量，用于存储程序名称。在Rust中，全局变量的处理方式与C有所不同。请提供更多关于此变量的信息，例如它的定义和类型。
pub static mut PROG_NAME: [u8; MAX_SIZE] = [b'\0'; MAX_SIZE];


showFileNames 函数: 这个函数用于显示文件名信息。我需要知道它的具体实现或功能，以便在Rust中正确实现。
pub fn show_file_names() {
 /* 省略 */
}

cadvise 函数: 这个函数用于提供可能的建议。与showFileNames一样，我需要了解它的实现细节。

pub fn cadvise() {
 /* 省略 */
}
cleanUpAndFail 函数: 这个函数用于清理资源并以特定的退出代码结束程序。请提供这个函数的具体实现。


pub fn clean_up_and_fail(in_name: &str, out_name: &str, prog_name: &str, output_handle_just_in_case: Option<&mut std::fs::File>) -> ! {
 /* 省略 */
}