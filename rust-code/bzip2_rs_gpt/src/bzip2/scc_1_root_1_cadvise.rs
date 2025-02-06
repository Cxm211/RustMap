


type Int32 = i32;
type Bool = bool;
type Char = char;
use crate::global_vars::bzip2_c5::*;
use crate::global_vars::bzip2_c1::*;


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
