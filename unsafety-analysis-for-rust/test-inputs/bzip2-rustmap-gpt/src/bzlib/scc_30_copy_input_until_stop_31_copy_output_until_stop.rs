use crate::global_vars::bzip2_c1::{EState,BZ2_crc32Table, print_estate};
use crate::bzlib::scc_29_root_22_add_pair_to_block::*;
// const BZ_M_IDLE: i32 = 1;
// const BZ_M_RUNNING: i32 = 2;
// const BZ_M_FLUSHING: i32 = 3;
// const BZ_M_FINISHING: i32 = 4;

// const BZ_S_OUTPUT: i32 = 1;
// const BZ_S_INPUT: i32 = 2;

// fn add_char_to_block(zs: &mut EState, zchh0: u32) {
//     // println!("进入函数 add_char_to_block");
//     let zchh = zchh0;
//     if zchh != zs.state_in_ch && zs.state_in_len == 1 {
//         // println!("条件分支1");
//         let ch = zs.state_in_ch as u8;
//         // BZ_UPDATE_CRC and other operations...
//         zs.inUse[zs.state_in_ch as usize] = true;
//         unsafe {
//             if let Some(block_ptr) = zs.block {
//                 // println!("正在写入 block");
//                 *block_ptr.add(zs.nblock as usize) = ch;
//             } else {
//                 // println!("警告：block 指针为空");
//             }
//         }
//         zs.nblock += 1;
//         zs.state_in_ch = zchh;
//     } else if zchh != zs.state_in_ch || zs.state_in_len == 255 {
//         // println!("条件分支2");
//         if zs.state_in_ch < 256 {
//             add_pair_to_block(zs);
//         }
//         zs.state_in_ch = zchh;
//         zs.state_in_len = 1;
//     } else {
//         // println!("条件分支3");
//         zs.state_in_len += 1;
//     }
//     // println!("离开函数 add_char_to_block");
// }

// pub fn copy_input_until_stop(zs: &mut EState) -> bool {
//     // println!("进入函数 copy_input_until_stop");
//     let mut progress_in = false;
//     if zs.mode == BZ_M_RUNNING {
//         // println!("模式：BZ_M_RUNNING");
//         while {
//             let block_full = zs.nblock >= zs.nblockMAX;
//             let no_input = unsafe { 
//                 zs.strm
//                     .map_or(true, |s| (*s).avail_in == 0)
//             };
//             !block_full && !no_input
//         } {
//             // println!("正在处理输入");
//             progress_in = true;
//             let next_in = unsafe {
//                 zs.strm
//                     .map_or(0, |s| *((*s).next_in) as u32)
//             };
//             add_char_to_block(zs, next_in);
//             // Updating the strm fields safely
//             unsafe {
//                 if let Some(strm) = zs.strm {
//                     (*strm).next_in = (*strm).next_in.add(1);
//                     (*strm).avail_in -= 1;
//                     (*strm).total_in_lo32 += 1;
//                     if (*strm).total_in_lo32 == 0 {
//                         (*strm).total_in_hi32 += 1;
//                     }
//                 }
//             }
//         }
//     } else {
//         // println!("模式：其他");
//         while {
//             let block_full = zs.nblock >= zs.nblockMAX;
//             let no_input = unsafe {
//                 zs.strm
//                     .map_or(true, |s| (*s).avail_in == 0)
//             };
//             let flush_finish_end = zs.avail_in_expect == 0;
//             !block_full && !no_input && !flush_finish_end
//         } {
//             // println!("正在处理输入（其他模式）");
//             progress_in = true;
//             let next_in = unsafe {
//                 zs.strm
//                     .map_or(0, |s| *((*s).next_in) as u32)
//             };
//             add_char_to_block(zs, next_in);
//             // Updating the strm fields safely
//             unsafe {
//                 if let Some(strm) = zs.strm {
//                     (*strm).next_in = (*strm).next_in.add(1);
//                     (*strm).avail_in -= 1;
//                     (*strm).total_in_lo32 += 1;
//                     if (*strm).total_in_lo32 == 0 {
//                         (*strm).total_in_hi32 += 1;
//                     }
//                     zs.avail_in_expect -= 1;
//                 }
//             }
//         }
//     }

//     // println!("离开函数 copy_input_until_stop");
//     progress_in
// }


pub fn copy_input_until_stop(s: &mut EState) -> bool {
    let mut progress_in = false;
    // print_estate(s, "38_EState_copy_input_until_stop_rust.json");
    // println!("当前模式：{}", s.mode);
    if s.mode == 2 {
        let mut counter = 0;
        while s.nblock < s.nblockMAX {
            counter += 1;
            // println!("模式2：处理 nblock < nblockMAX");
            let strm = unsafe { s.strm.as_ref().unwrap().as_mut().unwrap() };
            if strm.avail_in == 0 {
                // println!("输入流已耗尽");
                break;
            }
            progress_in = true;

            // println!("开始处理字符");
            let mut zchh = unsafe { *strm.next_in as u32 };
            if zchh > 255{ // 🏠🏠
                zchh = 255;
            }
            if counter == 1{
                // println!("c1: 当前字符zchh: {}, 即将被赋值给s.state_in_ch", zchh);
            }
            if counter == 2{
                // println!("c2: 当前字符zchh: {}, 即将被赋值给s.state_in_ch", zchh);
            }
            if zchh != s.state_in_ch && s.state_in_len == 1 {
                // println!("字符改变，处理单个字符");
                let ch = s.state_in_ch as u8;
                // println!("处理字符s.state_in_ch: {}", ch);
                
                s.blockCRC = (s.blockCRC << 8) ^ unsafe { BZ2_crc32Table[(s.blockCRC >> 24) as usize ^ ch as usize] };
                // println!("更新 blockCRC: {}", s.blockCRC);
                // 发现出错点在scc_38❌ 
                // pub   state_in_ch: UInt32,
                // println!("s.state_in_ch: {}; counter: {}", s.state_in_ch, counter);
                s.inUse[s.state_in_ch as usize] = true;
                // println!("设置 inUse[{}] 为 true", s.state_in_ch);
                
                // let block = unsafe { s.block.as_mut().unwrap() };
                let block = &s.block;
                // println!("获取 block 指针");
                // 💡💡💡
                // 确保 s.nblock 是有效的索引
                if s.nblock < s.block.len() as i32 {
                    s.block[s.nblock as usize] = ch;
                    // println!("写入位置: {}", s.nblock);
                } else {
                    // 如果 s.nblock 超出 s.block 的长度，则需要处理这种情况
                    // println!("警告：尝试写入超出 block 长度的索引 {}", s.nblock);
                    // 这里可以添加错误处理代码
                }

                // unsafe {
                //     let target = block.add(s.nblock as usize);
                //     // println!("写入位置: {:?}", target);
                //     *target = ch;
                // }
                // println!("在 block[{}] 写入字符 {}", s.nblock, ch);
                // println!("增加字符到块中：{}", ch);
                s.nblock += 1;
                if zchh <= 255{ 
                    s.state_in_ch = zchh;
                }
            } else if zchh != s.state_in_ch || s.state_in_len == 255 {
                // println!("字符重复或达到最大长度");
                if s.state_in_ch < 256 {
                    // println!("调用 add_pair_to_block");
                    add_pair_to_block(s);
                }
                s.state_in_ch = zchh;
                s.state_in_len = 1;
            } else {
                s.state_in_len += 1;
                // println!("增加 state_in_len，当前长度：{}", s.state_in_len);
            }
            
            strm.next_in = unsafe { strm.next_in.offset(1) };
            strm.avail_in -= 1;
            strm.total_in_lo32 += 1;
            // println!("更新流状态：next_in, avail_in, total_in_lo32");
            if strm.total_in_lo32 == 0 {
                strm.total_in_hi32 += 1;
                // println!("增加 total_in_hi32");
            }
        }
    } else {
        while s.nblock < s.nblockMAX {
            // println!("非模式2：处理 nblock < nblockMAX");
            let strm = unsafe { s.strm.as_ref().unwrap().as_mut().unwrap() };
            if strm.avail_in == 0 || s.avail_in_expect == 0 {
                // println!("输入流已耗尽或达到期望值");
                break;
            }
            progress_in = true;


            let zchh = unsafe { *strm.next_in as u32 };
            // println!("读取字符：{}", zchh);
            if zchh != s.state_in_ch && s.state_in_len == 1 {
                let ch = s.state_in_ch as u8;
                s.blockCRC = (s.blockCRC << 8) ^ unsafe { BZ2_crc32Table[(s.blockCRC >> 24) as usize ^ ch as usize] };
                s.inUse[s.state_in_ch as usize] = true;
                let block = &s.block;
                // 由于 s.block 现在是 Vec<u8> 类型，可以直接对其进行操作
                // 确保 s.nblock 是有效的索引 💡💡💡
                if s.nblock < s.block.len() as i32 {
                    s.block[s.nblock as usize] = ch;
                } else {
                    // 如果 s.nblock 超出 s.block 的长度，则需要处理这种情况
                    // 例如，可以扩展 Vec 或者记录错误
                    // println!("警告：尝试写入超出 block 长度的索引 {}", s.nblock);
                    // 可能的操作：s.block.push(ch); s.nblock += 1;
                    // 或者其他适当的错误处理
                }

                s.nblock += 1;
                s.state_in_ch = zchh;
            } else if zchh != s.state_in_ch || s.state_in_len == 255 {
                if s.state_in_ch < 256 {
                    add_pair_to_block(s);
                }
                if zchh <= 255{ 
                    s.state_in_ch = zchh;
                }
                s.state_in_len = 1;
            } else {
                s.state_in_len += 1;
            }

            strm.next_in = unsafe {strm.next_in.offset(1)} ;
            strm.avail_in -= 1;
            strm.total_in_lo32 += 1;
            if strm.total_in_lo32 == 0 {
                strm.total_in_hi32 += 1;
            }
            s.avail_in_expect -= 1;
        }
    }
    progress_in
}

pub fn copy_output_until_stop(s: &mut EState) -> bool {
    let mut progress_out = false;
    // println!("函数开始: copy_output_until_stop");

    if let Some(strm) = s.strm.as_mut() {
        loop {
            // println!("39_进入循环");
            // Wrap the unsafe operations inside an unsafe block
            unsafe {
                // println!("39_检查条件: avail_out 和 state_out_pos");
                if (*(*strm)).avail_out == 0 || s.state_out_pos as usize >= s.numZ as usize {
                    // println!("39_条件触发: 退出循环");
                    break;
                }
            
                progress_out = true;
                // 直接使用 zbits 的原生指针
                if !s.zbits.is_empty() {
                    // println!("39_zbits_ptr 不为空，进行操作");
                    *(*(*strm)).next_out = *s.zbits.as_ptr().add(s.state_out_pos as usize) as i8;
                    (*(*strm)).next_out = (*(*strm)).next_out.add(1);
                    
                    s.state_out_pos += 1;
                    (*(*strm)).avail_out -= 1;
                    (*(*strm)).total_out_lo32 = (*(*strm)).total_out_lo32.wrapping_add(1);
                
                    if (*(*strm)).total_out_lo32 == 0 {
                        // println!("39_total_out_lo32 为 0，增加 total_in_hi32");
                        (*(*strm)).total_in_hi32 += 1;
                    }
                } else {
                    // println!("39_zbits_ptr 为空，跳过当前循环");
                }
            }
        }
    } else {
        eprintln!("39_Warning: s.strm is None!");
        return false;
    }

    // println!("39_函数结束: progress_out = {}", progress_out);
    progress_out
}






#[cfg(test)]
mod tests {
    use super::*;
    use crate::global_vars::bzip2_c1::{EState,BZ2_crc32Table};
    #[test]
    fn test_copy_input_until_stop() {
        let mut s = EState::default();

        let result = copy_input_until_stop(&mut s);
        // println!("Result: {:?}", result);

        // 根据具体情况进行断言
        // assert!(result);

        // 如果需要进一步验证EState s的状态，你可以在这里添加其他断言
        // 例如:
        // assert_eq!(s.some_field, expected_value);
    }

    #[test]
    fn test_copy_output_until_stop() {
        let mut s = EState::default();

        let result = copy_output_until_stop(&mut s);
        // 打印result的值
        // println!("Result: {:?}", result);
        // 根据具体情况进行断言
        // assert!(result);

        // 如果需要进一步验证EState s的状态，你可以在这里添加其他断言
        // 例如:
        // assert_eq!(s.some_field, expected_value);
    }
}
