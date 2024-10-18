// use crate::compress::scc_67_sendMTFValues::*;
// use crate::compress::scc_26_root_18_makeMaps_e::*;
// use crate::compress::scc_36_root_24_BZ2_bsInitWrite::*;
// use crate::decompress::scc_63_BZ2_decompress::*;
use crate::global_vars::bzip2_c1::{EState};
use crate::huffman::scc_62_BZ2_hbMakeCodeLengths::*;
use crate::huffman::scc_17_root_14_BZ2_hbAssignCodes::*;
use crate::compress::scc_20_root_25_bsW::*;

// sendMTFValues ( EState* s )
pub fn sendMTFValues(s: &mut EState) {
    let mut v: i32;
    let mut t: i32;
    let mut j: i32;
    let mut gs: i32;
    let mut ge: i32;
    let mut totc: i32;
    let mut bt: i32 = 0;
    let mut bc: i32;
    let mut iter: i32;
    let mut n_selectors: i32 = 0;
    let mut alpha_size: i32;
    let mut min_len: i32;
    let mut max_len: i32;
    let mut sel_ctr: i32;
    let mut nGroups: i32;
    let mut n_bytes: i32;
    let mut cost = [0u16; 6];
    let mut fave = [0i32; 6];
    let mtfv = &s.mtfv;

    if s.verbosity >= 3 {
        eprintln!(
            "      {} in block, {} after MTF & 1-2 coding, {}+2 syms in use",
            s.nblock, s.nMTF, s.nInUse
        );
    }

    alpha_size = s.nInUse + 2;
    for t in 0..6 {
        for v in 0..alpha_size {
            s.len[t as usize][v as usize] = 15;
        }
    }

    assert!(s.nMTF > 0, "nMTF should be greater than 0");
    nGroups = match s.nMTF {
        0..=199 => 2,
        200..=599 => 3,
        600..=1199 => 4,
        1200..=2399 => 5,
        _ => 6,
    };


    // 平均分配MTF列表的代码
    {
        let mut n_part: i32;
        let mut rem_f: i32;
        let mut t_freq: i32;
        let mut a_freq: i32;
        n_part = nGroups;
        rem_f = s.nMTF;
        gs = 0;
        while n_part > 0 {
            t_freq = rem_f / n_part;
            ge = gs - 1;
            a_freq = 0;
            while a_freq < t_freq && ge < alpha_size - 1 {
                ge += 1;
                a_freq += s.mtfFreq[ge as usize];
            }
            if ge > gs && n_part != nGroups && n_part != 1 && ((nGroups - n_part) % 2 == 1) {
                a_freq -= s.mtfFreq[ge as usize];
                ge -= 1;
            }

            // 打印信息（如果verbosity级别高）
            if s.verbosity >= 3 {
                eprintln!(
                    "      initial group {}, [{}, .. {}], has {} syms ({:.1}%)",
                    n_part,
                    gs,
                    ge,
                    a_freq,
                    100.0 * a_freq as f32 / s.nMTF as f32
                );
            }

            for v in 0..alpha_size {
                s.len[n_part as usize - 1][v as usize] = if v >= gs && v <= ge { 0 } else { 15 };
            }
            n_part -= 1;
            gs = ge + 1;
            rem_f -= a_freq;
        }
    }


    // 大型forLoop 这是一个迭代过程，尝试为MTF列表中的每个符号找到最佳的编码
    for iter in 0..4 {
        for t in 0..nGroups as usize {
            fave[t as usize] = 0;
            for v in 0..alpha_size as usize {
                s.rfreq[t as usize][v] = 0;
            }
        }
    
        // 如果 nGroups 为 6，进行特殊处理
        if nGroups == 6 {
            for v in 0..alpha_size as usize {
                s.len_pack[v][0] = (s.len[1][v] as u32) << 16 | s.len[0][v] as u32;
                s.len_pack[v][1] = (s.len[3][v] as u32) << 16 | s.len[2][v] as u32;
                s.len_pack[v][2] = (s.len[5][v] as u32) << 16 | s.len[4][v] as u32;
            }
        }
        // 处理选择器和相关计算
        n_selectors = 0;
        totc = 0;
        gs = 0;
        while true {
            if gs >= s.nMTF {
                break;
            }
            let mut ge = gs + 50 - 1;
            if ge >= s.nMTF {
                ge = s.nMTF - 1;
            }
            for t in 0..nGroups {
                cost[t as usize] = 0;
            }
        
            if nGroups == 6 && 50 == ge - gs + 1 {
                let mut cost01: u32 = 0;
                let mut cost23: u32 = 0;
                let mut cost45: u32 = 0;
                let mut icv: u16;
        
                for i in 0..50 {
                    icv = mtfv[(gs + i) as usize];
                    cost01 += s.len_pack[icv as usize][0];
                    cost23 += s.len_pack[icv as usize][1];
                    cost45 += s.len_pack[icv as usize][2];
                }
        
                cost[0] = (cost01 & 0xffff) as u16;
                cost[1] = (cost01 >> 16) as u16;
                cost[2] = (cost23 & 0xffff) as u16;
                cost[3] = (cost23 >> 16) as u16;
                cost[4] = (cost45 & 0xffff) as u16;
                cost[5] = (cost45 >> 16) as u16;
            } else {
                for i in gs..=ge {
                    let icv = mtfv[i as usize];
                    for t in 0..nGroups {
                        cost[t as usize] += (s.len[t as usize][icv as usize]) as u16;
                    }
                }
            }
             // 寻找最佳组:
            bc = 999999999;
            bt = -1;
            for t in 0..nGroups {
                if cost[t as usize] < bc as u16{
                    bc = cost[t as usize] as i32;
                    bt = t as i32;
                }
            }
            totc += bc as i32;
            fave[bt as usize] += 1;
            s.selector[n_selectors as usize] = bt as u8;
            n_selectors += 1;
        
            if nGroups == 6 && 50 == ge - gs + 1 {
                for i in 0..50 {
                    s.rfreq[bt as usize][mtfv[(gs + i) as usize] as usize] += 1;
                }
            } else {
                for i in gs..=ge {
                    s.rfreq[bt as usize][mtfv[i as usize] as usize] += 1;
                }
            }
             // 最后，起始位置gs移动到下一个组。
            gs = ge + 1;
        }



         // 如果 verbosity 级别高，打印相关信息
        if s.verbosity >= 3 {
            eprintln!("      pass {}: size is {}, grp uses are ", iter + 1, totc / 8);
            for t in 0..nGroups as usize {
                eprint!("{} ", fave[t as usize]);
            }
            eprintln!();
        }

        // 调用 bz2_hb_make_code_lengths 函数
        for t in 0..nGroups as usize {
            bz2_hb_make_code_lengths(&mut s.len[t as usize], &s.rfreq[t as usize], alpha_size, 17);
        }
    }
    // 验证 nGroups 和 nSelectors 是否在预期范围内
    assert!(nGroups < 8, "nGroups should be less than 8");
    assert!(n_selectors < 32768 && n_selectors <= (2 + (900000 / 50)), "Invalid number of selectors");

    // 接下来处理选择器和MTF值的映射
    {
        let mut pos = [0u8; 6];
        let mut ll_i: u8;
        let mut tmp2: u8;
        let mut tmp: u8;
        for i in 0..nGroups as usize {
            pos[i as usize] = i as u8;
        }
        
        for i in 0..n_selectors as usize {
            ll_i = s.selector[i as usize];
            j = 0;
            tmp = pos[j as usize];
            while ll_i != tmp {
                j += 1;
                tmp2 = tmp;
                tmp = pos[j as usize];
                pos[j as usize] = tmp2;
            }
            pos[0] = tmp;
            s.selectorMtf[i as usize] = j as u8;
        }
    }   
    // 计算每个组的最小和最大长度
    for t in 0..nGroups {
        min_len = 32;
        max_len = 0;
        for i in 0..alpha_size {
            if s.len[t as usize][i as usize] as i32 > max_len {
                max_len = s.len[t as usize][i as usize] as i32;
            }
            if (s.len[t as usize][i as usize] as i32) < min_len {
                min_len = s.len[t as usize][i as usize] as i32;
            }
        }
        assert!(max_len <= 17, "Max length should be less than or equal to 17");
        assert!(min_len >= 1, "Min length should be greater than or equal to 1");
        // 生成Huffman编码
        bz2_hb_assign_codes(&mut s.code[t as usize], &s.len[t as usize], min_len as i32, max_len as i32, alpha_size);
    }

     /*--- Transmit the mapping table. ---*/ // 处理 inUse 数组和写入位流
     {
        let mut in_use_16 = [false; 16];
        // for i in 0..16 {
        //     in_use_16[i as usize] = s.inUse[i * 16..(i + 1) * 16].iter().any(|&x| x);
        // }
        for i in 0..16 {
            in_use_16[i] = false;
            for j in 0..16 {
                if s.inUse[i * 16 + j] {
                    in_use_16[i] = true;
                    break; // Break if any in_use is true
                }
            }
        }
    
        n_bytes = s.numZ;
        for i in 0..16 {
            if in_use_16[i] {
                bs_w(s, 1, 1);
            } else {
                bs_w(s, 1, 0);
            }
        }
        
        for i in 0..16 {
            if in_use_16[i] {
                for j in 0..16 {
                    if s.inUse[i * 16 + j] {
                        bs_w(s, 1, 1);
                    } else {
                        bs_w(s, 1, 0);
                    }
                }
            }
        }
        // 如果verbosity级别大于等于3，输出选择器数量
        if s.verbosity >= 3 {
            eprintln!("bytes: mapping {},", s.numZ - n_bytes);
        }
    }

    // 更新已处理的字节数
    n_bytes = s.numZ;
    bs_w(s, 3, nGroups as u32);         // Ensure s.bs_live == 4
    bs_w(s, 15, n_selectors as u32);     // Ensure s.bs_live == 19

    for i in 0..n_selectors {
        for j in 0..s.selectorMtf[i as usize] {
            bs_w(s, 1, 1);
        }
        bs_w(s, 1, 0);
    }
       // 如果verbosity级别大于等于3，则输出选择器的数量
    if s.verbosity >= 3 {
        eprintln!("selectors {},", s.numZ - n_bytes);
    }

    /*--- Now the coding tables. ---*/
    n_bytes = s.numZ;
    // 遍历所有的组
    for t in 0..nGroups {
        // 获取当前组的第一个长度
        let mut curr: i32 = s.len[t as usize][0] as i32;
        bs_w(s, 5, curr as u32);     // 将当前长度写入位流
        // 遍历所有alphaSize符号
        for i in 0..alpha_size {
            // 增加或减少长度以匹配符号长度，写入相应的位
            while curr < s.len[t as usize][i as usize] as i32{
                bs_w(s, 2, 2);
                curr += 1;
            }
            while curr > s.len[t as usize][i as usize] as i32 {
                bs_w(s, 2, 3);
                curr -= 1;
            }
            // 写入终止符
            bs_w(s, 1, 0);
        }
    }
    // 如果verbosity级别大于等于3，输出编码长度信息
    if s.verbosity >= 3 {
        eprintln!("code lengths {},", s.numZ - n_bytes);
    }



    /*--- And finally, the block data proper ---*/// 选择器计数器
    n_bytes = s.numZ;
    sel_ctr = 0;
    gs = 0;
    // 处理MTF序列
    
// Infinite loop for processing MTF sequence
while true {
    // Exit loop if gs exceeds n_mtf
    if gs >= s.nMTF {
        break;
    }

    // Calculate the end position of the current group
    let mut ge = gs + 50 - 1;
    if ge >= s.nMTF {
        ge = s.nMTF - 1;
    }

    // Assert check to ensure selector[sel_ctr] is less than n_groups
    assert!(s.selector[sel_ctr as usize] < nGroups as u8);

    // Special case: handling logic when n_groups is 6 and group size is 50
    if nGroups == 6 && 50 == ge - gs + 1 {
        for i in gs..=ge {
            let mtfv_i = s.mtfv[i as usize];
            let s_len_sel_sel_ctr = &s.len[s.selector[sel_ctr as usize] as usize];
            let s_code_sel_sel_ctr = &s.code[s.selector[sel_ctr  as usize] as usize];
            bs_w(s, s_len_sel_sel_ctr[mtfv_i as usize] as i32, s_code_sel_sel_ctr[mtfv_i as usize] as u32);
        }
    } else {
        // General case: iterate through the current MTF sequence
        for i in gs..=ge {
            bs_w(s, s.len[s.selector[sel_ctr as usize] as usize][s.mtfv[i as usize] as usize] as i32, s.code[s.selector[sel_ctr as usize] as usize][s.mtfv[i as usize] as usize] as u32);
        }
    }

    // Update gs and selector counter
    gs = ge + 1;
    sel_ctr += 1;
}
    
    // 断言检查，确保所有选择器都已处理
    assert!(sel_ctr == n_selectors, "BZ2_bz__AssertH__fail (3007)");

    // 如果verbosity级别大于等于3，输出最终的压缩数据大小
    if s.verbosity >= 3 {
        eprintln!("codes lengths {}\n", s.numZ - n_bytes);
    }




}