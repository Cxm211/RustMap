
use crate::bzlib::flush_RL::add_char_to_block;
use crate::compress::BZ2_compressBlock::bz2_compress_block;
use crate::bzlib::utils::isempty_rl;
use crate::bzlib::utils::prepare_new_block;


pub fn copy_input_until_stop(s: &mut EState) -> bool {
    let mut progress_in = false;

    if s.mode == 2 {
        while s.nblock < s.nblock_max {
            if let Some(strm) = &mut s.strm {
                if strm.avail_in == 0 {
                    break;
                }
                progress_in = true;

                add_char_to_block(s, strm.next_in[0] as u32);

                strm.next_in = &strm.next_in[1..];
                strm.avail_in -= 1;
                strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);

                if strm.total_in_lo32 == 0 {
                    strm.total_in_hi32 += 1;
                }
            } else {
                break;
            }
        }
    } else {
        while s.nblock < s.nblock_max {
            if let Some(strm) = &mut s.strm {
                if strm.avail_in == 0 || s.avail_in_expect == 0 {
                    break;
                }
                progress_in = true;

                add_char_to_block(s, strm.next_in[0] as u32);

                strm.next_in = &strm.next_in[1..];
                strm.avail_in -= 1;
                strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);

                if strm.total_in_lo32 == 0 {
                    strm.total_in_hi32 += 1;
                }
                s.avail_in_expect -= 1;
            } else {
                break;
            }
        }
    }

    progress_in
}


pub fn copy_output_until_stop(s: &mut EState) -> bool {
    let mut progress_out = false;

    while s.strm.is_some() {
        let strm = s.strm.as_mut().unwrap();

        if strm.avail_out == 0 {
            break;
        }

        if s.state_out_pos >= s.num_z {
            break;
        }

        progress_out = true;

        if let Some(zbits) = &s.zbits {
            strm.next_out[0] = zbits[s.state_out_pos as usize];
        }

        s.state_out_pos += 1;
        strm.avail_out -= 1;
        strm.next_out = &mut strm.next_out[1..];
        strm.total_out_lo32 = strm.total_out_lo32.wrapping_add(1);

        if strm.total_out_lo32 == 0 {
            strm.total_out_hi32 += 1;
        }
    }

    progress_out
}

pub fn handle_compress(strm: &mut bz_stream) -> bool {
    let mut progress_in = false;
    let mut progress_out = false;

    if let Some(s) = strm.state.as_mut() {
        loop {
            if s.state == 1 {
                progress_out |= copy_output_until_stop(s);

                if s.state_out_pos < s.num_z {
                    break;
                }

                if s.mode == 4 && s.avail_in_expect == 0 && isempty_rl(s) {
                    break;
                }

                prepare_new_block(s);
                s.state = 2;

                if s.mode == 3 && s.avail_in_expect == 0 && isempty_rl(s) {
                    break;
                }
            }

            if s.state == 2 {
                progress_in |= copy_input_until_stop(s);

                if s.mode != 2 && s.avail_in_expect == 0 {
                    flush_rl(s);
                    bz2_compress_block(s, s.mode == 4);
                    s.state = 1;
                } else if s.nblock >= s.nblock_max {
                    bz2_compress_block(s, false);
                    s.state = 1;
                } else if strm.avail_in == 0 {
                    break;
                }
            }
        }
    }

    progress_in || progress_out
}

pub fn bz2_bz_compress(strm: &mut bz_stream, action: i32) -> i32 {
    if strm.is_none() {
        return -2;
    }
    
    let s = match strm.state.as_mut() {
        Some(state) => state,
        None => return -2,
    };

    if !std::ptr::eq(s.strm.unwrap() as *const _, strm as *const _) {
        return -2;
    }

    loop {
        match s.mode {
            1 => return -1,
            2 => {
                if action == 0 {
                    let progress = handle_compress(strm);
                    return if progress { 1 } else { -2 };
                } else if action == 1 {
                    s.avail_in_expect = strm.avail_in;
                    s.mode = 3;
                    continue;
                } else if action == 2 {
                    s.avail_in_expect = strm.avail_in;
                    s.mode = 4;
                    continue;
                } else {
                    return -2;
                }
            }
            3 => {
                if action != 1 {
                    return -1;
                }
                if s.avail_in_expect != strm.avail_in {
                    return -1;
                }
                let progress = handle_compress(strm);
                if s.avail_in_expect > 0 || !isempty_rl(s) || s.state_out_pos < s.num_z {
                    return 2;
                }
                s.mode = 2;
                return 1;
            }
            4 => {
                if action != 2 {
                    return -1;
                }
                if s.avail_in_expect != strm.avail_in {
                    return -1;
                }
                let progress = handle_compress(strm);
                if !progress {
                    return -1;
                }
                if s.avail_in_expect > 0 || !isempty_rl(s) || s.state_out_pos < s.num_z {
                    return 3;
                }
                s.mode = 1;
                return 4;
            }
            _ => return 0,
        }
    }
}
