use crate::bzlib::scc_6_root_5_BZ2_indexIntoF::*;
use crate::global_vars::bzip2_c1::{BZ2_crc32Table, BZ2_rNums, DState};

pub fn un_rle_obuf_to_output_small(s: &mut DState) {
    let mut t = (*s).tPos as u32;
    let tt = (*s).tt as *mut u32;
    let ll4 = (*s).ll4 as *mut u8;
    let ll16 = (*s).ll16 as *mut u16;

    // println!("开始 un_rle_obuf_to_output_small 函数");
    // println!("初始 t = {}", t);

    // 注意: 根据Rust的安全性规则，访问裸指针是不安全的。
    // 因此，所有涉及到裸指针访问的代码都需要放在unsafe块中。

    unsafe {
        while (*(*s).strm).avail_out > 0 && t > 0 {
            // println!("循环开始: t = {}", t);
            t -= 1;

            let mut tTmp = *tt.offset((t >> 8) as isize);
            let uc = *ll4.offset((t & 0x000000ff) as isize);

            // println!("tTmp (之前) = {}", tTmp);
            // println!("uc (ll4) = {}", uc);

            tTmp |= uc as u32;
            let uc = *ll16.offset((t >> 8) as isize) as u8;

            // println!("tTmp (之后) = {}", tTmp);
            // println!("uc (ll16) = {}", uc);

            t = tTmp;

            if s.state_out_ch != 256 {
                // println!("处理 state_out, state_out_ch = {}", s.state_out_ch);

                while (*s).state_out_len > 0 && (*(*s).strm).avail_out > 0 {
                    // println!("输出字符: {}", s.state_out_ch);
                    *((*(*s).strm).next_out) = s.state_out_ch as i8;
                    (*(*s).strm).next_out = (*(*s).strm).next_out.add(1);
                    (*(*s).strm).avail_out -= 1;
                    (*(*s).strm).total_out_lo32 += 1;
                    if (*(*s).strm).total_out_lo32 == 0 {
                        (*(*s).strm).total_out_hi32 += 1;
                    }
                    (*s).state_out_len -= 1;
                }
                if (*s).state_out_len == 0 {
                    s.state_out_ch = 256;
                }
            }

            let n = bz2_index_into_f((t & 0xff) as i32, &s.cftab) as u32;

            // println!("n = {}", n);

            // Fetch data

            tTmp = *tt.offset((t >> 8) as isize);
            let uc = *ll4.offset((t & 0x000000ff) as isize);

            tTmp |= uc as u32;
            let uc = *ll16.offset((t >> 8) as isize) as u8;

            t = tTmp;

            if n != s.k0 as u32 {
                // println!("更新 s.k0, 原始值 = {}, 新值 = {}", s.k0, n);
                if n > s.k0 as u32 {
                    (*(*s).strm).total_out_lo32 += (n - s.k0 as u32) as u32;
                } else {
                    (*(*s).strm).total_out_lo32 -= (s.k0 as u32 - n) as u32;
                }

                s.k0 = n as i32;

                if (*(*s).strm).total_out_lo32 < 0 {
                    (*(*s).strm).total_out_hi32 -= 1;
                }
            }

            s.state_out_ch = s.seqToUnseq[n as usize] as usize;
            s.state_out_len = 1;

            // println!("设置 state_out_ch = {}, state_out_len = 1", s.state_out_ch);

            while (*s).state_out_len > 0 && (*(*s).strm).avail_out > 0 {
                // println!("输出字符: {}", s.state_out_ch);
                *((*(*s).strm).next_out) = s.state_out_ch as i8;
                (*(*s).strm).next_out = (*(*s).strm).next_out.add(1);
                (*(*s).strm).avail_out -= 1;
                (*(*s).strm).total_out_lo32 += 1;
                if (*(*s).strm).total_out_lo32 == 0 {
                    (*(*s).strm).total_out_hi32 += 1;
                }
                (*s).state_out_len -= 1;
            }
            if (*s).state_out_len == 0 {
                s.state_out_ch = 256;
            }
        }
    }

    (*s).tPos = t;
    // println!("函数结束, 最终 t = {}", t);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn free_dstate(_ds: DState) {
        // 如果有必要，释放资源的逻辑
    }

    #[test]
    fn test_un_rle_obuf_to_output_small() {
        let mut ds = DState::default();
        // 为ds的其他字段设置测试数据

        un_rle_obuf_to_output_small(&mut ds);
        
        // 此处，我们无法简单地检查函数的返回值，因为Rust函数没有返回值。
        // 因此，你可能需要检查ds的某些字段是否符合你的期望。

    }
}
