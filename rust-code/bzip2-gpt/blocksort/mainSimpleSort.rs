

use crate::blocksort::mainGtU::main_gt_u;

const INCS: [i32; 14] = [
    1, 4, 13, 40, 121, 364, 1093, 3280, 9841, 29524, 88573, 265720, 797161, 2391484,
];

pub fn main_simple_sort(
    ptr: &mut [u32],
    block: &[u8],
    quadrant: &[u16],
    nblock: u32,
    lo: i32,
    hi: i32,
    d: i32,
    budget: &mut i32,
) {
    let mut i: i32;
    let mut j: i32;
    let mut h: i32;
    let mut big_n: i32;
    let mut hp: i32;
    let mut v: u32;

    big_n = hi - lo + 1;
    if big_n < 2 {
        return;
    }

    // Find the appropriate index in the increments array
    hp = 0;
    while INCS[hp as usize] < big_n {
        hp += 1;
    }
    hp -= 1;

    // Sorting loop using shell sort with the increments array
    while hp >= 0 {
        h = INCS[hp as usize];
        i = lo + h;

        loop {
            if i > hi {
                break;
            }
            v = ptr[i as usize];
            j = i;

            while main_gt_u(
                ptr[(j - h) as usize] + d as u32,
                v + d as u32,
                block,
                quadrant,
                nblock,
                budget,
            ) {
                ptr[j as usize] = ptr[(j - h) as usize];
                j -= h;
                if j <= (lo + h - 1) {
                    break;
                }
            }
            ptr[j as usize] = v;
            i += 1;

            if i > hi {
                break;
            }
            v = ptr[i as usize];
            j = i;

            while main_gt_u(
                ptr[(j - h) as usize] + d as u32,
                v + d as u32,
                block,
                quadrant,
                nblock,
                budget,
            ) {
                ptr[j as usize] = ptr[(j - h) as usize];
                j -= h;
                if j <= (lo + h - 1) {
                    break;
                }
            }
            ptr[j as usize] = v;
            i += 1;

            if i > hi {
                break;
            }
            v = ptr[i as usize];
            j = i;

            while main_gt_u(
                ptr[(j - h) as usize] + d as u32,
                v + d as u32,
                block,
                quadrant,
                nblock,
                budget,
            ) {
                ptr[j as usize] = ptr[(j - h) as usize];
                j -= h;
                if j <= (lo + h - 1) {
                    break;
                }
            }
            ptr[j as usize] = v;
            i += 1;

            // If the budget is exhausted, return early
            if *budget < 0 {
                return;
            }
        }

        hp -= 1;
    }
}
