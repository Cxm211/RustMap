
use crate::blocksort::mainQSort3::main_qsort3;

const SETMASK: i32 = 1 << 21;
const CLEARMASK: i32 = !(SETMASK);
const MAIN_QSORT_SMALL_THRESH: i32 = 20;
const MAIN_QSORT_DEPTH_THRESH: i32 = 2 + 12;

pub fn bigfreq(b: i32, ftab: &[u32]) -> i32 {
    ftab[((b + 1) << 8) as usize] as i32 - ftab[(b << 8) as usize] as i32
}

pub fn main_sort(
    ptr: &mut [u32],
    block: &[u8],
    quadrant: &mut [u16],
    ftab: &mut [u32],
    nblock: i32,
    verb: i32,
    budget: &mut i32,
) {
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut ss: i32;
    let mut sb: i32;
    let mut running_order = [0i32; 256];
    let mut big_done = [false; 256];
    let mut copy_start = [0i32; 256];
    let mut copy_end = [0i32; 256];
    let mut c1: u8;
    let mut num_qsorted: i32;
    let mut s: u16;

    if verb >= 4 {
        println!("        main sort initialise ...");
    }

    // Initialize ftab
    for i in (0..=65536).rev() {
        ftab[i as usize] = 0;
    }

    // First pass
    j = (block[0] as i32) << 8;
    i = nblock - 1;
    while i >= 3 {
        quadrant[i as usize] = 0;
        j = (j >> 8) | ((block[i as usize] as u16) << 8) as i32;
        ftab[j as usize] += 1;
        quadrant[(i - 1) as usize] = 0;
        j = (j >> 8) | ((block[(i - 1) as usize] as u16) << 8) as i32;
        ftab[j as usize] += 1;
        quadrant[(i - 2) as usize] = 0;
        j = (j >> 8) | ((block[(i - 2) as usize] as u16) << 8) as i32;
        ftab[j as usize] += 1;
        quadrant[(i - 3) as usize] = 0;
        j = (j >> 8) | ((block[(i - 3) as usize] as u16) << 8) as i32;
        ftab[j as usize] += 1;
        i -= 4;
    }
    while i >= 0 {
        quadrant[i as usize] = 0;
        j = (j >> 8) | ((block[i as usize] as u16) << 8) as i32;
        ftab[j as usize] += 1;
        i -= 1;
    }

    // Set block and quadrant boundaries
    for i in 0..(2 + 12 + 18 + 2) {
        block[nblock as usize + i] = block[i];
        quadrant[nblock as usize + i] = 0;
    }

    if verb >= 4 {
        println!("        bucket sorting ...");
    }

    // Second pass
    for i in 1..=65536 {
        ftab[i as usize] += ftab[(i - 1) as usize];
    }

    s = (block[0] as u16) << 8;
    i = nblock - 1;
    while i >= 3 {
        s = (s >> 8) | ((block[i as usize] as u16) << 8);
        j = ftab[s as usize] - 1;
        ftab[s as usize] = j;
        ptr[j as usize] = i as u32;
        s = (s >> 8) | ((block[(i - 1) as usize] as u16) << 8);
        j = ftab[s as usize] - 1;
        ftab[s as usize] = j;
        ptr[j as usize] = (i - 1) as u32;
        s = (s >> 8) | ((block[(i - 2) as usize] as u16) << 8);
        j = ftab[s as usize] - 1;
        ftab[s as usize] = j;
        ptr[j as usize] = (i - 2) as u32;
        s = (s >> 8) | ((block[(i - 3) as usize] as u16) << 8);
        j = ftab[s as usize] - 1;
        ftab[s as usize] = j;
        ptr[j as usize] = (i - 3) as u32;
        i -= 4;
    }
    while i >= 0 {
        s = (s >> 8) | ((block[i as usize] as u16) << 8);
        j = ftab[s as usize] - 1;
        ftab[s as usize] = j;
        ptr[j as usize] = i as u32;
        i -= 1;
    }

    // Initialize running order and bigDone
    for i in 0..=255 {
        big_done[i as usize] = false;
        running_order[i as usize] = i;
    }

    // Sort the running order using BIGFREQ
    let mut vv: i32;
    let mut h = 1;
    while h <= 256 {
        h = 3 * h + 1;
    }
    while h > 1 {
        h /= 3;
        for i in h..=255 {
            vv = running_order[i as usize];
            j = i;
            while bigfreq(running_order[(j - h) as usize], ftab)
                > bigfreq(vv, ftab)
            {
                running_order[j as usize] = running_order[(j - h) as usize];
                j -= h;
                if j <= (h - 1) {
                    break;
                }
            }
            running_order[j as usize] = vv;
        }
    }

    num_qsorted = 0;

    // Main sorting loop
    for i in 0..=255 {
        ss = running_order[i as usize];
        for j in 0..=255 {
            if j != ss {
                sb = (ss << 8) + j;
                if ftab[sb as usize] & SETMASK == 0 {
                    let lo = ftab[sb as usize] & CLEARMASK;
                    let hi = (ftab[(sb + 1) as usize] & CLEARMASK) - 1;
                    if hi > lo {
                        if verb >= 4 {
                            println!(
                                "        qsort [0x{:x}, 0x{:x}] done {} this {}",
                                ss, j, num_qsorted, hi - lo + 1
                            );
                        }
                        main_qsort3(ptr, block, quadrant, nblock, lo, hi, 2, budget);
                        num_qsorted += hi - lo + 1;
                        if *budget < 0 {
                            return;
                        }
                    }
                }
                ftab[sb as usize] |= SETMASK;
            }
        }

        assert!(!big_done[ss as usize]);

        // Copy logic
        for j in 0..=255 {
            copy_start[j as usize] =
                ftab[((j << 8) + ss) as usize] & CLEARMASK;
            copy_end[j as usize] =
                (ftab[((j << 8) + ss + 1) as usize] & CLEARMASK) - 1;
        }
        for j in ftab[(ss << 8) as usize] & CLEARMASK..copy_start[ss as usize] {
            k = ptr[j as usize] as i32 - 1;
            if k < 0 {
                k += nblock;
            }
            c1 = block[k as usize];
            if !big_done[c1 as usize] {
                ptr[copy_start[c1 as usize] as usize] = k as u32;
                copy_start[c1 as usize] += 1;
            }
        }
        for j in (copy_end[ss as usize] + 1)..=(ftab[((ss + 1) << 8) as usize] & CLEARMASK) - 1 {
            k = ptr[j as usize] as i32 - 1;
            if k < 0 {
                k += nblock;
            }
            c1 = block[k as usize];
            if !big_done[c1 as usize] {
                ptr[copy_end[c1 as usize] as usize] = k as u32;
                copy_end[c1 as usize] -= 1;
            }
        }

        assert!(
            (copy_start[ss as usize] - 1 == copy_end[ss as usize])
                || (copy_start[ss as usize] == 0 && copy_end[ss as usize] == nblock - 1)
        );

        // Mark the current bucket as processed
        for j in 0..=255 {
            ftab[((j << 8) + ss) as usize] |= SETMASK;
        }
        big_done[ss as usize] = true;

        // Update quadrant
        if i < 255 {
            let bb_start = ftab[(ss << 8) as usize] & CLEARMASK;
            let bb_size = (ftab[((ss + 1) << 8) as usize] & CLEARMASK) - bb_start;
            let mut shifts = 0;

            while (bb_size >> shifts) > 65534 {
                shifts += 1;
            }

            for j in (0..bb_size).rev() {
                let a2update = ptr[(bb_start + j) as usize];
                let qval = (j >> shifts) as u16;
                quadrant[a2update as usize] = qval;
                if a2update < (2 + 12 + 18 + 2) as u32 {
                    quadrant[(a2update + nblock as u32) as usize] = qval;
                }
            }

            assert!(((bb_size - 1) >> shifts) <= 65535);
        }
    }

    if verb >= 4 {
        println!(
            "        {} pointers, {} sorted, {} scanned",
            nblock, num_qsorted, nblock - num_qsorted
        );
    }
}
