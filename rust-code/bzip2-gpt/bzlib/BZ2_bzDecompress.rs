use crate::decompress::BZ2_decompress::bz2_decompress;
use crate::bz_stream;
use crate::BZ2_CRC32_TABLE;
use crate::BZ2_rNums;
use crate::DState;
use crate::BZ2_bz__AssertH__fail;

pub fn un_rle_obuf_to_output_fast(s: &mut DState) -> bool {
    if s.block_randomised {
        while s.state_out_len > 0 || s.nblock_used < s.save_nblock + 1 {
            while s.state_out_len > 0 {
                let strm = s.strm.as_mut().unwrap();
                if strm.avail_out == 0 {
                    return false;
                }
                unsafe {
                    *strm.next_out = s.state_out_ch;
                    s.calculated_block_crc = (s.calculated_block_crc << 8)
                        ^ BZ2_CRC32_TABLE[((s.calculated_block_crc >> 24) ^ u32::from(s.state_out_ch)) as usize];
                    s.state_out_len -= 1;
                    strm.next_out = strm.next_out.add(1);
                    strm.avail_out -= 1;
                    strm.total_out_lo32 += 1;
                    if strm.total_out_lo32 == 0 {
                        strm.total_out_hi32 += 1;
                    }
                }
            }

            if s.nblock_used == s.save_nblock + 1 {
                return false;
            }

            if s.nblock_used > s.save_nblock + 1 {
                return true;
            }

            s.state_out_len = 1;
            s.state_out_ch = s.k0 as u8;
            if s.t_pos >= 100_000 * s.block_size_100k as u32 {
                return true;
            }
            s.t_pos = s.tt.as_mut().unwrap()[s.t_pos as usize];
            let mut k1 = (s.t_pos & 0xff) as u8;
            s.t_pos >>= 8;
            if s.r_n_to_go == 0 {
                s.r_n_to_go = BZ2_rNums[s.r_t_pos as usize];
                s.r_t_pos += 1;
                if s.r_t_pos == 512 {
                    s.r_t_pos = 0;
                }
            }
            s.r_n_to_go -= 1;
            k1 ^= if s.r_n_to_go == 1 { 1 } else { 0 };
            s.nblock_used += 1;
            if s.nblock_used == s.save_nblock + 1 {
                continue;
            }
            if k1 != s.k0 as u8 {
                s.k0 = k1 as i32;
                continue;
            }

            s.state_out_len = 2;
            if s.t_pos >= 100_000 * s.block_size_100k as u32 {
                return true;
            }
            s.t_pos = s.tt.as_mut().unwrap()[s.t_pos as usize];
            k1 = (s.t_pos & 0xff) as u8;
            s.t_pos >>= 8;
            if s.r_n_to_go == 0 {
                s.r_n_to_go = BZ2_rNums[s.r_t_pos as usize];
                s.r_t_pos += 1;
                if s.r_t_pos == 512 {
                    s.r_t_pos = 0;
                }
            }
            s.r_n_to_go -= 1;
            k1 ^= if s.r_n_to_go == 1 { 1 } else { 0 };
            s.nblock_used += 1;
            if s.nblock_used == s.save_nblock + 1 {
                continue;
            }
            if k1 != s.k0 as u8 {
                s.k0 = k1 as i32;
                continue;
            }

            s.state_out_len = 3;
            if s.t_pos >= 100_000 * s.block_size_100k as u32 {
                return true;
            }
            s.t_pos = s.tt.as_mut().unwrap()[s.t_pos as usize];
            k1 = (s.t_pos & 0xff) as u8;
            s.t_pos >>= 8;
            if s.r_n_to_go == 0 {
                s.r_n_to_go = BZ2_rNums[s.r_t_pos as usize];
                s.r_t_pos += 1;
                if s.r_t_pos == 512 {
                    s.r_t_pos = 0;
                }
            }
            s.r_n_to_go -= 1;
            k1 ^= if s.r_n_to_go == 1 { 1 } else { 0 };
            s.nblock_used += 1;
            if s.nblock_used == s.save_nblock + 1 {
                continue;
            }
            if k1 != s.k0 as u8 {
                s.k0 = k1 as i32;
                continue;
            }

            if s.t_pos >= 100_000 * s.block_size_100k as u32 {
                return true;
            }
            s.t_pos = s.tt.as_mut().unwrap()[s.t_pos as usize];
            k1 = (s.t_pos & 0xff) as u8;
            s.t_pos >>= 8;
            if s.r_n_to_go == 0 {
                s.r_n_to_go = BZ2_rNums[s.r_t_pos as usize];
                s.r_t_pos += 1;
                if s.r_t_pos == 512 {
                    s.r_t_pos = 0;
                }
            }
            s.r_n_to_go -= 1;
            s.k0 = i32::from(k1) ^ if s.r_n_to_go == 1 { 1 } else { 0 };
            s.state_out_len = k1 as i32 + 4;
            s.nblock_used += 1;
        }
    } else {
        let mut calculated_block_crc = s.calculated_block_crc;
        let mut state_out_ch = s.state_out_ch;
        let mut state_out_len = s.state_out_len;
        let mut nblock_used = s.nblock_used;
        let mut k0 = s.k0;
        let tt = s.tt.as_mut().unwrap();
        let mut t_pos = s.t_pos;

        while s.state_out_len > 0 || nblock_used < s.save_nblock + 1 {
            if state_out_len > 0 {
                while state_out_len > 0 && s.strm.as_mut().unwrap().avail_out > 0 {
                    unsafe {
                        *s.strm.as_mut().unwrap().next_out.add(s.strm.as_mut().unwrap().total_out_lo32 as usize) = state_out_ch;
                    }
                    calculated_block_crc = (calculated_block_crc << 8) ^ BZ2_CRC32_TABLE[((calculated_block_crc >> 24) ^ u32::from(state_out_ch)) as usize];
                    state_out_len -= 1;
                    s.strm.as_mut().unwrap().avail_out -= 1;
                    s.strm.as_mut().unwrap().total_out_lo32 += 1;
                    if s.strm.as_mut().unwrap().total_out_lo32 == 0 {
                        s.strm.as_mut().unwrap().total_out_hi32 += 1;
                    }
                }
            }

            if nblock_used > s.save_nblock + 1 {
                return true;
            }

            if nblock_used == s.save_nblock + 1 {
                state_out_len = 0;
                break;
            }

            state_out_ch = k0 as u8;
            if t_pos >= 100_000 * s.block_size_100k as u32 {
                return true;
            }
            t_pos = tt[t_pos as usize];
            let mut k1 = (t_pos & 0xff) as u8;
            t_pos >>= 8;
            nblock_used += 1;

            if k1 != s.k0 as u8 {
                s.k0 = k1 as i32;
                continue;
            }

            state_out_len = 2;
            if t_pos >= 100_000 * s.block_size_100k as u32 {
                return true;
            }
            t_pos = tt[t_pos as usize];
            k1 = (t_pos & 0xff) as u8;
            t_pos >>= 8;
            nblock_used += 1;
            if k1 != s.k0 as u8 {
                s.k0 = k1 as i32;
                continue;
            }

            state_out_len = 3;
            if t_pos >= 100_000 * s.block_size_100k as u32 {
                return true;
            }
            t_pos = tt[t_pos as usize];
            k1 = (t_pos & 0xff) as u8;
            t_pos >>= 8;
            nblock_used += 1;
            if k1 != s.k0 as u8 {
                s.k0 = k1 as i32;
                continue;
            }

            if t_pos >= 100_000 * s.block_size_100k as u32 {
                return true;
            }
            t_pos = tt[t_pos as usize];
            k1 = (t_pos & 0xff) as u8;
            t_pos >>= 8;
            nblock_used += 1;
            state_out_len = k1 as i32 + 4;

            if t_pos >= 100_000 * s.block_size_100k as u32 {
                return true;
            }
            t_pos = tt[t_pos as usize];
            k0 = (t_pos & 0xff) as i32;
            t_pos >>= 8;
            nblock_used += 1;
        }

        s.calculated_block_crc = calculated_block_crc;
        s.state_out_ch = state_out_ch;
        s.state_out_len = state_out_len;
        s.nblock_used = nblock_used;
        s.k0 = k0;
        s.t_pos = t_pos;
    }

    false
}

pub fn bz2_index_into_f(indx: i32, cftab: &[i32]) -> i32 {
    let mut nb = 0;
    let mut na = 256;

    while na - nb != 1 {
        let mid = (nb + na) >> 1;
        if indx >= cftab[mid as usize] {
            nb = mid;
        } else {
            na = mid;
        }
    }

    nb
}

pub fn un_rle_obuf_to_output_small(s: &mut DState) -> bool {
    let mut k1: u8;

    if s.block_randomised {
        loop {
            // Processing bytes while the block is randomised
            while s.strm.as_ref().unwrap().avail_out > 0 && s.state_out_len > 0 {
                let strm = s.strm.as_mut().unwrap();
                unsafe {
                    *strm.next_out = s.state_out_ch;
                    strm.next_out = strm.next_out.add(1);
                }
                s.calculated_block_crc = (s.calculated_block_crc << 8)
                    ^ BZ2_CRC32_TABLE[((s.calculated_block_crc >> 24) ^ (s.state_out_ch as u32)) as usize];
                s.state_out_len -= 1;
                strm.avail_out -= 1;
                strm.total_out_lo32 += 1;
                if strm.total_out_lo32 == 0 {
                    strm.total_out_hi32 += 1;
                }
            }

            if s.nblock_used == s.save_nblock + 1 {
                return false;
            }

            if s.nblock_used > s.save_nblock + 1 {
                return true;
            }

            s.state_out_len = 1;
            s.state_out_ch = s.k0 as u8;

            if s.t_pos >= (100_000 * s.block_size_100k) as u32 {
                return true;
            }

            k1 = bz2_index_into_f(s.t_pos as i32, &s.cftab) as u8;
            s.t_pos = (s.ll16.as_ref().unwrap()[s.t_pos as usize] as u32)
                | (((s.ll4.as_ref().unwrap()[(s.t_pos >> 1) as usize] >> (((s.t_pos << 2) & 0x4) as usize)) & 0xF) as u32) << 16;

            if s.r_n_to_go == 0 {
                s.r_n_to_go = BZ2_rNums[s.r_t_pos as usize];
                s.r_t_pos += 1;
                if s.r_t_pos == 512 {
                    s.r_t_pos = 0;
                }
            }
            s.r_n_to_go -= 1;
            k1 ^= if s.r_n_to_go == 1 { 1 } else { 0 };
            s.nblock_used += 1;

            if s.nblock_used == s.save_nblock + 1 {
                continue;
            }
            if k1 != s.k0 as u8 {
                s.k0 = k1 as i32;
                continue;
            }

            s.state_out_len = 2;
            if s.t_pos >= (100_000 * s.block_size_100k) as u32 {
                return true;
            }
            k1 = bz2_index_into_f(s.t_pos as i32, &s.cftab) as u8;
            s.t_pos = (s.ll16.as_ref().unwrap()[s.t_pos as usize] as u32)
                | (((s.ll4.as_ref().unwrap()[(s.t_pos >> 1) as usize] >> (((s.t_pos << 2) & 0x4) as usize)) & 0xF) as u32) << 16;

            s.nblock_used += 1;

            if s.nblock_used == s.save_nblock + 1 {
                continue;
            }
            if k1 != s.k0 as u8 {
                s.k0 = k1 as i32;
                continue;
            }

            s.state_out_len = 3;
            if s.t_pos >= (100_000 * s.block_size_100k) as u32 {
                return true;
            }
            k1 = bz2_index_into_f(s.t_pos as i32, &s.cftab) as u8;
            s.t_pos = (s.ll16.as_ref().unwrap()[s.t_pos as usize] as u32)
                | (((s.ll4.as_ref().unwrap()[(s.t_pos >> 1) as usize] >> (((s.t_pos << 2) & 0x4) as usize)) & 0xF) as u32) << 16;

            s.nblock_used += 1;

            if s.nblock_used == s.save_nblock + 1 {
                continue;
            }
            if k1 != s.k0 as u8 {
                s.k0 = k1 as i32;
                continue;
            }

            if s.t_pos >= (100_000 * s.block_size_100k) as u32 {
                return true;
            }
            k1 = bz2_index_into_f(s.t_pos as i32, &s.cftab) as u8;
            s.t_pos = (s.ll16.as_ref().unwrap()[s.t_pos as usize] as u32)
                | (((s.ll4.as_ref().unwrap()[(s.t_pos >> 1) as usize] >> (((s.t_pos << 2) & 0x4) as usize)) & 0xF) as u32) << 16;

            s.nblock_used += 1;
            s.state_out_len = (k1 as i32) + 4;

            if s.t_pos >= (100_000 * s.block_size_100k) as u32 {
                return true;
            }
            s.k0 = bz2_index_into_f(s.t_pos as i32, &s.cftab);
            s.t_pos = (s.ll16.as_ref().unwrap()[s.t_pos as usize] as u32)
                | (((s.ll4.as_ref().unwrap()[(s.t_pos >> 1) as usize] >> (((s.t_pos << 2) & 0x4) as usize)) & 0xF) as u32) << 16;

            s.nblock_used += 1;
        }
    } else {
        loop {
            // Non-randomised block processing
            while s.strm.as_ref().unwrap().avail_out > 0 && s.state_out_len > 0 {
                let strm = s.strm.as_mut().unwrap();
                unsafe {
                    *strm.next_out = s.state_out_ch;
                    strm.next_out = strm.next_out.add(1);
                }
                s.calculated_block_crc = (s.calculated_block_crc << 8)
                    ^ BZ2_CRC32_TABLE[((s.calculated_block_crc >> 24) ^ (s.state_out_ch as u32)) as usize];
                s.state_out_len -= 1;
                strm.avail_out -= 1;
                strm.total_out_lo32 += 1;
                if strm.total_out_lo32 == 0 {
                    strm.total_out_hi32 += 1;
                }
            }

            if s.nblock_used == s.save_nblock + 1 {
                return false;
            }

            if s.nblock_used > s.save_nblock + 1 {
                return true;
            }

            s.state_out_len = 1;
            s.state_out_ch = s.k0 as u8;

            if s.t_pos >= (100_000 * s.block_size_100k) as u32 {
                return true;
            }

            k1 = bz2_index_into_f(s.t_pos as i32, &s.cftab) as u8;
            s.t_pos = (s.ll16.as_ref().unwrap()[s.t_pos as usize] as u32)
                | (((s.ll4.as_ref().unwrap()[(s.t_pos >> 1) as usize] >> (((s.t_pos << 2) & 0x4) as usize)) & 0xF) as u32) << 16;

            s.nblock_used += 1;

            if s.nblock_used == s.save_nblock + 1 {
                continue;
            }
            if k1 != s.k0 as u8 {
                s.k0 = k1 as i32;
                continue;
            }

            s.state_out_len = 2;
            if s.t_pos >= (100_000 * s.block_size_100k) as u32 {
                return true;
            }
            k1 = bz2_index_into_f(s.t_pos as i32, &s.cftab) as u8;
            s.t_pos = (s.ll16.as_ref().unwrap()[s.t_pos as usize] as u32)
                | (((s.ll4.as_ref().unwrap()[(s.t_pos >> 1) as usize] >> (((s.t_pos << 2) & 0x4) as usize)) & 0xF) as u32) << 16;

            s.nblock_used += 1;

            if s.nblock_used == s.save_nblock + 1 {
                continue;
            }
            if k1 != s.k0 as u8 {
                s.k0 = k1 as i32;
                continue;
            }

            s.state_out_len = 3;
            if s.t_pos >= (100_000 * s.block_size_100k) as u32 {
                return true;
            }
            k1 = bz2_index_into_f(s.t_pos as i32, &s.cftab) as u8;
            s.t_pos = (s.ll16.as_ref().unwrap()[s.t_pos as usize] as u32)
                | (((s.ll4.as_ref().unwrap()[(s.t_pos >> 1) as usize] >> (((s.t_pos << 2) & 0x4) as usize)) & 0xF) as u32) << 16;

            s.nblock_used += 1;

            if s.nblock_used == s.save_nblock + 1 {
                continue;
            }
            if k1 != s.k0 as u8 {
                s.k0 = k1 as i32;
                continue;
            }

            if s.t_pos >= (100_000 * s.block_size_100k) as u32 {
                return true;
            }
            k1 = bz2_index_into_f(s.t_pos as i32, &s.cftab) as u8;
            s.t_pos = (s.ll16.as_ref().unwrap()[s.t_pos as usize] as u32)
                | (((s.ll4.as_ref().unwrap()[(s.t_pos >> 1) as usize] >> (((s.t_pos << 2) & 0x4) as usize)) & 0xF) as u32) << 16;

            s.nblock_used += 1;
            s.state_out_len = (k1 as i32) + 4;

            if s.t_pos >= (100_000 * s.block_size_100k) as u32 {
                return true;
            }
            s.k0 = bz2_index_into_f(s.t_pos as i32, &s.cftab);
            s.t_pos = (s.ll16.as_ref().unwrap()[s.t_pos as usize] as u32)
                | (((s.ll4.as_ref().unwrap()[(s.t_pos >> 1) as usize] >> (((s.t_pos << 2) & 0x4) as usize)) & 0xF) as u32) << 16;

            s.nblock_used += 1;
        }
    }
}

pub fn bz2_bz_decompress(strm: &mut bz_stream) -> i32 {
    // Check if the stream or state is null
    let s = match s {
        Some(state) => state,
        None => return -2,
    };

    // Properly convert `strm` to a raw pointer for comparison
    let strm_ptr = strm as *mut bz_stream;

    if s.strm.as_ref().map_or(true, |ptr| ptr as *const _ != strm_ptr as *const _) {
        return -2
    }

    // Main loop
    loop {
        match s.state {
            // If state is 1, return error -1
            1 => {return -1;}
            
            // If state is 2, handle decompression
            2 => {
                // Handle decompression based on the small_decompress flag
                let corrupt = if s.small_decompress {
                    un_rle_obuf_to_output_small(s)
                } else {
                    un_rle_obuf_to_output_fast(s)
                };

                // If corrupt, return error -4
                if corrupt {
                    return -4
                }

                // If block is completely used and state_out_len is 0
                if s.nblock_used == s.save_nblock + 1 && s.state_out_len == 0 {
                    s.calculated_block_crc = !s.calculated_block_crc;
                    
                    // Print debug information based on verbosity
                    if s.verbosity >= 3 {
                        eprintln!(
                            " {{0x{:08x}, 0x{:08x}}}",
                            s.stored_block_crc, s.calculated_block_crc
                        );
                    }
                    
                    if s.verbosity >= 2 {
                        eprint!("]");
                    }

                    // Check for CRC mismatch
                    if s.calculated_block_crc != s.stored_block_crc {
                        return -4
                    }

                    // Update combined CRC
                    s.calculated_combined_crc = (s.calculated_combined_crc << 1)
                        | (s.calculated_combined_crc >> 31);
                    s.calculated_combined_crc ^= s.calculated_block_crc;

                    // Change state to 14
                    s.state = 14;
                } else {
                    return 0
                }
            }

            // If state is 10 or higher, decompress
            10..=i32::MAX => {
                let r = bz2_decompress(s);
                if r == 4 {
                    // Print debug information based on verbosity
                    if s.verbosity >= 3 {
                        eprintln!(
                            "\n    combined CRCs: stored = 0x{:08x}, computed = 0x{:08x}",
                            s.stored_combined_crc, s.calculated_combined_crc
                        );
                    }

                    // Check for combined CRC mismatch
                    if s.calculated_combined_crc != s.stored_combined_crc {
                        return -4
                    }

                    return r
                }

                // If the state is not 2, return the decompression result
                if s.state != 2 {
                    return r
                }
            }

            // Assert failure if none of the conditions above are met
            _ => {
                BZ2_bz__AssertH__fail(6001);
            }
        }
    }
}
