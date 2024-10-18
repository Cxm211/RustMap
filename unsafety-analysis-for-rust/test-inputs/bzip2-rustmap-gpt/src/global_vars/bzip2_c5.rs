//  deleteOutputOnInterrupt
//  exitValue
//  forceOverwrite
pub static mut IN_NAME: [u8; 1034] = [0; 1034];
pub static mut OUT_NAME: [u8; 1034] = [0; 1034];
pub static mut TMP_NAME: [u8; 1034] = [0; 1034];
pub static mut PROG_NAME_REALLY: [u8; 1034] = [0; 1034]; 
pub static mut WORK_FACTOR: i32 = 0;
pub static mut exitValue: i32 = 0;  // 明确注释数字类型

// 这些全局变量被转换为静态可变变量
// 注意: 在Rust中，使用全局静态可变变量是不安全的，因此实际使用时应当小心
// 假设Int32是一个32位整数
type Int32 = i32;

// 假设Bool是布尔类型
type Bool = bool;



pub static mut VERBOSITY: Int32 = 0;

pub static mut TEST_FAILS_EXIST: Bool = false;

// pub static mut BLOCK_SIZE100K: Int32 = 0; // c2
pub static mut OP_MODE: Int32 = 0;
pub static mut SRC_MODE: Int32 = 0;

use std::fs::File;
pub static mut OUTPUT_HANDLE_JUST_IN_CASE: Option<File> = None;
// 如果Char是char的别名


// &'static str:
// 指向程序生命周期内的字符串字面量。
// 不拥有其内容，只是一个引用。
// 通常用于常量或固定的字符串。
// String:
// 动态分配的字符串，拥有其内容。
// 可以被修改（如增加、删除字符）。
// 通常用于运行时可能会发生变化的字符串。

// 对于你的情况，你应该使用 &'static str，因为你正在定义常量数组，这些字符串不会在运行时改变。这也更为高效，因为它不涉及动态内存分配。
// 使用&'static str来表示在Rust中的字符串字面量
pub const ZSUFFIX: [&'static str; 4] = [".bz2", ".bz", ".tbz2", ".tbz"];
pub const UNZSUFFIX: [&'static str; 4] = ["", "", ".tar", ".tar"];




//  opMode
//  outName
//  outputHandleJustInCase
// 假设Bool在C中表示布尔值



//  stat
//  struct stat
//    {
//  
//  
//  
//      __dev_t st_dev;
//  
//  
//  
//  
//      __ino_t st_ino;
//  
//  
//  
//  
//  
//  
//  
//      __nlink_t st_nlink;
//      __mode_t st_mode;
//  
//      __uid_t st_uid;
//      __gid_t st_gid;
//  
//      int __pad0;
//  
//      __dev_t st_rdev;
//  
//  
//  
//  
//      __off_t st_size;
//  
//  
//  
//      __blksize_t st_blksize;
//  
//      __blkcnt_t st_blocks;
//      struct timespec st_atim;
//      struct timespec st_mtim;
//      struct timespec st_ctim;
//      __syscall_slong_t __glibc_reserved[3];
//    }
