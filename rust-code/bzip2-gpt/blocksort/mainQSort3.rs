

use crate::blocksort::mainSimpleSort::main_simple_sort;
use crate::bzlib::BZ2_bz__AssertH__fail::BZ2_bz__AssertH__fail;

const MAIN_QSORT_SMALL_THRESH: i32 = 20;
const MAIN_QSORT_DEPTH_THRESH: i32 = 2 + 12;
const MAIN_QSORT_STACK_SIZE: usize = 100;

pub fn mswap(zz1: &mut u32, zz2: &mut u32) {
    let zztmp = *zz1;
    *zz1 = *zz2;
    *zz2 = zztmp;
}

pub fn mvswap(zzp1: i32, zzp2: i32, zzn: i32, ptr: &mut [u32]) {
    let mut yyp1 = zzp1;
    let mut yyp2 = zzp2;
    let mut yyn = zzn;
    while yyn > 0 {
        mswap(&mut ptr[yyp1 as usize], &mut ptr[yyp2 as usize]);
        yyp1 += 1;
        yyp2 += 1;
        yyn -= 1;
    }
}

pub fn mmed3(a: u8, b: u8, c: u8) -> u8 {
    let mut t;
    let mut a = a;
    let mut b = b;
    if a > b {
        t = a;
        a = b;
        b = t;
    }
    if b > c {
        b = c;
        if a > b {
            b = a;
        }
    }
    b
}

pub fn mmin(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

pub fn mpush(lz: i32, hz: i32, dz: i32, stack_lo: &mut [i32], stack_hi: &mut [i32], stack_d: &mut [i32], sp: &mut usize) {
    stack_lo[*sp] = lz;
    stack_hi[*sp] = hz;
    stack_d[*sp] = dz;
    *sp += 1;
}

pub fn mpop(lz: &mut i32, hz: &mut i32, dz: &mut i32, stack_lo: &mut [i32], stack_hi: &mut [i32], stack_d: &mut [i32], sp: &mut usize) {
    *sp -= 1;
    *lz = stack_lo[*sp];
    *hz = stack_hi[*sp];
    *dz = stack_d[*sp];
}

pub fn mnextsize(az: i32, next_hi: &[i32], next_lo: &[i32]) -> i32 {
    next_hi[az as usize] - next_lo[az as usize]
}

pub fn mnextswap(az: i32, bz: i32, next_lo: &mut [i32], next_hi: &mut [i32], next_d: &mut [i32]) {
    let tz = next_lo[az as usize];
    next_lo[az as usize] = next_lo[bz as usize];
    next_lo[bz as usize] = tz;

    let tz = next_hi[az as usize];
    next_hi[az as usize] = next_hi[bz as usize];
    next_hi[bz as usize] = tz;

    let tz = next_d[az as usize];
    next_d[az as usize] = next_d[bz as usize];
    next_d[bz as usize] = tz;
}

pub fn main_qsort3(
    ptr: &mut [u32],
    block: &[u8],
    quadrant: &[u16],
    nblock: i32,
    lo_st: i32,
    hi_st: i32,
    d_st: i32,
    budget: &mut i32,
) {
    let mut un_lo: i32;
    let mut un_hi: i32;
    let mut lt_lo: i32;
    let mut gt_hi: i32;
    let mut n: i32;
    let mut m: i32;
    let mut med: i32;
    let mut sp: usize = 0;
    let mut lo: i32;
    let mut hi: i32;
    let mut d: i32;

    let mut stack_lo = [0i32; MAIN_QSORT_STACK_SIZE];
    let mut stack_hi = [0i32; MAIN_QSORT_STACK_SIZE];
    let mut stack_d = [0i32; MAIN_QSORT_STACK_SIZE];

    let mut next_lo = [0i32; 3];
    let mut next_hi = [0i32; 3];
    let mut next_d = [0i32; 3];

    mpush(lo_st, hi_st, d_st, &mut stack_lo, &mut stack_hi, &mut stack_d, &mut sp);

    while sp > 0 {
        if sp >= MAIN_QSORT_STACK_SIZE - 2 {
            BZ2_bz__AssertH__fail(1001);
        }

        mpop(&mut lo, &mut hi, &mut d, &mut stack_lo, &mut stack_hi, &mut stack_d, &mut sp);

        if hi - lo < MAIN_QSORT_SMALL_THRESH || d > MAIN_QSORT_DEPTH_THRESH {
            main_simple_sort(ptr, block, quadrant, nblock, lo, hi, d, budget);
            if *budget < 0 {
                return;
            }
            continue;
        }

        med = mmed3(
            block[(ptr[lo] + d) as usize],
            block[(ptr[hi] + d) as usize],
            block[(ptr[((lo + hi) >> 1)] + d) as usize],
        ) as i32;

        un_lo = lt_lo = lo;
        un_hi = gt_hi = hi;

        loop {
            loop {
                if un_lo > un_hi {
                    break;
                }
                n = block[(ptr[un_lo] + d) as usize] as i32 - med;
                if n == 0 {
                    mswap(&mut ptr[un_lo as usize], &mut ptr[lt_lo as usize]);
                    lt_lo += 1;
                    un_lo += 1;
                    continue;
                }
                if n > 0 {
                    break;
                }
                un_lo += 1;
            }

            loop {
                if un_lo > un_hi {
                    break;
                }
                n = block[(ptr[un_hi] + d) as usize] as i32 - med;
                if n == 0 {
                    mswap(&mut ptr[un_hi as usize], &mut ptr[gt_hi as usize]);
                    gt_hi -= 1;
                    un_hi -= 1;
                    continue;
                }
                if n < 0 {
                    break;
                }
                un_hi -= 1;
            }

            if un_lo > un_hi {
                break;
            }

            mswap(&mut ptr[un_lo as usize], &mut ptr[un_hi as usize]);
            un_lo += 1;
            un_hi -= 1;
        }

        if gt_hi < lt_lo {
            mpush(lo, hi, d + 1, &mut stack_lo, &mut stack_hi, &mut stack_d, &mut sp);
            continue;
        }

        n = mmin(lt_lo - lo, un_lo - lt_lo);
        mvswap(lo, un_lo - n, n, ptr);
        m = mmin(hi - gt_hi, gt_hi - un_hi);
        mvswap(un_lo, hi - m + 1, m, ptr);

        n = lo + un_lo - lt_lo - 1;
        m = hi - (gt_hi - un_hi) + 1;

        next_lo[0] = lo;
        next_hi[0] = n;
        next_d[0] = d;
        next_lo[1] = m;
        next_hi[1] = hi;
        next_d[1] = d;
        next_lo[2] = n + 1;
        next_hi[2] = m - 1;
        next_d[2] = d + 1;

        if mnextsize(0, &next_hi, &next_lo) < mnextsize(1, &next_hi, &next_lo) {
            mnextswap(0, 1, &mut next_lo, &mut next_hi, &mut next_d);
        }
        if mnextsize(1, &next_hi, &next_lo) < mnextsize(2, &next_hi, &next_lo) {
            mnextswap(1, 2, &mut next_lo, &mut next_hi, &mut next_d);
        }
        if mnextsize(0, &next_hi, &next_lo) < mnextsize(1, &next_hi, &next_lo) {
            mnextswap(0, 1, &mut next_lo, &mut next_hi, &mut next_d);
        }

        mpush(next_lo[0], next_hi[0], next_d[0], &mut stack_lo, &mut stack_hi, &mut stack_d, &mut sp);
        mpush(next_lo[1], next_hi[1], next_d[1], &mut stack_lo, &mut stack_hi, &mut stack_d, &mut sp);
        mpush(next_lo[2], next_hi[2], next_d[2], &mut stack_lo, &mut stack_hi, &mut stack_d, &mut sp);
    }
}
