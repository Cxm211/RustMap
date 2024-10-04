#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn BZ2_bz__AssertH__fail(errcode: libc::c_int);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bz_stream {
    pub next_in: *mut libc::c_char,
    pub avail_in: libc::c_uint,
    pub total_in_lo32: libc::c_uint,
    pub total_in_hi32: libc::c_uint,
    pub next_out: *mut libc::c_char,
    pub avail_out: libc::c_uint,
    pub total_out_lo32: libc::c_uint,
    pub total_out_hi32: libc::c_uint,
    pub state: *mut libc::c_void,
    pub bzalloc: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
        ) -> *mut libc::c_void,
    >,
    pub bzfree: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
    pub opaque: *mut libc::c_void,
}
pub type Bool = libc::c_uchar;
pub type UChar = libc::c_uchar;
pub type Int32 = libc::c_int;
pub type UInt32 = libc::c_uint;
pub type UInt16 = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EState {
    pub strm: *mut bz_stream,
    pub mode: Int32,
    pub state: Int32,
    pub avail_in_expect: UInt32,
    pub arr1: *mut UInt32,
    pub arr2: *mut UInt32,
    pub ftab: *mut UInt32,
    pub origPtr: Int32,
    pub ptr: *mut UInt32,
    pub block: *mut UChar,
    pub mtfv: *mut UInt16,
    pub zbits: *mut UChar,
    pub workFactor: Int32,
    pub state_in_ch: UInt32,
    pub state_in_len: Int32,
    pub rNToGo: Int32,
    pub rTPos: Int32,
    pub nblock: Int32,
    pub nblockMAX: Int32,
    pub numZ: Int32,
    pub state_out_pos: Int32,
    pub nInUse: Int32,
    pub inUse: [Bool; 256],
    pub unseqToSeq: [UChar; 256],
    pub bsBuff: UInt32,
    pub bsLive: Int32,
    pub blockCRC: UInt32,
    pub combinedCRC: UInt32,
    pub verbosity: Int32,
    pub blockNo: Int32,
    pub blockSize100k: Int32,
    pub nMTF: Int32,
    pub mtfFreq: [Int32; 258],
    pub selector: [UChar; 18002],
    pub selectorMtf: [UChar; 18002],
    pub len: [[UChar; 258]; 6],
    pub code: [[Int32; 258]; 6],
    pub rfreq: [[Int32; 258]; 6],
    pub len_pack: [[UInt32; 4]; 258],
}
#[inline]
unsafe extern "C" fn fallbackSimpleSort(
    mut fmap: *mut UInt32,
    mut eclass: *mut UInt32,
    mut lo: Int32,
    mut hi: Int32,
) {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut tmp: Int32 = 0;
    let mut ec_tmp: UInt32 = 0;
    if lo == hi {
        return;
    }
    if hi - lo > 3 as libc::c_int {
        i = hi - 4 as libc::c_int;
        while i >= lo {
            tmp = *fmap.offset(i as isize) as Int32;
            ec_tmp = *eclass.offset(tmp as isize);
            j = i + 4 as libc::c_int;
            while j <= hi && ec_tmp > *eclass.offset(*fmap.offset(j as isize) as isize) {
                *fmap.offset((j - 4 as libc::c_int) as isize) = *fmap.offset(j as isize);
                j += 4 as libc::c_int;
            }
            *fmap.offset((j - 4 as libc::c_int) as isize) = tmp as UInt32;
            i -= 1;
            i;
        }
    }
    i = hi - 1 as libc::c_int;
    while i >= lo {
        tmp = *fmap.offset(i as isize) as Int32;
        ec_tmp = *eclass.offset(tmp as isize);
        j = i + 1 as libc::c_int;
        while j <= hi && ec_tmp > *eclass.offset(*fmap.offset(j as isize) as isize) {
            *fmap.offset((j - 1 as libc::c_int) as isize) = *fmap.offset(j as isize);
            j += 1;
            j;
        }
        *fmap.offset((j - 1 as libc::c_int) as isize) = tmp as UInt32;
        i -= 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn fswap(mut zz1: *mut UInt32, mut zz2: *mut UInt32) {
    let mut zztmp: UInt32 = *zz1;
    *zz1 = *zz2;
    *zz2 = zztmp;
}
#[inline]
unsafe extern "C" fn fvswap(
    mut zzp1: Int32,
    mut zzp2: Int32,
    mut zzn: Int32,
    mut fmap: *mut UInt32,
) {
    let mut yyp1: Int32 = zzp1;
    let mut yyp2: Int32 = zzp2;
    let mut yyn: Int32 = zzn;
    while yyn > 0 as libc::c_int {
        fswap(&mut *fmap.offset(yyp1 as isize), &mut *fmap.offset(yyp2 as isize));
        yyp1 += 1;
        yyp1;
        yyp2 += 1;
        yyp2;
        yyn -= 1;
        yyn;
    }
}
#[inline]
unsafe extern "C" fn fmin(mut a: Int32, mut b: Int32) -> Int32 {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn fpush(
    mut stackLo: *mut Int32,
    mut stackHi: *mut Int32,
    mut sp: *mut Int32,
    mut lz: Int32,
    mut hz: Int32,
) {
    *stackLo.offset(*sp as isize) = lz;
    *stackHi.offset(*sp as isize) = hz;
    *sp += 1;
    *sp;
}
#[inline]
unsafe extern "C" fn fpop(
    mut stackLo: *mut Int32,
    mut stackHi: *mut Int32,
    mut sp: *mut Int32,
    mut lz: *mut Int32,
    mut hz: *mut Int32,
) {
    *sp -= 1;
    *sp;
    *lz = *stackLo.offset(*sp as isize);
    *hz = *stackHi.offset(*sp as isize);
}
#[no_mangle]
pub static mut FALLBACK_QSORT_SMALL_THRESH: Int32 = 10 as libc::c_int;
#[no_mangle]
pub static mut FALLBACK_QSORT_STACK_SIZE: Int32 = 100 as libc::c_int;
unsafe extern "C" fn fallbackQSort3(
    mut fmap: *mut UInt32,
    mut eclass: *mut UInt32,
    mut loSt: Int32,
    mut hiSt: Int32,
) {
    let mut unLo: Int32 = 0;
    let mut unHi: Int32 = 0;
    let mut ltLo: Int32 = 0;
    let mut gtHi: Int32 = 0;
    let mut n: Int32 = 0;
    let mut m: Int32 = 0;
    let mut sp: Int32 = 0;
    let mut lo: Int32 = 0;
    let mut hi: Int32 = 0;
    let mut med: UInt32 = 0;
    let mut r: UInt32 = 0;
    let mut r3: UInt32 = 0;
    let vla = FALLBACK_QSORT_STACK_SIZE as usize;
    let mut stackLo: Vec::<Int32> = ::std::vec::from_elem(0, vla);
    let vla_0 = FALLBACK_QSORT_STACK_SIZE as usize;
    let mut stackHi: Vec::<Int32> = ::std::vec::from_elem(0, vla_0);
    r = 0 as libc::c_int as UInt32;
    sp = 0 as libc::c_int;
    fpush(stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), &mut sp, loSt, hiSt);
    while sp > 0 as libc::c_int {
        if !(sp < FALLBACK_QSORT_STACK_SIZE - 1 as libc::c_int) {
            BZ2_bz__AssertH__fail(1004 as libc::c_int);
        }
        fpop(stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), &mut sp, &mut lo, &mut hi);
        if hi - lo < FALLBACK_QSORT_SMALL_THRESH {
            fallbackSimpleSort(fmap, eclass, lo, hi);
        } else {
            r = r
                .wrapping_mul(7621 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_rem(32768 as libc::c_int as libc::c_uint);
            r3 = r.wrapping_rem(3 as libc::c_int as libc::c_uint);
            if r3 == 0 as libc::c_int as libc::c_uint {
                med = *eclass.offset(*fmap.offset(lo as isize) as isize);
            } else if r3 == 1 as libc::c_int as libc::c_uint {
                med = *eclass
                    .offset(
                        *fmap.offset((lo + hi >> 1 as libc::c_int) as isize) as isize,
                    );
            } else {
                med = *eclass.offset(*fmap.offset(hi as isize) as isize);
            }
            ltLo = lo;
            unLo = ltLo;
            gtHi = hi;
            unHi = gtHi;
            loop {
                while !(unLo > unHi) {
                    n = *eclass.offset(*fmap.offset(unLo as isize) as isize) as Int32
                        - med as Int32;
                    if n == 0 as libc::c_int {
                        fswap(
                            &mut *fmap.offset(unLo as isize),
                            &mut *fmap.offset(ltLo as isize),
                        );
                        ltLo += 1;
                        ltLo;
                        unLo += 1;
                        unLo;
                    } else {
                        if n > 0 as libc::c_int {
                            break;
                        }
                        unLo += 1;
                        unLo;
                    }
                }
                while !(unLo > unHi) {
                    n = *eclass.offset(*fmap.offset(unHi as isize) as isize) as Int32
                        - med as Int32;
                    if n == 0 as libc::c_int {
                        fswap(
                            &mut *fmap.offset(unHi as isize),
                            &mut *fmap.offset(gtHi as isize),
                        );
                        gtHi -= 1;
                        gtHi;
                        unHi -= 1;
                        unHi;
                    } else {
                        if n < 0 as libc::c_int {
                            break;
                        }
                        unHi -= 1;
                        unHi;
                    }
                }
                if unLo > unHi {
                    break;
                }
                fswap(
                    &mut *fmap.offset(unLo as isize),
                    &mut *fmap.offset(unHi as isize),
                );
                unLo += 1;
                unLo;
                unHi -= 1;
                unHi;
            }
            if gtHi < ltLo {
                continue;
            }
            n = fmin(ltLo - lo, unLo - ltLo);
            fvswap(lo, unLo - n, n, fmap);
            m = fmin(hi - gtHi, gtHi - unHi);
            fvswap(unLo, hi - m + 1 as libc::c_int, m, fmap);
            n = lo + unLo - ltLo - 1 as libc::c_int;
            m = hi - (gtHi - unHi) + 1 as libc::c_int;
            if n - lo > hi - m {
                fpush(stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), &mut sp, lo, n);
                fpush(stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), &mut sp, m, hi);
            } else {
                fpush(stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), &mut sp, m, hi);
                fpush(stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), &mut sp, lo, n);
            }
        }
    }
}
#[inline]
unsafe extern "C" fn SET_BH(mut bhtab: *mut UInt32, mut zz: Int32) {
    let ref mut fresh0 = *bhtab.offset((zz >> 5 as libc::c_int) as isize);
    *fresh0 |= (1 as libc::c_int as UInt32) << (zz & 31 as libc::c_int);
}
#[inline]
unsafe extern "C" fn CLEAR_BH(mut bhtab: *mut UInt32, mut zz: Int32) {
    let ref mut fresh1 = *bhtab.offset((zz >> 5 as libc::c_int) as isize);
    *fresh1 &= !((1 as libc::c_int as UInt32) << (zz & 31 as libc::c_int));
}
#[inline]
unsafe extern "C" fn ISSET_BH(mut bhtab: *mut UInt32, mut zz: Int32) -> Int32 {
    return (*bhtab.offset((zz >> 5 as libc::c_int) as isize)
        & (1 as libc::c_int as UInt32) << (zz & 31 as libc::c_int)) as Int32;
}
#[inline]
unsafe extern "C" fn WORD_BH(mut bhtab: *mut UInt32, mut zz: Int32) -> UInt32 {
    return *bhtab.offset((zz >> 5 as libc::c_int) as isize);
}
#[inline]
unsafe extern "C" fn UNALIGNED_BH(mut zz: Int32) -> Int32 {
    return zz & 0x1f as libc::c_int;
}
unsafe extern "C" fn fallbackSort(
    mut fmap: *mut UInt32,
    mut eclass: *mut UInt32,
    mut bhtab: *mut UInt32,
    mut nblock: Int32,
    mut verb: Int32,
) {
    let mut ftab: [Int32; 257] = [0; 257];
    let mut ftabCopy: [Int32; 256] = [0; 256];
    let mut H: Int32 = 0;
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut k: Int32 = 0;
    let mut l: Int32 = 0;
    let mut r: Int32 = 0;
    let mut cc: Int32 = 0;
    let mut cc1: Int32 = 0;
    let mut nNotDone: Int32 = 0;
    let mut nBhtab: Int32 = 0;
    let mut eclass8: *mut UChar = eclass as *mut UChar;
    if verb >= 4 as libc::c_int {
        fprintf(
            stderr,
            b"        bucket sorting ...\n\0" as *const u8 as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < 257 as libc::c_int {
        ftab[i as usize] = 0 as libc::c_int;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nblock {
        ftab[*eclass8.offset(i as isize) as usize] += 1;
        ftab[*eclass8.offset(i as isize) as usize];
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        ftabCopy[i as usize] = ftab[i as usize];
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i < 257 as libc::c_int {
        ftab[i as usize] += ftab[(i - 1 as libc::c_int) as usize];
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nblock {
        j = *eclass8.offset(i as isize) as Int32;
        k = ftab[j as usize] - 1 as libc::c_int;
        ftab[j as usize] = k;
        *fmap.offset(k as isize) = i as UInt32;
        i += 1;
        i;
    }
    nBhtab = 2 as libc::c_int + nblock / 32 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nBhtab {
        *bhtab.offset(i as isize) = 0 as libc::c_int as UInt32;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        SET_BH(bhtab, ftab[i as usize]);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        SET_BH(bhtab, nblock + 2 as libc::c_int * i);
        CLEAR_BH(bhtab, nblock + 2 as libc::c_int * i + 1 as libc::c_int);
        i += 1;
        i;
    }
    H = 1 as libc::c_int;
    loop {
        if verb >= 4 as libc::c_int {
            fprintf(
                stderr,
                b"        depth %6d has \0" as *const u8 as *const libc::c_char,
                H,
            );
        }
        j = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < nblock {
            if ISSET_BH(bhtab, i) != 0 {
                j = i;
            }
            k = (*fmap.offset(i as isize)).wrapping_sub(H as libc::c_uint) as Int32;
            if k < 0 as libc::c_int {
                k += nblock;
            }
            *eclass.offset(k as isize) = j as UInt32;
            i += 1;
            i;
        }
        nNotDone = 0 as libc::c_int;
        r = -(1 as libc::c_int);
        loop {
            k = r + 1 as libc::c_int;
            while ISSET_BH(bhtab, k) != 0 && UNALIGNED_BH(k) != 0 {
                k += 1;
                k;
            }
            if ISSET_BH(bhtab, k) != 0 {
                while WORD_BH(bhtab, k) == 0xffffffff as libc::c_uint {
                    k += 32 as libc::c_int;
                }
                while ISSET_BH(bhtab, k) != 0 {
                    k += 1;
                    k;
                }
            }
            l = k - 1 as libc::c_int;
            if l >= nblock {
                break;
            }
            while ISSET_BH(bhtab, k) == 0 && UNALIGNED_BH(k) != 0 {
                k += 1;
                k;
            }
            if ISSET_BH(bhtab, k) == 0 {
                while WORD_BH(bhtab, k) == 0 as libc::c_int as libc::c_uint {
                    k += 32 as libc::c_int;
                }
                while ISSET_BH(bhtab, k) == 0 {
                    k += 1;
                    k;
                }
            }
            r = k - 1 as libc::c_int;
            if r >= nblock {
                break;
            }
            if r > l {
                nNotDone += r - l + 1 as libc::c_int;
                fallbackQSort3(fmap, eclass, l, r);
                cc = -(1 as libc::c_int);
                i = l;
                while i <= r {
                    cc1 = *eclass.offset(*fmap.offset(i as isize) as isize) as Int32;
                    if cc != cc1 {
                        SET_BH(bhtab, i);
                        cc = cc1;
                    }
                    i += 1;
                    i;
                }
            }
        }
        if verb >= 4 as libc::c_int {
            fprintf(
                stderr,
                b"%6d unresolved strings\n\0" as *const u8 as *const libc::c_char,
                nNotDone,
            );
        }
        H *= 2 as libc::c_int;
        if H > nblock || nNotDone == 0 as libc::c_int {
            break;
        }
    }
    if verb >= 4 as libc::c_int {
        fprintf(
            stderr,
            b"        reconstructing block ...\n\0" as *const u8 as *const libc::c_char,
        );
    }
    j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nblock {
        while ftabCopy[j as usize] == 0 as libc::c_int {
            j += 1;
            j;
        }
        ftabCopy[j as usize] -= 1;
        ftabCopy[j as usize];
        *eclass8.offset(*fmap.offset(i as isize) as isize) = j as UChar;
        i += 1;
        i;
    }
    if !(j < 256 as libc::c_int) {
        BZ2_bz__AssertH__fail(1005 as libc::c_int);
    }
}
#[inline]
unsafe extern "C" fn mainGtU(
    mut i1: UInt32,
    mut i2: UInt32,
    mut block: *mut UChar,
    mut quadrant: *mut UInt16,
    mut nblock: UInt32,
    mut budget: *mut Int32,
) -> Bool {
    let mut k: Int32 = 0;
    let mut c1: UChar = 0;
    let mut c2: UChar = 0;
    let mut s1: UInt16 = 0;
    let mut s2: UInt16 = 0;
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    k = nblock.wrapping_add(8 as libc::c_int as libc::c_uint) as Int32;
    loop {
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as libc::c_int != c2 as libc::c_int {
            return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as libc::c_int != s2 as libc::c_int {
            return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int as Bool;
        }
        i1 = i1.wrapping_add(1);
        i1;
        i2 = i2.wrapping_add(1);
        i2;
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as libc::c_int != c2 as libc::c_int {
            return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as libc::c_int != s2 as libc::c_int {
            return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int as Bool;
        }
        i1 = i1.wrapping_add(1);
        i1;
        i2 = i2.wrapping_add(1);
        i2;
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as libc::c_int != c2 as libc::c_int {
            return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as libc::c_int != s2 as libc::c_int {
            return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int as Bool;
        }
        i1 = i1.wrapping_add(1);
        i1;
        i2 = i2.wrapping_add(1);
        i2;
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as libc::c_int != c2 as libc::c_int {
            return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as libc::c_int != s2 as libc::c_int {
            return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int as Bool;
        }
        i1 = i1.wrapping_add(1);
        i1;
        i2 = i2.wrapping_add(1);
        i2;
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as libc::c_int != c2 as libc::c_int {
            return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as libc::c_int != s2 as libc::c_int {
            return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int as Bool;
        }
        i1 = i1.wrapping_add(1);
        i1;
        i2 = i2.wrapping_add(1);
        i2;
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as libc::c_int != c2 as libc::c_int {
            return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as libc::c_int != s2 as libc::c_int {
            return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int as Bool;
        }
        i1 = i1.wrapping_add(1);
        i1;
        i2 = i2.wrapping_add(1);
        i2;
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as libc::c_int != c2 as libc::c_int {
            return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as libc::c_int != s2 as libc::c_int {
            return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int as Bool;
        }
        i1 = i1.wrapping_add(1);
        i1;
        i2 = i2.wrapping_add(1);
        i2;
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as libc::c_int != c2 as libc::c_int {
            return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as libc::c_int != s2 as libc::c_int {
            return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int as Bool;
        }
        i1 = i1.wrapping_add(1);
        i1;
        i2 = i2.wrapping_add(1);
        i2;
        if i1 >= nblock {
            i1 = (i1 as libc::c_uint).wrapping_sub(nblock) as UInt32 as UInt32;
        }
        if i2 >= nblock {
            i2 = (i2 as libc::c_uint).wrapping_sub(nblock) as UInt32 as UInt32;
        }
        k -= 8 as libc::c_int;
        *budget -= 1;
        *budget;
        if !(k >= 0 as libc::c_int) {
            break;
        }
    }
    return 0 as libc::c_int as Bool;
}
static mut incs: [Int32; 14] = [
    1 as libc::c_int,
    4 as libc::c_int,
    13 as libc::c_int,
    40 as libc::c_int,
    121 as libc::c_int,
    364 as libc::c_int,
    1093 as libc::c_int,
    3280 as libc::c_int,
    9841 as libc::c_int,
    29524 as libc::c_int,
    88573 as libc::c_int,
    265720 as libc::c_int,
    797161 as libc::c_int,
    2391484 as libc::c_int,
];
unsafe extern "C" fn mainSimpleSort(
    mut ptr: *mut UInt32,
    mut block: *mut UChar,
    mut quadrant: *mut UInt16,
    mut nblock: Int32,
    mut lo: Int32,
    mut hi: Int32,
    mut d: Int32,
    mut budget: *mut Int32,
) {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut h: Int32 = 0;
    let mut bigN: Int32 = 0;
    let mut hp: Int32 = 0;
    let mut v: UInt32 = 0;
    bigN = hi - lo + 1 as libc::c_int;
    if bigN < 2 as libc::c_int {
        return;
    }
    hp = 0 as libc::c_int;
    while incs[hp as usize] < bigN {
        hp += 1;
        hp;
    }
    hp -= 1;
    hp;
    while hp >= 0 as libc::c_int {
        h = incs[hp as usize];
        i = lo + h;
        while 1 as libc::c_int as Bool != 0 {
            if i > hi {
                break;
            }
            v = *ptr.offset(i as isize);
            j = i;
            while mainGtU(
                (*ptr.offset((j - h) as isize)).wrapping_add(d as libc::c_uint),
                v.wrapping_add(d as libc::c_uint),
                block,
                quadrant,
                nblock as UInt32,
                budget,
            ) != 0
            {
                *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                j = j - h;
                if j <= lo + h - 1 as libc::c_int {
                    break;
                }
            }
            *ptr.offset(j as isize) = v;
            i += 1;
            i;
            if i > hi {
                break;
            }
            v = *ptr.offset(i as isize);
            j = i;
            while mainGtU(
                (*ptr.offset((j - h) as isize)).wrapping_add(d as libc::c_uint),
                v.wrapping_add(d as libc::c_uint),
                block,
                quadrant,
                nblock as UInt32,
                budget,
            ) != 0
            {
                *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                j = j - h;
                if j <= lo + h - 1 as libc::c_int {
                    break;
                }
            }
            *ptr.offset(j as isize) = v;
            i += 1;
            i;
            if i > hi {
                break;
            }
            v = *ptr.offset(i as isize);
            j = i;
            while mainGtU(
                (*ptr.offset((j - h) as isize)).wrapping_add(d as libc::c_uint),
                v.wrapping_add(d as libc::c_uint),
                block,
                quadrant,
                nblock as UInt32,
                budget,
            ) != 0
            {
                *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                j = j - h;
                if j <= lo + h - 1 as libc::c_int {
                    break;
                }
            }
            *ptr.offset(j as isize) = v;
            i += 1;
            i;
            if *budget < 0 as libc::c_int {
                return;
            }
        }
        hp -= 1;
        hp;
    }
}
#[inline]
unsafe extern "C" fn mswap(mut zz1: *mut UInt32, mut zz2: *mut UInt32) {
    let mut zztmp: UInt32 = *zz1;
    *zz1 = *zz2;
    *zz2 = zztmp;
}
#[inline]
unsafe extern "C" fn mvswap(
    mut zzp1: Int32,
    mut zzp2: Int32,
    mut zzn: Int32,
    mut ptr: *mut UInt32,
) {
    let mut yyp1: Int32 = zzp1;
    let mut yyp2: Int32 = zzp2;
    let mut yyn: Int32 = zzn;
    while yyn > 0 as libc::c_int {
        mswap(&mut *ptr.offset(yyp1 as isize), &mut *ptr.offset(yyp2 as isize));
        yyp1 += 1;
        yyp1;
        yyp2 += 1;
        yyp2;
        yyn -= 1;
        yyn;
    }
}
#[inline]
unsafe extern "C" fn mmed3(mut a: UChar, mut b: UChar, mut c: UChar) -> UChar {
    let mut t: UChar = 0;
    if a as libc::c_int > b as libc::c_int {
        t = a;
        a = b;
        b = t;
    }
    if b as libc::c_int > c as libc::c_int {
        b = c;
        if a as libc::c_int > b as libc::c_int {
            b = a;
        }
    }
    return b;
}
#[inline]
unsafe extern "C" fn mmin(mut a: Int32, mut b: Int32) -> Int32 {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn mpush(
    mut lz: Int32,
    mut hz: Int32,
    mut dz: Int32,
    mut stackLo: *mut Int32,
    mut stackHi: *mut Int32,
    mut stackD: *mut Int32,
    mut sp: *mut Int32,
) {
    *stackLo.offset(*sp as isize) = lz;
    *stackHi.offset(*sp as isize) = hz;
    *stackD.offset(*sp as isize) = dz;
    *sp += 1;
    *sp;
}
#[inline]
unsafe extern "C" fn mpop(
    mut lz: *mut Int32,
    mut hz: *mut Int32,
    mut dz: *mut Int32,
    mut stackLo: *mut Int32,
    mut stackHi: *mut Int32,
    mut stackD: *mut Int32,
    mut sp: *mut Int32,
) {
    *sp -= 1;
    *sp;
    *lz = *stackLo.offset(*sp as isize);
    *hz = *stackHi.offset(*sp as isize);
    *dz = *stackD.offset(*sp as isize);
}
#[inline]
unsafe extern "C" fn mnextsize(
    mut az: Int32,
    mut nextHi: *mut Int32,
    mut nextLo: *mut Int32,
) -> Int32 {
    return *nextHi.offset(az as isize) - *nextLo.offset(az as isize);
}
#[inline]
unsafe extern "C" fn mnextswap(
    mut az: Int32,
    mut bz: Int32,
    mut nextLo: *mut Int32,
    mut nextHi: *mut Int32,
    mut nextD: *mut Int32,
) {
    let mut tz: Int32 = 0;
    tz = *nextLo.offset(az as isize);
    *nextLo.offset(az as isize) = *nextLo.offset(bz as isize);
    *nextLo.offset(bz as isize) = tz;
    tz = *nextHi.offset(az as isize);
    *nextHi.offset(az as isize) = *nextHi.offset(bz as isize);
    *nextHi.offset(bz as isize) = tz;
    tz = *nextD.offset(az as isize);
    *nextD.offset(az as isize) = *nextD.offset(bz as isize);
    *nextD.offset(bz as isize) = tz;
}
#[no_mangle]
pub static mut MAIN_QSORT_SMALL_THRESH: Int32 = 20 as libc::c_int;
#[no_mangle]
pub static mut MAIN_QSORT_DEPTH_THRESH: Int32 = 2 as libc::c_int + 12 as libc::c_int;
#[no_mangle]
pub static mut MAIN_QSORT_STACK_SIZE: Int32 = 100 as libc::c_int;
unsafe extern "C" fn mainQSort3(
    mut ptr: *mut UInt32,
    mut block: *mut UChar,
    mut quadrant: *mut UInt16,
    mut nblock: Int32,
    mut loSt: Int32,
    mut hiSt: Int32,
    mut dSt: Int32,
    mut budget: *mut Int32,
) {
    let mut unLo: Int32 = 0;
    let mut unHi: Int32 = 0;
    let mut ltLo: Int32 = 0;
    let mut gtHi: Int32 = 0;
    let mut n: Int32 = 0;
    let mut m: Int32 = 0;
    let mut med: Int32 = 0;
    let mut sp: Int32 = 0;
    let mut lo: Int32 = 0;
    let mut hi: Int32 = 0;
    let mut d: Int32 = 0;
    let vla = MAIN_QSORT_STACK_SIZE as usize;
    let mut stackLo: Vec::<Int32> = ::std::vec::from_elem(0, vla);
    let vla_0 = MAIN_QSORT_STACK_SIZE as usize;
    let mut stackHi: Vec::<Int32> = ::std::vec::from_elem(0, vla_0);
    let vla_1 = MAIN_QSORT_STACK_SIZE as usize;
    let mut stackD: Vec::<Int32> = ::std::vec::from_elem(0, vla_1);
    let mut nextLo: [Int32; 3] = [0; 3];
    let mut nextHi: [Int32; 3] = [0; 3];
    let mut nextD: [Int32; 3] = [0; 3];
    sp = 0 as libc::c_int;
    mpush(
        loSt,
        hiSt,
        dSt,
        stackLo.as_mut_ptr(),
        stackHi.as_mut_ptr(),
        stackD.as_mut_ptr(),
        &mut sp,
    );
    while sp > 0 as libc::c_int {
        if !(sp < MAIN_QSORT_STACK_SIZE - 2 as libc::c_int) {
            BZ2_bz__AssertH__fail(1001 as libc::c_int);
        }
        mpop(
            &mut lo,
            &mut hi,
            &mut d,
            stackLo.as_mut_ptr(),
            stackHi.as_mut_ptr(),
            stackD.as_mut_ptr(),
            &mut sp,
        );
        if hi - lo < MAIN_QSORT_SMALL_THRESH || d > MAIN_QSORT_DEPTH_THRESH {
            mainSimpleSort(ptr, block, quadrant, nblock, lo, hi, d, budget);
            if *budget < 0 as libc::c_int {
                return;
            }
        } else {
            med = mmed3(
                *block
                    .offset(
                        (*ptr.offset(lo as isize)).wrapping_add(d as libc::c_uint)
                            as isize,
                    ),
                *block
                    .offset(
                        (*ptr.offset(hi as isize)).wrapping_add(d as libc::c_uint)
                            as isize,
                    ),
                *block
                    .offset(
                        (*ptr.offset((lo + hi >> 1 as libc::c_int) as isize))
                            .wrapping_add(d as libc::c_uint) as isize,
                    ),
            ) as Int32;
            ltLo = lo;
            unLo = ltLo;
            gtHi = hi;
            unHi = gtHi;
            while 1 as libc::c_int as Bool != 0 {
                while 1 as libc::c_int as Bool != 0 {
                    if unLo > unHi {
                        break;
                    }
                    n = *block
                        .offset(
                            (*ptr.offset(unLo as isize)).wrapping_add(d as libc::c_uint)
                                as isize,
                        ) as Int32 - med;
                    if n == 0 as libc::c_int {
                        mswap(
                            &mut *ptr.offset(unLo as isize),
                            &mut *ptr.offset(ltLo as isize),
                        );
                        ltLo += 1;
                        ltLo;
                        unLo += 1;
                        unLo;
                    } else {
                        if n > 0 as libc::c_int {
                            break;
                        }
                        unLo += 1;
                        unLo;
                    }
                }
                while 1 as libc::c_int as Bool != 0 {
                    if unLo > unHi {
                        break;
                    }
                    n = *block
                        .offset(
                            (*ptr.offset(unHi as isize)).wrapping_add(d as libc::c_uint)
                                as isize,
                        ) as Int32 - med;
                    if n == 0 as libc::c_int {
                        mswap(
                            &mut *ptr.offset(unHi as isize),
                            &mut *ptr.offset(gtHi as isize),
                        );
                        gtHi -= 1;
                        gtHi;
                        unHi -= 1;
                        unHi;
                    } else {
                        if n < 0 as libc::c_int {
                            break;
                        }
                        unHi -= 1;
                        unHi;
                    }
                }
                if unLo > unHi {
                    break;
                }
                mswap(&mut *ptr.offset(unLo as isize), &mut *ptr.offset(unHi as isize));
                unLo += 1;
                unLo;
                unHi -= 1;
                unHi;
            }
            if gtHi < ltLo {
                mpush(
                    lo,
                    hi,
                    d + 1 as libc::c_int,
                    stackLo.as_mut_ptr(),
                    stackHi.as_mut_ptr(),
                    stackD.as_mut_ptr(),
                    &mut sp,
                );
            } else {
                n = mmin(ltLo - lo, unLo - ltLo);
                mvswap(lo, unLo - n, n, ptr);
                m = mmin(hi - gtHi, gtHi - unHi);
                mvswap(unLo, hi - m + 1 as libc::c_int, m, ptr);
                n = lo + unLo - ltLo - 1 as libc::c_int;
                m = hi - (gtHi - unHi) + 1 as libc::c_int;
                nextLo[0 as libc::c_int as usize] = lo;
                nextHi[0 as libc::c_int as usize] = n;
                nextD[0 as libc::c_int as usize] = d;
                nextLo[1 as libc::c_int as usize] = m;
                nextHi[1 as libc::c_int as usize] = hi;
                nextD[1 as libc::c_int as usize] = d;
                nextLo[2 as libc::c_int as usize] = n + 1 as libc::c_int;
                nextHi[2 as libc::c_int as usize] = m - 1 as libc::c_int;
                nextD[2 as libc::c_int as usize] = d + 1 as libc::c_int;
                if mnextsize(0 as libc::c_int, nextHi.as_mut_ptr(), nextLo.as_mut_ptr())
                    < mnextsize(
                        1 as libc::c_int,
                        nextHi.as_mut_ptr(),
                        nextLo.as_mut_ptr(),
                    )
                {
                    mnextswap(
                        0 as libc::c_int,
                        1 as libc::c_int,
                        nextLo.as_mut_ptr(),
                        nextHi.as_mut_ptr(),
                        nextD.as_mut_ptr(),
                    );
                }
                if mnextsize(1 as libc::c_int, nextHi.as_mut_ptr(), nextLo.as_mut_ptr())
                    < mnextsize(
                        2 as libc::c_int,
                        nextHi.as_mut_ptr(),
                        nextLo.as_mut_ptr(),
                    )
                {
                    mnextswap(
                        1 as libc::c_int,
                        2 as libc::c_int,
                        nextLo.as_mut_ptr(),
                        nextHi.as_mut_ptr(),
                        nextD.as_mut_ptr(),
                    );
                }
                if mnextsize(0 as libc::c_int, nextHi.as_mut_ptr(), nextLo.as_mut_ptr())
                    < mnextsize(
                        1 as libc::c_int,
                        nextHi.as_mut_ptr(),
                        nextLo.as_mut_ptr(),
                    )
                {
                    mnextswap(
                        0 as libc::c_int,
                        1 as libc::c_int,
                        nextLo.as_mut_ptr(),
                        nextHi.as_mut_ptr(),
                        nextD.as_mut_ptr(),
                    );
                }
                mpush(
                    nextLo[0 as libc::c_int as usize],
                    nextHi[0 as libc::c_int as usize],
                    nextD[0 as libc::c_int as usize],
                    stackLo.as_mut_ptr(),
                    stackHi.as_mut_ptr(),
                    stackD.as_mut_ptr(),
                    &mut sp,
                );
                mpush(
                    nextLo[1 as libc::c_int as usize],
                    nextHi[1 as libc::c_int as usize],
                    nextD[1 as libc::c_int as usize],
                    stackLo.as_mut_ptr(),
                    stackHi.as_mut_ptr(),
                    stackD.as_mut_ptr(),
                    &mut sp,
                );
                mpush(
                    nextLo[2 as libc::c_int as usize],
                    nextHi[2 as libc::c_int as usize],
                    nextD[2 as libc::c_int as usize],
                    stackLo.as_mut_ptr(),
                    stackHi.as_mut_ptr(),
                    stackD.as_mut_ptr(),
                    &mut sp,
                );
            }
        }
    }
}
#[inline]
unsafe extern "C" fn BIGFREQ(mut b: Int32, mut ftab: *mut UInt32) -> Int32 {
    return (*ftab.offset(((b + 1 as libc::c_int) << 8 as libc::c_int) as isize))
        .wrapping_sub(*ftab.offset((b << 8 as libc::c_int) as isize)) as Int32;
}
#[no_mangle]
pub static mut SETMASK: Int32 = (1 as libc::c_int) << 21 as libc::c_int;
#[no_mangle]
pub static mut CLEARMASK: Int32 = unsafe { !SETMASK };
unsafe extern "C" fn mainSort(
    mut ptr: *mut UInt32,
    mut block: *mut UChar,
    mut quadrant: *mut UInt16,
    mut ftab: *mut UInt32,
    mut nblock: Int32,
    mut verb: Int32,
    mut budget: *mut Int32,
) {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut k: Int32 = 0;
    let mut ss: Int32 = 0;
    let mut sb: Int32 = 0;
    let mut runningOrder: [Int32; 256] = [0; 256];
    let mut bigDone: [Bool; 256] = [0; 256];
    let mut copyStart: [Int32; 256] = [0; 256];
    let mut copyEnd: [Int32; 256] = [0; 256];
    let mut c1: UChar = 0;
    let mut numQSorted: Int32 = 0;
    let mut s: UInt16 = 0;
    if verb >= 4 as libc::c_int {
        fprintf(
            stderr,
            b"        main sort initialise ...\n\0" as *const u8 as *const libc::c_char,
        );
    }
    i = 65536 as libc::c_int;
    while i >= 0 as libc::c_int {
        *ftab.offset(i as isize) = 0 as libc::c_int as UInt32;
        i -= 1;
        i;
    }
    j = (*block.offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int;
    i = nblock - 1 as libc::c_int;
    while i >= 3 as libc::c_int {
        *quadrant.offset(i as isize) = 0 as libc::c_int as UInt16;
        j = j >> 8 as libc::c_int
            | (*block.offset(i as isize) as UInt16 as libc::c_int) << 8 as libc::c_int;
        let ref mut fresh2 = *ftab.offset(j as isize);
        *fresh2 = (*fresh2).wrapping_add(1);
        *fresh2;
        *quadrant.offset((i - 1 as libc::c_int) as isize) = 0 as libc::c_int as UInt16;
        j = j >> 8 as libc::c_int
            | (*block.offset((i - 1 as libc::c_int) as isize) as UInt16 as libc::c_int)
                << 8 as libc::c_int;
        let ref mut fresh3 = *ftab.offset(j as isize);
        *fresh3 = (*fresh3).wrapping_add(1);
        *fresh3;
        *quadrant.offset((i - 2 as libc::c_int) as isize) = 0 as libc::c_int as UInt16;
        j = j >> 8 as libc::c_int
            | (*block.offset((i - 2 as libc::c_int) as isize) as UInt16 as libc::c_int)
                << 8 as libc::c_int;
        let ref mut fresh4 = *ftab.offset(j as isize);
        *fresh4 = (*fresh4).wrapping_add(1);
        *fresh4;
        *quadrant.offset((i - 3 as libc::c_int) as isize) = 0 as libc::c_int as UInt16;
        j = j >> 8 as libc::c_int
            | (*block.offset((i - 3 as libc::c_int) as isize) as UInt16 as libc::c_int)
                << 8 as libc::c_int;
        let ref mut fresh5 = *ftab.offset(j as isize);
        *fresh5 = (*fresh5).wrapping_add(1);
        *fresh5;
        i -= 4 as libc::c_int;
    }
    while i >= 0 as libc::c_int {
        *quadrant.offset(i as isize) = 0 as libc::c_int as UInt16;
        j = j >> 8 as libc::c_int
            | (*block.offset(i as isize) as UInt16 as libc::c_int) << 8 as libc::c_int;
        let ref mut fresh6 = *ftab.offset(j as isize);
        *fresh6 = (*fresh6).wrapping_add(1);
        *fresh6;
        i -= 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int + 12 as libc::c_int + 18 as libc::c_int + 2 as libc::c_int
    {
        *block.offset((nblock + i) as isize) = *block.offset(i as isize);
        *quadrant.offset((nblock + i) as isize) = 0 as libc::c_int as UInt16;
        i += 1;
        i;
    }
    if verb >= 4 as libc::c_int {
        fprintf(
            stderr,
            b"        bucket sorting ...\n\0" as *const u8 as *const libc::c_char,
        );
    }
    i = 1 as libc::c_int;
    while i <= 65536 as libc::c_int {
        let ref mut fresh7 = *ftab.offset(i as isize);
        *fresh7 = (*fresh7 as libc::c_uint)
            .wrapping_add(*ftab.offset((i - 1 as libc::c_int) as isize)) as UInt32
            as UInt32;
        i += 1;
        i;
    }
    s = ((*block.offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
        as UInt16;
    i = nblock - 1 as libc::c_int;
    while i >= 3 as libc::c_int {
        s = (s as libc::c_int >> 8 as libc::c_int
            | (*block.offset(i as isize) as libc::c_int) << 8 as libc::c_int) as UInt16;
        j = (*ftab.offset(s as isize)).wrapping_sub(1 as libc::c_int as libc::c_uint)
            as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = i as UInt32;
        s = (s as libc::c_int >> 8 as libc::c_int
            | (*block.offset((i - 1 as libc::c_int) as isize) as libc::c_int)
                << 8 as libc::c_int) as UInt16;
        j = (*ftab.offset(s as isize)).wrapping_sub(1 as libc::c_int as libc::c_uint)
            as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = (i - 1 as libc::c_int) as UInt32;
        s = (s as libc::c_int >> 8 as libc::c_int
            | (*block.offset((i - 2 as libc::c_int) as isize) as libc::c_int)
                << 8 as libc::c_int) as UInt16;
        j = (*ftab.offset(s as isize)).wrapping_sub(1 as libc::c_int as libc::c_uint)
            as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = (i - 2 as libc::c_int) as UInt32;
        s = (s as libc::c_int >> 8 as libc::c_int
            | (*block.offset((i - 3 as libc::c_int) as isize) as libc::c_int)
                << 8 as libc::c_int) as UInt16;
        j = (*ftab.offset(s as isize)).wrapping_sub(1 as libc::c_int as libc::c_uint)
            as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = (i - 3 as libc::c_int) as UInt32;
        i -= 4 as libc::c_int;
    }
    while i >= 0 as libc::c_int {
        s = (s as libc::c_int >> 8 as libc::c_int
            | (*block.offset(i as isize) as libc::c_int) << 8 as libc::c_int) as UInt16;
        j = (*ftab.offset(s as isize)).wrapping_sub(1 as libc::c_int as libc::c_uint)
            as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = i as UInt32;
        i -= 1;
        i;
    }
    i = 0 as libc::c_int;
    while i <= 255 as libc::c_int {
        bigDone[i as usize] = 0 as libc::c_int as Bool;
        runningOrder[i as usize] = i;
        i += 1;
        i;
    }
    let mut vv: Int32 = 0;
    let mut h: Int32 = 1 as libc::c_int;
    loop {
        h = 3 as libc::c_int * h + 1 as libc::c_int;
        if !(h <= 256 as libc::c_int) {
            break;
        }
    }
    loop {
        h = h / 3 as libc::c_int;
        i = h;
        while i <= 255 as libc::c_int {
            vv = runningOrder[i as usize];
            j = i;
            while BIGFREQ(runningOrder[(j - h) as usize], ftab) > BIGFREQ(vv, ftab) {
                runningOrder[j as usize] = runningOrder[(j - h) as usize];
                j = j - h;
                if j <= h - 1 as libc::c_int {
                    break;
                }
            }
            runningOrder[j as usize] = vv;
            i += 1;
            i;
        }
        if !(h != 1 as libc::c_int) {
            break;
        }
    }
    numQSorted = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i <= 255 as libc::c_int {
        ss = runningOrder[i as usize];
        j = 0 as libc::c_int;
        while j <= 255 as libc::c_int {
            if j != ss {
                sb = (ss << 8 as libc::c_int) + j;
                if *ftab.offset(sb as isize) & SETMASK as libc::c_uint == 0 {
                    let mut lo: Int32 = (*ftab.offset(sb as isize)
                        & CLEARMASK as libc::c_uint) as Int32;
                    let mut hi: Int32 = (*ftab.offset((sb + 1 as libc::c_int) as isize)
                        & CLEARMASK as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as Int32;
                    if hi > lo {
                        if verb >= 4 as libc::c_int {
                            fprintf(
                                stderr,
                                b"        qsort [0x%x, 0x%x]   done %d   this %d\n\0"
                                    as *const u8 as *const libc::c_char,
                                ss,
                                j,
                                numQSorted,
                                hi - lo + 1 as libc::c_int,
                            );
                        }
                        mainQSort3(
                            ptr,
                            block,
                            quadrant,
                            nblock,
                            lo,
                            hi,
                            2 as libc::c_int,
                            budget,
                        );
                        numQSorted += hi - lo + 1 as libc::c_int;
                        if *budget < 0 as libc::c_int {
                            return;
                        }
                    }
                }
                let ref mut fresh8 = *ftab.offset(sb as isize);
                *fresh8 |= SETMASK as libc::c_uint;
            }
            j += 1;
            j;
        }
        if bigDone[ss as usize] != 0 {
            BZ2_bz__AssertH__fail(1006 as libc::c_int);
        }
        j = 0 as libc::c_int;
        while j <= 255 as libc::c_int {
            copyStart[j
                as usize] = (*ftab.offset(((j << 8 as libc::c_int) + ss) as isize)
                & CLEARMASK as libc::c_uint) as Int32;
            copyEnd[j
                as usize] = (*ftab
                .offset(((j << 8 as libc::c_int) + ss + 1 as libc::c_int) as isize)
                & CLEARMASK as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as Int32;
            j += 1;
            j;
        }
        j = (*ftab.offset((ss << 8 as libc::c_int) as isize) & CLEARMASK as libc::c_uint)
            as Int32;
        while j < copyStart[ss as usize] {
            k = (*ptr.offset(j as isize)).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as Int32;
            if k < 0 as libc::c_int {
                k += nblock;
            }
            c1 = *block.offset(k as isize);
            if bigDone[c1 as usize] == 0 {
                let fresh9 = copyStart[c1 as usize];
                copyStart[c1 as usize] = copyStart[c1 as usize] + 1;
                *ptr.offset(fresh9 as isize) = k as UInt32;
            }
            j += 1;
            j;
        }
        j = (*ftab.offset(((ss + 1 as libc::c_int) << 8 as libc::c_int) as isize)
            & CLEARMASK as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as Int32;
        while j > copyEnd[ss as usize] {
            k = (*ptr.offset(j as isize)).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as Int32;
            if k < 0 as libc::c_int {
                k += nblock;
            }
            c1 = *block.offset(k as isize);
            if bigDone[c1 as usize] == 0 {
                let fresh10 = copyEnd[c1 as usize];
                copyEnd[c1 as usize] = copyEnd[c1 as usize] - 1;
                *ptr.offset(fresh10 as isize) = k as UInt32;
            }
            j -= 1;
            j;
        }
        if !(copyStart[ss as usize] - 1 as libc::c_int == copyEnd[ss as usize]
            || copyStart[ss as usize] == 0 as libc::c_int
                && copyEnd[ss as usize] == nblock - 1 as libc::c_int)
        {
            BZ2_bz__AssertH__fail(1007 as libc::c_int);
        }
        j = 0 as libc::c_int;
        while j <= 255 as libc::c_int {
            let ref mut fresh11 = *ftab.offset(((j << 8 as libc::c_int) + ss) as isize);
            *fresh11 |= SETMASK as libc::c_uint;
            j += 1;
            j;
        }
        bigDone[ss as usize] = 1 as libc::c_int as Bool;
        if i < 255 as libc::c_int {
            let mut bbStart: Int32 = (*ftab.offset((ss << 8 as libc::c_int) as isize)
                & CLEARMASK as libc::c_uint) as Int32;
            let mut bbSize: Int32 = (*ftab
                .offset(((ss + 1 as libc::c_int) << 8 as libc::c_int) as isize)
                & CLEARMASK as libc::c_uint)
                .wrapping_sub(bbStart as libc::c_uint) as Int32;
            let mut shifts: Int32 = 0 as libc::c_int;
            while bbSize >> shifts > 65534 as libc::c_int {
                shifts += 1;
                shifts;
            }
            j = bbSize - 1 as libc::c_int;
            while j >= 0 as libc::c_int {
                let mut a2update: Int32 = *ptr.offset((bbStart + j) as isize) as Int32;
                let mut qVal: UInt16 = (j >> shifts) as UInt16;
                *quadrant.offset(a2update as isize) = qVal;
                if a2update
                    < 2 as libc::c_int + 12 as libc::c_int + 18 as libc::c_int
                        + 2 as libc::c_int
                {
                    *quadrant.offset((a2update + nblock) as isize) = qVal;
                }
                j -= 1;
                j;
            }
            if !(bbSize - 1 as libc::c_int >> shifts <= 65535 as libc::c_int) {
                BZ2_bz__AssertH__fail(1002 as libc::c_int);
            }
        }
        i += 1;
        i;
    }
    if verb >= 4 as libc::c_int {
        fprintf(
            stderr,
            b"        %d pointers, %d sorted, %d scanned\n\0" as *const u8
                as *const libc::c_char,
            nblock,
            numQSorted,
            nblock - numQSorted,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_blockSort(mut s: *mut EState) {
    let mut ptr: *mut UInt32 = (*s).ptr;
    let mut block: *mut UChar = (*s).block;
    let mut ftab: *mut UInt32 = (*s).ftab;
    let mut nblock: Int32 = (*s).nblock;
    let mut verb: Int32 = (*s).verbosity;
    let mut wfact: Int32 = (*s).workFactor;
    let mut quadrant: *mut UInt16 = 0 as *mut UInt16;
    let mut budget: Int32 = 0;
    let mut budgetInit: Int32 = 0;
    let mut i: Int32 = 0;
    if nblock < 10000 as libc::c_int {
        fallbackSort((*s).arr1, (*s).arr2, ftab, nblock, verb);
    } else {
        i = nblock
            + (2 as libc::c_int + 12 as libc::c_int + 18 as libc::c_int
                + 2 as libc::c_int);
        if i & 1 as libc::c_int != 0 {
            i += 1;
            i;
        }
        quadrant = &mut *block.offset(i as isize) as *mut UChar as *mut UInt16;
        if wfact < 1 as libc::c_int {
            wfact = 1 as libc::c_int;
        }
        if wfact > 100 as libc::c_int {
            wfact = 100 as libc::c_int;
        }
        budgetInit = nblock * ((wfact - 1 as libc::c_int) / 3 as libc::c_int);
        budget = budgetInit;
        mainSort(ptr, block, quadrant, ftab, nblock, verb, &mut budget);
        if verb >= 3 as libc::c_int {
            fprintf(
                stderr,
                b"      %d work, %d block, ratio %5.2f\n\0" as *const u8
                    as *const libc::c_char,
                budgetInit - budget,
                nblock,
                ((budgetInit - budget) as libc::c_float
                    / (if nblock == 0 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        nblock
                    }) as libc::c_float) as libc::c_double,
            );
        }
        if budget < 0 as libc::c_int {
            if verb >= 2 as libc::c_int {
                fprintf(
                    stderr,
                    b"    too repetitive; using fallback sorting algorithm\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            fallbackSort((*s).arr1, (*s).arr2, ftab, nblock, verb);
        }
    }
    (*s).origPtr = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*s).nblock {
        if *ptr.offset(i as isize) == 0 as libc::c_int as libc::c_uint {
            (*s).origPtr = i;
            break;
        } else {
            i += 1;
            i;
        }
    }
    if !((*s).origPtr != -(1 as libc::c_int)) {
        BZ2_bz__AssertH__fail(1003 as libc::c_int);
    }
}
