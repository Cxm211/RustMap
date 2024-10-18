

use crate::blocksort::fallbackSimpleSort::fallback_simple_sort;
use crate::bzlib::BZ2_bz__AssertH__fail::BZ2_bz__AssertH__fail;

const FALLBACK_QSORT_SMALL_THRESH: i32 = 10;
const FALLBACK_QSORT_STACK_SIZE: usize = 100;

pub fn fswap(zz1: &mut u32, zz2: &mut u32) {
    let zztmp = *zz1;
    *zz1 = *zz2;
    *zz2 = zztmp;
}

pub fn fvswap(zzp1: i32, zzp2: i32, zzn: i32, fmap: &mut [u32]) {
    let mut yyp1 = zzp1;
    let mut yyp2 = zzp2;
    let mut yyn = zzn;
    while yyn > 0 {
        fswap(&mut fmap[yyp1 as usize], &mut fmap[yyp2 as usize]);
        yyp1 += 1;
        yyp2 += 1;
        yyn -= 1;
    }
}

pub fn fmin(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

pub fn fpush(stack_lo: &mut [i32; FALLBACK_QSORT_STACK_SIZE], stack_hi: &mut [i32; FALLBACK_QSORT_STACK_SIZE], sp: &mut usize, lz: i32, hz: i32) {
    stack_lo[*sp] = lz;
    stack_hi[*sp] = hz;
    *sp += 1;
}

pub fn fpop(stack_lo: &mut [i32; FALLBACK_QSORT_STACK_SIZE], stack_hi: &mut [i32; FALLBACK_QSORT_STACK_SIZE], sp: &mut usize, lz: &mut i32, hz: &mut i32) {
    *sp -= 1;
    *lz = stack_lo[*sp];
    *hz = stack_hi[*sp];
}

pub fn fallback_qsort3(fmap: &mut [u32], eclass: &[u32], lo_st: i32, hi_st: i32) {
    let mut un_lo: i32;
    let mut un_hi: i32;
    let mut lt_lo: i32;
    let mut gt_hi: i32;
    let mut n: i32;
    let mut m: i32;
    let mut sp: usize = 0;
    let mut lo: i32;
    let mut hi: i32;
    let mut med: u32;
    let mut r: u32 = 0;
    let mut r3: u32;
    let mut stack_lo = [0i32; FALLBACK_QSORT_STACK_SIZE];
    let mut stack_hi = [0i32; FALLBACK_QSORT_STACK_SIZE];

    fpush(&mut stack_lo, &mut stack_hi, &mut sp, lo_st, hi_st);

    while sp > 0 {
        if !(sp < FALLBACK_QSORT_STACK_SIZE - 1) {
            BZ2_bz__AssertH__fail(1004);
        }

        fpop(&mut stack_lo, &mut stack_hi, &mut sp, &mut lo, &mut hi);
        if hi - lo < FALLBACK_QSORT_SMALL_THRESH {
            fallback_simple_sort(fmap, eclass, lo, hi);  // Assuming fallback_simple_sort is defined elsewhere
            continue;
        }

        r = ((r * 7621) + 1) % 32768;
        r3 = r % 3;

        med = if r3 == 0 {
            eclass[fmap[lo as usize]as usize]
        } else if r3 == 1 {
            eclass[fmap[((lo + hi) >> 1) as usize]as usize]
        } else {
            eclass[fmap[hi as usize]as usize]
        };

        lt_lo = lo;
        un_lo = lo;
        
        gt_hi = hi;
        un_hi = hi;

        loop {
            loop {
                if un_lo > un_hi {
                    break;
                }
                n = eclass[fmap[un_lo as usize] as usize] as i32 - med as i32;
                if n == 0 {
                    fswap(&mut fmap[un_lo as usize], &mut fmap[lt_lo as usize]);
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
                n = eclass[fmap[un_hi as usize] as usize] as i32 - med as i32;
                if n == 0 {
                    fswap(&mut fmap[un_hi as usize], &mut fmap[gt_hi as usize]);
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

            fswap(&mut fmap[un_lo as usize], &mut fmap[un_hi as usize]);
            un_lo += 1;
            un_hi -= 1;
        }

        if gt_hi < lt_lo {
            continue;
        }

        n = fmin(lt_lo - lo, un_lo - lt_lo);
        fvswap(lo, un_lo - n, n, fmap);
        m = fmin(hi - gt_hi, gt_hi - un_hi);
        fvswap(un_lo, hi - m + 1, m, fmap);

        n = lo + un_lo - lt_lo - 1;
        m = hi - (gt_hi - un_hi) + 1;

        if n - lo > hi - m {
            fpush(&mut stack_lo, &mut stack_hi, &mut sp, lo, n);
            fpush(&mut stack_lo, &mut stack_hi, &mut sp, m, hi);
        } else {
            fpush(&mut stack_lo, &mut stack_hi, &mut sp, m, hi);
            fpush(&mut stack_lo, &mut stack_hi, &mut sp, lo, n);
        }
    }
}
