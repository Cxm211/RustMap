//use libc;
extern "C" {
    pub type __sFILEX;
    fn BZ2_hbCreateDecodeTables(
        _: *mut i32,
        _: *mut i32,
        _: *mut i32,
        _: *mut u8,
        _: i32,
        _: i32,
        _: i32,
    );
    fn BZ2_indexIntoF(_: i32, _: *mut i32) -> i32;
    static mut BZ2_rNums: [i32; 512];
    fn BZ2_bz__AssertH__fail(errcode: i32);
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    static mut __stderrp: *mut FILE;
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
pub struct DState {
    pub strm: *mut bz_stream,
    pub state: i32,
    pub state_out_ch: u8,
    pub state_out_len: i32,
    pub blockRandomised: u8,
    pub rNToGo: i32,
    pub rTPos: i32,
    pub bsBuff: u32,
    pub bsLive: i32,
    pub blockSize100k: i32,
    pub smallDecompress: u8,
    pub currBlockNo: i32,
    pub verbosity: i32,
    pub origPtr: i32,
    pub tPos: u32,
    pub k0: i32,
    pub unzftab: [i32; 256],
    pub nblock_used: i32,
    pub cftab: [i32; 257],
    pub cftabCopy: [i32; 257],
    pub tt: *mut u32,
    pub ll16: *mut u16,
    pub ll4: *mut u8,
    pub storedBlockCRC: u32,
    pub storedCombinedCRC: u32,
    pub calculatedBlockCRC: u32,
    pub calculatedCombinedCRC: u32,
    pub nInUse: i32,
    pub inUse: [u8; 256],
    pub inUse16: [u8; 16],
    pub seqToUnseq: [u8; 256],
    pub mtfa: [u8; 4096],
    pub mtfbase: [i32; 16],
    pub selector: [u8; 18002],
    pub selectorMtf: [u8; 18002],
    pub len: [[u8; 258]; 6],
    pub limit: [[i32; 258]; 6],
    pub base: [[i32; 258]; 6],
    pub perm: [[i32; 258]; 6],
    pub minLens: [i32; 6],
    pub save_i: i32,
    pub save_j: i32,
    pub save_t: i32,
    pub save_alphaSize: i32,
    pub save_nGroups: i32,
    pub save_nSelectors: i32,
    pub save_EOB: i32,
    pub save_groupNo: i32,
    pub save_groupPos: i32,
    pub save_nextSym: i32,
    pub save_nblockMAX: i32,
    pub save_nblock: i32,
    pub save_es: i32,
    pub save_N: i32,
    pub save_curr: i32,
    pub save_zt: i32,
    pub save_zn: i32,
    pub save_zvec: i32,
    pub save_zj: i32,
    pub save_gSel: i32,
    pub save_gMinlen: i32,
    pub save_gLimit: *mut i32,
    pub save_gBase: *mut i32,
    pub save_gPerm: *mut i32,
}
extern "C" fn makeMaps_d(mut s: *mut DState) {
    let mut i: i32 = 0;
    (*s).nInUse = 0;
    i = 0;
    while i < 256 {
        if (*s).inUse[i as usize] != 0 {
            (*s).seqToUnseq[(*s).nInUse as usize] = i as u8;
            (*s).nInUse += 1;
            (*s).nInUse;
        }
        i += 1;
        i;
    }
}

#[no_mangle]
pub extern "C" fn BZ2_decompress(mut s: *mut DState) -> i32 {
    let mut current_block: u64;
    let mut uc: u8 = 0;
    let mut retVal: i32 = 0;
    let mut minLen: i32 = 0;
    let mut maxLen: i32 = 0;
    let mut strm: *mut bz_stream = (*s).strm;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut t: i32 = 0;
    let mut alphaSize: i32 = 0;
    let mut nGroups: i32 = 0;
    let mut nSelectors: i32 = 0;
    let mut EOB: i32 = 0;
    let mut groupNo: i32 = 0;
    let mut groupPos: i32 = 0;
    let mut nextSym: i32 = 0;
    let mut nblockMAX: i32 = 0;
    let mut nblock: i32 = 0;
    let mut es: i32 = 0;
    let mut N: i32 = 0;
    let mut curr: i32 = 0;
    let mut zt: i32 = 0;
    let mut zn: i32 = 0;
    let mut zvec: i32 = 0;
    let mut zj: i32 = 0;
    let mut gSel: i32 = 0;
    let mut gMinlen: i32 = 0;
    let mut gLimit: *mut i32 = 0 as *mut i32;
    let mut gBase: *mut i32 = 0 as *mut i32;
    let mut gPerm: *mut i32 = 0 as *mut i32;
    if (*s).state == 10 {
        (*s).save_i = 0;
        (*s).save_j = 0;
        (*s).save_t = 0;
        (*s).save_alphaSize = 0;
        (*s).save_nGroups = 0;
        (*s).save_nSelectors = 0;
        (*s).save_EOB = 0;
        (*s).save_groupNo = 0;
        (*s).save_groupPos = 0;
        (*s).save_nextSym = 0;
        (*s).save_nblockMAX = 0;
        (*s).save_nblock = 0;
        (*s).save_es = 0;
        (*s).save_N = 0;
        (*s).save_curr = 0;
        (*s).save_zt = 0;
        (*s).save_zn = 0;
        (*s).save_zvec = 0;
        (*s).save_zj = 0;
        (*s).save_gSel = 0;
        (*s).save_gMinlen = 0;
        (*s).save_gLimit = 0 as *mut i32;
        (*s).save_gBase = 0 as *mut i32;
        (*s).save_gPerm = 0 as *mut i32;
    }
    i = (*s).save_i;
    j = (*s).save_j;
    t = (*s).save_t;
    alphaSize = (*s).save_alphaSize;
    nGroups = (*s).save_nGroups;
    nSelectors = (*s).save_nSelectors;
    EOB = (*s).save_EOB;
    groupNo = (*s).save_groupNo;
    groupPos = (*s).save_groupPos;
    nextSym = (*s).save_nextSym;
    nblockMAX = (*s).save_nblockMAX;
    nblock = (*s).save_nblock;
    es = (*s).save_es;
    N = (*s).save_N;
    curr = (*s).save_curr;
    zt = (*s).save_zt;
    zn = (*s).save_zn;
    zvec = (*s).save_zvec;
    zj = (*s).save_zj;
    gSel = (*s).save_gSel;
    gMinlen = (*s).save_gMinlen;
    gLimit = (*s).save_gLimit;
    gBase = (*s).save_gBase;
    gPerm = (*s).save_gPerm;
    retVal = 0;
    unsafe {
        match (*s).state {
            10 => {
                (*s).state = 10;
                loop {
                    if !(1 != 0) {
                        current_block = 5235537862154438448;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v: u32 = 0;
                        v = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v as u8;
                        current_block = 5235537862154438448;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        if uc as i32 != 0x42 {
                            retVal = -5;
                            current_block = 14281071070443768567;
                        } else {
                            current_block = 11841202474256326307;
                        }
                    }
                }
            }
            11 => {
                current_block = 11841202474256326307;
            }
            12 => {
                current_block = 12283925169228341178;
            }
            13 => {
                current_block = 14443013666620032071;
            }
            14 => {
                current_block = 14146705567854054825;
            }
            15 => {
                current_block = 8754655818168429142;
            }
            16 => {
                current_block = 7647237118421008181;
            }
            17 => {
                current_block = 16189476996697164418;
            }
            18 => {
                current_block = 9909856762157297894;
            }
            19 => {
                current_block = 17922920647706410965;
            }
            20 => {
                current_block = 14558072342450928989;
            }
            21 => {
                current_block = 318700586514436239;
            }
            22 => {
                current_block = 5201407540844598732;
            }
            23 => {
                current_block = 15798322029668445206;
            }
            24 => {
                current_block = 12214724321751295962;
            }
            25 => {
                current_block = 8329812343241388296;
            }
            26 => {
                current_block = 9557502779390786832;
            }
            27 => {
                current_block = 14254196305078590431;
            }
            28 => {
                current_block = 12500452284755425741;
            }
            29 => {
                current_block = 15241790405117970060;
            }
            30 => {
                current_block = 13572320329052790897;
            }
            31 => {
                current_block = 6766584878581269858;
            }
            32 => {
                current_block = 16302043309746641865;
            }
            33 => {
                current_block = 3292440606114030865;
            }
            34 => {
                current_block = 3277642653280575212;
            }
            35 => {
                current_block = 13301436784058847816;
            }
            36 => {
                current_block = 10254712216801151959;
            }
            37 => {
                current_block = 1004930667005816745;
            }
            38 => {
                current_block = 7873764389453924156;
            }
            39 => {
                current_block = 17270018966125003511;
            }
            40 => {
                current_block = 16680933752313891718;
            }
            41 => {
                current_block = 12564072498539239379;
            }
            42 => {
                current_block = 2114447548858696371;
            }
            43 => {
                current_block = 17258531243884734787;
            }
            44 => {
                current_block = 12388873576323557050;
            }
            45 => {
                current_block = 71893914647641011;
            }
            46 => {
                current_block = 17903534107568821721;
            }
            47 => {
                current_block = 1917286098721871713;
            }
            48 => {
                current_block = 16613896490350108258;
            }
            49 => {
                current_block = 12847522633881934894;
            }
            50 => {
                current_block = 12956768993232374699;
            }
            _ => {
                if 0 == 0 {
                    BZ2_bz__AssertH__fail(4001);
                }
                if 0 == 0 {
                    BZ2_bz__AssertH__fail(4002);
                }
                current_block = 14281071070443768567;
            }
        }
        match current_block {
            11841202474256326307 => {
                (*s).state = 11;
                loop {
                    if !(1 != 0) {
                        current_block = 2168227384378665163;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_0: u32 = 0;
                        v_0 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_0 as u8;
                        current_block = 2168227384378665163;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        if uc as i32 != 0x5a {
                            retVal = -5;
                            current_block = 14281071070443768567;
                        } else {
                            current_block = 12283925169228341178;
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            12283925169228341178 => {
                (*s).state = 12;
                loop {
                    if !(1 != 0) {
                        current_block = 178030534879405462;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_1: u32 = 0;
                        v_1 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_1 as u8;
                        current_block = 178030534879405462;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        if uc as i32 != 0x68 {
                            retVal = -5;
                            current_block = 14281071070443768567;
                        } else {
                            current_block = 14443013666620032071;
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            14443013666620032071 => {
                (*s).state = 13;
                loop {
                    if !(1 != 0) {
                        current_block = 7639320476250304355;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_2: u32 = 0;
                        v_2 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        (*s).blockSize100k = v_2 as i32;
                        current_block = 7639320476250304355;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        if (*s).blockSize100k < 0x30 + 1 || (*s).blockSize100k > 0x30 + 9 {
                            retVal = -5;
                            current_block = 14281071070443768567;
                        } else {
                            (*s).blockSize100k -= 0x30;
                            if (*s).smallDecompress != 0 {
                                (*s).ll16 = ((*strm).bzalloc).expect("non-null function pointer")(
                                    (*strm).opaque,
                                    (((*s).blockSize100k * 100000i32) as u64)
                                        .wrapping_mul(::std::mem::size_of::<u16>() as u64)
                                        as i32,
                                    1,
                                ) as *mut u16;
                                (*s).ll4 = ((*strm).bzalloc).expect("non-null function pointer")(
                                    (*strm).opaque,
                                    ((1 + (*s).blockSize100k * 100000 >> 1i32) as u64)
                                        .wrapping_mul(::std::mem::size_of::<u8>() as u64)
                                        as i32,
                                    1,
                                ) as *mut u8;
                                if ((*s).ll16).is_null() || ((*s).ll4).is_null() {
                                    retVal = -3;
                                    current_block = 14281071070443768567;
                                } else {
                                    current_block = 14146705567854054825;
                                }
                            } else {
                                (*s).tt = ((*strm).bzalloc).expect("non-null function pointer")(
                                    (*strm).opaque,
                                    (((*s).blockSize100k * 100000i32) as u64)
                                        .wrapping_mul(::std::mem::size_of::<i32>() as u64)
                                        as i32,
                                    1,
                                ) as *mut u32;
                                if ((*s).tt).is_null() {
                                    retVal = -3;
                                    current_block = 14281071070443768567;
                                } else {
                                    current_block = 14146705567854054825;
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            14146705567854054825 => {
                (*s).state = 14;
                loop {
                    if !(1 != 0) {
                        current_block = 16937825661756021828;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_3: u32 = 0;
                        v_3 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_3 as u8;
                        current_block = 16937825661756021828;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        if uc as i32 == 0x17 {
                            current_block = 2114447548858696371;
                        } else if uc as i32 != 0x31 {
                            retVal = -4;
                            current_block = 14281071070443768567;
                        } else {
                            current_block = 8754655818168429142;
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            2114447548858696371 => {
                (*s).state = 42;
                loop {
                    if !(1 != 0) {
                        current_block = 13733404100380861831;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_32: u32 = 0;
                        v_32 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_32 as u8;
                        current_block = 13733404100380861831;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        if uc as i32 != 0x72 {
                            retVal = -4;
                            current_block = 14281071070443768567;
                        } else {
                            current_block = 17258531243884734787;
                        }
                    }
                }
            }
            8754655818168429142 => {
                (*s).state = 15;
                loop {
                    if !(1 != 0) {
                        current_block = 1228639923084383292;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_4: u32 = 0;
                        v_4 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_4 as u8;
                        current_block = 1228639923084383292;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        if uc as i32 != 0x41 {
                            retVal = -4;
                            current_block = 14281071070443768567;
                        } else {
                            current_block = 7647237118421008181;
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            17258531243884734787 => {
                (*s).state = 43;
                loop {
                    if !(1 != 0) {
                        current_block = 12721425419429475574;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_33: u32 = 0;
                        v_33 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_33 as u8;
                        current_block = 12721425419429475574;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        if uc as i32 != 0x45 {
                            retVal = -4;
                            current_block = 14281071070443768567;
                        } else {
                            current_block = 12388873576323557050;
                        }
                    }
                }
            }
            7647237118421008181 => {
                (*s).state = 16;
                loop {
                    if !(1 != 0) {
                        current_block = 9235179519944561532;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_5: u32 = 0;
                        v_5 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_5 as u8;
                        current_block = 9235179519944561532;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        if uc as i32 != 0x59 {
                            retVal = -4;
                            current_block = 14281071070443768567;
                        } else {
                            current_block = 16189476996697164418;
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            12388873576323557050 => {
                (*s).state = 44;
                loop {
                    if !(1 != 0) {
                        current_block = 13813414375753095368;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_34: u32 = 0;
                        v_34 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_34 as u8;
                        current_block = 13813414375753095368;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        if uc as i32 != 0x38 {
                            retVal = -4;
                            current_block = 14281071070443768567;
                        } else {
                            current_block = 71893914647641011;
                        }
                    }
                }
            }
            16189476996697164418 => {
                (*s).state = 17;
                loop {
                    if !(1 != 0) {
                        current_block = 12467039471581323981;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_6: u32 = 0;
                        v_6 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_6 as u8;
                        current_block = 12467039471581323981;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        if uc as i32 != 0x26 {
                            retVal = -4;
                            current_block = 14281071070443768567;
                        } else {
                            current_block = 9909856762157297894;
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            71893914647641011 => {
                (*s).state = 45;
                loop {
                    if !(1 != 0) {
                        current_block = 1472103348880861285;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_35: u32 = 0;
                        v_35 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_35 as u8;
                        current_block = 1472103348880861285;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        if uc as i32 != 0x50 {
                            retVal = -4;
                            current_block = 14281071070443768567;
                        } else {
                            current_block = 17903534107568821721;
                        }
                    }
                }
            }
            9909856762157297894 => {
                (*s).state = 18;
                loop {
                    if !(1 != 0) {
                        current_block = 13164310931121142693;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_7: u32 = 0;
                        v_7 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_7 as u8;
                        current_block = 13164310931121142693;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        if uc as i32 != 0x53 {
                            retVal = -4;
                            current_block = 14281071070443768567;
                        } else {
                            current_block = 17922920647706410965;
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            17903534107568821721 => {
                (*s).state = 46;
                loop {
                    if !(1 != 0) {
                        current_block = 8232347840743503282;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_36: u32 = 0;
                        v_36 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_36 as u8;
                        current_block = 8232347840743503282;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        if uc as i32 != 0x90 {
                            retVal = -4;
                            current_block = 14281071070443768567;
                        } else {
                            (*s).storedCombinedCRC = 0;
                            current_block = 1917286098721871713;
                        }
                    }
                }
            }
            17922920647706410965 => {
                (*s).state = 19;
                loop {
                    if !(1 != 0) {
                        current_block = 14723615986260991866;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_8: u32 = 0;
                        v_8 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_8 as u8;
                        current_block = 14723615986260991866;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        if uc as i32 != 0x59 {
                            retVal = -4;
                            current_block = 14281071070443768567;
                        } else {
                            (*s).currBlockNo += 1;
                            (*s).currBlockNo;
                            if (*s).verbosity >= 2 {
                                fprintf(
                                    __stderrp,
                                    b"\n    [%d: huff+mtf \0" as *const u8 as *const i8,
                                    (*s).currBlockNo,
                                );
                            };
                            (*s).storedBlockCRC = 0;
                            current_block = 14558072342450928989;
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            1917286098721871713 => {
                (*s).state = 47;
                loop {
                    if !(1 != 0) {
                        current_block = 5465979950226085365;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_37: u32 = 0;
                        v_37 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_37 as u8;
                        current_block = 5465979950226085365;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        (*s).storedCombinedCRC = (*s).storedCombinedCRC << 8 | uc as u32;
                        current_block = 16613896490350108258;
                    }
                }
            }
            14558072342450928989 => {
                (*s).state = 20;
                loop {
                    if !(1 != 0) {
                        current_block = 15627786036016112248;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_9: u32 = 0;
                        v_9 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_9 as u8;
                        current_block = 15627786036016112248;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        (*s).storedBlockCRC = (*s).storedBlockCRC << 8 | uc as u32;
                        current_block = 318700586514436239;
                    }
                }
            }
            _ => {}
        }
        match current_block {
            16613896490350108258 => {
                (*s).state = 48;
                loop {
                    if !(1 != 0) {
                        current_block = 3854366583354019639;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_38: u32 = 0;
                        v_38 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_38 as u8;
                        current_block = 3854366583354019639;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        (*s).storedCombinedCRC = (*s).storedCombinedCRC << 8 | uc as u32;
                        current_block = 12847522633881934894;
                    }
                }
            }
            318700586514436239 => {
                (*s).state = 21;
                loop {
                    if !(1 != 0) {
                        current_block = 13493279574219925475;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_10: u32 = 0;
                        v_10 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_10 as u8;
                        current_block = 13493279574219925475;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        (*s).storedBlockCRC = (*s).storedBlockCRC << 8 | uc as u32;
                        current_block = 5201407540844598732;
                    }
                }
            }
            _ => {}
        }
        match current_block {
            12847522633881934894 => {
                (*s).state = 49;
                loop {
                    if !(1 != 0) {
                        current_block = 12082794684616777938;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_39: u32 = 0;
                        v_39 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_39 as u8;
                        current_block = 12082794684616777938;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        (*s).storedCombinedCRC = (*s).storedCombinedCRC << 8 | uc as u32;
                        current_block = 12956768993232374699;
                    }
                }
            }
            5201407540844598732 => {
                (*s).state = 22;
                loop {
                    if !(1 != 0) {
                        current_block = 4839309778395429725;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_11: u32 = 0;
                        v_11 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_11 as u8;
                        current_block = 4839309778395429725;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        (*s).storedBlockCRC = (*s).storedBlockCRC << 8 | uc as u32;
                        current_block = 15798322029668445206;
                    }
                }
            }
            _ => {}
        }
        match current_block {
            15798322029668445206 => {
                (*s).state = 23;
                loop {
                    if !(1 != 0) {
                        current_block = 17937968408868551711;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_12: u32 = 0;
                        v_12 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_12 as u8;
                        current_block = 17937968408868551711;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        (*s).storedBlockCRC = (*s).storedBlockCRC << 8 | uc as u32;
                        current_block = 12214724321751295962;
                    }
                }
            }
            12956768993232374699 => {
                (*s).state = 50;
                loop {
                    if !(1 != 0) {
                        current_block = 6276941480907995842;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_40: u32 = 0;
                        v_40 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_40 as u8;
                        current_block = 6276941480907995842;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        (*s).storedCombinedCRC = (*s).storedCombinedCRC << 8 | uc as u32;
                        (*s).state = 1;
                        retVal = 4;
                        current_block = 14281071070443768567;
                    }
                }
            }
            _ => {}
        }
        match current_block {
            12214724321751295962 => {
                (*s).state = 24;
                loop {
                    if !(1 != 0) {
                        current_block = 7926734633677835471;
                        break;
                    }
                    if (*s).bsLive >= 1 {
                        let mut v_13: u32 = 0;
                        v_13 = (*s).bsBuff >> (*s).bsLive - 1 & ((1i32 << 1) - 1) as u32;
                        (*s).bsLive -= 1;
                        (*s).blockRandomised = v_13 as u8;
                        current_block = 7926734633677835471;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        (*s).origPtr = 0;
                        current_block = 8329812343241388296;
                    }
                }
            }
            _ => {}
        }
        match current_block {
            8329812343241388296 => {
                (*s).state = 25;
                loop {
                    if !(1 != 0) {
                        current_block = 5948065351908552372;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_14: u32 = 0;
                        v_14 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_14 as u8;
                        current_block = 5948065351908552372;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        (*s).origPtr = (*s).origPtr << 8 | uc as i32;
                        current_block = 9557502779390786832;
                    }
                }
            }
            _ => {}
        }
        match current_block {
            9557502779390786832 => {
                (*s).state = 26;
                loop {
                    if !(1 != 0) {
                        current_block = 8940662058537996670;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_15: u32 = 0;
                        v_15 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_15 as u8;
                        current_block = 8940662058537996670;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        (*s).origPtr = (*s).origPtr << 8 | uc as i32;
                        current_block = 14254196305078590431;
                    }
                }
            }
            _ => {}
        }
        match current_block {
            14254196305078590431 => {
                (*s).state = 27;
                loop {
                    if !(1 != 0) {
                        current_block = 13366002463409402866;
                        break;
                    }
                    if (*s).bsLive >= 8 {
                        let mut v_16: u32 = 0;
                        v_16 = (*s).bsBuff >> (*s).bsLive - 8 & ((1i32 << 8) - 1) as u32;
                        (*s).bsLive -= 8;
                        uc = v_16 as u8;
                        current_block = 13366002463409402866;
                        break;
                    } else if (*(*s).strm).avail_in == 0 {
                        retVal = 0;
                        current_block = 14281071070443768567;
                        break;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                        (*s).bsLive += 8;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).next_in;
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).avail_in;
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                        (*(*s).strm).total_in_lo32;
                        if (*(*s).strm).total_in_lo32 == 0 {
                            (*(*s).strm).total_in_hi32 =
                                ((*(*s).strm).total_in_hi32).wrapping_add(1);
                            (*(*s).strm).total_in_hi32;
                        }
                    }
                }
                match current_block {
                    14281071070443768567 => {}
                    _ => {
                        (*s).origPtr = (*s).origPtr << 8 | uc as i32;
                        if (*s).origPtr < 0 {
                            retVal = -4;
                            current_block = 14281071070443768567;
                        } else if (*s).origPtr > 10 + 100000 * (*s).blockSize100k {
                            retVal = -4;
                            current_block = 14281071070443768567;
                        } else {
                            i = 0;
                            current_block = 454873545234741267;
                        }
                    }
                }
            }
            _ => {}
        }
        'c_10721: loop {
            match current_block {
                14281071070443768567 => {
                    (*s).save_i = i;
                    break;
                }
                16680933752313891718 => {
                    (*s).state = 40;
                    while 1 != 0 {
                        if (*s).bsLive >= zn {
                            let mut v_30: u32 = 0;
                            v_30 = (*s).bsBuff >> (*s).bsLive - zn & ((1i32 << zn) - 1) as u32;
                            (*s).bsLive -= zn;
                            zvec = v_30 as i32;
                            break;
                        } else if (*(*s).strm).avail_in == 0 {
                            retVal = 0;
                            current_block = 14281071070443768567;
                            continue 'c_10721;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                            (*s).bsLive += 8;
                            (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                            (*(*s).strm).next_in;
                            (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                            (*(*s).strm).avail_in;
                            (*(*s).strm).total_in_lo32 =
                                ((*(*s).strm).total_in_lo32).wrapping_add(1);
                            (*(*s).strm).total_in_lo32;
                            if (*(*s).strm).total_in_lo32 == 0 {
                                (*(*s).strm).total_in_hi32 =
                                    ((*(*s).strm).total_in_hi32).wrapping_add(1);
                                (*(*s).strm).total_in_hi32;
                            }
                        }
                    }
                    current_block = 16348713635569416413;
                }
                17270018966125003511 => {
                    (*s).state = 39;
                    while 1 != 0 {
                        if (*s).bsLive >= 1 {
                            let mut v_29: u32 = 0;
                            v_29 = (*s).bsBuff >> (*s).bsLive - 1 & ((1i32 << 1) - 1) as u32;
                            (*s).bsLive -= 1;
                            zj = v_29 as i32;
                            break;
                        } else if (*(*s).strm).avail_in == 0 {
                            retVal = 0;
                            current_block = 14281071070443768567;
                            continue 'c_10721;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                            (*s).bsLive += 8;
                            (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                            (*(*s).strm).next_in;
                            (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                            (*(*s).strm).avail_in;
                            (*(*s).strm).total_in_lo32 =
                                ((*(*s).strm).total_in_lo32).wrapping_add(1);
                            (*(*s).strm).total_in_lo32;
                            if (*(*s).strm).total_in_lo32 == 0 {
                                (*(*s).strm).total_in_hi32 =
                                    ((*(*s).strm).total_in_hi32).wrapping_add(1);
                                (*(*s).strm).total_in_hi32;
                            }
                        }
                    }
                    zvec = zvec << 1 | zj;
                    current_block = 7923635230025172457;
                }
                7873764389453924156 => {
                    (*s).state = 38;
                    while 1 != 0 {
                        if (*s).bsLive >= zn {
                            let mut v_28: u32 = 0;
                            v_28 = (*s).bsBuff >> (*s).bsLive - zn & ((1i32 << zn) - 1) as u32;
                            (*s).bsLive -= zn;
                            zvec = v_28 as i32;
                            break;
                        } else if (*(*s).strm).avail_in == 0 {
                            retVal = 0;
                            current_block = 14281071070443768567;
                            continue 'c_10721;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                            (*s).bsLive += 8;
                            (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                            (*(*s).strm).next_in;
                            (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                            (*(*s).strm).avail_in;
                            (*(*s).strm).total_in_lo32 =
                                ((*(*s).strm).total_in_lo32).wrapping_add(1);
                            (*(*s).strm).total_in_lo32;
                            if (*(*s).strm).total_in_lo32 == 0 {
                                (*(*s).strm).total_in_hi32 =
                                    ((*(*s).strm).total_in_hi32).wrapping_add(1);
                                (*(*s).strm).total_in_hi32;
                            }
                        }
                    }
                    current_block = 7923635230025172457;
                }
                1004930667005816745 => {
                    (*s).state = 37;
                    while 1 != 0 {
                        if (*s).bsLive >= 1 {
                            let mut v_27: u32 = 0;
                            v_27 = (*s).bsBuff >> (*s).bsLive - 1 & ((1i32 << 1) - 1) as u32;
                            (*s).bsLive -= 1;
                            zj = v_27 as i32;
                            break;
                        } else if (*(*s).strm).avail_in == 0 {
                            retVal = 0;
                            current_block = 14281071070443768567;
                            continue 'c_10721;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                            (*s).bsLive += 8;
                            (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                            (*(*s).strm).next_in;
                            (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                            (*(*s).strm).avail_in;
                            (*(*s).strm).total_in_lo32 =
                                ((*(*s).strm).total_in_lo32).wrapping_add(1);
                            (*(*s).strm).total_in_lo32;
                            if (*(*s).strm).total_in_lo32 == 0 {
                                (*(*s).strm).total_in_hi32 =
                                    ((*(*s).strm).total_in_hi32).wrapping_add(1);
                                (*(*s).strm).total_in_hi32;
                            }
                        }
                    }
                    zvec = zvec << 1 | zj;
                    current_block = 9186389159759284570;
                }
                10254712216801151959 => {
                    (*s).state = 36;
                    while 1 != 0 {
                        if (*s).bsLive >= zn {
                            let mut v_26: u32 = 0;
                            v_26 = (*s).bsBuff >> (*s).bsLive - zn & ((1i32 << zn) - 1) as u32;
                            (*s).bsLive -= zn;
                            zvec = v_26 as i32;
                            break;
                        } else if (*(*s).strm).avail_in == 0 {
                            retVal = 0;
                            current_block = 14281071070443768567;
                            continue 'c_10721;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                            (*s).bsLive += 8;
                            (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                            (*(*s).strm).next_in;
                            (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                            (*(*s).strm).avail_in;
                            (*(*s).strm).total_in_lo32 =
                                ((*(*s).strm).total_in_lo32).wrapping_add(1);
                            (*(*s).strm).total_in_lo32;
                            if (*(*s).strm).total_in_lo32 == 0 {
                                (*(*s).strm).total_in_hi32 =
                                    ((*(*s).strm).total_in_hi32).wrapping_add(1);
                                (*(*s).strm).total_in_hi32;
                            }
                        }
                    }
                    current_block = 9186389159759284570;
                }
                13301436784058847816 => {
                    (*s).state = 35;
                    while 1 != 0 {
                        if (*s).bsLive >= 1 {
                            let mut v_25: u32 = 0;
                            v_25 = (*s).bsBuff >> (*s).bsLive - 1 & ((1i32 << 1) - 1) as u32;
                            (*s).bsLive -= 1;
                            uc = v_25 as u8;
                            break;
                        } else if (*(*s).strm).avail_in == 0 {
                            retVal = 0;
                            current_block = 14281071070443768567;
                            continue 'c_10721;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                            (*s).bsLive += 8;
                            (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                            (*(*s).strm).next_in;
                            (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                            (*(*s).strm).avail_in;
                            (*(*s).strm).total_in_lo32 =
                                ((*(*s).strm).total_in_lo32).wrapping_add(1);
                            (*(*s).strm).total_in_lo32;
                            if (*(*s).strm).total_in_lo32 == 0 {
                                (*(*s).strm).total_in_hi32 =
                                    ((*(*s).strm).total_in_hi32).wrapping_add(1);
                                (*(*s).strm).total_in_hi32;
                            }
                        }
                    }
                    if uc as i32 == 0 {
                        curr += 1;
                        curr;
                    } else {
                        curr -= 1;
                        curr;
                    }
                    current_block = 5533056661327372531;
                }
                3277642653280575212 => {
                    (*s).state = 34;
                    while 1 != 0 {
                        if (*s).bsLive >= 1 {
                            let mut v_24: u32 = 0;
                            v_24 = (*s).bsBuff >> (*s).bsLive - 1 & ((1i32 << 1) - 1) as u32;
                            (*s).bsLive -= 1;
                            uc = v_24 as u8;
                            break;
                        } else if (*(*s).strm).avail_in == 0 {
                            retVal = 0;
                            current_block = 14281071070443768567;
                            continue 'c_10721;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                            (*s).bsLive += 8;
                            (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                            (*(*s).strm).next_in;
                            (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                            (*(*s).strm).avail_in;
                            (*(*s).strm).total_in_lo32 =
                                ((*(*s).strm).total_in_lo32).wrapping_add(1);
                            (*(*s).strm).total_in_lo32;
                            if (*(*s).strm).total_in_lo32 == 0 {
                                (*(*s).strm).total_in_hi32 =
                                    ((*(*s).strm).total_in_hi32).wrapping_add(1);
                                (*(*s).strm).total_in_hi32;
                            }
                        }
                    }
                    if !(uc as i32 == 0) {
                        current_block = 13301436784058847816;
                        continue;
                    }
                    current_block = 7746242308555130918;
                }
                3292440606114030865 => {
                    (*s).state = 33;
                    while 1 != 0 {
                        if (*s).bsLive >= 5 {
                            let mut v_23: u32 = 0;
                            v_23 = (*s).bsBuff >> (*s).bsLive - 5 & ((1i32 << 5) - 1) as u32;
                            (*s).bsLive -= 5;
                            curr = v_23 as i32;
                            break;
                        } else if (*(*s).strm).avail_in == 0 {
                            retVal = 0;
                            current_block = 14281071070443768567;
                            continue 'c_10721;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                            (*s).bsLive += 8;
                            (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                            (*(*s).strm).next_in;
                            (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                            (*(*s).strm).avail_in;
                            (*(*s).strm).total_in_lo32 =
                                ((*(*s).strm).total_in_lo32).wrapping_add(1);
                            (*(*s).strm).total_in_lo32;
                            if (*(*s).strm).total_in_lo32 == 0 {
                                (*(*s).strm).total_in_hi32 =
                                    ((*(*s).strm).total_in_hi32).wrapping_add(1);
                                (*(*s).strm).total_in_hi32;
                            }
                        }
                    }
                    i = 0;
                    current_block = 16642413284942005565;
                }
                16302043309746641865 => {
                    (*s).state = 32;
                    while 1 != 0 {
                        if (*s).bsLive >= 1 {
                            let mut v_21: u32 = 0;
                            v_21 = (*s).bsBuff >> (*s).bsLive - 1 & ((1i32 << 1) - 1) as u32;
                            (*s).bsLive -= 1;
                            uc = v_21 as u8;
                            break;
                        } else if (*(*s).strm).avail_in == 0 {
                            retVal = 0;
                            current_block = 14281071070443768567;
                            continue 'c_10721;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                            (*s).bsLive += 8;
                            (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                            (*(*s).strm).next_in;
                            (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                            (*(*s).strm).avail_in;
                            (*(*s).strm).total_in_lo32 =
                                ((*(*s).strm).total_in_lo32).wrapping_add(1);
                            (*(*s).strm).total_in_lo32;
                            if (*(*s).strm).total_in_lo32 == 0 {
                                (*(*s).strm).total_in_hi32 =
                                    ((*(*s).strm).total_in_hi32).wrapping_add(1);
                                (*(*s).strm).total_in_hi32;
                            }
                        }
                    }
                    if uc as i32 == 0 {
                        current_block = 10081471997089450706;
                    } else {
                        j += 1;
                        j;
                        if j >= nGroups {
                            retVal = -4;
                            current_block = 14281071070443768567;
                            continue;
                        } else {
                            current_block = 16531797892856733396;
                        }
                    }
                }
                6766584878581269858 => {
                    (*s).state = 31;
                    while 1 != 0 {
                        if (*s).bsLive >= 15 {
                            let mut v_20: u32 = 0;
                            v_20 = (*s).bsBuff >> (*s).bsLive - 15 & ((1i32 << 15) - 1) as u32;
                            (*s).bsLive -= 15;
                            nSelectors = v_20 as i32;
                            break;
                        } else if (*(*s).strm).avail_in == 0 {
                            retVal = 0;
                            current_block = 14281071070443768567;
                            continue 'c_10721;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                            (*s).bsLive += 8;
                            (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                            (*(*s).strm).next_in;
                            (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                            (*(*s).strm).avail_in;
                            (*(*s).strm).total_in_lo32 =
                                ((*(*s).strm).total_in_lo32).wrapping_add(1);
                            (*(*s).strm).total_in_lo32;
                            if (*(*s).strm).total_in_lo32 == 0 {
                                (*(*s).strm).total_in_hi32 =
                                    ((*(*s).strm).total_in_hi32).wrapping_add(1);
                                (*(*s).strm).total_in_hi32;
                            }
                        }
                    }
                    if nSelectors < 1 {
                        retVal = -4;
                        current_block = 14281071070443768567;
                        continue;
                    } else {
                        i = 0;
                    }
                    current_block = 3503188808869013853;
                }
                13572320329052790897 => {
                    (*s).state = 30;
                    while 1 != 0 {
                        if (*s).bsLive >= 3 {
                            let mut v_19: u32 = 0;
                            v_19 = (*s).bsBuff >> (*s).bsLive - 3 & ((1i32 << 3) - 1) as u32;
                            (*s).bsLive -= 3;
                            nGroups = v_19 as i32;
                            break;
                        } else if (*(*s).strm).avail_in == 0 {
                            retVal = 0;
                            current_block = 14281071070443768567;
                            continue 'c_10721;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                            (*s).bsLive += 8;
                            (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                            (*(*s).strm).next_in;
                            (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                            (*(*s).strm).avail_in;
                            (*(*s).strm).total_in_lo32 =
                                ((*(*s).strm).total_in_lo32).wrapping_add(1);
                            (*(*s).strm).total_in_lo32;
                            if (*(*s).strm).total_in_lo32 == 0 {
                                (*(*s).strm).total_in_hi32 =
                                    ((*(*s).strm).total_in_hi32).wrapping_add(1);
                                (*(*s).strm).total_in_hi32;
                            }
                        }
                    }
                    if !(nGroups < 2 || nGroups > 6) {
                        current_block = 6766584878581269858;
                        continue;
                    }
                    retVal = -4;
                    current_block = 14281071070443768567;
                    continue;
                }
                15241790405117970060 => {
                    (*s).state = 29;
                    while 1 != 0 {
                        if (*s).bsLive >= 1 {
                            let mut v_18: u32 = 0;
                            v_18 = (*s).bsBuff >> (*s).bsLive - 1 & ((1i32 << 1) - 1) as u32;
                            (*s).bsLive -= 1;
                            uc = v_18 as u8;
                            break;
                        } else if (*(*s).strm).avail_in == 0 {
                            retVal = 0;
                            current_block = 14281071070443768567;
                            continue 'c_10721;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                            (*s).bsLive += 8;
                            (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                            (*(*s).strm).next_in;
                            (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                            (*(*s).strm).avail_in;
                            (*(*s).strm).total_in_lo32 =
                                ((*(*s).strm).total_in_lo32).wrapping_add(1);
                            (*(*s).strm).total_in_lo32;
                            if (*(*s).strm).total_in_lo32 == 0 {
                                (*(*s).strm).total_in_hi32 =
                                    ((*(*s).strm).total_in_hi32).wrapping_add(1);
                                (*(*s).strm).total_in_hi32;
                            }
                        }
                    }
                    if uc as i32 == 1 {
                        (*s).inUse[(i * 16 + j) as usize] = 1;
                    }
                    j += 1;
                    j;
                    current_block = 16953886395775657100;
                }
                454873545234741267 => {
                    if i < 16 {
                        current_block = 12500452284755425741;
                        continue;
                    }
                    i = 0;
                    while i < 256 {
                        (*s).inUse[i as usize] = 0;
                        i += 1;
                        i;
                    }
                    i = 0;
                    current_block = 15415362524153386998;
                }
                12500452284755425741 => {
                    (*s).state = 28;
                    while 1 != 0 {
                        if (*s).bsLive >= 1 {
                            let mut v_17: u32 = 0;
                            v_17 = (*s).bsBuff >> (*s).bsLive - 1 & ((1i32 << 1) - 1) as u32;
                            (*s).bsLive -= 1;
                            uc = v_17 as u8;
                            break;
                        } else if (*(*s).strm).avail_in == 0 {
                            retVal = 0;
                            current_block = 14281071070443768567;
                            continue 'c_10721;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                            (*s).bsLive += 8;
                            (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                            (*(*s).strm).next_in;
                            (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                            (*(*s).strm).avail_in;
                            (*(*s).strm).total_in_lo32 =
                                ((*(*s).strm).total_in_lo32).wrapping_add(1);
                            (*(*s).strm).total_in_lo32;
                            if (*(*s).strm).total_in_lo32 == 0 {
                                (*(*s).strm).total_in_hi32 =
                                    ((*(*s).strm).total_in_hi32).wrapping_add(1);
                                (*(*s).strm).total_in_hi32;
                            }
                        }
                    }
                    if uc as i32 == 1 {
                        (*s).inUse16[i as usize] = 1;
                    } else {
                        (*s).inUse16[i as usize] = 0;
                    }
                    i += 1;
                    i;
                    current_block = 454873545234741267;
                    continue;
                }
                _ => {
                    (*s).state = 41;
                    while 1 != 0 {
                        if (*s).bsLive >= 1 {
                            let mut v_31: u32 = 0;
                            v_31 = (*s).bsBuff >> (*s).bsLive - 1 & ((1i32 << 1) - 1) as u32;
                            (*s).bsLive -= 1;
                            zj = v_31 as i32;
                            break;
                        } else if (*(*s).strm).avail_in == 0 {
                            retVal = 0;
                            current_block = 14281071070443768567;
                            continue 'c_10721;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 | *((*(*s).strm).next_in as *mut u8) as u32;
                            (*s).bsLive += 8;
                            (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                            (*(*s).strm).next_in;
                            (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                            (*(*s).strm).avail_in;
                            (*(*s).strm).total_in_lo32 =
                                ((*(*s).strm).total_in_lo32).wrapping_add(1);
                            (*(*s).strm).total_in_lo32;
                            if (*(*s).strm).total_in_lo32 == 0 {
                                (*(*s).strm).total_in_hi32 =
                                    ((*(*s).strm).total_in_hi32).wrapping_add(1);
                                (*(*s).strm).total_in_hi32;
                            }
                        }
                    }
                    zvec = zvec << 1 | zj;
                    current_block = 16348713635569416413;
                }
            }
            match current_block {
                16348713635569416413 => {
                    if zn > 20 {
                        retVal = -4;
                        current_block = 14281071070443768567;
                        continue;
                    } else if zvec <= *gLimit.offset(zn as isize) {
                        if zvec - *gBase.offset(zn as isize) < 0
                            || zvec - *gBase.offset(zn as isize) >= 258
                        {
                            retVal = -4;
                            current_block = 14281071070443768567;
                            continue;
                        } else {
                            nextSym = *gPerm.offset((zvec - *gBase.offset(zn as isize)) as isize);
                        }
                    } else {
                        zn += 1;
                        zn;
                        current_block = 12564072498539239379;
                        continue;
                    }
                    current_block = 3575340618357869479;
                }
                7923635230025172457 => {
                    if zn > 20 {
                        retVal = -4;
                        current_block = 14281071070443768567;
                        continue;
                    } else if zvec <= *gLimit.offset(zn as isize) {
                        if zvec - *gBase.offset(zn as isize) < 0
                            || zvec - *gBase.offset(zn as isize) >= 258
                        {
                            retVal = -4;
                            current_block = 14281071070443768567;
                            continue;
                        } else {
                            nextSym = *gPerm.offset((zvec - *gBase.offset(zn as isize)) as isize);
                            if nextSym == 0 || nextSym == 1 {
                                current_block = 5649595406143318745;
                            } else {
                                es += 1;
                                es;
                                uc = (*s).seqToUnseq
                                    [(*s).mtfa[(*s).mtfbase[0 as usize] as usize] as usize];
                                (*s).unzftab[uc as usize] += es;
                                if (*s).smallDecompress != 0 {
                                    while es > 0 {
                                        if nblock >= nblockMAX {
                                            retVal = -4;
                                            current_block = 14281071070443768567;
                                            continue 'c_10721;
                                        } else {
                                            *((*s).ll16).offset(nblock as isize) = uc as u16;
                                            nblock += 1;
                                            nblock;
                                            es -= 1;
                                            es;
                                        }
                                    }
                                } else {
                                    while es > 0 {
                                        if nblock >= nblockMAX {
                                            retVal = -4;
                                            current_block = 14281071070443768567;
                                            continue 'c_10721;
                                        } else {
                                            *((*s).tt).offset(nblock as isize) = uc as u32;
                                            nblock += 1;
                                            nblock;
                                            es -= 1;
                                            es;
                                        }
                                    }
                                }
                                current_block = 3575340618357869479;
                            }
                        }
                    } else {
                        zn += 1;
                        zn;
                        current_block = 17270018966125003511;
                        continue;
                    }
                }
                9186389159759284570 => {
                    if zn > 20 {
                        retVal = -4;
                        current_block = 14281071070443768567;
                        continue;
                    } else if zvec <= *gLimit.offset(zn as isize) {
                        if zvec - *gBase.offset(zn as isize) < 0
                            || zvec - *gBase.offset(zn as isize) >= 258
                        {
                            retVal = -4;
                            current_block = 14281071070443768567;
                            continue;
                        } else {
                            nextSym = *gPerm.offset((zvec - *gBase.offset(zn as isize)) as isize);
                        }
                    } else {
                        zn += 1;
                        zn;
                        current_block = 1004930667005816745;
                        continue;
                    }
                    current_block = 3575340618357869479;
                }
                _ => {}
            }
            match current_block {
                3575340618357869479 => {
                    if 1 != 0 {
                        if nextSym == EOB {
                            current_block = 4069074773319880902;
                        } else {
                            if nextSym == 0 || nextSym == 1 {
                                es = -1;
                                N = 1;
                            } else if nblock >= nblockMAX {
                                retVal = -4;
                                current_block = 14281071070443768567;
                                continue;
                            } else {
                                let mut ii_0: i32 = 0;
                                let mut jj_0: i32 = 0;
                                let mut kk_0: i32 = 0;
                                let mut pp: i32 = 0;
                                let mut lno: i32 = 0;
                                let mut off: i32 = 0;
                                let mut nn: u32 = 0;
                                nn = (nextSym - 1i32) as u32;
                                if nn < 16 {
                                    pp = (*s).mtfbase[0 as usize];
                                    uc = (*s).mtfa[(pp as u32).wrapping_add(nn) as usize];
                                    while nn > 3 {
                                        let mut z: i32 = (pp as u32).wrapping_add(nn) as i32;
                                        (*s).mtfa[z as usize] = (*s).mtfa[(z - 1i32) as usize];
                                        (*s).mtfa[(z - 1i32) as usize] =
                                            (*s).mtfa[(z - 2i32) as usize];
                                        (*s).mtfa[(z - 2i32) as usize] =
                                            (*s).mtfa[(z - 3i32) as usize];
                                        (*s).mtfa[(z - 3i32) as usize] =
                                            (*s).mtfa[(z - 4i32) as usize];
                                        nn = nn.wrapping_sub(4);
                                    }
                                    while nn > 0 {
                                        (*s).mtfa[(pp as u32).wrapping_add(nn) as usize] = (*s)
                                            .mtfa
                                            [(pp as u32).wrapping_add(nn).wrapping_sub(1) as usize];
                                        nn = nn.wrapping_sub(1);
                                        nn;
                                    }
                                    (*s).mtfa[pp as usize] = uc;
                                } else {
                                    lno = (nn / 16u32) as i32;
                                    off = (nn % 16u32) as i32;
                                    pp = (*s).mtfbase[lno as usize] + off;
                                    uc = (*s).mtfa[pp as usize];
                                    while pp > (*s).mtfbase[lno as usize] {
                                        (*s).mtfa[pp as usize] = (*s).mtfa[(pp - 1i32) as usize];
                                        pp -= 1;
                                        pp;
                                    }
                                    (*s).mtfbase[lno as usize] += 1;
                                    (*s).mtfbase[lno as usize];
                                    while lno > 0 {
                                        (*s).mtfbase[lno as usize] -= 1;
                                        (*s).mtfbase[lno as usize];
                                        (*s).mtfa[(*s).mtfbase[lno as usize] as usize] = (*s).mtfa
                                            [((*s).mtfbase[(lno - 1i32) as usize] + 16 - 1i32)
                                                as usize];
                                        lno -= 1;
                                        lno;
                                    }
                                    (*s).mtfbase[0 as usize] -= 1;
                                    (*s).mtfbase[0 as usize];
                                    (*s).mtfa[(*s).mtfbase[0 as usize] as usize] = uc;
                                    if (*s).mtfbase[0 as usize] == 0 {
                                        kk_0 = 4096 - 1;
                                        ii_0 = 256 / 16 - 1;
                                        while ii_0 >= 0 {
                                            jj_0 = 16 - 1;
                                            while jj_0 >= 0 {
                                                (*s).mtfa[kk_0 as usize] = (*s).mtfa
                                                    [((*s).mtfbase[ii_0 as usize] + jj_0) as usize];
                                                kk_0 -= 1;
                                                kk_0;
                                                jj_0 -= 1;
                                                jj_0;
                                            }
                                            (*s).mtfbase[ii_0 as usize] = kk_0 + 1;
                                            ii_0 -= 1;
                                            ii_0;
                                        }
                                    }
                                };
                                (*s).unzftab[(*s).seqToUnseq[uc as usize] as usize] += 1;
                                (*s).unzftab[(*s).seqToUnseq[uc as usize] as usize];
                                if (*s).smallDecompress != 0 {
                                    *((*s).ll16).offset(nblock as isize) =
                                        (*s).seqToUnseq[uc as usize] as u16;
                                } else {
                                    *((*s).tt).offset(nblock as isize) =
                                        (*s).seqToUnseq[uc as usize] as u32;
                                }
                                nblock += 1;
                                nblock;
                                if groupPos == 0 {
                                    groupNo += 1;
                                    groupNo;
                                    if groupNo >= nSelectors {
                                        retVal = -4;
                                        current_block = 14281071070443768567;
                                        continue;
                                    } else {
                                        groupPos = 50;
                                        gSel = (*s).selector[groupNo as usize] as i32;
                                        gMinlen = (*s).minLens[gSel as usize];
                                        gLimit = &mut *(*((*s).limit)
                                            .as_mut_ptr()
                                            .offset(gSel as isize))
                                        .as_mut_ptr()
                                        .offset(0 as isize)
                                            as *mut i32;
                                        gPerm =
                                            &mut *(*((*s).perm).as_mut_ptr().offset(gSel as isize))
                                                .as_mut_ptr()
                                                .offset(0 as isize)
                                                as *mut i32;
                                        gBase =
                                            &mut *(*((*s).base).as_mut_ptr().offset(gSel as isize))
                                                .as_mut_ptr()
                                                .offset(0 as isize)
                                                as *mut i32;
                                    }
                                }
                                groupPos -= 1;
                                groupPos;
                                zn = gMinlen;
                                current_block = 16680933752313891718;
                                continue;
                            }
                            current_block = 5649595406143318745;
                        }
                    } else {
                        current_block = 4069074773319880902;
                    }
                    match current_block {
                        5649595406143318745 => {}
                        _ => {
                            if (*s).origPtr < 0 || (*s).origPtr >= nblock {
                                retVal = -4;
                                current_block = 14281071070443768567;
                                continue;
                            } else {
                                i = 0;
                                while i <= 255 {
                                    if (*s).unzftab[i as usize] < 0
                                        || (*s).unzftab[i as usize] > nblock
                                    {
                                        retVal = -4;
                                        current_block = 14281071070443768567;
                                        continue 'c_10721;
                                    } else {
                                        i += 1;
                                        i;
                                    }
                                }
                                (*s).cftab[0 as usize] = 0;
                                i = 1;
                                while i <= 256 {
                                    (*s).cftab[i as usize] = (*s).unzftab[(i - 1i32) as usize];
                                    i += 1;
                                    i;
                                }
                                i = 1;
                                while i <= 256 {
                                    (*s).cftab[i as usize] += (*s).cftab[(i - 1i32) as usize];
                                    i += 1;
                                    i;
                                }
                                i = 0;
                                while i <= 256 {
                                    if (*s).cftab[i as usize] < 0 || (*s).cftab[i as usize] > nblock
                                    {
                                        retVal = -4;
                                        current_block = 14281071070443768567;
                                        continue 'c_10721;
                                    } else {
                                        i += 1;
                                        i;
                                    }
                                }
                                i = 1;
                                while i <= 256 {
                                    if (*s).cftab[(i - 1i32) as usize] > (*s).cftab[i as usize] {
                                        retVal = -4;
                                        current_block = 14281071070443768567;
                                        continue 'c_10721;
                                    } else {
                                        i += 1;
                                        i;
                                    }
                                }
                                (*s).state_out_len = 0;
                                (*s).state_out_ch = 0;
                                (*s).calculatedBlockCRC = 0xffffffff;
                                (*s).state = 2;
                                if (*s).verbosity >= 2 {
                                    fprintf(__stderrp, b"rt+rld\0" as *const u8 as *const i8);
                                }
                                if (*s).smallDecompress != 0 {
                                    i = 0;
                                    while i <= 256 {
                                        (*s).cftabCopy[i as usize] = (*s).cftab[i as usize];
                                        i += 1;
                                        i;
                                    }
                                    i = 0;
                                    while i < nblock {
                                        uc = *((*s).ll16).offset(i as isize) as u8;
                                        *((*s).ll16).offset(i as isize) =
                                            ((*s).cftabCopy[uc as usize] & 0xffffi32) as u16;
                                        if i & 0x1 == 0 {
                                            *((*s).ll4).offset((i >> 1i32) as isize) =
                                                (*((*s).ll4).offset((i >> 1i32) as isize) as i32
                                                    & 0xf0
                                                    | (*s).cftabCopy[uc as usize] >> 16i32)
                                                    as u8;
                                        } else {
                                            *((*s).ll4).offset((i >> 1i32) as isize) =
                                                (*((*s).ll4).offset((i >> 1i32) as isize) as i32
                                                    & 0xf
                                                    | ((*s).cftabCopy[uc as usize] >> 16i32) << 4)
                                                    as u8;
                                        };
                                        (*s).cftabCopy[uc as usize] += 1;
                                        (*s).cftabCopy[uc as usize];
                                        i += 1;
                                        i;
                                    }
                                    i = (*s).origPtr;
                                    j = (*((*s).ll16).offset(i as isize) as u32
                                        | (*((*s).ll4).offset((i >> 1i32) as isize) as u32
                                            >> (i << 2 & 0x4i32)
                                            & 0xf)
                                            << 16) as i32;
                                    loop {
                                        let mut tmp_0: i32 = (*((*s).ll16).offset(j as isize)
                                            as u32
                                            | (*((*s).ll4).offset((j >> 1i32) as isize) as u32
                                                >> (j << 2 & 0x4i32)
                                                & 0xf)
                                                << 16)
                                            as i32;
                                        *((*s).ll16).offset(j as isize) = (i & 0xffffi32) as u16;
                                        if j & 0x1 == 0 {
                                            *((*s).ll4).offset((j >> 1i32) as isize) =
                                                (*((*s).ll4).offset((j >> 1i32) as isize) as i32
                                                    & 0xf0
                                                    | i >> 16i32)
                                                    as u8;
                                        } else {
                                            *((*s).ll4).offset((j >> 1i32) as isize) =
                                                (*((*s).ll4).offset((j >> 1i32) as isize) as i32
                                                    & 0xf
                                                    | (i >> 16i32) << 4)
                                                    as u8;
                                        }
                                        i = j;
                                        j = tmp_0;
                                        if !(i != (*s).origPtr) {
                                            break;
                                        }
                                    }
                                    (*s).tPos = (*s).origPtr as u32;
                                    (*s).nblock_used = 0;
                                    if (*s).blockRandomised != 0 {
                                        (*s).rNToGo = 0;
                                        (*s).rTPos = 0;
                                        if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                                            return 1;
                                        };
                                        (*s).k0 = BZ2_indexIntoF(
                                            (*s).tPos as i32,
                                            ((*s).cftab).as_mut_ptr(),
                                        );
                                        (*s).tPos = *((*s).ll16).offset((*s).tPos as isize) as u32
                                            | (*((*s).ll4).offset(((*s).tPos >> 1i32) as isize)
                                                as u32
                                                >> ((*s).tPos << 2 & 0x4)
                                                & 0xf)
                                                << 16;
                                        (*s).nblock_used += 1;
                                        (*s).nblock_used;
                                        if (*s).rNToGo == 0 {
                                            (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                                            (*s).rTPos += 1;
                                            (*s).rTPos;
                                            if (*s).rTPos == 512 {
                                                (*s).rTPos = 0;
                                            }
                                        };
                                        (*s).rNToGo -= 1;
                                        (*s).rNToGo;
                                        (*s).k0 ^= if (*s).rNToGo == 1 { 1 } else { 0 };
                                    } else {
                                        if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                                            return 1;
                                        };
                                        (*s).k0 = BZ2_indexIntoF(
                                            (*s).tPos as i32,
                                            ((*s).cftab).as_mut_ptr(),
                                        );
                                        (*s).tPos = *((*s).ll16).offset((*s).tPos as isize) as u32
                                            | (*((*s).ll4).offset(((*s).tPos >> 1i32) as isize)
                                                as u32
                                                >> ((*s).tPos << 2 & 0x4)
                                                & 0xf)
                                                << 16;
                                        (*s).nblock_used += 1;
                                        (*s).nblock_used;
                                    }
                                } else {
                                    i = 0;
                                    while i < nblock {
                                        uc = (*((*s).tt).offset(i as isize) & 0xffu32) as u8;
                                        *((*s).tt).offset((*s).cftab[uc as usize] as isize) |=
                                            (i << 8i32) as u32;
                                        (*s).cftab[uc as usize] += 1;
                                        (*s).cftab[uc as usize];
                                        i += 1;
                                        i;
                                    }
                                    (*s).tPos = *((*s).tt).offset((*s).origPtr as isize) >> 8;
                                    (*s).nblock_used = 0;
                                    if (*s).blockRandomised != 0 {
                                        (*s).rNToGo = 0;
                                        (*s).rTPos = 0;
                                        if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                                            return 1;
                                        };
                                        (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                                        (*s).k0 = ((*s).tPos & 0xffu32) as i32;
                                        (*s).tPos >>= 8;
                                        (*s).nblock_used += 1;
                                        (*s).nblock_used;
                                        if (*s).rNToGo == 0 {
                                            (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                                            (*s).rTPos += 1;
                                            (*s).rTPos;
                                            if (*s).rTPos == 512 {
                                                (*s).rTPos = 0;
                                            }
                                        };
                                        (*s).rNToGo -= 1;
                                        (*s).rNToGo;
                                        (*s).k0 ^= if (*s).rNToGo == 1 { 1 } else { 0 };
                                    } else {
                                        if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                                            return 1;
                                        };
                                        (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                                        (*s).k0 = ((*s).tPos & 0xffu32) as i32;
                                        (*s).tPos >>= 8;
                                        (*s).nblock_used += 1;
                                        (*s).nblock_used;
                                    }
                                }
                                retVal = 0;
                                current_block = 14281071070443768567;
                                continue;
                            }
                        }
                    }
                }
                _ => {}
            }
            match current_block {
                5649595406143318745 => {
                    if N >= 2 * 1024 * 1024 {
                        retVal = -4;
                        current_block = 14281071070443768567;
                        continue;
                    } else {
                        if nextSym == 0 {
                            es = es + (0 + 1) * N;
                        } else if nextSym == 1 {
                            es = es + (1 + 1) * N;
                        }
                        N = N * 2;
                        if groupPos == 0 {
                            groupNo += 1;
                            groupNo;
                            if groupNo >= nSelectors {
                                retVal = -4;
                                current_block = 14281071070443768567;
                                continue;
                            } else {
                                groupPos = 50;
                                gSel = (*s).selector[groupNo as usize] as i32;
                                gMinlen = (*s).minLens[gSel as usize];
                                gLimit = &mut *(*((*s).limit).as_mut_ptr().offset(gSel as isize))
                                    .as_mut_ptr()
                                    .offset(0 as isize)
                                    as *mut i32;
                                gPerm = &mut *(*((*s).perm).as_mut_ptr().offset(gSel as isize))
                                    .as_mut_ptr()
                                    .offset(0 as isize)
                                    as *mut i32;
                                gBase = &mut *(*((*s).base).as_mut_ptr().offset(gSel as isize))
                                    .as_mut_ptr()
                                    .offset(0 as isize)
                                    as *mut i32;
                            }
                        }
                        groupPos -= 1;
                        groupPos;
                        zn = gMinlen;
                        current_block = 7873764389453924156;
                        continue;
                    }
                }
                _ => {}
            }
            loop {
                match current_block {
                    16953886395775657100 => {
                        if j < 16 {
                            current_block = 15241790405117970060;
                            continue 'c_10721;
                        }
                    }
                    3503188808869013853 => {
                        if i < nSelectors {
                            j = 0;
                            current_block = 16531797892856733396;
                            continue;
                        } else {
                            if nSelectors > 2 + 900000 / 50 {
                                nSelectors = 2 + 900000 / 50;
                            }
                            let mut pos: [u8; 6] = [0; 6];
                            let mut tmp: u8 = 0;
                            let mut v_22: u8 = 0;
                            v_22 = 0;
                            while (v_22 as i32) < nGroups {
                                pos[v_22 as usize] = v_22;
                                v_22 = v_22.wrapping_add(1);
                                v_22;
                            }
                            i = 0;
                            while i < nSelectors {
                                v_22 = (*s).selectorMtf[i as usize];
                                tmp = pos[v_22 as usize];
                                while v_22 as i32 > 0 {
                                    pos[v_22 as usize] = pos[(v_22 as i32 - 1i32) as usize];
                                    v_22 = v_22.wrapping_sub(1);
                                    v_22;
                                }
                                pos[0 as usize] = tmp;
                                (*s).selector[i as usize] = tmp;
                                i += 1;
                                i;
                            }
                            t = 0;
                            current_block = 2488856075421756534;
                            break;
                        }
                    }
                    15415362524153386998 => {
                        if i < 16 {
                            if (*s).inUse16[i as usize] != 0 {
                                j = 0;
                                current_block = 16953886395775657100;
                                continue;
                            }
                        } else {
                            makeMaps_d(s);
                            if (*s).nInUse == 0 {
                                current_block = 12571193857528100212;
                                break;
                            } else {
                                current_block = 9416928054198617439;
                                break;
                            }
                        }
                    }
                    7746242308555130918 => {
                        (*s).len[t as usize][i as usize] = curr as u8;
                        i += 1;
                        i;
                        current_block = 16642413284942005565;
                        continue;
                    }
                    16642413284942005565 => {
                        if i < alphaSize {
                            current_block = 5533056661327372531;
                            continue;
                        }
                        t += 1;
                        t;
                        current_block = 2488856075421756534;
                        break;
                    }
                    10081471997089450706 => {
                        if i < 2 + 900000 / 50 {
                            (*s).selectorMtf[i as usize] = j as u8;
                        }
                        i += 1;
                        i;
                        current_block = 3503188808869013853;
                        continue;
                    }
                    16531797892856733396 => {
                        if 1 != 0 {
                            current_block = 16302043309746641865;
                            continue 'c_10721;
                        } else {
                            current_block = 10081471997089450706;
                            continue;
                        }
                    }
                    _ => {
                        if !(1 != 0) {
                            current_block = 7746242308555130918;
                            continue;
                        }
                        if !(curr < 1 || curr > 20) {
                            current_block = 3277642653280575212;
                            continue 'c_10721;
                        }
                        retVal = -4;
                        current_block = 14281071070443768567;
                        continue 'c_10721;
                    }
                }
                i += 1;
                i;
                current_block = 15415362524153386998;
            }
            match current_block {
                9416928054198617439 => {
                    alphaSize = (*s).nInUse + 2;
                    current_block = 13572320329052790897;
                }
                12571193857528100212 => {
                    retVal = -4;
                    current_block = 14281071070443768567;
                }
                _ => {
                    if t < nGroups {
                        current_block = 3292440606114030865;
                        continue;
                    }
                    t = 0;
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
                        BZ2_hbCreateDecodeTables(
                            &mut *(*((*s).limit).as_mut_ptr().offset(t as isize))
                                .as_mut_ptr()
                                .offset(0 as isize),
                            &mut *(*((*s).base).as_mut_ptr().offset(t as isize))
                                .as_mut_ptr()
                                .offset(0 as isize),
                            &mut *(*((*s).perm).as_mut_ptr().offset(t as isize))
                                .as_mut_ptr()
                                .offset(0 as isize),
                            &mut *(*((*s).len).as_mut_ptr().offset(t as isize))
                                .as_mut_ptr()
                                .offset(0 as isize),
                            minLen,
                            maxLen,
                            alphaSize,
                        );
                        (*s).minLens[t as usize] = minLen;
                        t += 1;
                        t;
                    }
                    EOB = (*s).nInUse + 1;
                    nblockMAX = 100000 * (*s).blockSize100k;
                    groupNo = -1;
                    groupPos = 0;
                    i = 0;
                    while i <= 255 {
                        (*s).unzftab[i as usize] = 0;
                        i += 1;
                        i;
                    }
                    let mut ii: i32 = 0;
                    let mut jj: i32 = 0;
                    let mut kk: i32 = 0;
                    kk = 4096 - 1;
                    ii = 256 / 16 - 1;
                    while ii >= 0 {
                        jj = 16 - 1;
                        while jj >= 0 {
                            (*s).mtfa[kk as usize] = (ii * 16 + jj) as u8;
                            kk -= 1;
                            kk;
                            jj -= 1;
                            jj;
                        }
                        (*s).mtfbase[ii as usize] = kk + 1;
                        ii -= 1;
                        ii;
                    }
                    nblock = 0;
                    if groupPos == 0 {
                        groupNo += 1;
                        groupNo;
                        if groupNo >= nSelectors {
                            retVal = -4;
                            current_block = 14281071070443768567;
                            continue;
                        } else {
                            groupPos = 50;
                            gSel = (*s).selector[groupNo as usize] as i32;
                            gMinlen = (*s).minLens[gSel as usize];
                            gLimit = &mut *(*((*s).limit).as_mut_ptr().offset(gSel as isize))
                                .as_mut_ptr()
                                .offset(0 as isize)
                                as *mut i32;
                            gPerm = &mut *(*((*s).perm).as_mut_ptr().offset(gSel as isize))
                                .as_mut_ptr()
                                .offset(0 as isize) as *mut i32;
                            gBase = &mut *(*((*s).base).as_mut_ptr().offset(gSel as isize))
                                .as_mut_ptr()
                                .offset(0 as isize) as *mut i32;
                        }
                    }
                    groupPos -= 1;
                    groupPos;
                    zn = gMinlen;
                    current_block = 10254712216801151959;
                }
            }
        }
        (*s).save_j = j;
    }
    (*s).save_t = t;
    (*s).save_alphaSize = alphaSize;
    (*s).save_nGroups = nGroups;
    (*s).save_nSelectors = nSelectors;
    (*s).save_EOB = EOB;
    (*s).save_groupNo = groupNo;
    (*s).save_groupPos = groupPos;
    (*s).save_nextSym = nextSym;
    (*s).save_nblockMAX = nblockMAX;
    (*s).save_nblock = nblock;
    (*s).save_es = es;
    (*s).save_N = N;
    (*s).save_curr = curr;
    (*s).save_zt = zt;
    (*s).save_zn = zn;
    (*s).save_zvec = zvec;
    (*s).save_zj = zj;
    (*s).save_gSel = gSel;
    (*s).save_gMinlen = gMinlen;
    (*s).save_gLimit = gLimit;
    (*s).save_gBase = gBase;
    (*s).save_gPerm = gPerm;
    return retVal;
}
