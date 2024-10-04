

use crate::compress::bsW::bs_w;
use crate::compress::sendMTFValues::send_mtf_values;
use crate::blocksort::BZ2_blockSort::bz2_block_sort;
use crate::EState;
use std::slice;

pub fn bz2_bs_init_write(s: &mut EState) {
    s.bs_live = 0;
    s.bs_buff = 0;
}

pub fn bs_finish_write(s: &mut EState) {
    while s.bs_live > 0 {
        s.zbits.as_mut().unwrap()[s.num_z as usize] = (s.bs_buff >> 24) as u8; // `UChar` is `u8` in Rust
        s.num_z += 1;
        s.bs_buff <<= 8;
        s.bs_live -= 8;
    }
}

pub fn bs_put_uint32(s: &mut EState, u: u32) {
    bs_w(s, 8, ((u >> 24) & 0xff) as u32);
    bs_w(s, 8, ((u >> 16) & 0xff) as u32);
    bs_w(s, 8, ((u >> 8) & 0xff) as u32);
    bs_w(s, 8, (u & 0xff) as u32);
}

pub fn bs_put_uchar(s: &mut EState, c: u8) {
    bs_w(s, 8, c as u32);
}

pub fn make_maps_e(s: &mut EState) {
    s.n_in_use = 0;
    for i in 0..256 {
        if s.in_use[i as usize] {
            s.unseq_to_seq[i as usize] = s.n_in_use as u8;
            s.n_in_use += 1;
        }
    }
}

pub fn generate_mtf_values(s: &mut EState) {
    let mut yy = [0u8; 256]; // Equivalent to `UChar yy[256]`
    let mut z_pend = 0;
    let mut wr = 0;
    let eob = s.n_in_use + 1;

    let ptr = s.ptr.as_ref().unwrap();
    let block = s.block.as_ref().unwrap();
    let mtfv = s.mtfv.as_mut().unwrap();

    // Call makeMaps_e
    make_maps_e(s);

    // Initialize mtfFreq
    for i in 0..=eob {
        s.mtf_freq[i as usize] = 0;
    }

    // Initialize `yy` array
    for i in 0..s.n_in_use {
        yy[i as usize] = i as u8;
    }

    // Main loop
    for i in 0..s.nblock {
        let mut j = ptr[i as usize] as i32 - 1;
        if j < 0 {
            j += s.nblock;
        }

        let ll_i = s.unseq_to_seq[block[j as usize] as usize];

        if yy[0] == ll_i {
            z_pend += 1;
        } else {
            if z_pend > 0 {
                z_pend -= 1;
                while z_pend > 0 {
                    if z_pend & 1 != 0 {
                        mtfv[wr] = 1;
                        wr += 1;
                        s.mtf_freq[1] += 1;
                    } else {
                        mtfv[wr] = 0;
                        wr += 1;
                        s.mtf_freq[0] += 1;
                    }
                    if z_pend < 2 {
                        break;
                    }
                    z_pend = (z_pend - 2) / 2;
                }
                z_pend = 0;
            }

            // Move-to-front transform
            let mut rtmp = yy[1];
            yy[1] = yy[0];
            let mut ryy_j = 1;
            while ll_i != rtmp {
                let rtmp2 = rtmp;
                rtmp = yy[ryy_j + 1];
                yy[ryy_j] = rtmp2;
                ryy_j += 1;
            }
            yy[0] = rtmp;
            let j_mtf = ryy_j as i32;

            mtfv[wr] = (j_mtf + 1) as u16;
            wr += 1;
            s.mtf_freq[(j_mtf + 1) as usize] += 1;
        }
    }

    // Handle remaining `zPend`
    if z_pend > 0 {
        z_pend -= 1;
        while z_pend > 0 {
            if z_pend & 1 != 0 {
                mtfv[wr] = 1;
                wr += 1;
                s.mtf_freq[1] += 1;
            } else {
                mtfv[wr] = 0;
                wr += 1;
                s.mtf_freq[0] += 1;
            }
            if z_pend < 2 {
                break;
            }
            z_pend = (z_pend - 2) / 2;
        }
        z_pend = 0;
    }

    // End-of-block marker
    mtfv[wr] = eob as u16;
    wr += 1;
    s.mtf_freq[eob as usize] += 1;

    // Set the number of MTF symbols
    s.n_mtf = wr as i32;
}

pub fn bz2_compress_block(s: &mut EState, is_last_block: bool) {
    if s.nblock > 0 {
        // Invert the block CRC
        s.block_crc = !s.block_crc;

        // Update the combined CRC
        s.combined_crc = (s.combined_crc << 1) | (s.combined_crc >> 31);
        s.combined_crc ^= s.block_crc;

        if s.block_no > 1 {
            s.num_z = 0;
        }

        if s.verbosity >= 2 {
            eprintln!(
                "    block {}: crc = 0x{:08x}, combined CRC = 0x{:08x}, size = {}",
                s.block_no, s.block_crc, s.combined_crc, s.nblock
            );
        }

        // Sort the block
        bz2_block_sort(s);
    }

    // Set zbits pointer to the appropriate part of the array
    let arr2 = s.arr2.as_mut().unwrap(); // Assume `arr2` is always there
    let start_index = s.nblock as usize;

    // Calculate the length of the slice starting from `start_index`
    let len = arr2.len() - start_index;

    // Convert the `u32` slice to a `u8` slice
    let byte_slice = unsafe {
        slice::from_raw_parts_mut(arr2[start_index..].as_mut_ptr() as *mut u8, len * std::mem::size_of::<u32>())
    };

    // Set `s.zbits` to the newly created `u8` slice wrapped in `Some`
    s.zbits = Some(byte_slice);

    // Write the header if this is the first block
    if s.block_no == 1 {
        bz2_bs_init_write(s);
        bs_put_uchar(s, 0x42); // 'B'
        bs_put_uchar(s, 0x5a); // 'Z'
        bs_put_uchar(s, 0x68); // 'h'
        bs_put_uchar(s, 0x30 + s.block_size_100k as u8);
    }

    // Handle the non-empty block case
    if s.nblock > 0 {
        // Emit the block magic number '1AY&SY'
        bs_put_uchar(s, 0x31); // '1'
        bs_put_uchar(s, 0x41); // 'A'
        bs_put_uchar(s, 0x59); // 'Y'
        bs_put_uchar(s, 0x26); // '&'
        bs_put_uchar(s, 0x53); // 'S'
        bs_put_uchar(s, 0x59); // 'Y'

        // Write the block CRC
        bs_put_uint32(s, s.block_crc);

        // Mark it as a normal block
        bs_w(s, 1, 0);

        // Write the original pointer
        bs_w(s, 24, s.orig_ptr as u32);

        // Generate and send the MTF values
        generate_mtf_values(s);
        send_mtf_values(s);
    }

    // Handle the last block
    if is_last_block {
        // Emit the end-of-stream magic number '177245385090'
        bs_put_uchar(s, 0x17);
        bs_put_uchar(s, 0x72);
        bs_put_uchar(s, 0x45);
        bs_put_uchar(s, 0x38);
        bs_put_uchar(s, 0x50);
        bs_put_uchar(s, 0x90);

        // Write the final combined CRC
        bs_put_uint32(s, s.combined_crc);

        if s.verbosity >= 2 {
            eprintln!("    final combined CRC = 0x{:08x}", s.combined_crc);
        }

        // Finish writing the bitstream
        bs_finish_write(s);
    }
}
