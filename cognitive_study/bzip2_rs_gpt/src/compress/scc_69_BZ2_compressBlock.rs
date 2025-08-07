use crate::global_vars::bzip2_c1::*;
use crate::global_vars::sleep_lib::*;
use crate::compress::scc_18_root_15_makeMaps_e_scc_19_generateMTFValues::generate_mtf_values;
use crate::blocksort::scc_72_BZ2_blockSort::block_sort;
use crate::compress::scc_63_sendMTFValues::*;
use crate::compress::scc_28_root_21_BZ2_bsInitWrite::*;
use crate::global_vars::conversion::*;
use std::io::{Cursor, Write, Read};
use std::alloc::{alloc, realloc, Layout};
use std::process::exit;



            










pub fn bs_w(s: &mut EState, n: Int32, v: UInt32) {
    while s.bsLive >= 8 {
        // 在赋值前确保有足够的空间
        if s.numZ as usize >= s.zbits.len() {
            s.zbits.resize(s.numZ as usize + 1, 0); // 以 0 填充新空间
        }

        s.zbits[s.numZ as usize] = (s.bsBuff >> 24) as UChar;
        update_arr2(s);
        s.numZ += 1;
        s.bsBuff <<= 8;
        s.bsLive -= 8;
    }

    if 32 - s.bsLive - n < 0 {
        // 处理可能的下溢错误
        // println!("Error: Underflow in shifting operation");
        return;
    }

    s.bsBuff |= v << (32 - s.bsLive - n) as u32;
    s.bsLive += n;
}

pub fn bs_put_uint32(s: &mut EState, u: u32) {
    bs_w(s, 8, (u >> 24) & 0xff);
    bs_w(s, 8, (u >> 16) & 0xff);
    bs_w(s, 8, (u >>  8) & 0xff);
    bs_w(s, 8,  u        & 0xff);
}

pub fn bs_put_uchar(s: &mut EState, c: u8) {
    bs_w(s, 8, c as u32);
}

pub fn bs_finish_write(s: &mut EState) {
    while s.bsLive > 0 {
        // 在赋值前确保有足够的空间
        if s.numZ as usize >= s.zbits.len() {
            s.zbits.resize(s.numZ as usize + 1, 0); // 以 0 填充新空间
        }

        s.zbits[s.numZ as usize] = (s.bsBuff >> 24) as UChar;
        update_arr2(s); // 假设这个函数更新arr2以反映zbits的改变
        s.numZ += 1;
        s.bsBuff <<= 8;
        s.bsLive -= 8;
    }
}


fn update_zbits(estate: &mut EState) {
    let mut arr2 = rebuild_arr2_from_block(&estate.block);
    let nblock = estate.nblock as usize;
    if nblock >= arr2.len() * 4 {
        // println!("Error: nblock is out of range for arr2");
        return;
    }

    let offset = nblock / 4;

    if offset >= arr2.len() {
        // println!("Error: Offset out of range for arr2");
        return;
    }

    let remaining_bytes = arr2.len() * 4 - nblock;
    estate.zbits.clear();
    estate.zbits.reserve(remaining_bytes);

    for &num in &arr2[offset..] {
        let bytes = if cfg!(target_endian = "little") {
            num.to_le_bytes()
        } else {
            num.to_be_bytes()
        };

        let start_index = if estate.zbits.is_empty() { nblock % 4 } else { 0 };
        estate.zbits.extend_from_slice(&bytes[start_index..]);
    }
}

fn update_arr2(estate: &mut EState) {
    let nblock = estate.nblock as usize;
    let mut arr2 = rebuild_arr2_from_block(&estate.block);
    if nblock >= arr2.len() * 4 {
        // println!("Error: nblock is out of range for arr2");
        return;
    }

    let mut byte_index = nblock % 4;
    let mut arr2_index = nblock / 4;

    for &zbit in &estate.zbits {
        if arr2_index >= arr2.len() {
            break; // 防止越界
        }

        // 根据字节序将字节放到正确的位置
        if cfg!(target_endian = "little") {
            arr2[arr2_index] &= !(0xFF << (byte_index * 8));
            arr2[arr2_index] |= (zbit as u32) << (byte_index * 8);
        } else {
            arr2[arr2_index] &= !(0xFF << ((3 - byte_index) * 8));
            arr2[arr2_index] |= (zbit as u32) << ((3 - byte_index) * 8);
        }

        byte_index += 1;
        if byte_index == 4 {
            byte_index = 0;
            arr2_index += 1;
        }
    }
    estate.block = build_block_from_arr2(&arr2);
}

pub fn bz2_compress_block(s: &mut EState, is_last_block: bool) {
    // println!("开始压缩数据块");
    // print_estate(s, "73_compressBlock_BEFORE_71_BZ_FINALISE_CRC_rust.json");
    // execute_command("cat 73_compressBlock_BEFORE_71_BZ_FINALISE_CRC_rust.json | cut -c 1-100 > /root/crown-rust/bzip2_rs_gpt/73_enter.rs.json");
    // my_sleep(1000);
    if s.nblock > 0 {
        // println!("正在处理数据块，块号：{}", s.blockNo);

        // 计算当前数据块的CRC
        s.blockCRC = !s.blockCRC;
        // println!("计算当前数据块的CRC完成");

        // 更新累积的CRC值
        s.combinedCRC = (s.combinedCRC << 1) | (s.combinedCRC >> 31);
        s.combinedCRC ^= s.blockCRC;
        // println!("更新累积CRC值完成");

        // 如果不是第一个数据块，则重置numZ为0
        if s.blockNo > 1 {
            s.numZ = 0;
            // println!("非首个数据块，重置numZ为0");
        }

        // 输出诊断信息
        if s.verbosity >= 2 {
             println!(
                "    block {}: crc = 0x{:08x}, combined CRC = 0x{:08x}, size = {}",
                s.blockNo, s.blockCRC, s.combinedCRC, s.nblock
            );
        }

        // 对数据块进行排序
        // // println!("开始对数据块进行排序");
        // let mut ptr_before_gmtf = unsafe { Vec::from_raw_parts(s.p9tr.unwrap(), (100000 * s.blockSize100k) as usize, (100000 * s.blockSize100k) as usize) };
        // // println!("ptr_before_gmtf: {:?}",ptr_before_gmtf);

        // print_estate(s, "73compressB_72_before_block_sort_rust.json");
        // execute_command("cat 73compressB_72_before_block_sort_rust.json | cut -c 1-100 > 73_before.rs.txt");
        block_sort(s);
        // print_estate(s, "73compressB_72_before_block_sort_rust.json");
        // execute_command("cat 73compressB_72_before_block_sort_rust.json | cut -c 1-100 > 73_after.rs.txt");
        // my_sleep(1000);
        // println!("73compressB_72_blocksort_排序完成");
        // exit(1);
        // let mut ptr_rr = unsafe { Vec::from_raw_parts(s.ptr.unwrap(), (s.nblock + 34) as usize, (s.nblock +34) as usize) };
        // // println!("ptr_rr🌲: {:?}",ptr_rr);
        // // println!("数据块排序完成");
    }
     
    // 设置zbits指针
    // println!("设置zbits指针");
    update_zbits(s);

    // print_estate(s, "73_zbitsD_rs.json");
    // execute_command("cat 73_zbitsD_rs.json | cut -c 1-100 > /root/crown-rust/73_zbits.rs.json");
      
    /* 💡💡💡
        在 Rust 中，将 s.arr2 中的一部分转换为 Vec<u8> 的形式并不像在 C 中那样直接。
        由于 Rust 强调类型安全和内存安全，这样的转换需要更多考虑。
        在您提供的 C 代码中，s->zbits 被赋值为 s->arr2 的一个偏移量处的地址，将这部分内存视为 UChar*（即 u8*）。
        在 Rust 中实现这种操作需要小心，因为直接转换可能会违反 Rust 的内存安全原则。
        Rust 中的可能实现方式
        由于 Rust 不允许直接按字节重新解释类型，您需要创建一个新的 Vec<u8>，并将 arr2 中从 nblock 开始的部分复制或转换到这个新 Vec 中。
        这样的转换取决于您的具体需求，例如 arr2 中的数据是如何组织的
     */
    // 假设 s 是 EState 的实例
    // 检查 nblock 是否在 arr2 的范围内

    // 下面的去掉了，见update_zbits
    // let arr2 = rebuild_arr2_from_block(&s.block);
    // let nblock_usize = s.nblock as usize;
    // if s.nblock < arr2.len() as i32 {
    //     // 从 arr2 中复制数据到 zbits
    //     // 根据您的需求，这里可能需要调整
    //     // 例如，如果您想将 arr2 中的每个 u32 分解为四个 u8
    //     s.zbits = arr2[nblock_usize..].iter().flat_map(|&x| x.to_ne_bytes().to_vec()).collect();
    // } else {
    //     // nblock 超出 arr2 的范围，需要处理错误
    //     // println!("错误：nblock 值 {} 超出 arr2 长度 {}", s.nblock, arr2.len());
    // }

    // unsafe {
    //     s.zbits = if let Some(arr2_ptr) = s.arr2 {
    //         // 将 *mut u32 转换为 *mut u8
    //         let arr2_ptr_u8 = arr2_ptr.add(s.nblock as usize) as *mut u8;
    //         Vec::from_raw_parts(arr2_ptr_u8, 0, 0) // 使用转换后的指针创建Vec
    //     } else {
    //         Vec::new() // 如果arr2是None，设置为空的Vec<u8>
    //     };
    // }
    // println!("zbits指针设置完成");

    // 如果是第一个块，则输出bzip2的文件标头
    if s.blockNo == 1 {
        // println!("处理首个数据块，写入bzip2文件头");
        bz2_bs_init_write(s);
        bs_put_uchar(s, 0x42); // 'B'

        // print_estate(s, "73_ADa_rs.json");
        // execute_command("cat 73_ADa_rs.json | cut -c 1-100 > /root/crown-rust/73_A.rs.txt");
        bs_put_uchar(s, 0x5a); // 'Z'

        // print_estate(s, "73_BDa_rs.json");
        // execute_command("cat 73_BDa_rs.json | cut -c 1-100 > /root/crown-rust/73_B.rs.txt");
        bs_put_uchar(s, 0x68); // 'h'

        // print_estate(s, "73_CDa_rs.json");
        // execute_command("cat 73_CDa_rs.json | cut -c 1-100 > /root/crown-rust/73_C.rs.txt");
        bs_put_uchar(s, 0x30 + s.blockSize100k as u8); // '0' + blockSize100k
        // print_estate(s, "73_bN1Da_rs.json");
        // execute_command("cat 73_bN1Da_rs.json | cut -c 1-100 > /root/crown-rust/bzip2_rs_gpt/73_bN1.rs.txt");
        // println!("bzip2文件头写入完成");
    }
    //没区别🐦和🦀️

    if s.nblock > 0 {
        // 输出固定的标记
        // println!("输出固定的标记");
        let magic: [u8; 6] = [0x31, 0x41, 0x59, 0x26, 0x53, 0x59];
        for &byte in &magic {
            bs_put_uchar(s, byte);
        }

        // 输出块的CRC和原始指针
        bs_put_uint32(s, s.blockCRC);
        bs_w(s, 1, 0);
        bs_w(s, 24, s.origPtr as u32);// s.origPtr==1

        // println!("输出块的CRC和原始指针完成");

        // print_estate(s, "73_qian_MTF_DA_rs.json");
        // execute_command("cat 73_qian_MTF_DA_rs.json | cut -c 1-100 > /root/crown-rust/bzip2_rs_gpt/73_qian_MTF_rs.txt");
        // 生成和发送MTF值
        // // println!("生成和发送MTF值");
        // let mut ptr_before_gmtf = unsafe { Vec::from_raw_parts(s.ptr.unwrap(), (s.nblock + 34) as usize, s.nblock as usize) };
        // // println!("ptr_before_gmtf: {:?}",ptr_before_gmtf);
        generate_mtf_values(s);
        // print_estate(s, "73_G_DA_rs.json");
        // execute_command("cat 73_G_DA_rs.json | cut -c 1-100 > /root/crown-rust/bzip2_rs_gpt/73_G_rs.txt");
        // 绝对无误
        // 🔥 s.bsLive == 25 🔥 
        sendMTFValues(s);
        // 🔥🔥🔥 s.bsLive == 9 🔥🔥🔥 

        // println!("MTF值生成和发送完成");
    }

    // print_estate(s, "73_S_DA_rs.json");
    // execute_command("cat 73_S_DA_rs.json | cut -c 1-100 > /root/crown-rust/bzip2_rs_gpt/73_S_rs.txt");
    // 如果是最后一个数据块
    if is_last_block {
        // println!("处理最后一个数据块"); // s.bsLive==9
        // 输出结束标记和累积的CRC
        let trailer: [u8; 6] = [0x17, 0x72, 0x45, 0x38, 0x50, 0x90];
        // s.bsLive==9 不一致
        for &byte in &trailer {
            bs_put_uchar(s, byte);
        }
        // s.bsLive==9
        bs_put_uint32(s, s.combinedCRC);

        // 输出诊断信息
        if s.verbosity >= 2 {
            // println!("    final combined CRC = 0x{:08x}", s.combinedCRC);
        }
        // s.bsLive==9
        // 完成写操作
        bs_finish_write(s);
        // println!("最后一个数据块处理完成");
    }

    // print_estate(s, "73_jieshuDA.rs.json");
    // execute_command("cat 73_jieshuDA.rs.json | cut -c 1-100 > /root/crown-rust/bzip2_rs_gpt/73_jieshu.rs.txt");

    // println!("数据块压缩流程结束");
}


