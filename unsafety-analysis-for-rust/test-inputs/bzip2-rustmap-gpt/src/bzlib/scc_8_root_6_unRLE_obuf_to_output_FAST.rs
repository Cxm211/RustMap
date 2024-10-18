
use crate::global_vars::bzip2_c1::{BZ2_crc32Table, BZ2_rNums, DState};


// unRLE_obuf_to_output_FAST ( DState* s )
type Int32 = i32;
type UInt32 = u32;
type UInt16 = u16;
type UChar = u8;
type Bool = bool;
pub fn unRLE_obuf_to_output_FAST(s: &mut DState) {
    let mut j = s.save_j as usize;
    let mut ch_prev = s.tPos as UInt32; 
    let mut ch2 = s.k0 as Int32; 
    let mut count = s.save_t; 
    let mut r_n_wrap = s.rNToGo;
    let mut r_t_pos = s.rTPos;

    // println!("开始 unRLE_obuf_to_output_FAST 函数");

    while j >= 0 {
        // println!("循环迭代, j = {}", j);

        // 检查输出
        if unsafe {(*s.strm).next_out.is_null()} || unsafe {(*s.strm).avail_out} == 0 || s.state_out_len == 0 {
            // println!("检测到输出为空或没有可用空间，保存状态并返回");
            s.save_j = j as Int32;
            s.tPos = ch_prev;
            s.k0 = ch2;
            s.save_t = count;
            s.rNToGo = r_n_wrap;
            s.rTPos = r_t_pos;
            return;
        }

        if j != 0 {
            // println!("处理 j != 0 的情况");
            // Drain the output buffer as long as there's data left to be written
            unsafe {
                let n = if s.state_out_len < (*s.strm).avail_out as i32 {
                    s.state_out_len
                } else {
                    (*s.strm).avail_out as i32
                } as usize;

                // println!("将要写入 {} 个字节", n);

                (*s.strm).avail_out -= n as u32;
                s.state_out_len -= n as i32;
            }
            loop {
                unsafe {
                    *(*s.strm).next_out = s.state_out_ch as i8;
                    // println!("写入字符: {}", s.state_out_ch);
                    (*s.strm).total_out_lo32 += 1;
                    (*s.strm).next_out = (*s.strm).next_out.add(1);
                }

                j -= 1;
                if j == 0 { break; }
            }

            if j == 0 {
                // println!("更新 ch_prev 和 count");
                ch_prev = ch2 as UInt32;
                count -= 1;
                if count == 0 {
                    ch2 = ch_prev as Int32;
                    count = r_n_wrap;
                    r_n_wrap -= 1;
                    if r_n_wrap < 0 {
                        r_t_pos += 1;
                        if r_t_pos == 512 { r_t_pos = 0; }
                        r_n_wrap = unsafe { BZ2_rNums[r_t_pos as usize] } - 1;
                    }
                }
            }
        } else {
            // Handle the RLE input data
            // println!("处理 RLE 输入数据");
            s.state_out_ch = ch2 as usize;
            j = unsafe {BZ2_rNums[r_t_pos as usize]} as usize;
            r_t_pos += 1;
            if r_t_pos == 512 { r_t_pos = 0; }
            r_n_wrap = unsafe {BZ2_rNums[r_t_pos as usize]} - 1;
            s.save_j = j as Int32;
            s.k0 = ch2;
            s.save_t = count;
            s.rNToGo = r_n_wrap;
            s.rTPos = r_t_pos;
            continue;
        }
    }
}
