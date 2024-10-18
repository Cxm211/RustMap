// use crate::global_vars::bzip2_c2::{inFileName, progName};


// readError ( void )
use std::io::{self, Write}; // 引入Rust标准库中的I/O模块

// 明确注释所有数字类型
type Char = char; 
type MaybeUInt64 = u64;

// 这些变量在C代码中是全局的，但在Rust中，我们可以将它们存储为常量或结构体的字段
static IN_FILE_NAME: [Char; 2000] = ['\0'; 2000];
static PROG_NAME: [Char; 2000] = ['\0'; 2000];

// 我暂时保留这两个为静态变量，但在真实应用中，它们最好放在一个结构体或枚举中
static mut BYTES_OUT: MaybeUInt64 = 0;
static mut BYTES_IN: MaybeUInt64 = 0;

// 为了避免使用不安全代码，这个函数改为返回一个Result
// 错误消息使用Rust的方式进行处理
pub fn read_error() -> io::Result<()> {
    eprintln!(
        "{}: I/O error reading `{}`, possible reason follows.",
        PROG_NAME.iter().collect::<String>(),
        IN_FILE_NAME.iter().collect::<String>()
    );
    // 这里使用io::Error，模拟C的perror
    let last_os_error = io::Error::last_os_error();
    eprintln!("{}: {}", PROG_NAME.iter().collect::<String>(), last_os_error);
    eprintln!(
        "{}: warning: output file(s) may be incomplete.",
        PROG_NAME.iter().collect::<String>()
    );
    // 在Rust中，通常是使用Result来返回错误，而不是像C中那样使用exit
    Err(io::Error::new(io::ErrorKind::Other, "Read error"))
}
