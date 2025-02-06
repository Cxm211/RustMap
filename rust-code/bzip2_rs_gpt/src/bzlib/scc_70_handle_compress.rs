use crate::global_vars::bzip2_c1::{bz_stream, EState};
use crate::compress::scc_69_BZ2_compressBlock::bz2_compress_block;
use crate::bzlib::scc_25_root_33_prepare_new_block_init_RL_isempty_RL_flush_RL_bzlib_static::*;
use crate::bzlib::scc_30_copy_input_until_stop_31_copy_output_until_stop::*;
use crate::global_vars::sleep_lib::*;
pub fn handle_compress(strm: &mut bz_stream) -> bool {
    let mut progress_in = false;
    let mut progress_out = false;
    let s = unsafe { &mut *strm.state }; // Unsafe because it's a raw pointer
   
    let mut counter = 0;
    loop {
        counter += 1;
        match s.state {
            BZ_S_OUTPUT => {
                // println!("74_s.state_in_ch-handle_compress: {}",s.state_in_ch);
                progress_out |= copy_output_until_stop(s);
                if s.state_out_pos < s.numZ {
                    break;
                }
                if s.mode == BZ_M_FINISHING && s.avail_in_expect == 0 && is_empty_rl(s) {
                    break;
                }
                prepare_new_block(s);
                s.state = BZ_S_INPUT;
                if s.mode == BZ_M_FLUSHING && s.avail_in_expect == 0 && is_empty_rl(s) {
                    break;
                }
            }
            BZ_S_INPUT => {
                progress_in |= copy_input_until_stop(s);
                if s.mode != BZ_M_RUNNING && s.avail_in_expect == 0 {
                    flush_rl(s);
                    // println!("File: {}, Line: {}, Column: {}", file!(), line!(), column!());
                    // println!("74_counter: {}", counter);
                    bz2_compress_block(s, s.mode == BZ_M_FINISHING);
                    s.state = BZ_S_OUTPUT;
                } else if s.nblock >= s.nblockMAX {
                    bz2_compress_block(s, false);
                    s.state = BZ_S_OUTPUT;
                } else if strm.avail_in == 0 {
                    break;
                }
            }
            _ => {}
        }
    }

    progress_in || progress_out
}

const BZ_M_IDLE: i32 = 1;
const BZ_M_RUNNING: i32 = 2;
const BZ_M_FLUSHING: i32 = 3;
const BZ_M_FINISHING: i32 = 4;

const BZ_S_OUTPUT: i32 = 1;
const BZ_S_INPUT: i32 = 2;
