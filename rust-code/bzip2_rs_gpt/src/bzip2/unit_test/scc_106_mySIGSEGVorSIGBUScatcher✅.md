pub static mut PROG_NAME: [u8; MAX_SIZE] = [b'\0'; MAX_SIZE];
pub static mut IN_NAME: [u8; 1034] = [0; 1034];
pub static mut OUT_NAME: [u8; 1034] = [0; 1034];
pub static mut TMP_NAME: [u8; 1034] = [0; 1034];
pub static mut exitValue: i32 = 0;  // 明确注释数字类型

pub fn set_exit(v: i32) {  // 明确注释数字类型
    // 使用 unsafe 代码块来访问和修改全局变量
    unsafe {
        if v > exitValue {
            exitValue = v;
        }
    }
}