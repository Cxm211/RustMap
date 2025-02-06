
全局变量: 请提供deleteOutputOnInterrupt、srcMode、outName、inName、noisy、progName、setExit、testFailsExist这些全局变量的定义和类型。
外部函数: panic、copyFileName、containsDubiousChars、fileExists、testStream 函数的定义需要被提供。
结构体和类型: 这段代码中用到了FILE、Bool 和 struct stat，我需要知道 Rust 环境中这些类型的对应物。
错误处理: C 代码使用strerror和__errno_location()来处理错误。在 Rust 中，我们通常会使用不同的错误处理方式。
请提供这些信息，以便我能准确地转换这段代码。


pub const MAX_SIZE: usize = 2000;
pub static mut PROG_NAME: [u8; MAX_SIZE] = [b'\0'; MAX_SIZE];
pub static mut DELETE_OUTPUT_ON_INTERRUPT: Bool = false;
pub static mut OP_MODE: Int32 = 0;
pub static mut SRC_MODE: Int32 = 0;
pub static mut NOISY: bool = false;
pub static mut TEST_FAILS_EXIST: bool = false;

pub fn test_stream(z_stream: &mut File) {
/ * 省略 */
}

pub fn file_exists(name: &str) -> bool {
/ * 省略 */
}

pub fn copy_file_name(to: &mut String, from: &str) {
/ * 省略 */
}

pub fn contains_dubious_chars(name: &str) -> bool {
/ * 省略 */
}


pub const MAX_SIZE: usize = 2000;
pub static mut PROG_NAME: [u8; MAX_SIZE] = [b'\0'; MAX_SIZE];