//use libc;
extern "C" {
    pub type __sFILEX;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn BZ2_bz__AssertH__fail(errcode: i32);
    fn BZ2_blockSort(_: *mut EState);
    fn BZ2_hbAssignCodes(_: *mut i32, _: *mut u8, _: i32, _: i32, _: i32);
    fn BZ2_hbMakeCodeLengths(_: *mut u8, _: *mut i32, _: i32, _: i32);
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
#[no_mangle]
pub extern "C" fn BZ2_bsInitWrite(mut s: *mut EState) {
    (*s).bsLive = 0;
    (*s).bsBuff = 0;
}

extern "C" fn bsFinishWrite(mut s: *mut EState) {
    unsafe {
        while (*s).bsLive > 0 {
            *((*s).zbits).offset((*s).numZ as isize) = ((*s).bsBuff >> 24i32) as u8;
            (*s).numZ += 1;
            (*s).numZ;
            (*s).bsBuff <<= 8;
            (*s).bsLive -= 8;
        }
    }
}

#[inline]
extern "C" fn bsW(mut s: *mut EState, mut n: i32, mut v: u32) {
    unsafe {
        while (*s).bsLive >= 8 {
            *((*s).zbits).offset((*s).numZ as isize) = ((*s).bsBuff >> 24i32) as u8;
            (*s).numZ += 1;
            (*s).numZ;
            (*s).bsBuff <<= 8;
            (*s).bsLive -= 8;
        }
        (*s).bsBuff |= v << 32 - (*s).bsLive - n;
    }
    (*s).bsLive += n;
}

extern "C" fn bsPutUInt32(mut s: *mut EState, mut u: u32) {
    bsW(s, 8, ((u >> 24i32) as i64 & 0xff) as u32);
    bsW(s, 8, ((u >> 16i32) as i64 & 0xff) as u32);
    bsW(s, 8, ((u >> 8i32) as i64 & 0xff) as u32);
    bsW(s, 8, (u as i64 & 0xffi64) as u32);
}

extern "C" fn bsPutUChar(mut s: *mut EState, mut c: u8) {
    bsW(s, 8, c as u32);
}

extern "C" fn makeMaps_e(mut s: *mut EState) {
    let mut i: i32 = 0;
    (*s).nInUse = 0;
    i = 0;
    while i < 256 {
        if (*s).inUse[i as usize] != 0 {
            (*s).unseqToSeq[i as usize] = (*s).nInUse as u8;
            (*s).nInUse += 1;
            (*s).nInUse;
        }
        i += 1;
        i;
    }
}

extern "C" fn generateMTFValues(mut s: *mut EState) {
    let mut yy: [u8; 256] = [0; 256];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut zPend: i32 = 0;
    let mut wr: i32 = 0;
    let mut EOB: i32 = 0;
    let mut ptr: *mut u32 = (*s).ptr;
    let mut block: *mut u8 = (*s).block;
    let mut mtfv: *mut u16 = (*s).mtfv;
    makeMaps_e(s);
    EOB = (*s).nInUse + 1;
    i = 0;
    while i <= EOB {
        (*s).mtfFreq[i as usize] = 0;
        i += 1;
        i;
    }
    wr = 0;
    zPend = 0;
    i = 0;
    while i < (*s).nInUse {
        yy[i as usize] = i as u8;
        i += 1;
        i;
    }
    i = 0;
    unsafe {
        while i < (*s).nblock {
            let mut ll_i: u8 = 0;
            j = (*ptr.offset(i as isize)).wrapping_sub(1) as i32;
            if j < 0 {
                j += (*s).nblock;
            }
            ll_i = (*s).unseqToSeq[*block.offset(j as isize) as usize];
            if yy[0 as usize] as i32 == ll_i as i32 {
                zPend += 1;
                zPend;
            } else {
                if zPend > 0 {
                    zPend -= 1;
                    zPend;
                    while 1 != 0 {
                        if zPend & 1 != 0 {
                            *mtfv.offset(wr as isize) = 1;
                            wr += 1;
                            wr;
                            (*s).mtfFreq[1 as usize] += 1;
                            (*s).mtfFreq[1 as usize];
                        } else {
                            *mtfv.offset(wr as isize) = 0;
                            wr += 1;
                            wr;
                            (*s).mtfFreq[0 as usize] += 1;
                            (*s).mtfFreq[0 as usize];
                        }
                        if zPend < 2 {
                            break;
                        }
                        zPend = (zPend - 2) / 2;
                    }
                    zPend = 0;
                }
                let mut rtmp: u8 = 0;
                let mut ryy_j: *mut u8 = 0 as *mut u8;
                let mut rll_i: u8 = 0;
                rtmp = yy[1 as usize];
                yy[1 as usize] = yy[0 as usize];
                ryy_j = &mut *yy.as_mut_ptr().offset(1 as isize) as *mut u8;
                rll_i = ll_i;
                while rll_i as i32 != rtmp as i32 {
                    let mut rtmp2: u8 = 0;
                    ryy_j = ryy_j.offset(1);
                    ryy_j;
                    rtmp2 = rtmp;
                    rtmp = *ryy_j;
                    *ryy_j = rtmp2;
                }
                yy[0 as usize] = rtmp;
                j = ryy_j.offset_from(&mut *yy.as_mut_ptr().offset(0 as isize) as *mut u8) as i32;
                *mtfv.offset(wr as isize) = (j + 1i32) as u16;
                wr += 1;
                wr;
                (*s).mtfFreq[(j + 1i32) as usize] += 1;
                (*s).mtfFreq[(j + 1i32) as usize];
            }
            i += 1;
            i;
        }
        if zPend > 0 {
            zPend -= 1;
            zPend;
            while 1 != 0 {
                if zPend & 1 != 0 {
                    *mtfv.offset(wr as isize) = 1;
                    wr += 1;
                    wr;
                    (*s).mtfFreq[1 as usize] += 1;
                    (*s).mtfFreq[1 as usize];
                } else {
                    *mtfv.offset(wr as isize) = 0;
                    wr += 1;
                    wr;
                    (*s).mtfFreq[0 as usize] += 1;
                    (*s).mtfFreq[0 as usize];
                }
                if zPend < 2 {
                    break;
                }
                zPend = (zPend - 2) / 2;
            }
            zPend = 0;
        };
        *mtfv.offset(wr as isize) = EOB as u16;
    }
    wr += 1;
    wr;
    (*s).mtfFreq[EOB as usize] += 1;
    (*s).mtfFreq[EOB as usize];
    (*s).nMTF = wr;
}

extern "C" fn sendMTFValues(mut s: *mut EState) {
    let mut v: i32 = 0;
    let mut t: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut gs: i32 = 0;
    let mut ge: i32 = 0;
    let mut totc: i32 = 0;
    let mut bt: i32 = 0;
    let mut bc: i32 = 0;
    let mut iter: i32 = 0;
    let mut nSelectors: i32 = 0;
    let mut alphaSize: i32 = 0;
    let mut minLen: i32 = 0;
    let mut maxLen: i32 = 0;
    let mut selCtr: i32 = 0;
    let mut nGroups: i32 = 0;
    let mut nBytes: i32 = 0;
    let mut cost: [u16; 6] = [0; 6];
    let mut fave: [i32; 6] = [0; 6];
    let mut mtfv: *mut u16 = (*s).mtfv;
    unsafe {
        if (*s).verbosity >= 3 {
            fprintf(
                __stderrp,
                b"      %d in block, %d after MTF & 1-2 coding, %d+2 syms in use\n\0" as *const u8
                    as *const i8,
                (*s).nblock,
                (*s).nMTF,
                (*s).nInUse,
            );
        }
    }
    alphaSize = (*s).nInUse + 2;
    t = 0;
    while t < 6 {
        v = 0;
        while v < alphaSize {
            (*s).len[t as usize][v as usize] = 15;
            v += 1;
            v;
        }
        t += 1;
        t;
    }
    unsafe {
        if !((*s).nMTF > 0) {
            BZ2_bz__AssertH__fail(3001);
        }
    }
    if (*s).nMTF < 200 {
        nGroups = 2;
    } else if (*s).nMTF < 600 {
        nGroups = 3;
    } else if (*s).nMTF < 1200 {
        nGroups = 4;
    } else if (*s).nMTF < 2400 {
        nGroups = 5;
    } else {
        nGroups = 6;
    }
    let mut nPart: i32 = 0;
    let mut remF: i32 = 0;
    let mut tFreq: i32 = 0;
    let mut aFreq: i32 = 0;
    nPart = nGroups;
    remF = (*s).nMTF;
    gs = 0;
    unsafe {
        while nPart > 0 {
            tFreq = remF / nPart;
            ge = gs - 1;
            aFreq = 0;
            while aFreq < tFreq && ge < alphaSize - 1 {
                ge += 1;
                ge;
                aFreq += (*s).mtfFreq[ge as usize];
            }
            if ge > gs && nPart != nGroups && nPart != 1 && (nGroups - nPart) % 2 == 1 {
                aFreq -= (*s).mtfFreq[ge as usize];
                ge -= 1;
                ge;
            }
            if (*s).verbosity >= 3 {
                fprintf(
                    __stderrp,
                    b"      initial group %d, [%d .. %d], has %d syms (%4.1f%%)\n\0" as *const u8
                        as *const i8,
                    nPart,
                    gs,
                    ge,
                    aFreq,
                    100.0f64 * aFreq as std::os::raw::c_float as f64 / (*s).nMTF as std::os::raw::c_float as f64,
                );
            }
            v = 0;
            while v < alphaSize {
                if v >= gs && v <= ge {
                    (*s).len[(nPart - 1i32) as usize][v as usize] = 0;
                } else {
                    (*s).len[(nPart - 1i32) as usize][v as usize] = 15;
                }
                v += 1;
                v;
            }
            nPart -= 1;
            nPart;
            gs = ge + 1;
            remF -= aFreq;
        }
    }
    iter = 0;
    unsafe {
        while iter < 4 {
            t = 0;
            while t < nGroups {
                fave[t as usize] = 0;
                t += 1;
                t;
            }
            t = 0;
            while t < nGroups {
                v = 0;
                while v < alphaSize {
                    (*s).rfreq[t as usize][v as usize] = 0;
                    v += 1;
                    v;
                }
                t += 1;
                t;
            }
            if nGroups == 6 {
                v = 0;
                while v < alphaSize {
                    (*s).len_pack[v as usize][0 as usize] =
                        (((*s).len[1 as usize][v as usize] as i32) << 16
                            | (*s).len[0 as usize][v as usize] as i32)
                            as u32;
                    (*s).len_pack[v as usize][1 as usize] =
                        (((*s).len[3 as usize][v as usize] as i32) << 16
                            | (*s).len[2 as usize][v as usize] as i32)
                            as u32;
                    (*s).len_pack[v as usize][2 as usize] =
                        (((*s).len[5 as usize][v as usize] as i32) << 16
                            | (*s).len[4 as usize][v as usize] as i32)
                            as u32;
                    v += 1;
                    v;
                }
            }
            nSelectors = 0;
            totc = 0;
            gs = 0;
            while 1 != 0 {
                if gs >= (*s).nMTF {
                    break;
                }
                ge = gs + 50 - 1;
                if ge >= (*s).nMTF {
                    ge = (*s).nMTF - 1;
                }
                t = 0;
                while t < nGroups {
                    cost[t as usize] = 0;
                    t += 1;
                    t;
                }
                if nGroups == 6 && 50 == ge - gs + 1 {
                    let mut cost01: u32 = 0;
                    let mut cost23: u32 = 0;
                    let mut cost45: u32 = 0;
                    let mut icv: u16 = 0;
                    cost45 = 0;
                    cost23 = cost45;
                    cost01 = cost23;
                    icv = *mtfv.offset((gs + 0i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 1i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 2i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 3i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 4i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 5i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 6i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 7i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 8i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 9i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 10i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 11i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 12i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 13i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 14i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 15i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 16i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 17i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 18i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 19i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 20i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 21i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 22i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 23i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 24i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 25i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 26i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 27i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 28i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 29i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 30i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 31i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 32i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 33i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 34i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 35i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 36i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 37i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 38i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 39i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 40i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 41i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 42i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 43i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 44i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 45i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 46i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 47i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 48i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    icv = *mtfv.offset((gs + 49i32) as isize);
                    cost01 = cost01.wrapping_add((*s).len_pack[icv as usize][0 as usize]);
                    cost23 = cost23.wrapping_add((*s).len_pack[icv as usize][1 as usize]);
                    cost45 = cost45.wrapping_add((*s).len_pack[icv as usize][2 as usize]);
                    cost[0 as usize] = (cost01 & 0xffffu32) as u16;
                    cost[1 as usize] = (cost01 >> 16i32) as u16;
                    cost[2 as usize] = (cost23 & 0xffffu32) as u16;
                    cost[3 as usize] = (cost23 >> 16i32) as u16;
                    cost[4 as usize] = (cost45 & 0xffffu32) as u16;
                    cost[5 as usize] = (cost45 >> 16i32) as u16;
                } else {
                    i = gs;
                    while i <= ge {
                        let mut icv_0: u16 = *mtfv.offset(i as isize);
                        t = 0;
                        while t < nGroups {
                            cost[t as usize] = (cost[t as usize] as i32
                                + (*s).len[t as usize][icv_0 as usize] as i32)
                                as u16;
                            t += 1;
                            t;
                        }
                        i += 1;
                        i;
                    }
                }
                bc = 999999999;
                bt = -1;
                t = 0;
                while t < nGroups {
                    if (cost[t as usize] as i32) < bc {
                        bc = cost[t as usize] as i32;
                        bt = t;
                    }
                    t += 1;
                    t;
                }
                totc += bc;
                fave[bt as usize] += 1;
                fave[bt as usize];
                (*s).selector[nSelectors as usize] = bt as u8;
                nSelectors += 1;
                nSelectors;
                if nGroups == 6 && 50 == ge - gs + 1 {
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 0i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 0i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 1i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 1i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 2i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 2i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 3i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 3i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 4i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 4i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 5i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 5i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 6i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 6i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 7i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 7i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 8i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 8i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 9i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 9i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 10i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 10i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 11i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 11i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 12i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 12i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 13i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 13i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 14i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 14i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 15i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 15i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 16i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 16i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 17i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 17i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 18i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 18i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 19i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 19i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 20i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 20i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 21i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 21i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 22i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 22i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 23i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 23i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 24i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 24i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 25i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 25i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 26i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 26i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 27i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 27i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 28i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 28i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 29i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 29i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 30i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 30i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 31i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 31i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 32i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 32i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 33i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 33i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 34i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 34i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 35i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 35i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 36i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 36i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 37i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 37i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 38i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 38i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 39i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 39i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 40i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 40i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 41i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 41i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 42i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 42i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 43i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 43i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 44i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 44i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 45i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 45i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 46i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 46i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 47i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 47i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 48i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 48i32) as isize) as usize];
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 49i32) as isize) as usize] += 1;
                    (*s).rfreq[bt as usize][*mtfv.offset((gs + 49i32) as isize) as usize];
                } else {
                    i = gs;
                    while i <= ge {
                        (*s).rfreq[bt as usize][*mtfv.offset(i as isize) as usize] += 1;
                        (*s).rfreq[bt as usize][*mtfv.offset(i as isize) as usize];
                        i += 1;
                        i;
                    }
                }
                gs = ge + 1;
            }
            if (*s).verbosity >= 3 {
                fprintf(
                    __stderrp,
                    b"      pass %d: size is %d, grp uses are \0" as *const u8 as *const i8,
                    iter + 1,
                    totc / 8,
                );
                t = 0;
                while t < nGroups {
                    fprintf(
                        __stderrp,
                        b"%d \0" as *const u8 as *const i8,
                        fave[t as usize],
                    );
                    t += 1;
                    t;
                }
                fprintf(__stderrp, b"\n\0" as *const u8 as *const i8);
            }
            t = 0;
            while t < nGroups {
                BZ2_hbMakeCodeLengths(
                    &mut *(*((*s).len).as_mut_ptr().offset(t as isize))
                        .as_mut_ptr()
                        .offset(0 as isize),
                    &mut *(*((*s).rfreq).as_mut_ptr().offset(t as isize))
                        .as_mut_ptr()
                        .offset(0 as isize),
                    alphaSize,
                    17,
                );
                t += 1;
                t;
            }
            iter += 1;
            iter;
        }
        if !(nGroups < 8) {
            BZ2_bz__AssertH__fail(3002);
        }
        if !(nSelectors < 32768 && nSelectors <= 2 + 900000 / 50) {
            BZ2_bz__AssertH__fail(3003);
        }
    }
    let mut pos: [u8; 6] = [0; 6];
    let mut ll_i: u8 = 0;
    let mut tmp2: u8 = 0;
    let mut tmp: u8 = 0;
    i = 0;
    while i < nGroups {
        pos[i as usize] = i as u8;
        i += 1;
        i;
    }
    i = 0;
    while i < nSelectors {
        ll_i = (*s).selector[i as usize];
        j = 0;
        tmp = pos[j as usize];
        while ll_i as i32 != tmp as i32 {
            j += 1;
            j;
            tmp2 = tmp;
            tmp = pos[j as usize];
            pos[j as usize] = tmp2;
        }
        pos[0 as usize] = tmp;
        (*s).selectorMtf[i as usize] = j as u8;
        i += 1;
        i;
    }
    t = 0;
    unsafe {
        while t < nGroups {
            minLen = 32;
            maxLen = 0;
            i = 0;
            while i < alphaSize {
                if (*s).len[t as usize][i as usize] as i32 > maxLen {
                    maxLen = (*s).len[t as usize][i as usize] as i32;
                }
                if ((*s).len[t as usize][i as usize] as i32) < minLen {
                    minLen = (*s).len[t as usize][i as usize] as i32;
                }
                i += 1;
                i;
            }
            if maxLen > 17 {
                BZ2_bz__AssertH__fail(3004);
            }
            if minLen < 1 {
                BZ2_bz__AssertH__fail(3005);
            }
            BZ2_hbAssignCodes(
                &mut *(*((*s).code).as_mut_ptr().offset(t as isize))
                    .as_mut_ptr()
                    .offset(0 as isize),
                &mut *(*((*s).len).as_mut_ptr().offset(t as isize))
                    .as_mut_ptr()
                    .offset(0 as isize),
                minLen,
                maxLen,
                alphaSize,
            );
            t += 1;
            t;
        }
    }
    let mut inUse16: [u8; 16] = [0; 16];
    i = 0;
    while i < 16 {
        inUse16[i as usize] = 0;
        j = 0;
        while j < 16 {
            if (*s).inUse[(i * 16 + j) as usize] != 0 {
                inUse16[i as usize] = 1;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    nBytes = (*s).numZ;
    i = 0;
    while i < 16 {
        if inUse16[i as usize] != 0 {
            bsW(s, 1, 1);
        } else {
            bsW(s, 1, 0);
        }
        i += 1;
        i;
    }
    i = 0;
    while i < 16 {
        if inUse16[i as usize] != 0 {
            j = 0;
            while j < 16 {
                if (*s).inUse[(i * 16 + j) as usize] != 0 {
                    bsW(s, 1, 1);
                } else {
                    bsW(s, 1, 0);
                }
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
    unsafe {
        if (*s).verbosity >= 3 {
            fprintf(
                __stderrp,
                b"      bytes: mapping %d, \0" as *const u8 as *const i8,
                (*s).numZ - nBytes,
            );
        }
    }
    nBytes = (*s).numZ;
    bsW(s, 3, nGroups as u32);
    bsW(s, 15, nSelectors as u32);
    i = 0;
    while i < nSelectors {
        j = 0;
        while j < (*s).selectorMtf[i as usize] as i32 {
            bsW(s, 1, 1);
            j += 1;
            j;
        }
        bsW(s, 1, 0);
        i += 1;
        i;
    }
    unsafe {
        if (*s).verbosity >= 3 {
            fprintf(
                __stderrp,
                b"selectors %d, \0" as *const u8 as *const i8,
                (*s).numZ - nBytes,
            );
        }
    }
    nBytes = (*s).numZ;
    t = 0;
    while t < nGroups {
        let mut curr: i32 = (*s).len[t as usize][0 as usize] as i32;
        bsW(s, 5, curr as u32);
        i = 0;
        while i < alphaSize {
            while curr < (*s).len[t as usize][i as usize] as i32 {
                bsW(s, 2, 2);
                curr += 1;
                curr;
            }
            while curr > (*s).len[t as usize][i as usize] as i32 {
                bsW(s, 2, 3);
                curr -= 1;
                curr;
            }
            bsW(s, 1, 0);
            i += 1;
            i;
        }
        t += 1;
        t;
    }
    unsafe {
        if (*s).verbosity >= 3 {
            fprintf(
                __stderrp,
                b"code lengths %d, \0" as *const u8 as *const i8,
                (*s).numZ - nBytes,
            );
        }
    }
    nBytes = (*s).numZ;
    selCtr = 0;
    gs = 0;
    unsafe {
        while 1 != 0 {
            if gs >= (*s).nMTF {
                break;
            }
            ge = gs + 50 - 1;
            if ge >= (*s).nMTF {
                ge = (*s).nMTF - 1;
            }
            if !(((*s).selector[selCtr as usize] as i32) < nGroups) {
                BZ2_bz__AssertH__fail(3006);
            }
            if nGroups == 6 && 50 == ge - gs + 1 {
                let mut mtfv_i: u16 = 0;
                let mut s_len_sel_selCtr: *mut u8 = &mut *(*((*s).len)
                    .as_mut_ptr()
                    .offset(*((*s).selector).as_mut_ptr().offset(selCtr as isize) as isize))
                .as_mut_ptr()
                .offset(0 as isize) as *mut u8;
                let mut s_code_sel_selCtr: *mut i32 = &mut *(*((*s).code)
                    .as_mut_ptr()
                    .offset(*((*s).selector).as_mut_ptr().offset(selCtr as isize) as isize))
                .as_mut_ptr()
                .offset(0 as isize)
                    as *mut i32;
                mtfv_i = *mtfv.offset((gs + 0i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 1i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 2i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 3i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 4i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 5i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 6i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 7i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 8i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 9i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 10i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 11i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 12i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 13i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 14i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 15i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 16i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 17i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 18i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 19i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 20i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 21i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 22i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 23i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 24i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 25i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 26i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 27i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 28i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 29i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 30i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 31i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 32i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 33i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 34i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 35i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 36i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 37i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 38i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 39i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 40i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 41i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 42i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 43i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 44i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 45i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 46i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 47i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 48i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
                mtfv_i = *mtfv.offset((gs + 49i32) as isize);
                bsW(
                    s,
                    *s_len_sel_selCtr.offset(mtfv_i as isize) as i32,
                    *s_code_sel_selCtr.offset(mtfv_i as isize) as u32,
                );
            } else {
                i = gs;
                while i <= ge {
                    bsW(
                        s,
                        (*s).len[(*s).selector[selCtr as usize] as usize]
                            [*mtfv.offset(i as isize) as usize] as i32,
                        (*s).code[(*s).selector[selCtr as usize] as usize]
                            [*mtfv.offset(i as isize) as usize] as u32,
                    );
                    i += 1;
                    i;
                }
            }
            gs = ge + 1;
            selCtr += 1;
            selCtr;
        }
        if !(selCtr == nSelectors) {
            BZ2_bz__AssertH__fail(3007);
        }
        if (*s).verbosity >= 3 {
            fprintf(
                __stderrp,
                b"codes %d\n\0" as *const u8 as *const i8,
                (*s).numZ - nBytes,
            );
        }
    }
}

#[no_mangle]
pub extern "C" fn BZ2_compressBlock(mut s: *mut EState, mut is_last_block: u8) {
    unsafe {
        if (*s).nblock > 0 {
            (*s).blockCRC = !(*s).blockCRC;
            (*s).combinedCRC = (*s).combinedCRC << 1 | (*s).combinedCRC >> 31;
            (*s).combinedCRC ^= (*s).blockCRC;
            if (*s).blockNo > 1 {
                (*s).numZ = 0;
            }
            if (*s).verbosity >= 2 {
                fprintf(
                    __stderrp,
                    b"    block %d: crc = 0x%08x, combined CRC = 0x%08x, size = %d\n\0" as *const u8
                        as *const i8,
                    (*s).blockNo,
                    (*s).blockCRC,
                    (*s).combinedCRC,
                    (*s).nblock,
                );
            }
            BZ2_blockSort(s);
        };
        (*s).zbits = &mut *((*s).arr2 as *mut u8).offset((*s).nblock as isize) as *mut u8;
    }
    if (*s).blockNo == 1 {
        BZ2_bsInitWrite(s);
        bsPutUChar(s, 0x42);
        bsPutUChar(s, 0x5a);
        bsPutUChar(s, 0x68);
        bsPutUChar(s, (0x30 + (*s).blockSize100k) as u8);
    }
    if (*s).nblock > 0 {
        bsPutUChar(s, 0x31);
        bsPutUChar(s, 0x41);
        bsPutUChar(s, 0x59);
        bsPutUChar(s, 0x26);
        bsPutUChar(s, 0x53);
        bsPutUChar(s, 0x59);
        bsPutUInt32(s, (*s).blockCRC);
        bsW(s, 1, 0);
        bsW(s, 24, (*s).origPtr as u32);
        generateMTFValues(s);
        sendMTFValues(s);
    }
    unsafe {
        if is_last_block != 0 {
            bsPutUChar(s, 0x17);
            bsPutUChar(s, 0x72);
            bsPutUChar(s, 0x45);
            bsPutUChar(s, 0x38);
            bsPutUChar(s, 0x50);
            bsPutUChar(s, 0x90);
            bsPutUInt32(s, (*s).combinedCRC);
            if (*s).verbosity >= 2 {
                fprintf(
                    __stderrp,
                    b"    final combined CRC = 0x%08x\n   \0" as *const u8 as *const i8,
                    (*s).combinedCRC,
                );
            }
            bsFinishWrite(s);
        }
    }
}
