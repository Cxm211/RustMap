

use crate::blocksort::fallbackQSort3::fallback_qsort3;

pub fn set_bh(bhtab: &mut [u32], zz: i32) {
    bhtab[(zz >> 5) as usize] |= 1u32 << (zz & 31);
}

pub fn clear_bh(bhtab: &mut [u32], zz: i32) {
    bhtab[(zz >> 5) as usize] &= !(1u32 << (zz & 31));
}

pub fn isset_bh(bhtab: &[u32], zz: i32) -> bool {
    (bhtab[(zz >> 5) as usize] & (1u32 << (zz & 31))) != 0
}

pub fn word_bh(bhtab: &[u32], zz: i32) -> u32 {
    bhtab[(zz >> 5) as usize]
}

pub fn unaligned_bh(zz: i32) -> bool {
    (zz & 0x01f) != 0
}

pub fn fallback_sort(
    fmap: &mut [u32],
    eclass: &mut [u32],
    bhtab: &mut [u32],
    nblock: i32,
    verb: i32,
) {
    let mut ftab = [0i32; 257];
    let mut ftab_copy = [0i32; 256];
    let mut h: i32;
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut l: i32;
    let mut r: i32;
    let mut cc: i32;
    let mut cc1: i32;
    let mut n_not_done: i32;
    let n_bhtab: i32;
    let eclass8 = &mut eclass[..] as &mut [u8];  // Treat eclass as byte slice for UChar

    if verb >= 4 {
        println!("        bucket sorting ...");
    }
    
    for i in 0..257 {
        ftab[i] = 0;
    }
    
    for i in 0..nblock {
        ftab[eclass8[i as usize] as usize] += 1;
    }

    ftab_copy.copy_from_slice(&ftab[0..256]);

    for i in 1..257 {
        ftab[i] += ftab[i - 1];
    }

    for i in 0..nblock {
        j = eclass8[i as usize] as i32;
        k = ftab[j as usize] - 1;
        ftab[j as usize] = k;
        fmap[k as usize] = i as u32;
    }

    n_bhtab = 2 + (nblock / 32);
    for i in 0..n_bhtab {
        bhtab[i as usize] = 0;
    }

    for i in 0..256 {
        set_bh(bhtab, ftab[i]);
    }

    for i in 0..32 {
        set_bh(bhtab, nblock + 2 * i);
        clear_bh(bhtab, nblock + 2 * i + 1);
    }

    h = 1;
    loop {
        if verb >= 4 {
            println!("        depth {:6} has ", h);
        }

        j = 0;
        for i in 0..nblock {
            if isset_bh(bhtab, i) {
                j = i;
            }
            k = fmap[i as usize] as i32 - h;
            if k < 0 {
                k += nblock;
            }
            eclass[k as usize] = j as u32;
        }

        n_not_done = 0;
        r = -1;
        loop {
            k = r + 1;
            while isset_bh(bhtab, k) && unaligned_bh(k) {
                k += 1;
            }
            if isset_bh(bhtab, k) {
                while word_bh(bhtab, k) == 0xffffffff {
                    k += 32;
                }
                while isset_bh(bhtab, k) {
                    k += 1;
                }
            }
            l = k - 1;
            if l >= nblock {
                break;
            }
            while !isset_bh(bhtab, k) && unaligned_bh(k) {
                k += 1;
            }
            if !isset_bh(bhtab, k) {
                while word_bh(bhtab, k) == 0x00000000 {
                    k += 32;
                }
                while !isset_bh(bhtab, k) {
                    k += 1;
                }
            }
            r = k - 1;
            if r >= nblock {
                break;
            }

            if r > l {
                n_not_done += r - l + 1;
                fallback_qsort3(fmap, eclass, l, r);

                cc = -1;
                for i in l..=r {
                    cc1 = eclass[fmap[i as usize] as usize] as i32;
                    if cc != cc1 {
                        set_bh(bhtab, i);
                        cc = cc1;
                    }
                }
            }
        }

        if verb >= 4 {
            println!("{:6} unresolved strings", n_not_done);
        }

        h *= 2;
        if h > nblock || n_not_done == 0 {
            break;
        }
    }

    if verb >= 4 {
        println!("        reconstructing block ...");
    }

    j = 0;
    for i in 0..nblock {
        while ftab_copy[j as usize] == 0 {
            j += 1;
        }
        ftab_copy[j as usize] -= 1;
        eclass8[fmap[i as usize] as usize] = j as u8;
    }

    if j >= 256 {
        BZ2_bz__AssertH__fail(1005);
    }
}
