pub static mut IN_NAME: [u8; 1034] = [0; 1034];
pub static mut OUT_NAME: [u8; 1034] = [0; 1034];
pub static mut TMP_NAME: [u8; 1034] = [0; 1034];
pub static mut PROG_NAME_REALLY: [u8; 1034] = [0; 1034]; 
pub static mut WORK_FACTOR: i32 = 0;
pub static mut exitValue: i32 = 0;  // 明确注释数字类型

type Int32 = i32;

type Bool = bool;



pub static mut VERBOSITY: Int32 = 0;

pub static mut TEST_FAILS_EXIST: Bool = false;

pub static mut OP_MODE: Int32 = 0;
pub static mut SRC_MODE: Int32 = 0;

use std::fs::File;
pub static mut OUTPUT_HANDLE_JUST_IN_CASE: Option<File> = None;



pub const ZSUFFIX: [&'static str; 4] = [".bz2", ".bz", ".tbz2", ".tbz"];
pub const UNZSUFFIX: [&'static str; 4] = ["", "", ".tar", ".tar"];







