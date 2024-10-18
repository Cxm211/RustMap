//use libc;
extern "C" {
    pub type __sFILEX;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    static mut __stderrp: *mut FILE;
    fn BZ2_bz__AssertH__fail(errcode: i32);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut u8,
    pub _size: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut u8,
    pub _r: i32,
    pub _w: i32,
    pub _flags: i16,
    pub _file: i16,
    pub _bf: __sbuf,
    pub _lbfsize: i32,
    pub _cookie: *mut std::os::raw::c_void,
    pub _close: Option<unsafe extern "C" fn(*mut std::os::raw::c_void) -> i32>,
    pub _read: Option<unsafe extern "C" fn(*mut std::os::raw::c_void, *mut i8, i32) -> i32>,
    pub _seek: Option<unsafe extern "C" fn(*mut std::os::raw::c_void, i64, i32) -> i64>,
    pub _write: Option<unsafe extern "C" fn(*mut std::os::raw::c_void, *const i8, i32) -> i32>,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: i32,
    pub _ubuf: [u8; 3],
    pub _nbuf: [u8; 1],
    pub _lb: __sbuf,
    pub _blksize: i32,
    pub _offset: i64,
}
pub type FILE = __sFILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bz_stream {
    pub next_in: *mut i8,
    pub avail_in: u32,
    pub total_in_lo32: u32,
    pub total_in_hi32: u32,
    pub next_out: *mut i8,
    pub avail_out: u32,
    pub total_out_lo32: u32,
    pub total_out_hi32: u32,
    pub state: *mut std::os::raw::c_void,
    pub bzalloc: Option<unsafe extern "C" fn(*mut std::os::raw::c_void, i32, i32) -> *mut std::os::raw::c_void>,
    pub bzfree: Option<unsafe extern "C" fn(*mut std::os::raw::c_void, *mut std::os::raw::c_void) -> ()>,
    pub opaque: *mut std::os::raw::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EState {
    pub strm: *mut bz_stream,
    pub mode: i32,
    pub state: i32,
    pub avail_in_expect: u32,
    pub arr1: *mut u32,
    pub arr2: *mut u32,
    pub ftab: *mut u32,
    pub origPtr: i32,
    pub ptr: *mut u32,
    pub block: *mut u8,
    pub mtfv: *mut u16,
    pub zbits: *mut u8,
    pub workFactor: i32,
    pub state_in_ch: u32,
    pub state_in_len: i32,
    pub rNToGo: i32,
    pub rTPos: i32,
    pub nblock: i32,
    pub nblockMAX: i32,
    pub numZ: i32,
    pub state_out_pos: i32,
    pub nInUse: i32,
    pub inUse: [u8; 256],
    pub unseqToSeq: [u8; 256],
    pub bsBuff: u32,
    pub bsLive: i32,
    pub blockCRC: u32,
    pub combinedCRC: u32,
    pub verbosity: i32,
    pub blockNo: i32,
    pub blockSize100k: i32,
    pub nMTF: i32,
    pub mtfFreq: [i32; 258],
    pub selector: [u8; 18002],
    pub selectorMtf: [u8; 18002],
    pub len: [[u8; 258]; 6],
    pub code: [[i32; 258]; 6],
    pub rfreq: [[i32; 258]; 6],
    pub len_pack: [[u32; 4]; 258],
}
#[inline]
extern "C" fn fallbackSimpleSort(
    mut fmap: *mut u32,
    mut eclass: *mut u32,
    mut lo: i32,
    mut hi: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut tmp: i32 = 0;
    let mut ec_tmp: u32 = 0;
    if lo == hi {
        return;
    }
    unsafe {
        if hi - lo > 3 {
            i = hi - 4;
            while i >= lo {
                tmp = *fmap.offset(i as isize) as i32;
                ec_tmp = *eclass.offset(tmp as isize);
                j = i + 4;
                while j <= hi && ec_tmp > *eclass.offset(*fmap.offset(j as isize) as isize) {
                    *fmap.offset((j - 4i32) as isize) = *fmap.offset(j as isize);
                    j += 4;
                }
                *fmap.offset((j - 4i32) as isize) = tmp as u32;
                i -= 1;
                i;
            }
        }
    }
    i = hi - 1;
    unsafe {
        while i >= lo {
            tmp = *fmap.offset(i as isize) as i32;
            ec_tmp = *eclass.offset(tmp as isize);
            j = i + 1;
            while j <= hi && ec_tmp > *eclass.offset(*fmap.offset(j as isize) as isize) {
                *fmap.offset((j - 1i32) as isize) = *fmap.offset(j as isize);
                j += 1;
                j;
            }
            *fmap.offset((j - 1i32) as isize) = tmp as u32;
            i -= 1;
            i;
        }
    }
}

extern "C" fn fallbackQSort3(
    mut fmap: *mut u32,
    mut eclass: *mut u32,
    mut loSt: i32,
    mut hiSt: i32,
) {
    let mut unLo: i32 = 0;
    let mut unHi: i32 = 0;
    let mut ltLo: i32 = 0;
    let mut gtHi: i32 = 0;
    let mut n: i32 = 0;
    let mut m: i32 = 0;
    let mut sp: i32 = 0;
    let mut lo: i32 = 0;
    let mut hi: i32 = 0;
    let mut med: u32 = 0;
    let mut r: u32 = 0;
    let mut r3: u32 = 0;
    let mut stackLo: [i32; 100] = [0; 100];
    let mut stackHi: [i32; 100] = [0; 100];
    r = 0;
    sp = 0;
    stackLo[sp as usize] = loSt;
    stackHi[sp as usize] = hiSt;
    sp += 1;
    sp;
    unsafe {
        while sp > 0 {
            if !(sp < 100 - 1) {
                BZ2_bz__AssertH__fail(1004);
            }
            sp -= 1;
            sp;
            lo = stackLo[sp as usize];
            hi = stackHi[sp as usize];
            if hi - lo < 10 {
                fallbackSimpleSort(fmap, eclass, lo, hi);
            } else {
                r = (r * 7621u32).wrapping_add(1) % 32768;
                r3 = r % 3;
                if r3 == 0 {
                    med = *eclass.offset(*fmap.offset(lo as isize) as isize);
                } else if r3 == 1 {
                    med = *eclass.offset(*fmap.offset((lo + hi >> 1i32) as isize) as isize);
                } else {
                    med = *eclass.offset(*fmap.offset(hi as isize) as isize);
                }
                ltLo = lo;
                unLo = ltLo;
                gtHi = hi;
                unHi = gtHi;
                loop {
                    while !(unLo > unHi) {
                        n = *eclass.offset(*fmap.offset(unLo as isize) as isize) as i32
                            - med as i32;
                        if n == 0 {
                            let mut zztmp: i32 = *fmap.offset(unLo as isize) as i32;
                            *fmap.offset(unLo as isize) = *fmap.offset(ltLo as isize);
                            *fmap.offset(ltLo as isize) = zztmp as u32;
                            ltLo += 1;
                            ltLo;
                            unLo += 1;
                            unLo;
                        } else {
                            if n > 0 {
                                break;
                            }
                            unLo += 1;
                            unLo;
                        }
                    }
                    while !(unLo > unHi) {
                        n = *eclass.offset(*fmap.offset(unHi as isize) as isize) as i32
                            - med as i32;
                        if n == 0 {
                            let mut zztmp_0: i32 = *fmap.offset(unHi as isize) as i32;
                            *fmap.offset(unHi as isize) = *fmap.offset(gtHi as isize);
                            *fmap.offset(gtHi as isize) = zztmp_0 as u32;
                            gtHi -= 1;
                            gtHi;
                            unHi -= 1;
                            unHi;
                        } else {
                            if n < 0 {
                                break;
                            }
                            unHi -= 1;
                            unHi;
                        }
                    }
                    if unLo > unHi {
                        break;
                    }
                    let mut zztmp_1: i32 = *fmap.offset(unLo as isize) as i32;
                    *fmap.offset(unLo as isize) = *fmap.offset(unHi as isize);
                    *fmap.offset(unHi as isize) = zztmp_1 as u32;
                    unLo += 1;
                    unLo;
                    unHi -= 1;
                    unHi;
                }
                if gtHi < ltLo {
                    continue;
                }
                n = if ltLo - lo < unLo - ltLo {
                    ltLo - lo
                } else {
                    unLo - ltLo
                };
                let mut yyp1: i32 = lo;
                let mut yyp2: i32 = unLo - n;
                let mut yyn: i32 = n;
                while yyn > 0 {
                    let mut zztmp_2: i32 = *fmap.offset(yyp1 as isize) as i32;
                    *fmap.offset(yyp1 as isize) = *fmap.offset(yyp2 as isize);
                    *fmap.offset(yyp2 as isize) = zztmp_2 as u32;
                    yyp1 += 1;
                    yyp1;
                    yyp2 += 1;
                    yyp2;
                    yyn -= 1;
                    yyn;
                }
                m = if hi - gtHi < gtHi - unHi {
                    hi - gtHi
                } else {
                    gtHi - unHi
                };
                let mut yyp1_0: i32 = unLo;
                let mut yyp2_0: i32 = hi - m + 1;
                let mut yyn_0: i32 = m;
                while yyn_0 > 0 {
                    let mut zztmp_3: i32 = *fmap.offset(yyp1_0 as isize) as i32;
                    *fmap.offset(yyp1_0 as isize) = *fmap.offset(yyp2_0 as isize);
                    *fmap.offset(yyp2_0 as isize) = zztmp_3 as u32;
                    yyp1_0 += 1;
                    yyp1_0;
                    yyp2_0 += 1;
                    yyp2_0;
                    yyn_0 -= 1;
                    yyn_0;
                }
                n = lo + unLo - ltLo - 1;
                m = hi - (gtHi - unHi) + 1;
                if n - lo > hi - m {
                    stackLo[sp as usize] = lo;
                    stackHi[sp as usize] = n;
                    sp += 1;
                    sp;
                    stackLo[sp as usize] = m;
                    stackHi[sp as usize] = hi;
                    sp += 1;
                    sp;
                } else {
                    stackLo[sp as usize] = m;
                    stackHi[sp as usize] = hi;
                    sp += 1;
                    sp;
                    stackLo[sp as usize] = lo;
                    stackHi[sp as usize] = n;
                    sp += 1;
                    sp;
                }
            }
        }
    }
}

extern "C" fn fallbackSort(
    mut fmap: *mut u32,
    mut eclass: *mut u32,
    mut bhtab: *mut u32,
    mut nblock: i32,
    mut verb: i32,
) {
    let mut ftab: [i32; 257] = [0; 257];
    let mut ftabCopy: [i32; 256] = [0; 256];
    let mut H: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut r: i32 = 0;
    let mut cc: i32 = 0;
    let mut cc1: i32 = 0;
    let mut nNotDone: i32 = 0;
    let mut nBhtab: i32 = 0;
    let mut eclass8: *mut u8 = eclass as *mut u8;
    unsafe {
        if verb >= 4 {
            fprintf(
                __stderrp,
                b"        bucket sorting ...\n\0" as *const u8 as *const i8,
            );
        }
    }
    i = 0;
    while i < 257 {
        ftab[i as usize] = 0;
        i += 1;
        i;
    }
    i = 0;
    unsafe {
        while i < nblock {
            ftab[*eclass8.offset(i as isize) as usize] += 1;
            ftab[*eclass8.offset(i as isize) as usize];
            i += 1;
            i;
        }
    }
    i = 0;
    while i < 256 {
        ftabCopy[i as usize] = ftab[i as usize];
        i += 1;
        i;
    }
    i = 1;
    while i < 257 {
        ftab[i as usize] += ftab[(i - 1i32) as usize];
        i += 1;
        i;
    }
    i = 0;
    unsafe {
        while i < nblock {
            j = *eclass8.offset(i as isize) as i32;
            k = ftab[j as usize] - 1;
            ftab[j as usize] = k;
            *fmap.offset(k as isize) = i as u32;
            i += 1;
            i;
        }
    }
    nBhtab = 2 + nblock / 32;
    i = 0;
    unsafe {
        while i < nBhtab {
            *bhtab.offset(i as isize) = 0;
            i += 1;
            i;
        }
    }
    i = 0;
    unsafe {
        while i < 256 {
            *bhtab.offset((ftab[i as usize] >> 5i32) as isize) |= 1 << (ftab[i as usize] & 31);
            i += 1;
            i;
        }
    }
    i = 0;
    unsafe {
        while i < 32 {
            *bhtab.offset((nblock + 2 * i >> 5i32) as isize) |= 1 << (nblock + 2 * i & 31);
            *bhtab.offset((nblock + 2 * i + 1 >> 5i32) as isize) &=
                !(1 << (nblock + 2 * i + 1 & 31));
            i += 1;
            i;
        }
    }
    H = 1;
    unsafe {
        loop {
            if verb >= 4 {
                fprintf(
                    __stderrp,
                    b"        depth %6d has \0" as *const u8 as *const i8,
                    H,
                );
            }
            j = 0;
            i = 0;
            while i < nblock {
                if *bhtab.offset((i >> 5i32) as isize) & 1 << (i & 31) != 0 {
                    j = i;
                }
                k = (*fmap.offset(i as isize)).wrapping_sub(H as u32) as i32;
                if k < 0 {
                    k += nblock;
                };
                *eclass.offset(k as isize) = j as u32;
                i += 1;
                i;
            }
            nNotDone = 0;
            r = -1;
            loop {
                k = r + 1;
                while *bhtab.offset((k >> 5i32) as isize) & 1 << (k & 31) != 0 && k & 0x1f != 0 {
                    k += 1;
                    k;
                }
                if *bhtab.offset((k >> 5i32) as isize) & 1 << (k & 31) != 0 {
                    while *bhtab.offset((k >> 5i32) as isize) == 0xffffffff {
                        k += 32;
                    }
                    while *bhtab.offset((k >> 5i32) as isize) & 1 << (k & 31) != 0 {
                        k += 1;
                        k;
                    }
                }
                l = k - 1;
                if l >= nblock {
                    break;
                }
                while *bhtab.offset((k >> 5i32) as isize) & 1 << (k & 31) == 0 && k & 0x1f != 0 {
                    k += 1;
                    k;
                }
                if *bhtab.offset((k >> 5i32) as isize) & 1 << (k & 31) == 0 {
                    while *bhtab.offset((k >> 5i32) as isize) == 0 {
                        k += 32;
                    }
                    while *bhtab.offset((k >> 5i32) as isize) & 1 << (k & 31) == 0 {
                        k += 1;
                        k;
                    }
                }
                r = k - 1;
                if r >= nblock {
                    break;
                }
                if r > l {
                    nNotDone += r - l + 1;
                    fallbackQSort3(fmap, eclass, l, r);
                    cc = -1;
                    i = l;
                    while i <= r {
                        cc1 = *eclass.offset(*fmap.offset(i as isize) as isize) as i32;
                        if cc != cc1 {
                            *bhtab.offset((i >> 5i32) as isize) |= 1 << (i & 31);
                            cc = cc1;
                        }
                        i += 1;
                        i;
                    }
                }
            }
            if verb >= 4 {
                fprintf(
                    __stderrp,
                    b"%6d unresolved strings\n\0" as *const u8 as *const i8,
                    nNotDone,
                );
            }
            H *= 2;
            if H > nblock || nNotDone == 0 {
                break;
            }
        }
        if verb >= 4 {
            fprintf(
                __stderrp,
                b"        reconstructing block ...\n\0" as *const u8 as *const i8,
            );
        }
    }
    j = 0;
    i = 0;
    unsafe {
        while i < nblock {
            while ftabCopy[j as usize] == 0 {
                j += 1;
                j;
            }
            ftabCopy[j as usize] -= 1;
            ftabCopy[j as usize];
            *eclass8.offset(*fmap.offset(i as isize) as isize) = j as u8;
            i += 1;
            i;
        }
        if !(j < 256) {
            BZ2_bz__AssertH__fail(1005);
        }
    }
}

#[inline]
extern "C" fn mainGtU(
    mut i1: u32,
    mut i2: u32,
    mut block: *mut u8,
    mut quadrant: *mut u16,
    mut nblock: u32,
    mut budget: *mut i32,
) -> u8 {
    let mut k: i32 = 0;
    let mut c1: u8 = 0;
    let mut c2: u8 = 0;
    let mut s1: u16 = 0;
    let mut s2: u16 = 0;
    unsafe {
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
    }
    if c1 as i32 != c2 as i32 {
        return (c1 as i32 > c2 as i32) as u8;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    unsafe {
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
    }
    if c1 as i32 != c2 as i32 {
        return (c1 as i32 > c2 as i32) as u8;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    unsafe {
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
    }
    if c1 as i32 != c2 as i32 {
        return (c1 as i32 > c2 as i32) as u8;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    unsafe {
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
    }
    if c1 as i32 != c2 as i32 {
        return (c1 as i32 > c2 as i32) as u8;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    unsafe {
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
    }
    if c1 as i32 != c2 as i32 {
        return (c1 as i32 > c2 as i32) as u8;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    unsafe {
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
    }
    if c1 as i32 != c2 as i32 {
        return (c1 as i32 > c2 as i32) as u8;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    unsafe {
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
    }
    if c1 as i32 != c2 as i32 {
        return (c1 as i32 > c2 as i32) as u8;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    unsafe {
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
    }
    if c1 as i32 != c2 as i32 {
        return (c1 as i32 > c2 as i32) as u8;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    unsafe {
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
    }
    if c1 as i32 != c2 as i32 {
        return (c1 as i32 > c2 as i32) as u8;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    unsafe {
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
    }
    if c1 as i32 != c2 as i32 {
        return (c1 as i32 > c2 as i32) as u8;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    unsafe {
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
    }
    if c1 as i32 != c2 as i32 {
        return (c1 as i32 > c2 as i32) as u8;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    unsafe {
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
    }
    if c1 as i32 != c2 as i32 {
        return (c1 as i32 > c2 as i32) as u8;
    }
    i1 = i1.wrapping_add(1);
    i1;
    i2 = i2.wrapping_add(1);
    i2;
    k = nblock.wrapping_add(8) as i32;
    unsafe {
        loop {
            c1 = *block.offset(i1 as isize);
            c2 = *block.offset(i2 as isize);
            if c1 as i32 != c2 as i32 {
                return (c1 as i32 > c2 as i32) as u8;
            }
            s1 = *quadrant.offset(i1 as isize);
            s2 = *quadrant.offset(i2 as isize);
            if s1 as i32 != s2 as i32 {
                return (s1 as i32 > s2 as i32) as u8;
            }
            i1 = i1.wrapping_add(1);
            i1;
            i2 = i2.wrapping_add(1);
            i2;
            c1 = *block.offset(i1 as isize);
            c2 = *block.offset(i2 as isize);
            if c1 as i32 != c2 as i32 {
                return (c1 as i32 > c2 as i32) as u8;
            }
            s1 = *quadrant.offset(i1 as isize);
            s2 = *quadrant.offset(i2 as isize);
            if s1 as i32 != s2 as i32 {
                return (s1 as i32 > s2 as i32) as u8;
            }
            i1 = i1.wrapping_add(1);
            i1;
            i2 = i2.wrapping_add(1);
            i2;
            c1 = *block.offset(i1 as isize);
            c2 = *block.offset(i2 as isize);
            if c1 as i32 != c2 as i32 {
                return (c1 as i32 > c2 as i32) as u8;
            }
            s1 = *quadrant.offset(i1 as isize);
            s2 = *quadrant.offset(i2 as isize);
            if s1 as i32 != s2 as i32 {
                return (s1 as i32 > s2 as i32) as u8;
            }
            i1 = i1.wrapping_add(1);
            i1;
            i2 = i2.wrapping_add(1);
            i2;
            c1 = *block.offset(i1 as isize);
            c2 = *block.offset(i2 as isize);
            if c1 as i32 != c2 as i32 {
                return (c1 as i32 > c2 as i32) as u8;
            }
            s1 = *quadrant.offset(i1 as isize);
            s2 = *quadrant.offset(i2 as isize);
            if s1 as i32 != s2 as i32 {
                return (s1 as i32 > s2 as i32) as u8;
            }
            i1 = i1.wrapping_add(1);
            i1;
            i2 = i2.wrapping_add(1);
            i2;
            c1 = *block.offset(i1 as isize);
            c2 = *block.offset(i2 as isize);
            if c1 as i32 != c2 as i32 {
                return (c1 as i32 > c2 as i32) as u8;
            }
            s1 = *quadrant.offset(i1 as isize);
            s2 = *quadrant.offset(i2 as isize);
            if s1 as i32 != s2 as i32 {
                return (s1 as i32 > s2 as i32) as u8;
            }
            i1 = i1.wrapping_add(1);
            i1;
            i2 = i2.wrapping_add(1);
            i2;
            c1 = *block.offset(i1 as isize);
            c2 = *block.offset(i2 as isize);
            if c1 as i32 != c2 as i32 {
                return (c1 as i32 > c2 as i32) as u8;
            }
            s1 = *quadrant.offset(i1 as isize);
            s2 = *quadrant.offset(i2 as isize);
            if s1 as i32 != s2 as i32 {
                return (s1 as i32 > s2 as i32) as u8;
            }
            i1 = i1.wrapping_add(1);
            i1;
            i2 = i2.wrapping_add(1);
            i2;
            c1 = *block.offset(i1 as isize);
            c2 = *block.offset(i2 as isize);
            if c1 as i32 != c2 as i32 {
                return (c1 as i32 > c2 as i32) as u8;
            }
            s1 = *quadrant.offset(i1 as isize);
            s2 = *quadrant.offset(i2 as isize);
            if s1 as i32 != s2 as i32 {
                return (s1 as i32 > s2 as i32) as u8;
            }
            i1 = i1.wrapping_add(1);
            i1;
            i2 = i2.wrapping_add(1);
            i2;
            c1 = *block.offset(i1 as isize);
            c2 = *block.offset(i2 as isize);
            if c1 as i32 != c2 as i32 {
                return (c1 as i32 > c2 as i32) as u8;
            }
            s1 = *quadrant.offset(i1 as isize);
            s2 = *quadrant.offset(i2 as isize);
            if s1 as i32 != s2 as i32 {
                return (s1 as i32 > s2 as i32) as u8;
            }
            i1 = i1.wrapping_add(1);
            i1;
            i2 = i2.wrapping_add(1);
            i2;
            if i1 >= nblock {
                i1 = i1.wrapping_sub(nblock);
            }
            if i2 >= nblock {
                i2 = i2.wrapping_sub(nblock);
            }
            k -= 8;
            *budget -= 1;
            *budget;
            if !(k >= 0) {
                break;
            }
        }
    }
    return 0;
}

static mut incs: [i32; 14] = [
    1, 4, 13, 40, 121, 364, 1093, 3280, 9841, 29524, 88573, 265720, 797161, 2391484,
];
extern "C" fn mainSimpleSort(
    mut ptr: *mut u32,
    mut block: *mut u8,
    mut quadrant: *mut u16,
    mut nblock: i32,
    mut lo: i32,
    mut hi: i32,
    mut d: i32,
    mut budget: *mut i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut h: i32 = 0;
    let mut bigN: i32 = 0;
    let mut hp: i32 = 0;
    let mut v: u32 = 0;
    bigN = hi - lo + 1;
    if bigN < 2 {
        return;
    }
    hp = 0;
    unsafe {
        while incs[hp as usize] < bigN {
            hp += 1;
            hp;
        }
    }
    hp -= 1;
    hp;
    unsafe {
        while hp >= 0 {
            h = incs[hp as usize];
            i = lo + h;
            while 1 != 0 {
                if i > hi {
                    break;
                }
                v = *ptr.offset(i as isize);
                j = i;
                while mainGtU(
                    (*ptr.offset((j - h) as isize)).wrapping_add(d as u32),
                    v.wrapping_add(d as u32),
                    block,
                    quadrant,
                    nblock as u32,
                    budget,
                ) != 0
                {
                    *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                    j = j - h;
                    if j <= lo + h - 1 {
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
                    (*ptr.offset((j - h) as isize)).wrapping_add(d as u32),
                    v.wrapping_add(d as u32),
                    block,
                    quadrant,
                    nblock as u32,
                    budget,
                ) != 0
                {
                    *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                    j = j - h;
                    if j <= lo + h - 1 {
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
                    (*ptr.offset((j - h) as isize)).wrapping_add(d as u32),
                    v.wrapping_add(d as u32),
                    block,
                    quadrant,
                    nblock as u32,
                    budget,
                ) != 0
                {
                    *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                    j = j - h;
                    if j <= lo + h - 1 {
                        break;
                    }
                }
                *ptr.offset(j as isize) = v;
                i += 1;
                i;
                if *budget < 0 {
                    return;
                }
            }
            hp -= 1;
            hp;
        }
    }
}

#[inline]
extern "C" fn mmed3(mut a: u8, mut b: u8, mut c: u8) -> u8 {
    let mut t: u8 = 0;
    if a as i32 > b as i32 {
        t = a;
        a = b;
        b = t;
    }
    if b as i32 > c as i32 {
        b = c;
        if a as i32 > b as i32 {
            b = a;
        }
    }
    return b;
}

extern "C" fn mainQSort3(
    mut ptr: *mut u32,
    mut block: *mut u8,
    mut quadrant: *mut u16,
    mut nblock: i32,
    mut loSt: i32,
    mut hiSt: i32,
    mut dSt: i32,
    mut budget: *mut i32,
) {
    let mut unLo: i32 = 0;
    let mut unHi: i32 = 0;
    let mut ltLo: i32 = 0;
    let mut gtHi: i32 = 0;
    let mut n: i32 = 0;
    let mut m: i32 = 0;
    let mut med: i32 = 0;
    let mut sp: i32 = 0;
    let mut lo: i32 = 0;
    let mut hi: i32 = 0;
    let mut d: i32 = 0;
    let mut stackLo: [i32; 100] = [0; 100];
    let mut stackHi: [i32; 100] = [0; 100];
    let mut stackD: [i32; 100] = [0; 100];
    let mut nextLo: [i32; 3] = [0; 3];
    let mut nextHi: [i32; 3] = [0; 3];
    let mut nextD: [i32; 3] = [0; 3];
    sp = 0;
    stackLo[sp as usize] = loSt;
    stackHi[sp as usize] = hiSt;
    stackD[sp as usize] = dSt;
    sp += 1;
    sp;
    unsafe {
        while sp > 0 {
            if !(sp < 100 - 2) {
                BZ2_bz__AssertH__fail(1001);
            }
            sp -= 1;
            sp;
            lo = stackLo[sp as usize];
            hi = stackHi[sp as usize];
            d = stackD[sp as usize];
            if hi - lo < 20 || d > 2 + 12 {
                mainSimpleSort(ptr, block, quadrant, nblock, lo, hi, d, budget);
                if *budget < 0 {
                    return;
                }
            } else {
                med = mmed3(
                    *block.offset((*ptr.offset(lo as isize)).wrapping_add(d as u32) as isize),
                    *block.offset((*ptr.offset(hi as isize)).wrapping_add(d as u32) as isize),
                    *block.offset(
                        (*ptr.offset((lo + hi >> 1i32) as isize)).wrapping_add(d as u32) as isize,
                    ),
                ) as i32;
                ltLo = lo;
                unLo = ltLo;
                gtHi = hi;
                unHi = gtHi;
                while 1 != 0 {
                    while 1 != 0 {
                        if unLo > unHi {
                            break;
                        }
                        n = *block
                            .offset((*ptr.offset(unLo as isize)).wrapping_add(d as u32) as isize)
                            as i32
                            - med;
                        if n == 0 {
                            let mut zztmp: i32 = *ptr.offset(unLo as isize) as i32;
                            *ptr.offset(unLo as isize) = *ptr.offset(ltLo as isize);
                            *ptr.offset(ltLo as isize) = zztmp as u32;
                            ltLo += 1;
                            ltLo;
                            unLo += 1;
                            unLo;
                        } else {
                            if n > 0 {
                                break;
                            }
                            unLo += 1;
                            unLo;
                        }
                    }
                    while 1 != 0 {
                        if unLo > unHi {
                            break;
                        }
                        n = *block
                            .offset((*ptr.offset(unHi as isize)).wrapping_add(d as u32) as isize)
                            as i32
                            - med;
                        if n == 0 {
                            let mut zztmp_0: i32 = *ptr.offset(unHi as isize) as i32;
                            *ptr.offset(unHi as isize) = *ptr.offset(gtHi as isize);
                            *ptr.offset(gtHi as isize) = zztmp_0 as u32;
                            gtHi -= 1;
                            gtHi;
                            unHi -= 1;
                            unHi;
                        } else {
                            if n < 0 {
                                break;
                            }
                            unHi -= 1;
                            unHi;
                        }
                    }
                    if unLo > unHi {
                        break;
                    }
                    let mut zztmp_1: i32 = *ptr.offset(unLo as isize) as i32;
                    *ptr.offset(unLo as isize) = *ptr.offset(unHi as isize);
                    *ptr.offset(unHi as isize) = zztmp_1 as u32;
                    unLo += 1;
                    unLo;
                    unHi -= 1;
                    unHi;
                }
                if gtHi < ltLo {
                    stackLo[sp as usize] = lo;
                    stackHi[sp as usize] = hi;
                    stackD[sp as usize] = d + 1;
                    sp += 1;
                    sp;
                } else {
                    n = if ltLo - lo < unLo - ltLo {
                        ltLo - lo
                    } else {
                        unLo - ltLo
                    };
                    let mut yyp1: i32 = lo;
                    let mut yyp2: i32 = unLo - n;
                    let mut yyn: i32 = n;
                    while yyn > 0 {
                        let mut zztmp_2: i32 = *ptr.offset(yyp1 as isize) as i32;
                        *ptr.offset(yyp1 as isize) = *ptr.offset(yyp2 as isize);
                        *ptr.offset(yyp2 as isize) = zztmp_2 as u32;
                        yyp1 += 1;
                        yyp1;
                        yyp2 += 1;
                        yyp2;
                        yyn -= 1;
                        yyn;
                    }
                    m = if hi - gtHi < gtHi - unHi {
                        hi - gtHi
                    } else {
                        gtHi - unHi
                    };
                    let mut yyp1_0: i32 = unLo;
                    let mut yyp2_0: i32 = hi - m + 1;
                    let mut yyn_0: i32 = m;
                    while yyn_0 > 0 {
                        let mut zztmp_3: i32 = *ptr.offset(yyp1_0 as isize) as i32;
                        *ptr.offset(yyp1_0 as isize) = *ptr.offset(yyp2_0 as isize);
                        *ptr.offset(yyp2_0 as isize) = zztmp_3 as u32;
                        yyp1_0 += 1;
                        yyp1_0;
                        yyp2_0 += 1;
                        yyp2_0;
                        yyn_0 -= 1;
                        yyn_0;
                    }
                    n = lo + unLo - ltLo - 1;
                    m = hi - (gtHi - unHi) + 1;
                    nextLo[0 as usize] = lo;
                    nextHi[0 as usize] = n;
                    nextD[0 as usize] = d;
                    nextLo[1 as usize] = m;
                    nextHi[1 as usize] = hi;
                    nextD[1 as usize] = d;
                    nextLo[2 as usize] = n + 1;
                    nextHi[2 as usize] = m - 1;
                    nextD[2 as usize] = d + 1;
                    if nextHi[0 as usize] - nextLo[0 as usize]
                        < nextHi[1 as usize] - nextLo[1 as usize]
                    {
                        let mut tz: i32 = 0;
                        tz = nextLo[0 as usize];
                        nextLo[0 as usize] = nextLo[1 as usize];
                        nextLo[1 as usize] = tz;
                        tz = nextHi[0 as usize];
                        nextHi[0 as usize] = nextHi[1 as usize];
                        nextHi[1 as usize] = tz;
                        tz = nextD[0 as usize];
                        nextD[0 as usize] = nextD[1 as usize];
                        nextD[1 as usize] = tz;
                    }
                    if nextHi[1 as usize] - nextLo[1 as usize]
                        < nextHi[2 as usize] - nextLo[2 as usize]
                    {
                        let mut tz_0: i32 = 0;
                        tz_0 = nextLo[1 as usize];
                        nextLo[1 as usize] = nextLo[2 as usize];
                        nextLo[2 as usize] = tz_0;
                        tz_0 = nextHi[1 as usize];
                        nextHi[1 as usize] = nextHi[2 as usize];
                        nextHi[2 as usize] = tz_0;
                        tz_0 = nextD[1 as usize];
                        nextD[1 as usize] = nextD[2 as usize];
                        nextD[2 as usize] = tz_0;
                    }
                    if nextHi[0 as usize] - nextLo[0 as usize]
                        < nextHi[1 as usize] - nextLo[1 as usize]
                    {
                        let mut tz_1: i32 = 0;
                        tz_1 = nextLo[0 as usize];
                        nextLo[0 as usize] = nextLo[1 as usize];
                        nextLo[1 as usize] = tz_1;
                        tz_1 = nextHi[0 as usize];
                        nextHi[0 as usize] = nextHi[1 as usize];
                        nextHi[1 as usize] = tz_1;
                        tz_1 = nextD[0 as usize];
                        nextD[0 as usize] = nextD[1 as usize];
                        nextD[1 as usize] = tz_1;
                    }
                    stackLo[sp as usize] = nextLo[0 as usize];
                    stackHi[sp as usize] = nextHi[0 as usize];
                    stackD[sp as usize] = nextD[0 as usize];
                    sp += 1;
                    sp;
                    stackLo[sp as usize] = nextLo[1 as usize];
                    stackHi[sp as usize] = nextHi[1 as usize];
                    stackD[sp as usize] = nextD[1 as usize];
                    sp += 1;
                    sp;
                    stackLo[sp as usize] = nextLo[2 as usize];
                    stackHi[sp as usize] = nextHi[2 as usize];
                    stackD[sp as usize] = nextD[2 as usize];
                    sp += 1;
                    sp;
                }
            }
        }
    }
}

extern "C" fn mainSort(
    mut ptr: *mut u32,
    mut block: *mut u8,
    mut quadrant: *mut u16,
    mut ftab: *mut u32,
    mut nblock: i32,
    mut verb: i32,
    mut budget: *mut i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ss: i32 = 0;
    let mut sb: i32 = 0;
    let mut runningOrder: [i32; 256] = [0; 256];
    let mut bigDone: [u8; 256] = [0; 256];
    let mut copyStart: [i32; 256] = [0; 256];
    let mut copyEnd: [i32; 256] = [0; 256];
    let mut c1: u8 = 0;
    let mut numQSorted: i32 = 0;
    let mut s: u16 = 0;
    unsafe {
        if verb >= 4 {
            fprintf(
                __stderrp,
                b"        main sort initialise ...\n\0" as *const u8 as *const i8,
            );
        }
    }
    i = 65536;
    unsafe {
        while i >= 0 {
            *ftab.offset(i as isize) = 0;
            i -= 1;
            i;
        }
        j = (*block.offset(0 as isize) as i32) << 8;
    }
    i = nblock - 1;
    unsafe {
        while i >= 3 {
            *quadrant.offset(i as isize) = 0;
            j = j >> 8 | (*block.offset(i as isize) as i32) << 8;
            let ref mut fresh0 = *ftab.offset(j as isize);
            *fresh0 = (*fresh0).wrapping_add(1);
            *fresh0;
            *quadrant.offset((i - 1i32) as isize) = 0;
            j = j >> 8 | (*block.offset((i - 1i32) as isize) as i32) << 8;
            let ref mut fresh1 = *ftab.offset(j as isize);
            *fresh1 = (*fresh1).wrapping_add(1);
            *fresh1;
            *quadrant.offset((i - 2i32) as isize) = 0;
            j = j >> 8 | (*block.offset((i - 2i32) as isize) as i32) << 8;
            let ref mut fresh2 = *ftab.offset(j as isize);
            *fresh2 = (*fresh2).wrapping_add(1);
            *fresh2;
            *quadrant.offset((i - 3i32) as isize) = 0;
            j = j >> 8 | (*block.offset((i - 3i32) as isize) as i32) << 8;
            let ref mut fresh3 = *ftab.offset(j as isize);
            *fresh3 = (*fresh3).wrapping_add(1);
            *fresh3;
            i -= 4;
        }
        while i >= 0 {
            *quadrant.offset(i as isize) = 0;
            j = j >> 8 | (*block.offset(i as isize) as i32) << 8;
            let ref mut fresh4 = *ftab.offset(j as isize);
            *fresh4 = (*fresh4).wrapping_add(1);
            *fresh4;
            i -= 1;
            i;
        }
    }
    i = 0;
    unsafe {
        while i < 2 + 12 + 18 + 2 {
            *block.offset((nblock + i) as isize) = *block.offset(i as isize);
            *quadrant.offset((nblock + i) as isize) = 0;
            i += 1;
            i;
        }
        if verb >= 4 {
            fprintf(
                __stderrp,
                b"        bucket sorting ...\n\0" as *const u8 as *const i8,
            );
        }
    }
    i = 1;
    unsafe {
        while i <= 65536 {
            let ref mut fresh5 = *ftab.offset(i as isize);
            *fresh5 = (*fresh5).wrapping_add(*ftab.offset((i - 1i32) as isize));
            i += 1;
            i;
        }
        s = ((*block.offset(0 as isize) as i32) << 8i32) as u16;
    }
    i = nblock - 1;
    unsafe {
        while i >= 3 {
            s = (s as i32 >> 8 | (*block.offset(i as isize) as i32) << 8i32) as u16;
            j = (*ftab.offset(s as isize)).wrapping_sub(1) as i32;
            *ftab.offset(s as isize) = j as u32;
            *ptr.offset(j as isize) = i as u32;
            s = (s as i32 >> 8 | (*block.offset((i - 1i32) as isize) as i32) << 8i32) as u16;
            j = (*ftab.offset(s as isize)).wrapping_sub(1) as i32;
            *ftab.offset(s as isize) = j as u32;
            *ptr.offset(j as isize) = (i - 1i32) as u32;
            s = (s as i32 >> 8 | (*block.offset((i - 2i32) as isize) as i32) << 8i32) as u16;
            j = (*ftab.offset(s as isize)).wrapping_sub(1) as i32;
            *ftab.offset(s as isize) = j as u32;
            *ptr.offset(j as isize) = (i - 2i32) as u32;
            s = (s as i32 >> 8 | (*block.offset((i - 3i32) as isize) as i32) << 8i32) as u16;
            j = (*ftab.offset(s as isize)).wrapping_sub(1) as i32;
            *ftab.offset(s as isize) = j as u32;
            *ptr.offset(j as isize) = (i - 3i32) as u32;
            i -= 4;
        }
        while i >= 0 {
            s = (s as i32 >> 8 | (*block.offset(i as isize) as i32) << 8i32) as u16;
            j = (*ftab.offset(s as isize)).wrapping_sub(1) as i32;
            *ftab.offset(s as isize) = j as u32;
            *ptr.offset(j as isize) = i as u32;
            i -= 1;
            i;
        }
    }
    i = 0;
    while i <= 255 {
        bigDone[i as usize] = 0;
        runningOrder[i as usize] = i;
        i += 1;
        i;
    }
    let mut vv: i32 = 0;
    let mut h: i32 = 1;
    loop {
        h = 3 * h + 1;
        if !(h <= 256) {
            break;
        }
    }
    unsafe {
        loop {
            h = h / 3;
            i = h;
            while i <= 255 {
                vv = runningOrder[i as usize];
                j = i;
                while (*ftab.offset(((runningOrder[(j - h) as usize] + 1i32) << 8i32) as isize))
                    .wrapping_sub(*ftab.offset((runningOrder[(j - h) as usize] << 8i32) as isize))
                    > (*ftab.offset(((vv + 1i32) << 8i32) as isize))
                        .wrapping_sub(*ftab.offset((vv << 8i32) as isize))
                {
                    runningOrder[j as usize] = runningOrder[(j - h) as usize];
                    j = j - h;
                    if j <= h - 1 {
                        break;
                    }
                }
                runningOrder[j as usize] = vv;
                i += 1;
                i;
            }
            if !(h != 1) {
                break;
            }
        }
    }
    numQSorted = 0;
    i = 0;
    unsafe {
        while i <= 255 {
            ss = runningOrder[i as usize];
            j = 0;
            while j <= 255 {
                if j != ss {
                    sb = (ss << 8) + j;
                    if *ftab.offset(sb as isize) & (1i32 << 21) as u32 == 0 {
                        let mut lo: i32 =
                            (*ftab.offset(sb as isize) & !(1i32 << 21i32) as u32) as i32;
                        let mut hi: i32 = (*ftab.offset((sb + 1i32) as isize)
                            & !(1i32 << 21) as u32)
                            .wrapping_sub(1) as i32;
                        if hi > lo {
                            if verb >= 4 {
                                fprintf(
                                    __stderrp,
                                    b"        qsort [0x%x, 0x%x]   done %d   this %d\n\0"
                                        as *const u8
                                        as *const i8,
                                    ss,
                                    j,
                                    numQSorted,
                                    hi - lo + 1,
                                );
                            }
                            mainQSort3(ptr, block, quadrant, nblock, lo, hi, 2, budget);
                            numQSorted += hi - lo + 1;
                            if *budget < 0 {
                                return;
                            }
                        }
                    };
                    *ftab.offset(sb as isize) |= (1i32 << 21) as u32;
                }
                j += 1;
                j;
            }
            if bigDone[ss as usize] != 0 {
                BZ2_bz__AssertH__fail(1006);
            }
            j = 0;
            while j <= 255 {
                copyStart[j as usize] =
                    (*ftab.offset(((j << 8i32) + ss) as isize) & !(1i32 << 21i32) as u32) as i32;
                copyEnd[j as usize] = (*ftab.offset(((j << 8i32) + ss + 1i32) as isize)
                    & !(1i32 << 21) as u32)
                    .wrapping_sub(1) as i32;
                j += 1;
                j;
            }
            j = (*ftab.offset((ss << 8i32) as isize) & !(1i32 << 21i32) as u32) as i32;
            while j < copyStart[ss as usize] {
                k = (*ptr.offset(j as isize)).wrapping_sub(1) as i32;
                if k < 0 {
                    k += nblock;
                }
                c1 = *block.offset(k as isize);
                if bigDone[c1 as usize] == 0 {
                    let fresh6 = copyStart[c1 as usize];
                    copyStart[c1 as usize] = copyStart[c1 as usize] + 1;
                    *ptr.offset(fresh6 as isize) = k as u32;
                }
                j += 1;
                j;
            }
            j = (*ftab.offset(((ss + 1i32) << 8i32) as isize) & !(1i32 << 21) as u32)
                .wrapping_sub(1) as i32;
            while j > copyEnd[ss as usize] {
                k = (*ptr.offset(j as isize)).wrapping_sub(1) as i32;
                if k < 0 {
                    k += nblock;
                }
                c1 = *block.offset(k as isize);
                if bigDone[c1 as usize] == 0 {
                    let fresh7 = copyEnd[c1 as usize];
                    copyEnd[c1 as usize] = copyEnd[c1 as usize] - 1;
                    *ptr.offset(fresh7 as isize) = k as u32;
                }
                j -= 1;
                j;
            }
            if !(copyStart[ss as usize] - 1 == copyEnd[ss as usize]
                || copyStart[ss as usize] == 0 && copyEnd[ss as usize] == nblock - 1)
            {
                BZ2_bz__AssertH__fail(1007);
            }
            j = 0;
            while j <= 255 {
                *ftab.offset(((j << 8i32) + ss) as isize) |= (1i32 << 21) as u32;
                j += 1;
                j;
            }
            bigDone[ss as usize] = 1;
            if i < 255 {
                let mut bbStart: i32 =
                    (*ftab.offset((ss << 8i32) as isize) & !(1i32 << 21i32) as u32) as i32;
                let mut bbSize: i32 = (*ftab.offset(((ss + 1i32) << 8i32) as isize)
                    & !(1i32 << 21) as u32)
                    .wrapping_sub(bbStart as u32) as i32;
                let mut shifts: i32 = 0;
                while bbSize >> shifts > 65534 {
                    shifts += 1;
                    shifts;
                }
                j = bbSize - 1;
                while j >= 0 {
                    let mut a2update: i32 = *ptr.offset((bbStart + j) as isize) as i32;
                    let mut qVal: u16 = (j >> shifts) as u16;
                    *quadrant.offset(a2update as isize) = qVal;
                    if a2update < 2 + 12 + 18 + 2 {
                        *quadrant.offset((a2update + nblock) as isize) = qVal;
                    }
                    j -= 1;
                    j;
                }
                if !(bbSize - 1 >> shifts <= 65535) {
                    BZ2_bz__AssertH__fail(1002);
                }
            }
            i += 1;
            i;
        }
        if verb >= 4 {
            fprintf(
                __stderrp,
                b"        %d pointers, %d sorted, %d scanned\n\0" as *const u8 as *const i8,
                nblock,
                numQSorted,
                nblock - numQSorted,
            );
        }
    }
}

#[no_mangle]
pub extern "C" fn BZ2_blockSort(mut s: *mut EState) {
    let mut ptr: *mut u32 = (*s).ptr;
    let mut block: *mut u8 = (*s).block;
    let mut ftab: *mut u32 = (*s).ftab;
    let mut nblock: i32 = (*s).nblock;
    let mut verb: i32 = (*s).verbosity;
    let mut wfact: i32 = (*s).workFactor;
    let mut quadrant: *mut u16 = 0 as *mut u16;
    let mut budget: i32 = 0;
    let mut budgetInit: i32 = 0;
    let mut i: i32 = 0;
    unsafe {
        if nblock < 10000 {
            fallbackSort((*s).arr1, (*s).arr2, ftab, nblock, verb);
        } else {
            i = nblock + (2 + 12 + 18 + 2);
            if i & 1 != 0 {
                i += 1;
                i;
            }
            quadrant = &mut *block.offset(i as isize) as *mut u8 as *mut u16;
            if wfact < 1 {
                wfact = 1;
            }
            if wfact > 100 {
                wfact = 100;
            }
            budgetInit = nblock * ((wfact - 1) / 3);
            budget = budgetInit;
            mainSort(ptr, block, quadrant, ftab, nblock, verb, &mut budget);
            if verb >= 3 {
                fprintf(
                    __stderrp,
                    b"      %d work, %d block, ratio %5.2f\n\0" as *const u8 as *const i8,
                    budgetInit - budget,
                    nblock,
                    ((budgetInit - budget) as std::os::raw::c_float
                        / (if nblock == 0i32 { 1i32 } else { nblock }) as std::os::raw::c_float)
                        as f64,
                );
            }
            if budget < 0 {
                if verb >= 2 {
                    fprintf(
                        __stderrp,
                        b"    too repetitive; using fallback sorting algorithm\n\0" as *const u8
                            as *const i8,
                    );
                }
                fallbackSort((*s).arr1, (*s).arr2, ftab, nblock, verb);
            }
        };
    }
    (*s).origPtr = -1;
    i = 0;
    unsafe {
        while i < (*s).nblock {
            if *ptr.offset(i as isize) == 0 {
                (*s).origPtr = i;
                break;
            } else {
                i += 1;
                i;
            }
        }
        if !((*s).origPtr != -1) {
            BZ2_bz__AssertH__fail(1003);
        }
    }
}
