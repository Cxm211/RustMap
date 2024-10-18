use crate::huffman::BZ2_hbCreateDecodeTables::bz2_hb_create_decode_tables;
use crate::bzlib::BZ2_bzDecompress::bz2_index_into_f;
use crate::BZ2_rNums;
use crate::bz_stream;
use crate::DState;
use crate::BZ2_bz__AssertH__fail;

pub fn make_maps_d(s: &mut DState) {
    s.n_in_use = 0;
    for i in 0..256 {
        if s.in_use[i] {
            s.seq_to_unseq[s.n_in_use as usize] = i as u8;  // `i` is cast to `u8` to match `seq_to_unseq` type
            s.n_in_use += 1;
        }
    }
}

pub fn bz2_decompress(s: &mut DState) -> i32 {
    // Replace libc::c_int and Int32 with i32 for consistency.
    let mut current_block: u64; // Equivalent to an unsigned 64-bit integer
    let mut uc: u8 = 0; // `UChar` is equivalent to `u8`
    let mut ret_val: i32 = 0; // `Int32` is equivalent to `i32`
    let mut min_len: i32 = 0;
    let mut max_len: i32 = 0;
    let mut strm: Option<&mut bz_stream> = s.strm; 
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut t: i32 = 0;
    let mut alpha_size: i32 = 0;
    let mut n_groups: i32 = 0;
    let mut n_selectors: i32 = 0;
    let mut eob: i32 = 0; // `EOB` in lowercase for Rust naming conventions
    let mut group_no: i32 = 0;
    let mut group_pos: i32 = 0;
    let mut next_sym: i32 = 0;
    let mut nblock_max: i32 = 0;
    let mut nblock: i32 = 0;
    let mut es: i32 = 0;
    let mut n: i32 = 0; // `N` in lowercase for Rust naming conventions
    let mut curr: i32 = 0;
    let mut zt: i32 = 0;
    let mut zn: i32 = 0;
    let mut zvec: i32 = 0;
    let mut zj: i32 = 0;
    let mut g_sel: i32 = 0;
    let mut g_minlen: i32 = 0;
    let mut g_limit: *mut i32 = std::ptr::null_mut(); // Null pointer equivalent in Rust
    let mut g_base: *mut i32 = std::ptr::null_mut();
    let mut g_perm: *mut i32 = std::ptr::null_mut();
        
    if s.state == 10 {
        // Resetting state fields to initial values
        s.save_i = 0;
        s.save_j = 0;
        s.save_t = 0;
        s.save_alpha_size = 0;
        s.save_n_groups = 0;
        s.save_n_selectors = 0;
        s.save_eob = 0;
        s.save_group_no = 0;
        s.save_group_pos = 0;
        s.save_next_sym = 0;
        s.save_nblock_max = 0;
        s.save_nblock = 0;
        s.save_es = 0;
        s.save_n = 0;
        s.save_curr = 0;
        s.save_zt = 0;
        s.save_zn = 0;
        s.save_zvec = 0;
        s.save_zj = 0;
        s.save_g_sel = 0;
        s.save_g_minlen = 0;
        s.save_g_limit = None; // Replacing raw pointers with Option
        s.save_g_base = None;  // Using Option for nullable pointers
        s.save_g_perm = None;  // To safely represent absence
    }

    // Assigning saved values to local variables
    let mut i = s.save_i;
    let mut j = s.save_j;
    let mut t = s.save_t;
    let mut alpha_size = s.save_alpha_size;
    let mut n_groups = s.save_n_groups;
    let mut n_selectors = s.save_n_selectors;
    let mut eob = s.save_eob;
    let mut group_no = s.save_group_no;
    let mut group_pos = s.save_group_pos;
    let mut next_sym = s.save_next_sym;
    let mut nblock_max = s.save_nblock_max;
    let mut nblock = s.save_nblock;
    let mut es = s.save_es;
    let mut n = s.save_n;
    let mut curr = s.save_curr;
    let mut zt = s.save_zt;
    let mut zn = s.save_zn;
    let mut zvec = s.save_zvec;
    let mut zj = s.save_zj;
    let mut g_sel = s.save_g_sel;
    let mut g_minlen = s.save_g_minlen;

    // Replace raw pointers to arrays with Option<&mut [i32]>
    let mut g_limit = s.save_g_limit.as_deref_mut().unwrap();
    let mut g_base = s.save_g_base.as_deref_mut().unwrap();
    let mut g_perm = s.save_g_perm.as_deref_mut().unwrap();

    ret_val = 0;

    match s.state {
        10 => {
            s.state = 10;
            loop {
                if !(true) {
                    current_block = 5235537862154438448;
                    break;
                }
                if s.bs_live >= 8 {
                    let v: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v as u8;
                    current_block = 5235537862154438448;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else {
                        // Fill the buffer with more bits
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    if uc as i32 != 0x42 {
                        ret_val = -5;
                        current_block = 8589876134443771061;
                    } else {
                        current_block = 647119486518834161;
                    }
                }
            }
        }
        11 => {
            current_block = 647119486518834161;
        }
        12 => {
            current_block = 15889152483458450077;
        }
        13 => {
            current_block = 8448021602604729824;
        }
        14 => {
            current_block = 1137006006685247392;
        }
        15 => {
            current_block = 16838365919992687769;
        }
        16 => {
            current_block = 3146547798068041444;
        }
        17 => {
            current_block = 7851151787501427912;
        }
        18 => {
            current_block = 5385056118645264483;
        }
        19 => {
            current_block = 2650936886724671381;
        }
        20 => {
            current_block = 5050664923561989785;
        }
        21 => {
            current_block = 12902030304110886147;
        }
        22 => {
            current_block = 1377993327617910642;
        }
        23 => {
            current_block = 16023805999610312643;
        }
        24 => {
            current_block = 625368880462740329;
        }
        25 => {
            current_block = 7513025209029308572;
        }
        26 => {
            current_block = 14851987212441076408;
        }
        27 => {
            current_block = 18214988272284434366;
        }
        28 => {
            current_block = 15014971834746686171;
        }
        29 => {
            current_block = 10115417614016365113;
        }
        30 => {
            current_block = 3137761655869617204;
        }
        31 => {
            current_block = 10469605675374413955;
        }
        32 => {
            current_block = 3927265264033028722;
        }
        33 => {
            current_block = 9633808932868333306;
        }
        34 => {
            current_block = 14062207274631763586;
        }
        35 => {
            current_block = 11754277565739695648;
        }
        36 => {
            current_block = 11392177284674087225;
        }
        37 => {
            current_block = 5497486814301402828;
        }
        38 => {
            current_block = 2173067795562692423;
        }
        39 => {
            current_block = 6204206077467629369;
        }
        40 => {
            current_block = 4758579651109969911;
        }
        41 => {
            current_block = 3245403061610108151;
        }
        42 => {
            current_block = 9781088206068579112;
        }
        43 => {
            current_block = 16726645251394399897;
        }
        44 => {
            current_block = 16731359125117492067;
        }
        45 => {
            current_block = 4364717973876642758;
        }
        46 => {
            current_block = 6723035467051578708;
        }
        47 => {
            current_block = 1857046018890652364;
        }
        48 => {
            current_block = 9372366546304504960;
        }
        49 => {
            current_block = 6899222889821136044;
        }
        50 => {
            current_block = 1146832167730825028;
        }
        _ => {
            if false {
                BZ2_bz__AssertH__fail(4001);
            }
            if false {
                BZ2_bz__AssertH__fail(4002);
            }
            current_block = 8589876134443771061;
        }
    }
    match current_block {
        647119486518834161 => {
            s.state = 11;
    
            loop {
                if !(true) {
                    current_block = 2168227384378665163;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_0: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_0 as u8;
                    current_block = 2168227384378665163;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    if uc as i32 != 0x5a {
                        ret_val = -5;
                        current_block = 8589876134443771061;
                    } else {
                        current_block = 15889152483458450077;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        15889152483458450077 => {
            s.state = 12;
    
            loop {
                if !(true) {
                    current_block = 178030534879405462;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_1: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_1 as u8;
                    current_block = 178030534879405462;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    if uc as i32 != 0x68 {
                        ret_val = -5;
                        current_block = 8589876134443771061;
                    } else {
                        current_block = 8448021602604729824;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        8448021602604729824 => {
            s.state = 13;
    
            loop {
                if !(true) {
                    current_block = 7639320476250304355;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_2: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    s.block_size_100k = v_2 as i32;
                    current_block = 7639320476250304355;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    if s.block_size_100k < 0x30 + 1 || s.block_size_100k > 0x30 + 9 {
                        ret_val = -5;
                        current_block = 8589876134443771061;
                    } else {
                        s.block_size_100k -= 0x30;
    
                        if s.small_decompress  {
                            if let Some(strm) = s.strm.as_ref() {
                                let ll16_vec = vec![0; (s.block_size_100k * 100000) as usize];
                                let ll16_boxed_slice = ll16_vec.into_boxed_slice();
                                s.ll16 = Some(Box::leak(ll16_boxed_slice).as_mut());
    
                                let ll4_vec = vec![0; (1 + (s.block_size_100k * 100000) / 2) as usize];
                                let ll4_boxed_slice = ll4_vec.into_boxed_slice();
                                s.ll4 = Some(Box::leak(ll4_boxed_slice).as_mut());
    
                                if s.ll16.is_none() || s.ll4.is_none() {
                                    ret_val = -3;
                                    current_block = 8589876134443771061;
                                } else {
                                    current_block = 1137006006685247392;
                                }
                            }
                        } else {
                            if let Some(strm) = s.strm.as_ref() {
                                let tt_vec = vec![0; (s.block_size_100k * 100000) as usize];
                                let tt_boxed_slice = tt_vec.into_boxed_slice();
                                s.tt = Some(Box::leak(tt_boxed_slice).as_mut());
    
                                if s.tt.is_none() {
                                    ret_val = -3;
                                    current_block = 8589876134443771061;
                                } else {
                                    current_block = 1137006006685247392;
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        1137006006685247392 => {
            s.state = 14;
    
            loop {
                if !(true) {
                    current_block = 16937825661756021828;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_3: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_3 as u8;
                    current_block = 16937825661756021828;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    if uc == 0x17 {
                        current_block = 9781088206068579112;
                    } else if uc != 0x31 {
                        ret_val = -4;
                        current_block = 8589876134443771061;
                    } else {
                        current_block = 16838365919992687769;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        9781088206068579112 => {
            s.state = 42;
    
            loop {
                if !(true) {
                    current_block = 13733404100380861831;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_32: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_32 as u8;
                    current_block = 13733404100380861831;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    if uc != 0x72 {
                        ret_val = -4;
                        current_block = 8589876134443771061;
                    } else {
                        current_block = 16726645251394399897;
                    }
                }
            }
        }
        16838365919992687769 => {
            s.state = 15;
    
            loop {
                if !(true) {
                    current_block = 1228639923084383292;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_4: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_4 as u8;
                    current_block = 1228639923084383292;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    if uc != 0x41 {
                        ret_val = -4;
                        current_block = 8589876134443771061;
                    } else {
                        current_block = 3146547798068041444;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        16726645251394399897 => {
            s.state = 43;
    
            loop {
                if !(true) {
                    current_block = 12721425419429475574;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_33: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_33 as u8;
                    current_block = 12721425419429475574;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    if uc != 0x45 {
                        ret_val = -4;
                        current_block = 8589876134443771061;
                    } else {
                        current_block = 16731359125117492067;
                    }
                }
            }
        }
        3146547798068041444 => {
            s.state = 16;
    
            loop {
                if !(true) {
                    current_block = 9235179519944561532;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_5: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_5 as u8;
                    current_block = 9235179519944561532;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    if uc != 0x59 {
                        ret_val = -4;
                        current_block = 8589876134443771061;
                    } else {
                        current_block = 7851151787501427912;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        16731359125117492067 => {
            s.state = 44;
    
            loop {
                if !(true) {
                    current_block = 13813414375753095368;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_34: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_34 as u8;
                    current_block = 13813414375753095368;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    if uc != 0x38 {
                        ret_val = -4;
                        current_block = 8589876134443771061;
                    } else {
                        current_block = 4364717973876642758;
                    }
                }
            }
        }
        7851151787501427912 => {
            s.state = 17;
    
            loop {
                if !(true) {
                    current_block = 12467039471581323981;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_6: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_6 as u8;
                    current_block = 12467039471581323981;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    if uc != 0x26 {
                        ret_val = -4;
                        current_block = 8589876134443771061;
                    } else {
                        current_block = 5385056118645264483;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        4364717973876642758 => {
            s.state = 45;
    
            loop {
                if !(true) {
                    current_block = 1472103348880861285;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_35: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_35 as u8;
                    current_block = 1472103348880861285;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    if uc != 0x50 {
                        ret_val = -4;
                        current_block = 8589876134443771061;
                    } else {
                        current_block = 6723035467051578708;
                    }
                }
            }
        }
        5385056118645264483 => {
            s.state = 18;
    
            loop {
                if !(true) {
                    current_block = 13164310931121142693;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_7: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_7 as u8;
                    current_block = 13164310931121142693;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    if uc != 0x53 {
                        ret_val = -4;
                        current_block = 8589876134443771061;
                    } else {
                        current_block = 2650936886724671381;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        6723035467051578708 => {
            s.state = 46;
    
            loop {
                if !(true) {
                    current_block = 8232347840743503282;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_36: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_36 as u8;
                    current_block = 8232347840743503282;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    if uc != 0x90 {
                        ret_val = -4;
                        current_block = 8589876134443771061;
                    } else {
                        s.stored_combined_crc = 0;
                        current_block = 1857046018890652364;
                    }
                }
            }
        }
        2650936886724671381 => {
            s.state = 19;
    
            loop {
                if !(true) {
                    current_block = 14723615986260991866;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_8: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_8 as u8;
                    current_block = 14723615986260991866;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    if uc != 0x59 {
                        ret_val = -4;
                        current_block = 8589876134443771061;
                    } else {
                        s.curr_block_no += 1;
    
                        if s.verbosity >= 2 {
                            // In Rust, you can use eprintln! for error output
                            eprintln!("\n    [{}: huff+mtf", s.curr_block_no);
                        }
    
                        s.stored_block_crc = 0;
                        current_block = 5050664923561989785;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        1857046018890652364 => {
            s.state = 47;
    
            loop {
                if !(true) {
                    current_block = 5465979950226085365;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_37: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_37 as u8;
                    current_block = 5465979950226085365;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    s.stored_combined_crc = (s.stored_combined_crc << 8) | uc as u32;
                    current_block = 9372366546304504960;
                }
            }
        }
        5050664923561989785 => {
            s.state = 20;
    
            loop {
                if !(true) {
                    current_block = 15627786036016112248;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_9: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_9 as u8;
                    current_block = 15627786036016112248;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    s.stored_block_crc = (s.stored_block_crc << 8) | uc as u32;
                    current_block = 12902030304110886147;
                }
            }
        }
        _ => {}
    }
    match current_block {
        9372366546304504960 => {
            s.state = 48;
    
            loop {
                if !(true) {
                    current_block = 3854366583354019639;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_38: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_38 as u8;
                    current_block = 3854366583354019639;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    s.stored_combined_crc = (s.stored_combined_crc << 8) | uc as u32;
                    current_block = 6899222889821136044;
                }
            }
        }
        12902030304110886147 => {
            s.state = 21;
    
            loop {
                if !(true) {
                    current_block = 13493279574219925475;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_10: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_10 as u8;
                    current_block = 13493279574219925475;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    s.stored_block_crc = (s.stored_block_crc << 8) | uc as u32;
                    current_block = 1377993327617910642;
                }
            }
        }
        _ => {}
    }
    match current_block {
        6899222889821136044 => {
            s.state = 49;
    
            loop {
                if !(true) {
                    current_block = 12082794684616777938;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_39: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_39 as u8;
                    current_block = 12082794684616777938;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    s.stored_combined_crc = (s.stored_combined_crc << 8) | uc as u32;
                    current_block = 1146832167730825028;
                }
            }
        }
        1377993327617910642 => {
            s.state = 22;
    
            loop {
                if !(true) {
                    current_block = 4839309778395429725;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_11: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_11 as u8;
                    current_block = 4839309778395429725;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    s.stored_block_crc = (s.stored_block_crc << 8) | uc as u32;
                    current_block = 16023805999610312643;
                }
            }
        }
        _ => {}
    }
    match current_block {
        16023805999610312643 => {
            s.state = 23;
    
            loop {
                if !(true) {
                    current_block = 17937968408868551711;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_12: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_12 as u8;
                    current_block = 17937968408868551711;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    s.stored_block_crc = (s.stored_block_crc << 8) | uc as u32;
                    current_block = 625368880462740329;
                }
            }
        }
        1146832167730825028 => {
            s.state = 50;
    
            loop {
                if !(true) {
                    current_block = 6276941480907995842;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_40: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_40 as u8;
                    current_block = 6276941480907995842;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    s.stored_combined_crc = (s.stored_combined_crc << 8) | uc as u32;
                    s.state = 1;
                    ret_val = 4;
                    current_block = 8589876134443771061;
                }
            }
        }
        _ => {}
    }
    match current_block {
        625368880462740329 => {
            s.state = 24;
    
            loop {
                if !(true) {
                    current_block = 7926734633677835471;
                    break;
                }
    
                if s.bs_live >= 1 {
                    let v_13: u32 = (s.bs_buff >> (s.bs_live - 1)) & 0x1;
                    s.bs_live -= 1;
                    s.block_randomised = v_13 != 0;
                    current_block = 7926734633677835471;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    s.orig_ptr = 0;
                    current_block = 7513025209029308572;
                }
            }
        }
        _ => {}
    }
    match current_block {
        7513025209029308572 => {
            s.state = 25;
    
            loop {
                if !(true) {
                    current_block = 5948065351908552372;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_14: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_14 as u8;
                    current_block = 5948065351908552372;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    s.orig_ptr = (s.orig_ptr << 8) | uc as i32;
                    current_block = 14851987212441076408;
                }
            }
        }
        _ => {}
    }
    
    match current_block {
        14851987212441076408 => {
            s.state = 26;
    
            loop {
                if !(true) {
                    current_block = 8940662058537996670;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_15: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_15 as u8;
                    current_block = 8940662058537996670;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    s.orig_ptr = (s.orig_ptr << 8) | uc as i32;
                    current_block = 18214988272284434366;
                }
            }
        }
        _ => {}
    }
    
    match current_block {
        18214988272284434366 => {
            s.state = 27;
    
            loop {
                if !(true) {
                    current_block = 13366002463409402866;
                    break;
                }
    
                if s.bs_live >= 8 {
                    let v_16: u32 = (s.bs_buff >> (s.bs_live - 8)) & 0xFF;
                    s.bs_live -= 8;
                    uc = v_16 as u8;
                    current_block = 13366002463409402866;
                    break;
                } else if let Some(strm) =  s.strm.as_ref() {
                    if strm.avail_in == 0 {
                        ret_val = 0;
                        current_block = 8589876134443771061;
                        break;
                    } else if !strm.next_in.is_null() {
                        unsafe {
                            // Fill the buffer with more bits
                            s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                            s.bs_live += 8;
            
                            // Update stream input pointer
                            strm.next_in = strm.next_in.add(1);
                            strm.avail_in = strm.avail_in.wrapping_sub(1);
                            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                            if strm.total_in_lo32 == 0 {
                                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                            }
                        }
                    }
                }
            }
    
            match current_block {
                8589876134443771061 => {}
                _ => {
                    s.orig_ptr = (s.orig_ptr << 8) | uc as i32;
    
                    if s.orig_ptr < 0 {
                        ret_val = -4;
                        current_block = 8589876134443771061;
                    } else if s.orig_ptr > 10 + 100000 * s.block_size_100k {
                        ret_val = -4;
                        current_block = 8589876134443771061;
                    } else {
                        i = 0;
                        current_block = 454873545234741267;
                    }
                }
            }
        }
        _ => {}
    }
    'c_10057: loop {
        match current_block {
            8589876134443771061 => {
                s.save_i = i;
                break;
            }
            4758579651109969911 => {
                s.state = 40;
        
                loop {
                    if s.bs_live >= zn {
                        let v_30: u32 = (s.bs_buff >> (s.bs_live - zn)) & ((1 << zn) - 1) as u32;
                        s.bs_live -= zn;
                        zvec = v_30 as i32;
                        break;
                    } else if let Some(strm) =  s.strm.as_ref() {
                        if strm.avail_in == 0 {
                            ret_val = 0;
                            current_block = 8589876134443771061;
                            continue;
                        } else if !strm.next_in.is_null() {
                            unsafe {
                                // Fill the buffer with more bits
                                s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                                s.bs_live += 8;
                
                                // Update stream input pointer
                                strm.next_in = strm.next_in.add(1);
                                strm.avail_in = strm.avail_in.wrapping_sub(1);
                                strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                                if strm.total_in_lo32 == 0 {
                                    strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                                }
                            }
                        }
                    }
                }
        
                current_block = 16348713635569416413;
            }
            6204206077467629369 => {
                s.state = 39;
        
                loop {
                    if s.bs_live >= 1 {
                        let v_29: u32 = (s.bs_buff >> (s.bs_live - 1)) & 0x1;
                        s.bs_live -= 1;
                        zj = v_29 as i32;
                        break;
                    } else if let Some(strm) =  s.strm.as_ref() {
                        if strm.avail_in == 0 {
                            ret_val = 0;
                            current_block = 8589876134443771061;
                            continue;
                        } else if !strm.next_in.is_null() {
                            // Fill the buffer with more bits
                            unsafe {
                // Fill the buffer with more bits
                s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                s.bs_live += 8;

                // Update stream input pointer
                strm.next_in = strm.next_in.add(1);
                strm.avail_in = strm.avail_in.wrapping_sub(1);
                strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                if strm.total_in_lo32 == 0 {
                    strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                }
            }
                        }
                    }
                }
        
                zvec = (zvec << 1) | zj;
                current_block = 7923635230025172457;
            }
            2173067795562692423 => {
                s.state = 38;
        
                loop {
                    if s.bs_live >= zn {
                        let v_28: u32 = (s.bs_buff >> (s.bs_live - zn)) & ((1 << zn) - 1) as u32;
                        s.bs_live -= zn;
                        zvec = v_28 as i32;
                        break;
                    } else if let Some(strm) =  s.strm.as_ref() {
                        if strm.avail_in == 0 {
                            ret_val = 0;
                            current_block = 8589876134443771061;
                            continue;
                        } else if !strm.next_in.is_null() {
                            // Fill the buffer with more bits
                            unsafe {
                // Fill the buffer with more bits
                s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                s.bs_live += 8;

                // Update stream input pointer
                strm.next_in = strm.next_in.add(1);
                strm.avail_in = strm.avail_in.wrapping_sub(1);
                strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                if strm.total_in_lo32 == 0 {
                    strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                }
            }
                        }
                    }
                }
        
                current_block = 7923635230025172457;
            }
            5497486814301402828 => {
                s.state = 37;
        
                loop {
                    if s.bs_live >= 1 {
                        let v_27: u32 = (s.bs_buff >> (s.bs_live - 1)) & 0x1;
                        s.bs_live -= 1;
                        zj = v_27 as i32;
                        break;
                    } else if let Some(strm) =  s.strm.as_ref() {
                        if strm.avail_in == 0 {
                            ret_val = 0;
                            current_block = 8589876134443771061;
                            continue;
                        } else if !strm.next_in.is_null() {
                            // Fill the buffer with more bits
                            unsafe {
                // Fill the buffer with more bits
                s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                s.bs_live += 8;

                // Update stream input pointer
                strm.next_in = strm.next_in.add(1);
                strm.avail_in = strm.avail_in.wrapping_sub(1);
                strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                if strm.total_in_lo32 == 0 {
                    strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                }
            }
                        }
                    }
                }
        
                zvec = (zvec << 1) | zj;
                current_block = 9186389159759284570;
            }
            11392177284674087225 => {
                s.state = 36;
        
                loop {
                    if s.bs_live >= zn {
                        let v_26: u32 = (s.bs_buff >> (s.bs_live - zn)) & ((1 << zn) - 1) as u32;
                        s.bs_live -= zn;
                        zvec = v_26 as i32;
                        break;
                    } else if let Some(strm) =  s.strm.as_ref() {
                        if strm.avail_in == 0 {
                            ret_val = 0;
                            current_block = 8589876134443771061;
                            continue;
                        } else if !strm.next_in.is_null() {
                            // Fill the buffer with more bits
                            unsafe {
                // Fill the buffer with more bits
                s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                s.bs_live += 8;

                // Update stream input pointer
                strm.next_in = strm.next_in.add(1);
                strm.avail_in = strm.avail_in.wrapping_sub(1);
                strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                if strm.total_in_lo32 == 0 {
                    strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                }
            }
                        }
                    }
                }
        
                current_block = 9186389159759284570;
            }
            11754277565739695648 => {
                s.state = 35;
        
                loop {
                    if s.bs_live >= 1 {
                        let v_25: u32 = (s.bs_buff >> (s.bs_live - 1)) & 0x1;
                        s.bs_live -= 1;
                        uc = v_25 as u8;
                        break;
                    } else if let Some(strm) =  s.strm.as_ref() {
                        if strm.avail_in == 0 {
                            ret_val = 0;
                            current_block = 8589876134443771061;
                            continue;
                        } else if !strm.next_in.is_null() {
                            // Fill the buffer with more bits
                            unsafe {
                // Fill the buffer with more bits
                s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                s.bs_live += 8;

                // Update stream input pointer
                strm.next_in = strm.next_in.add(1);
                strm.avail_in = strm.avail_in.wrapping_sub(1);
                strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                if strm.total_in_lo32 == 0 {
                    strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                }
            }
                        }
                    }
                }
        
                if uc == 0 {
                    curr += 1;
                } else {
                    curr -= 1;
                }
                current_block = 5533056661327372531;
            }
            14062207274631763586 => {
                s.state = 34;
        
                loop {
                    if s.bs_live >= 1 {
                        let v_24: u32 = (s.bs_buff >> (s.bs_live - 1)) & 0x1;
                        s.bs_live -= 1;
                        uc = v_24 as u8;
                        break;
                    } else if let Some(strm) =  s.strm.as_ref() {
                        if strm.avail_in == 0 {
                            ret_val = 0;
                            current_block = 8589876134443771061;
                            continue;
                        } else if !strm.next_in.is_null() {
                            // Fill the buffer with more bits
                            unsafe {
                // Fill the buffer with more bits
                s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                s.bs_live += 8;

                // Update stream input pointer
                strm.next_in = strm.next_in.add(1);
                strm.avail_in = strm.avail_in.wrapping_sub(1);
                strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                if strm.total_in_lo32 == 0 {
                    strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                }
            }
                        }
                    }
                }
        
                if uc != 0 {
                    current_block = 11754277565739695648;
                    continue;
                }
                current_block = 7746242308555130918;
            }
            9633808932868333306 => {
                s.state = 33;
        
                loop {
                    if s.bs_live >= 5 {
                        let v_23: u32 = (s.bs_buff >> (s.bs_live - 5)) & 0x1F;
                        s.bs_live -= 5;
                        curr = v_23 as i32;
                        break;
                    } else if let Some(strm) = s.strm.as_ref() {
                        if strm.avail_in == 0 {
                            ret_val = 0;
                            current_block = 8589876134443771061;
                            continue;
                        } else if !strm.next_in.is_null() {
                            // Fill the buffer with more bits
                            unsafe {
                // Fill the buffer with more bits
                s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                s.bs_live += 8;

                // Update stream input pointer
                strm.next_in = strm.next_in.add(1);
                strm.avail_in = strm.avail_in.wrapping_sub(1);
                strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                if strm.total_in_lo32 == 0 {
                    strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                }
            }
                        }
                    }
                }
        
                i = 0;
                current_block = 16642413284942005565;
            }
            3927265264033028722 => {
                s.state = 32;
        
                loop {
                    if s.bs_live >= 1 {
                        let v_21: u32 = (s.bs_buff >> (s.bs_live - 1)) & 0x1;
                        s.bs_live -= 1;
                        uc = v_21 as u8;
                        break;
                    } else if let Some(strm) = s.strm.as_ref() {
                        if strm.avail_in == 0 {
                            ret_val = 0;
                            current_block = 8589876134443771061;
                            continue;
                        } else if !strm.next_in.is_null() {
                            // Fill the buffer with more bits
                            unsafe {
                // Fill the buffer with more bits
                s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                s.bs_live += 8;

                // Update stream input pointer
                strm.next_in = strm.next_in.add(1);
                strm.avail_in = strm.avail_in.wrapping_sub(1);
                strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                if strm.total_in_lo32 == 0 {
                    strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                }
            }
                        }
                    }
                }
        
                if uc == 0 {
                    current_block = 10081471997089450706;
                } else {
                    j += 1;
                    if j >= n_groups {
                        ret_val = -4;
                        current_block = 8589876134443771061;
                    } else {
                        current_block = 16531797892856733396;
                    }
                }
            }
            10469605675374413955 => {
                s.state = 31;
        
                loop {
                    if s.bs_live >= 15 {
                        let v_20: u32 = (s.bs_buff >> (s.bs_live - 15)) & 0x7FFF;
                        s.bs_live -= 15;
                        n_selectors = v_20 as i32;
                        break;
                    } else if let Some(strm) = s.strm.as_ref() {
                        if strm.avail_in == 0 {
                            ret_val = 0;
                            current_block = 8589876134443771061;
                            continue;
                        } else if !strm.next_in.is_null() {
                            // Fill the buffer with more bits
                            unsafe {
                // Fill the buffer with more bits
                s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                s.bs_live += 8;

                // Update stream input pointer
                strm.next_in = strm.next_in.add(1);
                strm.avail_in = strm.avail_in.wrapping_sub(1);
                strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                if strm.total_in_lo32 == 0 {
                    strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                }
            }
                        }
                    }
                }
        
                if n_selectors < 1 {
                    ret_val = -4;
                    current_block = 8589876134443771061;
                } else {
                    i = 0;
                    current_block = 3503188808869013853;
                }
            }
            3137761655869617204 => {
                s.state = 30;
        
                loop {
                    if s.bs_live >= 3 {
                        let v_19: u32 = (s.bs_buff >> (s.bs_live - 3)) & 0x07;
                        s.bs_live -= 3;
                        n_groups = v_19 as i32;
                        break;
                    } else if let Some(strm) = s.strm.as_ref() {
                        if strm.avail_in == 0 {
                            ret_val = 0;
                            current_block = 8589876134443771061;
                            continue;
                        } else if !strm.next_in.is_null() {
                            // Fill the buffer with more bits
                            unsafe {
                // Fill the buffer with more bits
                s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                s.bs_live += 8;

                // Update stream input pointer
                strm.next_in = strm.next_in.add(1);
                strm.avail_in = strm.avail_in.wrapping_sub(1);
                strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                if strm.total_in_lo32 == 0 {
                    strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                }
            }
                        }
                    }
                }
        
                if n_groups < 2 || n_groups > 6 {
                    ret_val = -4;
                    current_block = 8589876134443771061;
                } else {
                    current_block = 10469605675374413955;
                }
            }
            10115417614016365113 => {
                s.state = 29;
        
                loop {
                    if s.bs_live >= 1 {
                        let v_18: u32 = (s.bs_buff >> (s.bs_live - 1)) & 0x01;
                        s.bs_live -= 1;
                        uc = v_18 as u8;
                        break;
                    } else if let Some(strm) = s.strm.as_ref() {
                        if strm.avail_in == 0 {
                            ret_val = 0;
                            current_block = 8589876134443771061;
                            continue;
                        } else if !strm.next_in.is_null() {
                            // Fill the buffer with more bits
                            unsafe {
                // Fill the buffer with more bits
                s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                s.bs_live += 8;

                // Update stream input pointer
                strm.next_in = strm.next_in.add(1);
                strm.avail_in = strm.avail_in.wrapping_sub(1);
                strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                if strm.total_in_lo32 == 0 {
                    strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                }
            }
                        }
                    }
                }
        
                if uc == 1 {
                    s.in_use[(i * 16 + j) as usize] = true;
                }
                j += 1;
                current_block = 16953886395775657100;
            }
            454873545234741267 => {
                if i < 16 {
                    current_block = 15014971834746686171;
                    continue;
                }
                i = 0;
                while i < 256 {
                    s.in_use[i as usize] = false;
                    i += 1;
                }
                i = 0;
                current_block = 15415362524153386998;
            }
            15014971834746686171 => {
                s.state = 28;
        
                loop {
                    if s.bs_live >= 1 {
                        let v_17: u32 = (s.bs_buff >> (s.bs_live - 1)) & 0x01;
                        s.bs_live -= 1;
                        uc = v_17 as u8;
                        break;
                    } else if let Some(strm) = s.strm.as_ref() {
                        if strm.avail_in == 0 {
                            ret_val = 0;
                            current_block = 8589876134443771061;
                            continue;
                        } else if !strm.next_in.is_null() {
                            // Fill the buffer with more bits
                            unsafe {
                // Fill the buffer with more bits
                s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                s.bs_live += 8;

                // Update stream input pointer
                strm.next_in = strm.next_in.add(1);
                strm.avail_in = strm.avail_in.wrapping_sub(1);
                strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                if strm.total_in_lo32 == 0 {
                    strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                }
            }
                        }
                    }
                }
        
                if uc == 1 {
                    s.in_use16[i as usize] = true;
                } else {
                    s.in_use16[i as usize] = false;
                }
                i += 1;
                current_block = 454873545234741267;
            }
            _ => {
                s.state = 41;
        
                loop {
                    if s.bs_live >= 1 {
                        let v_31: u32 = (s.bs_buff >> (s.bs_live - 1)) & 0x01;
                        s.bs_live -= 1;
                        zj = v_31 as i32;
                        break;
                    } else if let Some(strm) = s.strm.as_ref() {
                        if strm.avail_in == 0 {
                            ret_val = 0;
                            current_block = 8589876134443771061;
                            break;
                        } else if !strm.next_in.is_null() {
                            // Fill the buffer with more bits
                            unsafe {
                // Fill the buffer with more bits
                s.bs_buff = (s.bs_buff << 8) | *strm.next_in as u32;
                s.bs_live += 8;

                // Update stream input pointer
                strm.next_in = strm.next_in.add(1);
                strm.avail_in = strm.avail_in.wrapping_sub(1);
                strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                if strm.total_in_lo32 == 0 {
                    strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
                }
            }
                        }
                    }
                }
        
                zvec = (zvec << 1) | zj;
                current_block = 16348713635569416413;
            }
        }
        match current_block {
            16348713635569416413 => {
                if zn > 20 {
                    ret_val = -4;
                    current_block = 8589876134443771061;
                    continue;
                } else if zvec <= g_limit[zn as usize] {
                    if (zvec - g_base[zn as usize]) < 0 || (zvec - g_base[zn as usize]) >= 258 {
                        ret_val = -4;
                        current_block = 8589876134443771061;
                        continue;
                    } else {
                        next_sym = g_perm[(zvec - g_base[zn as usize]) as usize];
                    }
                } else {
                    zn += 1;
                    zn;
                    current_block = 3245403061610108151;
                    continue;
                }
                current_block = 3575340618357869479;
            }
            7923635230025172457 => {
                if zn > 20 {
                    ret_val = -4;
                    current_block = 8589876134443771061;
                    continue;
                } else if zvec <= g_limit[zn as usize] {
                    if (zvec - g_base[zn as usize]) < 0 || (zvec - g_base[zn as usize]) >= 258 {
                        ret_val = -4;
                        current_block = 8589876134443771061;
                        continue;
                    } else {
                        next_sym = g_perm[(zvec - g_base[zn as usize]) as usize];
                        if next_sym == 0 || next_sym == 1 {
                            current_block = 5649595406143318745;
                        } else {
                            es += 1;
                            let uc = s.seq_to_unseq[s.mtfa[s.mtfbase[0] as usize] as usize];
                            s.unzftab[uc as usize] += es;
        
                            if s.small_decompress {
                                while es > 0 {
                                    if nblock >= nblock_max {
                                        ret_val = -4;
                                        current_block = 8589876134443771061;
                                        continue 'c_10057;
                                    } else {
                                        s.ll16.as_mut().unwrap()[nblock as usize] = uc as u16;
                                        nblock += 1;
                                        es -= 1;
                                    }
                                }
                            } else {
                                while es > 0 {
                                    if nblock >= nblock_max {
                                        ret_val = -4;
                                        current_block = 8589876134443771061;
                                        continue 'c_10057;
                                    } else {
                                        s.tt.as_mut().unwrap()[nblock as usize] = uc as u32;
                                        nblock += 1;
                                        es -= 1;
                                    }
                                }
                            }
                            current_block = 3575340618357869479;
                        }
                    }
                } else {
                    zn += 1;
                    zn;
                    current_block = 6204206077467629369;
                    continue;
                }
            }
            9186389159759284570 => {
                if zn > 20 {
                    ret_val = -4;
                    current_block = 8589876134443771061;
                    continue;
                } else if zvec <= g_limit[zn as usize] {
                    if (zvec - g_base[zn as usize]) < 0 || (zvec - g_base[zn as usize]) >= 258 {
                        ret_val = -4;
                        current_block = 8589876134443771061;
                        continue;
                    } else {
                        next_sym = g_perm[(zvec - g_base[zn as usize]) as usize];
                    }
                } else {
                    zn += 1;
                    zn;
                    current_block = 5497486814301402828;
                    continue;
                }
                current_block = 3575340618357869479;
            }
            _ => {}
        }
        match current_block {
            3575340618357869479 => {
                if true {
                    if next_sym == eob {
                        current_block = 4069074773319880902;
                    } else {
                        if next_sym == 0 || next_sym == 1 {
                            es = -1;
                            n = 1;
                        } else if nblock >= nblock_max {
                            ret_val = -4;
                            current_block = 8589876134443771061;
                            continue;
                        } else {
                            let mut nn: u32 = (next_sym - 1) as u32;
                            if nn < 16 {
                                let pp = s.mtfbase[0] as u32;
                                uc = s.mtfa[(pp + nn) as usize]; 
                                while nn > 3 {
                                    let z = (pp + nn) as usize;
                                    s.mtfa[z] = s.mtfa[z - 1];
                                    s.mtfa[z - 1] = s.mtfa[z - 2];
                                    s.mtfa[z - 2] = s.mtfa[z - 3];
                                    s.mtfa[z - 3] = s.mtfa[z - 4];
                                    nn -= 4;
                                }
                                while nn > 0 {
                                    s.mtfa[(pp + nn) as usize] = s.mtfa[(pp + nn - 1) as usize];
                                    nn -= 1;
                                }
                                s.mtfa[pp as usize] = uc;
                            } else {
                                let lno = (nn / 16) as i32;
                                let off = (nn % 16) as i32;
                                let mut pp = s.mtfbase[lno as usize] + off;
                                uc = s.mtfa[pp as usize];
                                while pp > s.mtfbase[lno as usize] {
                                    s.mtfa[pp as usize] = s.mtfa[(pp - 1) as usize];
                                    pp -= 1;
                                }
                                s.mtfbase[lno as usize] += 1;
                                let mut lno = lno;
                                while lno > 0 {
                                    s.mtfbase[lno as usize] -= 1;
                                    s.mtfa[s.mtfbase[lno as usize] as usize] =
                                        s.mtfa[(s.mtfbase[(lno - 1) as usize] + 15) as usize];
                                    lno -= 1;
                                }
                                s.mtfbase[0] -= 1;
                                s.mtfa[s.mtfbase[0] as usize] = uc;
        
                                if s.mtfbase[0] == 0 {
                                    let mut kk = 4095;
                                    let mut ii = 15;
                                    while ii >= 0 {
                                        let mut jj = 15;
                                        while jj >= 0 {
                                            s.mtfa[kk] = s.mtfa[(s.mtfbase[ii] + jj) as usize];
                                            kk -= 1;
                                            jj -= 1;
                                        }
                                        s.mtfbase[ii] = (kk + 1) as i32;
                                        ii -= 1;
                                    }
                                }
                            }
                            s.unzftab[s.seq_to_unseq[uc as usize] as usize] += 1;
                            if s.small_decompress {
                                s.ll16.as_mut().unwrap()[nblock as usize] = s.seq_to_unseq[uc as usize] as u16;
                            } else {
                                s.tt.as_mut().unwrap()[nblock as usize] = s.seq_to_unseq[uc as usize] as u32;
                            }
                            nblock += 1;
        
                            if group_pos == 0 {
                                group_no += 1;
                                if group_no >= n_selectors {
                                    ret_val = -4;
                                    current_block = 8589876134443771061;
                                    continue;
                                } else {
                                    group_pos = 50;
                                    g_sel = s.selector[group_no as usize] as i32;
                                    g_minlen = s.min_lens[g_sel as usize];
                                    g_limit = &mut s.limit[g_sel as usize];
                                    g_perm = &mut s.perm[g_sel as usize];
                                    g_base = &mut s.base[g_sel as usize];
                                    
                                }
                            }
                            group_pos -= 1;
                            zn = g_minlen;
                            current_block = 4758579651109969911;
                            continue;
                        }
                        current_block = 5649595406143318745;
                    }
                } else {
                    current_block = 4069074773319880902;
                }
                match current_block {
                    5649595406143318745 => {}
                    _ => {
                        if s.orig_ptr < 0 || s.orig_ptr >= nblock {
                            ret_val = -4;
                            current_block = 8589876134443771061;
                            continue;
                        } else {
                            i = 0;
                            while i <= 255 {
                                if s.unzftab[i as usize] < 0 || s.unzftab[i as usize] > nblock {
                                    ret_val = -4;
                                    current_block = 8589876134443771061;
                                    continue 'c_10057;
                                }
                                i += 1;
                            }

                            s.cftab[0] = 0;
                            i = 1;
                            while i <= 256 {
                                s.cftab[i as usize] = s.unzftab[(i - 1) as usize];
                                i += 1;
                            }

                            i = 1;
                            while i <= 256 {
                                s.cftab[i as usize] += s.cftab[(i - 1) as usize];
                                i += 1;
                            }

                            i = 0;
                            while i <= 256 {
                                if s.cftab[i as usize] < 0 || s.cftab[i as usize] > nblock {
                                    ret_val = -4;
                                    current_block = 8589876134443771061;
                                    continue 'c_10057;
                                }
                                i += 1;
                            }
                            s.state_out_len = 0;
                            s.state_out_ch = 0 as u8;
                            s.calculated_block_crc = 0xffffffff_u32;
                            s.state = 2;

                            if s.verbosity >= 2 {
                                eprintln!("rt+rld");
                            }

                            if s.small_decompress {
                                // Copy cftab
                                for i in 0..=256 {
                                    s.cftab_copy[i] = s.cftab[i];
                                }
                            
                                // Update ll16 and ll4
                                for i in 0..nblock {
                                    let uc = s.ll16.as_mut().unwrap()[i as usize] as u8;
                                    s.ll16.as_mut().unwrap()[i as usize] = (s.cftab_copy[uc as usize] & 0xffff) as u16;
                            
                                    if i & 0x1 == 0 {
                                        s.ll4.as_mut().unwrap()[(i >> 1) as usize] = (s.ll4.as_mut().unwrap()[(i >> 1) as usize] & 0xf0)
                                            | (s.cftab_copy[uc as usize] >> 16) as u8;
                                    } else {
                                        s.ll4.as_mut().unwrap()[(i >> 1) as usize] = (s.ll4.as_mut().unwrap()[(i >> 1) as usize] & 0x0f)
                                            | ((s.cftab_copy[uc as usize] >> 16) << 4) as u8;
                                    }                                    
                            
                                    s.cftab_copy[uc as usize] += 1;
                                }
                            
                                // Update tPos and nblock_used
                                let mut i = s.orig_ptr;
                                let mut j = ((s.ll16.as_mut().unwrap()[i as usize] as u32) | ((s.ll4.as_mut().unwrap()[(i >> 1) as usize] as u32 >> ((i << 2) & 0x4)) & 0xf) << 16) as i32;
                            
                                loop {
                                    let tmp = ((s.ll16.as_mut().unwrap()[j as usize] as u32) | ((s.ll4.as_mut().unwrap()[j as usize >> 1] as u32 >> ((j << 2) & 0x4)) & 0xf) << 16) as i32;
                            
                                    s.ll16.as_mut().unwrap()[j as usize] = (i & 0xffff) as u16;
                            
                                    if j & 0x1 == 0 {
                                        s.ll4.as_mut().unwrap()[(j >> 1) as usize] = (s.ll4.as_mut().unwrap()[(j >> 1) as usize] & 0xf0) | (i >> 16) as u8;
                                    } else {
                                        s.ll4.as_mut().unwrap()[(j >> 1) as usize] = (s.ll4.as_mut().unwrap()[(j >> 1) as usize] & 0x0f) | ((i >> 16) << 4) as u8;
                                    }                                    
                            
                                    i = j;
                                    j = tmp;
                            
                                    if i == s.orig_ptr {
                                        break;
                                    }
                                }
                            
                                s.t_pos = s.orig_ptr as u32;
                                s.nblock_used = 0;
                            
                                if s.block_randomised {
                                    s.r_n_to_go = 0;
                                    s.r_t_pos = 0;
                            
                                    if s.t_pos >= 100_000_u32.wrapping_mul(s.block_size_100k as u32) {
                                        return 1;
                                    }
                            
                                    s.k0 = bz2_index_into_f(s.t_pos as i32, &mut s.cftab);
                            
                                    s.t_pos = s.ll16.as_mut().unwrap()[s.t_pos as usize] as u32
                                        | ((s.ll4.as_mut().unwrap()[(s.t_pos >> 1) as usize] as u32 >> ((s.t_pos << 2) & 0x4)) & 0xf) << 16;
                            
                                    s.nblock_used += 1;
                            
                                    if s.r_n_to_go == 0 {
                                        s.r_n_to_go = BZ2_rNums[s.r_t_pos as usize];
                                        s.r_t_pos += 1;
                            
                                        if s.r_t_pos == 512 {
                                            s.r_t_pos = 0;
                                        }
                                    }
                            
                                    s.r_n_to_go -= 1;
                            
                                    s.k0 ^= if s.r_n_to_go == 1 { 1 } else { 0 };
                                } else {
                                    if s.t_pos >= 100_000_u32.wrapping_mul(s.block_size_100k as u32) {
                                        return 1;
                                    }
                            
                                    s.k0 = bz2_index_into_f(s.t_pos as i32, &mut s.cftab);
                            
                                    s.t_pos = s.ll16.as_mut().unwrap()[s.t_pos as usize] as u32
                                        | ((s.ll4.as_mut().unwrap()[(s.t_pos >> 1) as usize] as u32 >> ((s.t_pos << 2) & 0x4)) & 0xf) << 16;
                            
                                    s.nblock_used += 1;
                                }
                            } else {
                                // Update tt
                                for i in 0..nblock {
                                    let uc = (s.tt.as_mut().unwrap()[i as usize] & 0xff) as u8;
                                    s.tt.as_mut().unwrap()[s.cftab[uc as usize] as usize] |= (i << 8) as u32;
                                    s.cftab[uc as usize] += 1;
                                }

                                // Update tPos and nblock_used
                                s.t_pos = s.tt.as_mut().unwrap()[s.orig_ptr as usize] >> 8;
                                s.nblock_used = 0;

                                if s.block_randomised {
                                    s.r_n_to_go = 0;
                                    s.r_t_pos = 0;

                                    if s.t_pos >= 100_000_u32.wrapping_mul(s.block_size_100k as u32) {
                                        return 1;
                                    }

                                    s.t_pos = s.tt.as_mut().unwrap()[s.t_pos as usize];
                                    s.k0 = ((s.t_pos & 0xff) as u8) as i32;
                                    s.t_pos >>= 8;

                                    s.nblock_used += 1;

                                    if s.r_n_to_go == 0 {
                                        s.r_n_to_go = BZ2_rNums[s.r_t_pos as usize];
                                        s.r_t_pos += 1;

                                        if s.r_t_pos == 512 {
                                            s.r_t_pos = 0;
                                        }
                                    }

                                    s.r_n_to_go -= 1;

                                    s.k0 ^= if s.r_n_to_go == 1 { 1 } else { 0 };
                                } else {
                                    if s.t_pos >= 100_000_u32.wrapping_mul(s.block_size_100k as u32) {
                                        return 1;
                                    }

                                    s.t_pos = s.tt.as_mut().unwrap()[s.t_pos as usize];
                                    s.k0 = ((s.t_pos & 0xff) as u8) as i32;
                                    s.t_pos >>= 8;

                                    s.nblock_used += 1;
                                }
                            } 
                            ret_val = 0;
                            current_block = 8589876134443771061;
                            continue;
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            5649595406143318745 => {
                if n >= 2 * 1024 * 1024 {
                    ret_val = -4;
                    current_block = 8589876134443771061;
                    continue;
                } else {
                    if next_sym == 0 {
                        es += (0 + 1) * n;
                    } else if next_sym == 1 {
                        es += (1 + 1) * n;
                    }
                    n *= 2;
        
                    if group_pos == 0 {
                        group_no += 1;
                        if group_no >= n_selectors {
                            ret_val = -4;
                            current_block = 8589876134443771061;
                            continue;
                        } else {
                            group_pos = 50;
                            g_sel = s.selector[group_no as usize] as i32;
                            g_minlen = s.min_lens[g_sel as usize];
                            g_limit = &mut s.limit[g_sel as usize];
                            g_perm = &mut s.perm[g_sel as usize];
                            g_base = &mut s.base[g_sel as usize];                            
                        }
                    }
        
                    group_pos -= 1;
                    zn = g_minlen;
                    current_block = 2173067795562692423;
                    continue;
                }
            }
            _ => {}
        }
        loop {
            match current_block {
                16953886395775657100 => {
                    if j < 16 {
                        current_block = 10115417614016365113;
                        continue 'c_10057;
                    }
                }
                3503188808869013853 => {
                    if i < n_selectors {
                        j = 0;
                        current_block = 16531797892856733396;
                        continue;
                    } else {
                        if n_selectors > 2 + 900_000 / 50 {
                            n_selectors = 2 + 900_000 / 50;
                        }
                        let mut pos: [u8; 6] = [0; 6];
                        let mut tmp: u8;
                        let mut v_22: u8 = 0;
                        while (v_22 as i32) < n_groups {
                            pos[v_22 as usize] = v_22;
                            v_22 = v_22.wrapping_add(1);
                        }
                        i = 0;
                        while i < n_selectors {
                            v_22 = s.selector_mtf[i as usize];
                            tmp = pos[v_22 as usize];
                            while v_22 > 0 {
                                pos[v_22 as usize] = pos[(v_22 - 1) as usize];
                                v_22 = v_22.wrapping_sub(1);
                            }
                            pos[0] = tmp;
                            s.selector[i as usize] = tmp;
                            i += 1;
                        }
                        t = 0;
                        current_block = 2488856075421756534;
                        break;
                    }
                }
                15415362524153386998 => {
                    if i < 16 {
                        if s.in_use16[i as usize] {
                            j = 0;
                            current_block = 16953886395775657100;
                            continue;
                        }
                    } else {
                        make_maps_d(s);
                        if s.n_in_use == 0 {
                            current_block = 12571193857528100212;
                            break;
                        } else {
                            current_block = 9416928054198617439;
                            break;
                        }
                    }
                }
                7746242308555130918 => {
                    s.len[t as usize][i as usize] = curr as u8;
                    i += 1;
                    current_block = 16642413284942005565;
                    continue;
                }
                16642413284942005565 => {
                    if i < alpha_size {
                        current_block = 5533056661327372531;
                        continue;
                    }
                    t += 1;
                    current_block = 2488856075421756534;
                    break;
                }
                10081471997089450706 => {
                    if i < 2 + 900_000 / 50 {
                        s.selector_mtf[i as usize] = j as u8;
                    }
                    i += 1;
                    current_block = 3503188808869013853;
                    continue;
                }
                16531797892856733396 => {
                    if true {
                        current_block = 3927265264033028722;
                        continue 'c_10057;
                    } else {
                        current_block = 10081471997089450706;
                        continue;
                    }
                }
                _ => {
                    if !true {
                        current_block = 7746242308555130918;
                        continue;
                    }
                    if !(curr < 1 || curr > 20) {
                        current_block = 14062207274631763586;
                        continue 'c_10057;
                    }
                    ret_val = -4;
                    current_block = 8589876134443771061;
                    continue 'c_10057;
                }
            }
            i += 1;
            current_block = 15415362524153386998;
        }
        match current_block {
            9416928054198617439 => {
                alpha_size = s.n_in_use + 2;
                current_block = 3137761655869617204;
            }
            12571193857528100212 => {
                ret_val = -4;
                current_block = 8589876134443771061;
            }
            _ => {
                if t < n_groups {
                    current_block = 9633808932868333306;
                    continue;
                }
                t = 0;
                while t < n_groups {
                    min_len = 32;
                    max_len = 0;
                    i = 0;
                    while i < alpha_size {
                        if s.len[t as usize][i as usize] as i32 > max_len {
                            max_len = s.len[t as usize][i as usize] as i32;
                        }
                        if (s.len[t as usize][i as usize] as i32) < min_len {
                            min_len = s.len[t as usize][i as usize] as i32;
                        }
                        i += 1;
                    }
                    bz2_hb_create_decode_tables(
                        &mut s.limit[t as usize][..],   // Passing the entire row as a slice
                        &mut s.base[t as usize][..],    // Passing the entire row as a slice
                        &mut s.perm[t as usize][..],    // Passing the entire row as a slice
                        &s.len[t as usize][..], 
                        min_len,
                        max_len,
                        alpha_size,
                    );
                    s.min_lens[t as usize] = min_len;
                    t += 1;
                }
                eob = s.n_in_use + 1;
                nblock_max = 100_000 * s.block_size_100k;
                group_no = -1;
                group_pos = 0;
                i = 0;
                while i <= 255 {
                    s.unzftab[i as usize] = 0;
                    i += 1;
                }
                let mut ii: i32;
                let mut jj: i32;
                let mut kk: i32;
                kk = 4096 - 1;
                ii = 256 / 16 - 1;
                while ii >= 0 {
                    jj = 16 - 1;
                    while jj >= 0 {
                        s.mtfa[kk as usize] = (ii * 16 + jj) as u8;
                        kk -= 1;
                        jj -= 1;
                    }
                    s.mtfbase[ii as usize] = kk + 1;
                    ii -= 1;
                }
                nblock = 0;
                if group_pos == 0 {
                    group_no += 1;
                    if group_no >= n_selectors {
                        ret_val = -4;
                        current_block = 8589876134443771061;
                        continue;
                    } else {
                        group_pos = 50;
                        g_sel = s.selector[group_no as usize] as i32;
                        g_minlen = s.min_lens[g_sel as usize];
                        g_limit = &mut s.limit[g_sel as usize];
                        g_perm = &mut s.perm[g_sel as usize];
                        g_base = &mut s.base[g_sel as usize];                        
                    }
                }
                group_pos -= 1;
                zn = g_minlen;
                current_block = 11392177284674087225;
            }
        }
    }
    s.save_j = j;
    s.save_t = t;
    s.save_alpha_size = alpha_size;
    s.save_n_groups = n_groups;
    s.save_n_selectors = n_selectors;
    s.save_eob = eob;
    s.save_group_no = group_no;
    s.save_group_pos = group_pos;
    s.save_next_sym = next_sym;
    s.save_nblock_max = nblock_max;
    s.save_nblock = nblock;
    s.save_es = es;
    s.save_n = n;
    s.save_curr = curr;
    s.save_zt = zt;
    s.save_zn = zn;
    s.save_zvec = zvec;
    s.save_zj = zj;
    s.save_g_sel = g_sel;
    s.save_g_minlen = g_minlen;
    s.save_g_limit = Some(g_limit);
    s.save_g_base = Some(g_base);
    s.save_g_perm = Some(g_perm);

    return ret_val;
}
    
    
    
    
    
    
    
    
    
    