use crate::bzip2::scc_119_compress::SRC_MODE;
use crate::global_vars::bzip2_c5::OP_MODE;
use crate::bzip2::scc_78_usage::*;
use crate::bzip2::scc_118_uncompress::*;
use crate::global_vars::bzip2_c1::*;
use crate::global_vars::bzip2_c2::*;
use crate::global_vars::bzip2_c5::*;
use std::{env, process, io};
use crate::bzip2::scc_101_mkCell_102_snoc_string_103_add_flags_from_env_var::*;
use crate::bzip2::scc_113_root_54_redundant::*;
use crate::bzip2::scc_79_license::*;
use crate::bzip2::scc_119_compress::*;
use crate::bzip2::scc_90_root_52_setExit::*;
use crate::global_vars::conversion::*;
use std::path::Path;

fn process_link_list(cell_option: &mut Option<Box<Cell>>, decode: &mut bool) {
    if let Some(cell) = cell_option {
        if let Some(ref name) = cell.name {
            if name.as_str() == "--" {
                *decode = false;
                return;
            } else if name.starts_with('-') && *decode {
                // 如果参数以 '-' 开头，并且 continue_evaluation 为 true
                // 则跳过此节点的处理
                // println!("跳过参数: {}", name);  // 示例: 打印信息
                // ... 其他跳过节点时的处理逻辑 ...
            } else {
                unsafe {
                    NUM_FILE_NAMES += 1;
                    LONGEST_FILE_NAME = LONGEST_FILE_NAME.max(name.len() as i32);
                }
            }
        }

        // 递归调用处理下一个节点
        if cell.link.is_some() {
            process_link_list(&mut cell.link, decode);
        }
    }
}

fn process_cells(cell: &Option<Box<Cell>>, decode: &mut bool) {
    if let Some(cell) = cell {
        if let Some(name) = &cell.name {
            if name.as_str() == "--" {
                *decode = false;
                return;
            }
            if name.starts_with('-') && *decode {
                // 递归调用处理下一个节点
                process_cells(&cell.link, decode);
                return;
            }
            unsafe { NUM_FILES_PROCESSED += 1; }
            // println!("cell.name: {}", name);
            if let Err(e) = compress(Some(name)) {
                eprintln!("压缩过程中出错: {}", e);
            }
        }
        // 递归调用处理下一个节点
        process_cells(&cell.link, decode);
    }
}



pub fn main(argv: &[String], gou: &[String])  -> Result<(), io::Error> {
    // let argv: Vec<String> = env::args().collect();
    let argv = argv;
    let argc = gou.len() as i32;
    let mut decode: bool;
    // 初始化变量
    unsafe{
        SMALL_MODE = false;
        KEEP_INPUT_FILES = false;
        FORCE_OVERWRITE = false;
        NOISY = true;
        verbosity = 0;
        blockSize100k = 9;
        TEST_FAILS_EXIST = false;
        UNZ_FAILS_EXIST = false;
        NUM_FILE_NAMES = 0;
        NUM_FILES_PROCESSED = 0;
        workFactor = 30;
        DELETE_OUTPUT_ON_INTERRUPT = false;
        EXIT_VALUE = 0;
        OP_MODE = 1;
        LONGEST_FILE_NAME = 7;
        
    }

    let mut i = 0;
    let mut j = 0;

    decode = true;

    // let mut in_name = String::from("(none)");
    // let mut out_name = String::from("(none)");
    // let mut prog_name_really = argv[0].clone();
    // let prog_name = &prog_name_really;

    // 确保类型大小正确
    // // println!("121_main_检查数据类型大小");
    // if std::mem::size_of::<i32>() != 4 || std::mem::size_of::<u16>() != 2 || std::mem::size_of::<char>() != 1 {
    //     // println!("121_main_数据类型大小错误");
    //     config_error();
    //     return Err(io::Error::new(io::ErrorKind::Other, "配置错误"));
    // }


    unsafe {
        let prog_name_str = env::args().nth(0).unwrap_or_else(|| String::from("(unknown)"));
        PROG_NAME_REALLY = string_to_u8_array_1034(prog_name_str.clone());
        let p_str = Path::new(&prog_name_str)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        PROG_NAME = string_to_u8_array_2000(p_str.to_string());
    }

    // 处理命令行参数
    // println!("121_main_开始处理命令行参数");
    let mut arg_list = Some(Box::new(Cell::mk_cell()));
    Cell::add_flags_from_env_var(&mut arg_list, "BZIP2");
    // println!("121_main_添加环境变量 BZIP2");

    Cell::add_flags_from_env_var(&mut arg_list, "BZIP");
    // println!("121_main_添加环境变量 BZIP");

    for arg in argv.iter().skip(1) {
        // println!("121_main_处理参数: {}", arg);
        arg_list = Some(Box::new(arg_list.take().unwrap_or_else(|| Box::new(Cell::mk_cell())).snoc_string(arg)));
    }

    // println!("After doing with arg_list");
    if let Some(ref cell) = arg_list {
        cell.printLinkLists();
    }
    // 下面进入arg_list的使用

    // let mut arg_list = Vec::new();
    // add_flags_from_env_var(&mut arg_list, "BZIP2");
    // add_flags_from_env_var(&mut arg_list, "BZIP");
    // arg_list.extend(env::args().skip(1));
    unsafe {
        LONGEST_FILE_NAME = 7;
        NUM_FILE_NAMES = 0;
    }
    decode = true;
    // 递归遍历 🔥
    process_link_list(&mut arg_list, &mut decode);
    // for aa in &arg_list {
    //      // 解引用 `Box<String>` 以获取 `String` 的引用
    //     if let Some(ref name) = aa.name {
    //         if name.as_str() == "--" {
    //             decode = false;
    //             continue;
    //         }
    //         if name.starts_with('-') && decode {
    //             continue;
    //         }
    //         unsafe {
    //             NUM_FILE_NAMES += 1;
    //             LONGEST_FILE_NAME = LONGEST_FILE_NAME.max(name.len() as i32);
    //         }
    //     }
    // }

    unsafe {SRC_MODE = if NUM_FILE_NAMES == 0 { 1 } else { 3 };}
    unsafe {OP_MODE = 1;}
    unsafe {
        let mut prog_name_str = u8_array_to_string(&PROG_NAME);
        if prog_name_str.contains("unzip") || prog_name_str.contains("UNZIP") {
            OP_MODE = 2;
        }
        if prog_name_str.contains("z2cat") || prog_name_str.contains("Z2CAT") || prog_name_str.contains("zcat") || prog_name_str.contains("ZCAT") {
            OP_MODE = 2;
        }
    }


    // 遍历arg_list处理每个命令行参数
    let mut aa = &arg_list;
    while let Some(cell) = aa {
        if let Some(name) = &cell.name {
            if name.as_str() == "--"  {
                break;
            }

            if name.starts_with('-') && !name.starts_with("--") {
                for ch in name.chars().skip(1) {
                    match ch {
                        'c' => unsafe { SRC_MODE = 2; },
                        'd' => unsafe { OP_MODE = 2; },
                        'z' => unsafe { OP_MODE = 1; },
                        'f' => unsafe { FORCE_OVERWRITE = true; },
                        't' => unsafe { OP_MODE = 3; },
                        'k' => unsafe { KEEP_INPUT_FILES = true; },
                        's' => unsafe { SMALL_MODE = true; },
                        'q' => unsafe { NOISY = false; },
                        '1' => unsafe { blockSize100k = 1; },
                        '2' => unsafe { blockSize100k = 2; },
                        '3' => unsafe { blockSize100k = 3; },
                        '4' => unsafe { blockSize100k = 4; },
                        '5' => unsafe { blockSize100k = 5; },
                        '6' => unsafe { blockSize100k = 6; },
                        '7' => unsafe { blockSize100k = 7; },
                        '8' => unsafe { blockSize100k = 8; },
                        '9' => unsafe { blockSize100k = 9; },
                        // ...处理其他字符...
                        'L' => unsafe { license(); },
                        'v' => unsafe { VERBOSITY += 1; },
                        'h' => {
                            unsafe{
                                let prog_name_str = std::str::from_utf8(&PROG_NAME).expect("PROG_NAME 不是有效的 UTF-8 字符串");
                                usage(prog_name_str);
                            }
                            // process::exit(0);
                        },
                        _ => {
                            unsafe {
                            eprintln!("第一match {}: Bad flag `{}`", String::from_utf8_lossy(&PROG_NAME), name);
                            let prog_name_str = std::str::from_utf8(&PROG_NAME).expect("PROG_NAME 不是有效的 UTF-8 字符串");
                                usage(prog_name_str);
                            }
                            // process::exit(1);
                        }
                    }
                }
            }
        }
        aa = &cell.link;
    }


    // 遍历arg_list处理每个长格式命令行参数
    let mut aa = &arg_list;
    while let Some(cell) = aa {
        if let Some(name) = &cell.name {
            match name.as_str() {
                "--" => break,
                "--stdout" => unsafe { SRC_MODE = 2; },
                "--decompress" => unsafe { OP_MODE = 2; },
                "--compress" => unsafe { OP_MODE = 1; },
                "--force" => unsafe { FORCE_OVERWRITE = true; },
                "--test" => unsafe { OP_MODE = 3; },
                "--keep" => unsafe { KEEP_INPUT_FILES = true; },
                "--small" => unsafe { SMALL_MODE = true; },
                "--quiet" => unsafe { NOISY = false; },
                "--version" | "--license" => license(),
                "--exponential" => unsafe { WORK_FACTOR = 1; },
                "--repetitive-best" | "--repetitive-fast" => redundant(&name.as_bytes()),
                "--fast" => unsafe { blockSize100k = 1; },
                "--best" => unsafe { blockSize100k = 9; },
                "--verbose" => unsafe { VERBOSITY += 1; },
                "--help" => {
                    unsafe{
                        let prog_name_str = std::str::from_utf8(&PROG_NAME).expect("PROG_NAME 不是有效的 UTF-8 字符串");
                        usage(prog_name_str);
                    }
                    // process::exit(0);
                },
                _ if name.starts_with("--") => {
                    unsafe{ 
                        eprintln!("第二match {}: Bad flag `{}`", String::from_utf8_lossy(&PROG_NAME), name);
                        let prog_name_str = std::str::from_utf8(&PROG_NAME).expect("PROG_NAME 不是有效的 UTF-8 字符串");
                        usage(prog_name_str);
                    }
                    // process::exit(0);
                },
                _ => {}
            }
        }
        aa = &cell.link;
    }



    // 设置verbosity限制
    unsafe {
        if VERBOSITY > 4 {
            VERBOSITY = 4;
        }
        if OP_MODE == 1 && SMALL_MODE && blockSize100k > 2 {
            unsafe {blockSize100k = 2; }
        }
        if OP_MODE == 3 && SRC_MODE == 2 {
            unsafe {eprintln!("{}: -c and -t cannot be used together.", String::from_utf8_lossy(&PROG_NAME));}
            // process::exit(1);
        }
        if SRC_MODE == 2 && NUM_FILE_NAMES == 0 {
            SRC_MODE = 1;
        }
        if OP_MODE != 1 {
            unsafe {blockSize100k = 0; }
        }
    }
    // 根据操作模式执行相应的操作
    unsafe {
        // println!("OP_MODE: {}", OP_MODE);
        // println!("SRC_MODE: {}", SRC_MODE);
        match OP_MODE {
            1 => {
                if SRC_MODE == 1 {
                    compress(None)?;
                } else {
                    decode = true;
                    process_cells(&arg_list, &mut decode);
                }
            },
            2 => {
                UNZ_FAILS_EXIST = false;
                if SRC_MODE == 1 {
                    uncompress(None);
                } else {
                    decode = true;
                    for cell in &arg_list {
                        if let Some(name) = &cell.name {
                            if name.as_str() == "--"  {
                                decode = false;
                                continue;
                            }
                            if name.starts_with('-') && decode {
                                continue;
                            }
                            unsafe {NUM_FILE_NAMES += 1;}
                            uncompress(Some(name));
                        }
                    }
                }
                if UNZ_FAILS_EXIST {
                    set_exit(2);
                    // process::exit(EXIT_VALUE);
                }
            },
            _ => {
                // 处理测试模式
                // ...
            },
        }
    }

    // println!("121_main_所有命令行参数处理完成");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        // 模拟命令行参数
        let simulated_args = vec![
            String::from("./bzip2"),
            String::from("-k"),
            String::from("/root/crown-rust/bzip2_rs_gpt/11Nov.txt"),
            // String::from("/root/crown-rust/bzip2_rs_gpt/sample1.ref"),
            // String::from("/root/crown-rust/bzip2_rs_gpt/abc.txt"),
        ];
            // String::from("-1"),
            // String::from("< sample1.ref"),
            // String::from("> sample1mao.rb2"),
   

        // 调用 main 函数
        let result = main(&simulated_args,&simulated_args.clone());
        // assert!(result.is_ok());
    }
}