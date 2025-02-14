
use crate::compress::bsW::bs_w;
use crate::huffman::BZ2_hbMakeCodeLengths:bz2_hb_make_code_lengths;
use crate::huffman::BZ2_hbAssignCodes:bz2_hb_assign_codes;
use crate::EState;
use crate::BZ2_bz__AssertH__fail;

pub fn send_mtf_values(s: &mut EState) {
    let mut v: i32;
    let mut t: i32;
    let mut i: i32;
    let mut j: i32;
    let mut gs: i32;
    let mut ge: i32;
    let mut totc: i32;
    let mut bt: i32;
    let mut bc: i32;
    let mut iter: i32;
    let mut n_selectors: i32;
    let mut alpha_size: i32;
    let mut min_len: i32;
    let mut max_len: i32;
    let mut sel_ctr: i32;
    let mut n_groups: i32;
    let mut n_bytes: i32;
    let mut fave = [0i32; 6];
    let mut cost = [0u16; 6];

    let mtfv = s.mtfv.as_mut().unwrap();

    if s.verbosity >= 3 {
        eprintln!(
            "      {} in block, {} after MTF & 1-2 coding, {}+2 syms in use",
            s.nblock, s.n_mtf, s.n_in_use
        );
    }

    alpha_size = s.n_in_use + 2;
    for t in 0..6 {
        for v in 0..alpha_size {
            s.len[t as usize][v as usize] = 15;
        }
    }

    if s.n_mtf <= 0 {
        BZ2_bz__AssertH__fail(3001);
    }

    n_groups = if s.n_mtf < 200 {
        2
    } else if s.n_mtf < 600 {
        3
    } else if s.n_mtf < 1200 {
        4
    } else if s.n_mtf < 2400 {
        5
    } else {
        6
    };

    let mut rem_f = s.n_mtf;
    let mut n_part = n_groups;
    gs = 0;
    while n_part > 0 {
        let mut t_freq = rem_f / n_part;
        ge = gs - 1;
        let mut a_freq = 0;
        while a_freq < t_freq && ge < alpha_size - 1 {
            ge += 1;
            a_freq += s.mtf_freq[ge as usize];
        }

        if ge > gs && n_part != n_groups && n_part != 1 && (n_groups - n_part) % 2 == 1 {
            a_freq -= s.mtf_freq[ge as usize];
            ge -= 1;
        }

        if s.verbosity >= 3 {
            eprintln!(
                "      initial group {}, [{} .. {}], has {} syms ({:.1}%)",
                n_part,
                gs,
                ge,
                a_freq,
                (100.0 * (a_freq as f64)) / (s.n_mtf as f64)
            );
        }

        for v in 0..alpha_size {
            if v >= gs && v <= ge {
                s.len[(n_part - 1) as usize][v as usize] = 0;
            } else {
                s.len[(n_part - 1) as usize][v as usize] = 15;
            }
        }

        n_part -= 1;
        gs = ge + 1;
        rem_f -= a_freq;
    }

    // Iterate 4 times
    for iter in 0..4 {
        for t in 0..n_groups {
            fave[t as usize] = 0;
            for v in 0..alpha_size {
                s.rfreq[t as usize][v as usize] = 0;
            }
        }

        // Handle packing of lengths for optimization
        if n_groups == 6 {
            for v in 0..alpha_size {
                s.len_pack[v as usize][0] = (s.len[1][v as usize] << 16) as u32 | s.len[0][v as usize] as u32;
                s.len_pack[v as usize][1] = (s.len[3][v as usize] << 16) as u32 | s.len[2][v as usize] as u32;
                s.len_pack[v as usize][2] = (s.len[5][v as usize] << 16) as u32 | s.len[4][v as usize] as u32;
            }
        }

        n_selectors = 0;
        totc = 0;
        gs = 0;

        while gs < s.n_mtf {
            ge = gs + 50 - 1;
            if ge >= s.n_mtf {
                ge = s.n_mtf - 1;
            }

            for t in 0..n_groups {
                cost[t as usize] = 0;
            }

            if n_groups == 6 && ge - gs + 1 == 50 {
                let mut cost01 = 0u32;
                let mut cost23 = 0u32;
                let mut cost45 = 0u32;

                for idx in gs..=ge {
                    let icv = mtfv[idx as usize];
                    cost01 += s.len_pack[icv as usize][0] as u32;
                    cost23 += s.len_pack[icv as usize][1] as u32;
                    cost45 += s.len_pack[icv as usize][2] as u32;
                }

                cost[0] = cost01 as u16;
                cost[1] = (cost01 >> 16) as u16;
                cost[2] = cost23 as u16;
                cost[3] = (cost23 >> 16) as u16;
                cost[4] = cost45 as u16;
                cost[5] = (cost45 >> 16) as u16;
            } else {
                for i in gs..=ge {
                    let icv = mtfv[i as usize];
                    for t in 0..n_groups {
                        cost[t as usize] += s.len[t as usize][icv as usize] as u16;
                    }
                }
            }

            // Find the group with the lowest cost
            bc = i32::MAX;
            bt = -1;
            for t in 0..n_groups {
                if i32::from(cost[t as usize]) < bc {
                    bc = cost[t as usize] as i32;
                    bt = t;
                }
            }

            totc += bc;
            fave[bt as usize] += 1;
            s.selector[n_selectors as usize] = bt as u8;
            n_selectors += 1;

            for i in gs..=ge {
                s.rfreq[bt as usize][mtfv[i as usize] as usize] += 1;
            }

            gs = ge + 1;
        }

        if s.verbosity >= 3 {
            eprintln!("      pass {}: size is {}, grp uses are {:?}", iter + 1, totc / 8, &fave);
        }

        for t in 0..n_groups {
            bz2_hb_make_code_lengths(&mut s.len[t as usize], &mut s.rfreq[t as usize], alpha_size, 17);
        }
    }

    if n_groups >= 8 {
        BZ2_bz__AssertH__fail(3002);
    }

    if n_selectors >= 32768 || n_selectors > 2 + (900000 / 50) {
        BZ2_bz__AssertH__fail(3003);
    }

    // Handle selector MTF (Move-To-Front) encoding
    let mut pos = [0u8; 6];
    for i in 0..n_groups {
        pos[i as usize] = i as u8;
    }

    for i in 0..n_selectors {
        let mut ll_i = s.selector[i as usize];
        let mut j = 0;
        let mut tmp = pos[j as usize];
        while ll_i != tmp {
            j += 1;
            let tmp2 = tmp;
            tmp = pos[j as usize];
            pos[j as usize] = tmp2;
        }
        pos[0] = tmp;
        s.selector_mtf[i as usize] = j as u8;
    }

    // Generate the code lengths and codes
    for t in 0..n_groups {
        min_len = 32;
        max_len = 0;
        for i in 0..alpha_size {
            if i32::from(s.len[t as usize][i as usize]) > max_len {
                max_len = s.len[t as usize][i as usize] as i32;
            }
            if i32::from(s.len[t as usize][i as usize]) < min_len {
                min_len = s.len[t as usize][i as usize] as i32;
            }
        }

        if max_len > 17 {
            BZ2_bz__AssertH__fail(3004);
        }

        if min_len < 1 {
            BZ2_bz__AssertH__fail(3005);
        }

        bz2_hb_assign_codes(&mut s.code[t as usize], &s.len[t as usize], min_len, max_len, alpha_size);
    }

    // Emit the encoded information
    n_bytes = s.num_z;
    let mut in_use16 = [false; 16];

    for i in 0..16 {
        for j in 0..16 {
            if s.in_use[(i * 16 + j) as usize] {
                in_use16[i as usize] = true;
            }
        }
    }

    // Write the mapping table for inUse16
    for i in 0..16 {
        if in_use16[i as usize] {
            bs_w(s, 1, 1);
        } else {
            bs_w(s, 1, 0);
        }
    }

    // Write the actual symbols that are in use
    for i in 0..16 {
        if in_use16[i as usize] {
            for j in 0..16 {
                if s.in_use[(i * 16 + j) as usize] {
                    bs_w(s, 1, 1);
                } else {
                    bs_w(s, 1, 0);
                }
            }
        }
    }

    if s.verbosity >= 3 {
        eprintln!("      bytes: mapping {}", s.num_z - n_bytes);
    }

    // Emit the number of groups and selectors
    n_bytes = s.num_z;
    bs_w(s, 3, n_groups.try_into().unwrap());
    bs_w(s, 15, n_selectors.try_into().unwrap());


    for i in 0..n_selectors {
        for j in 0..s.selector_mtf[i as usize] {
            bs_w(s, 1, 1);
        }
        bs_w(s, 1, 0);
    }

    if s.verbosity >= 3 {
        eprintln!("      selectors {}", s.num_z - n_bytes);
    }

    // Emit the Huffman code lengths for each group
    n_bytes = s.num_z;
    for t in 0..n_groups {
        let mut curr = s.len[t as usize][0];
        bs_w(s, 5, curr as u32);

        for i in 1..alpha_size {
            while curr as  < s.len[t as usize][i as usize] {
                bs_w(s, 2, 2);
                curr += 1;
            }
            while curr > s.len[t as usize][i as usize] {
                bs_w(s, 2, 3);
                curr -= 1;
            }
            bs_w(s, 1, 0);
        }
    }

    if s.verbosity >= 3 {
        eprintln!("      code lengths {}", s.num_z - n_bytes);
    }

    // Send the selectors and the encoded values
    n_bytes = s.num_z;
    sel_ctr = 0;
    gs = 0;

    while gs < s.n_mtf {
        ge = gs + 50 - 1;
        if ge >= s.n_mtf {
            ge = s.n_mtf - 1;
        }

        if i32::from(s.selector[sel_ctr as usize]) >= n_groups {
            BZ2_bz__AssertH__fail(3006);
        }

        if n_groups == 6 && ge - gs + 1 == 50 {
            for idx in gs..=ge {
                let mtfv_i = mtfv[idx as usize];
                let s_len_sel_sel_ctr = &s.len[s.selector[sel_ctr as usize] as usize];
                let s_code_sel_sel_ctr = &s.code[s.selector[sel_ctr as usize] as usize];

                bs_w(s, s_len_sel_sel_ctr[mtfv_i as usize] as i32, s_code_sel_sel_ctr[mtfv_i as usize].try_into().unwrap());
            }
        } else {
            for i in gs..=ge {
                bs_w(
                    s,
                    s.len[s.selector[sel_ctr as usize] as usize][mtfv[i as usize] as usize] as i32,
                    s.code[s.selector[sel_ctr as usize] as usize][mtfv[i as usize] as usize].try_into().unwrap(),
                );
            }
        }

        gs = ge + 1;
        sel_ctr += 1;
    }

    if sel_ctr != n_selectors {
        BZ2_bz__AssertH__fail(3007);
    }

    if s.verbosity >= 3 {
        eprintln!("      codes {}", s.num_z - n_bytes);
    }
}
