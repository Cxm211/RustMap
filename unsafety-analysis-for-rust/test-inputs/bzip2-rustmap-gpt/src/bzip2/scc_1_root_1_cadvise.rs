// use crate::global_vars::bzip2_c5::{noisy};


// cadvise ( void )

// 假设 Int32 和 Bool 的定义如下:
type Int32 = i32;
type Bool = bool;
type Char = char; // 假设 Char 是 char，但可能需要根据上下文进一步确认
use crate::global_vars::bzip2_c5::*;
use crate::global_vars::bzip2_c1::*;
// 全局变量
// static mut VERBOSITY: Int32 = 0;
// static mut KEEP_INPUT_FILES: Bool = false;
// static mut SMALL_MODE: Bool = false;
// static mut DELETE_OUTPUT_ON_INTERRUPT: Bool = false;
// static mut FORCE_OVERWRITE: Bool = false;
// static mut TEST_FAILS_EXIST: Bool = false;
// static mut UNZ_FAILS_EXIST: Bool = false;
// static mut NOISY: Bool = false;
// static mut NUM_FILES_PROCESSED: Int32 = 0;
// static mut BLOCK_SIZE_100K: Int32 = 0;
// static mut EXIT_VALUE: Int32 = 0;
// static mut OP_MODE: Int32 = 0;
// static mut SRC_MODE: Int32 = 0;

// static mut IN_NAME: [Char; 1034] = ['\0'; 1034]; 
// static mut OUT_NAME: [Char; 1034] = ['\0'; 1034]; 

pub fn cadvise() {
    unsafe {
        if NOISY {
            eprintln!("\nIt is possible that the compressed file(s) have become corrupted.\nYou can use the -tvv option to test integrity of such files.\n\nYou can use the `bzip2recover' program to attempt to recover\ndata from undamaged sections of corrupted files.\n\n");
        }
    }
}

pub fn show_file_names() {
    unsafe {
        if NOISY {
            eprintln!("\tInput file = {:?}, output file = {:?}", 
                      u8_array_to_string(&IN_NAME), 
                      u8_array_to_string(&OUT_NAME));
        }
    }
}

pub fn u8_array_to_string(arr: &[u8; 1034]) -> String {
    String::from_utf8_lossy(arr).into_owned()
}


pub fn char_array_to_string(arr: &[Char; 1034]) -> String {
    let byte_slice: Vec<u8> = arr.iter().filter_map(|&c| {
        if c as u32 <= 0x7F {
            Some(c as u8)
        } else {
            None
        }
    }).collect();
    String::from_utf8_lossy(&byte_slice).into_owned()
}
