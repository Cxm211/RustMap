pub fn bz2_hb_create_decode_tables(
    limit: &mut [i32],  // Equivalent to `Int32*`
    base: &mut [i32],   // Equivalent to `Int32*`
    perm: &mut [i32],   // Equivalent to `Int32*`
    length: &[u8],      // Equivalent to `UChar*` (in Rust, it's `[u8]`)
    min_len: i32,
    max_len: i32,
    alpha_size: i32,
) {
    let mut pp: i32 = 0;
    let mut vec: i32 = 0;

    // Step 1: Assign the permutation (perm) table
    for i in min_len..=max_len {
        for j in 0..alpha_size {
            if length[j as usize] == i as u8 {
                perm[pp as usize] = j;
                pp += 1;
            }
        }
    }

    // Step 2: Initialize the base array
    for i in 0..23 {
        base[i as usize] = 0;
    }
    for i in 0..alpha_size {
        base[(length[i as usize] + 1) as usize] += 1;
    }

    // Step 3: Build the cumulative base array
    for i in 1..23 {
        base[i as usize] += base[(i - 1) as usize];
    }

    // Step 4: Initialize the limit array
    for i in 0..23 {
        limit[i as usize] = 0;
    }

    // Step 5: Set up the limit array and adjust `vec`
    for i in min_len..=max_len {
        vec += base[(i + 1) as usize] - base[i as usize];
        limit[i as usize] = vec - 1;
        vec <<= 1;  // Shift `vec` left to prepare for the next length
    }

    // Step 6: Adjust the base array for decoding
    for i in (min_len + 1)..=max_len {
        base[i as usize] = ((limit[(i - 1) as usize] + 1) << 1) - base[i as usize];
    }
}
