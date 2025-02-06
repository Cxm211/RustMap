

use std::io::{self, Write}; // 引入Rust标准库中的I/O模块

type Char = char; 
type MaybeUInt64 = u64;

static IN_FILE_NAME: [Char; 2000] = ['\0'; 2000];
static PROG_NAME: [Char; 2000] = ['\0'; 2000];

static mut BYTES_OUT: MaybeUInt64 = 0;
static mut BYTES_IN: MaybeUInt64 = 0;

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
