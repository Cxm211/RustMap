

pub fn add_pair_to_block(s: &mut EState) {
    let ch: u8 = s.state_in_ch as u8;

    // Update blockCRC for each character in the sequence
    for _ in 0..s.state_in_len {
        s.block_crc = (s.block_crc << 8) ^ BZ2_CRC32_TABLE[((s.block_crc >> 24) ^ (ch as u32)) as usize];
    }

    // Mark character as in use
    s.in_use[s.state_in_ch as usize] = true;

    // Add characters to block based on state_in_len
    match s.state_in_len {
        1 => {
            s.block[s.nblock] = ch;
            s.nblock += 1;
        }
        2 => {
            s.block[s.nblock] = ch;
            s.nblock += 1;
            s.block[s.nblock] = ch;
            s.nblock += 1;
        }
        3 => {
            s.block[s.nblock] = ch;
            s.nblock += 1;
            s.block[s.nblock] = ch;
            s.nblock += 1;
            s.block[s.nblock] = ch;
            s.nblock += 1;
        }
        _ => {
            s.in_use[(s.state_in_len - 4) as usize] = true;
            s.block[s.nblock] = ch;
            s.nblock += 1;
            s.block[s.nblock] = ch;
            s.nblock += 1;
            s.block[s.nblock] = ch;
            s.nblock += 1;
            s.block[s.nblock] = ch;
            s.nblock += 1;
            s.block[s.nblock] = (s.state_in_len - 4) as u8;
            s.nblock += 1;
        }
    }
}

pub fn add_char_to_block(s: &mut EState, zchh0: u32) {
    let zchh = zchh0;

    if zchh != s.state_in_ch && s.state_in_len == 1 {
        if let Some(block) = s.block.as_mut() {
            let ch = s.state_in_ch as u8;
            s.block_crc = (s.block_crc << 8) ^ BZ2_CRC32_TABLE[((s.block_crc >> 24) ^ ch as u32) as usize];
            s.in_use[s.state_in_ch as usize] = true;
            block[s.nblock as usize] = ch;
            s.nblock += 1;
        }
        s.state_in_ch = zchh;
    } else if zchh != s.state_in_ch || s.state_in_len == 255 {
        if s.state_in_ch < 256 {
            add_pair_to_block(s);
        }
        s.state_in_ch = zchh;
        s.state_in_len = 1;
    } else {
        s.state_in_len += 1;
    }
}

pub fn flush_rl(s: &mut EState) {
    if s.state_in_ch < 256 {
        add_pair_to_block(s);
    }
    init_rl(s);
}
