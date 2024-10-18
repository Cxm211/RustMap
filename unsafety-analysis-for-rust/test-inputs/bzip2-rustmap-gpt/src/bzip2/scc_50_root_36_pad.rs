
use std::io::Write;
use std::fs::File;
use std::io::prelude::*;
use crate::global_vars::bzip2_c1::*;
pub fn pad(s: &str, output: &mut dyn Write) {
    let s_len = s.len() as i32; 

    unsafe {
        if s_len >= LONGEST_FILE_NAME {
            return;
        }
        let spaces_to_print = LONGEST_FILE_NAME - s_len;
        let spaces = " ".repeat(spaces_to_print as usize);
        output.write_all(spaces.as_bytes()).unwrap();
    }
}


// &mut dyn Write是一个动态分发的引用，它可以引用任何实现了Write trait的对象。Write trait定义了所有可以执行写入操作的类型必须实现的方法。
// 因此，这个参数意味着它可以接收任何可以进行写操作的对象。

// 这样的设计非常灵活，可以使函数适应不同的输出需求。例如，你可以向此函数传入一个文件引用（File对象）来将输出写入文件，或者传入一个Vec<u8>来收集输出到内存中的向量
// 将空格字符串写入output引用的对象中。这样，我们不再依赖于特定的输出源（如stderr），而是可以选择任何实现了Write trait的对象。
// 这种设计提供了更大的灵活性。你可以使用这个函数来写入几乎任何东西：文件、网络流、内存缓冲区等，只要它们实现了Write trait。

// 举个例子，在测试中，你可以传递一个File对象作为output，这样函数的输出就会写入这个文件；在另一个场景中，你可能想直接将输出保存在内存中，那么你可以传递一个Vec<u8>。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad() {
        // 假设longestFileName为10用于测试
        unsafe { LONGEST_FILE_NAME = 10; }
    
        // 打开一个临时文件用于写入
        let mut tmp_file = File::create("tmp.txt").expect("无法打开临时文件");
    
        // 调用pad函数来输出空格到临时文件
        pad("test", &mut tmp_file);
    
        // 读取临时文件的内容
        let mut tmp = File::open("tmp.txt").expect("无法读取临时文件");
        let mut buffer = String::new();
        tmp.read_to_string(&mut buffer).unwrap();
    
        // 断言：检查读取的内容是否是6个空格
        assert_eq!(buffer, "      ");
    
        // 删除临时文件
        std::fs::remove_file("tmp.txt").unwrap();
    }
}




// use crate::global_vars::bzip2_c5::{LONGEST_FILE_NAME};
// use std::io::{self, Write, Cursor, Seek};

// use std::process::Command;  // 导入Command
// use std::fs::File;   // 导入文件操作
// // 假设longestFileName是一个可变的全局变量，其类型是i32
// // 在Rust中，全局变量必须声明为静态，并具有一个固定的生命周期

// // 该函数的目的是对齐字符串s的长度到longestFileName，通过向stderr打印空格实现对齐。
// // 1. 首先，它检查s的长度是否已经大于或等于LONGEST_FILE_NAME。如果是，则直接返回，
// pub fn pad(s: &str) {
//     let s_len = s.len() as i32; // 明确注释数字类型

//     // 2. 如果s的长度已经大于或等于longestFileName，则直接返回，不做任何操作。
//     // 如果s的长度为4，而LONGEST_FILE_NAME为10，则函数会打印6个空格。
//    // 如果s的长度已经大于或等于longestFileName，则直接返回，不做任何操作。
//    unsafe { // 使用unsafe块，因为我们正在访问静态可变变量
//         if s_len >= LONGEST_FILE_NAME {
//             return;
//         }
//         // 计算并打印需要的空格数量，使得s的长度对齐到longestFileName。
//         let spaces_to_print = LONGEST_FILE_NAME - s_len;
//         eprint!("{}", " ".repeat(spaces_to_print as usize)); // 使用Rust库
//     }
// }
