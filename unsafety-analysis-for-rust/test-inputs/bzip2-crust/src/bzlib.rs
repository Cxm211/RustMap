//use libc;
extern "C" {
    pub type __sFILEX;
    static mut __stdinp: *mut FILE;
    static mut __stdoutp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn fclose(_: *mut FILE) -> i32;
    fn ferror(_: *mut FILE) -> i32;
    fn fflush(_: *mut FILE) -> i32;
    fn fgetc(_: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fread(_: *mut std::os::raw::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn malloc(_: u64) -> *mut std::os::raw::c_void;
    fn free(_: *mut std::os::raw::c_void);
    fn exit(_: i32) -> !;
    fn fwrite(_: *const std::os::raw::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn ungetc(_: i32, _: *mut FILE) -> i32;
    fn fdopen(_: i32, _: *const i8) -> *mut FILE;
    static mut _DefaultRuneLocale: _RuneLocale;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn BZ2_compressBlock(_: *mut EState, _: u8);
    static mut BZ2_crc32Table: [u32; 256];
    fn BZ2_decompress(_: *mut DState) -> i32;
    static mut BZ2_rNums: [i32; 512];
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
pub struct _RuneEntry {
    pub __min: i32,
    pub __max: i32,
    pub __map: i32,
    pub __types: *mut u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: i32,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [i8; 14],
    pub __mask: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [i8; 8],
    pub __encoding: [i8; 32],
    pub __sgetrune: Option<unsafe extern "C" fn(*const i8, u64, *mut *const i8) -> i32>,
    pub __sputrune: Option<unsafe extern "C" fn(i32, *mut i8, u64, *mut *mut i8) -> i32>,
    pub __invalid_rune: i32,
    pub __runetype: [u32; 256],
    pub __maplower: [i32; 256],
    pub __mapupper: [i32; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut std::os::raw::c_void,
    pub __variable_len: i32,
    pub __ncharclasses: i32,
    pub __charclasses: *mut _RuneCharClass,
}
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
pub type BZFILE = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bzFile {
    pub handle: *mut FILE,
    pub buf: [i8; 5000],
    pub bufN: i32,
    pub writing: u8,
    pub strm: bz_stream,
    pub lastErr: i32,
    pub initialisedOk: u8,
}
#[inline]
extern "C" fn __isctype(mut _c: i32, mut _f: u64) -> i32 {
    unsafe {
        return if _c < 0 || _c >= 1 << 8 {
            0
        } else {
            (_DefaultRuneLocale.__runetype[_c as usize] as u64 & _f != 0) as i32
        };
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub extern "C" fn isdigit(mut _c: i32) -> i32 {
    return __isctype(_c, 0x400);
}

#[no_mangle]
pub extern "C" fn BZ2_bz__AssertH__fail(mut errcode: i32) {
    unsafe {
        fprintf (__stderrp,
b"\n\nbzip2/libbzip2: internal error number %d.\nThis is a bug in bzip2/libbzip2, %s.\nPlease report it to: bzip2-devel@sourceware.org.  If this happened\nwhen you were using some program which uses libbzip2 as a\ncomponent, you should also report this bug to the author(s)\nof that program.  Please make an effort to report this bug;\ntimely and accurate bug reports eventually lead to higher\nquality software.  Thanks.\n\n\0"
         as * const u8 as * const i8, errcode, BZ2_bzlibVersion (),);
        if errcode == 1007 {
            fprintf (__stderrp,
b"\n*** A special note about internal error number 1007 ***\n\nExperience suggests that a common cause of i.e. 1007\nis unreliable memory or other hardware.  The 1007 assertion\njust happens to cross-check the results of huge numbers of\nmemory reads/writes, and so acts (unintendedly) as a stress\ntest of your memory system.\n\nI suggest the following: try compressing the file again,\npossibly monitoring progress in detail with the -vv flag.\n\n* If the error cannot be reproduced, and/or happens at different\n  points in compression, you may have a flaky memory system.\n  Try a memory-test program.  I have used Memtest86\n  (www.memtest86.com).  At the time of writing it is free (GPLd).\n  Memtest86 tests memory much more thorougly than your BIOSs\n  power-on test, and may find failures that the BIOS doesn't.\n\n* If the error can be repeatably reproduced, this is a bug in\n  bzip2, and I would very much like to hear about it.  Please\n  let me know, and, ideally, save a copy of the file causing the\n  problem -- without which I will be unable to investigate it.\n\n\0"
             as * const u8 as * const i8,);
        }
        exit(3);
    }
}

extern "C" fn bz_config_ok() -> i32 {
    if ::std::mem::size_of::<i32>() as u64 != 4 {
        return 0;
    }
    if ::std::mem::size_of::<i16>() as u64 != 2 {
        return 0;
    }
    if ::std::mem::size_of::<i8>() as u64 != 1 {
        return 0;
    }
    return 1;
}

extern "C" fn default_bzalloc(
    mut opaque: *mut std::os::raw::c_void,
    mut items: i32,
    mut size: i32,
) -> *mut std::os::raw::c_void {
    unsafe {
        let mut v: *mut std::os::raw::c_void = malloc((items * size) as u64);
        return v;
    }
}

extern "C" fn default_bzfree(mut opaque: *mut std::os::raw::c_void, mut addr: *mut std::os::raw::c_void) {
    unsafe {
        if !addr.is_null() {
            free(addr);
        }
    }
}

extern "C" fn prepare_new_block(mut s: *mut EState) {
    let mut i: i32 = 0;
    (*s).nblock = 0;
    (*s).numZ = 0;
    (*s).state_out_pos = 0;
    (*s).blockCRC = 0xffffffff;
    i = 0;
    while i < 256 {
        (*s).inUse[i as usize] = 0;
        i += 1;
        i;
    }
    (*s).blockNo += 1;
    (*s).blockNo;
}

extern "C" fn init_RL(mut s: *mut EState) {
    (*s).state_in_ch = 256;
    (*s).state_in_len = 0;
}

extern "C" fn isempty_RL(mut s: *mut EState) -> u8 {
    if (*s).state_in_ch < 256 && (*s).state_in_len > 0 {
        return 0;
    } else {
        return 1;
    };
}

#[no_mangle]
pub extern "C" fn BZ2_bzCompressInit(
    mut strm: *mut bz_stream,
    mut blockSize100k: i32,
    mut verbosity: i32,
    mut workFactor: i32,
) -> i32 {
    let mut n: i32 = 0;
    let mut s: *mut EState = 0 as *mut EState;
    if bz_config_ok() == 0 {
        return -9;
    }
    if strm.is_null()
        || blockSize100k < 1
        || blockSize100k > 9
        || workFactor < 0
        || workFactor > 250
    {
        return -2;
    }
    if workFactor == 0 {
        workFactor = 30;
    }
    if ((*strm).bzalloc).is_none() {
        (*strm).bzalloc = Some(
            default_bzalloc
                as unsafe extern "C" fn(*mut std::os::raw::c_void, i32, i32) -> *mut std::os::raw::c_void,
        );
    }
    if ((*strm).bzfree).is_none() {
        (*strm).bzfree = Some(
            default_bzfree as unsafe extern "C" fn(*mut std::os::raw::c_void, *mut std::os::raw::c_void) -> (),
        );
    }
    s = ((*strm).bzalloc).expect("non-null function pointer")(
        (*strm).opaque,
        ::std::mem::size_of::<EState>() as i32,
        1,
    ) as *mut EState;
    if s.is_null() {
        return -3;
    };
    (*s).strm = strm;
    (*s).arr1 = 0 as *mut u32;
    (*s).arr2 = 0 as *mut u32;
    (*s).ftab = 0 as *mut u32;
    n = 100000 * blockSize100k;
    (*s).arr1 = ((*strm).bzalloc).expect("non-null function pointer")(
        (*strm).opaque,
        (n as u64).wrapping_mul(::std::mem::size_of::<u32>() as u64) as i32,
        1,
    ) as *mut u32;
    (*s).arr2 = ((*strm).bzalloc).expect("non-null function pointer")(
        (*strm).opaque,
        ((n + (2 + 12 + 18 + 2i32)) as u64).wrapping_mul(::std::mem::size_of::<u32>() as u64)
            as i32,
        1,
    ) as *mut u32;
    (*s).ftab = ((*strm).bzalloc).expect("non-null function pointer")(
        (*strm).opaque,
        65537u64.wrapping_mul(::std::mem::size_of::<u32>() as u64) as i32,
        1,
    ) as *mut u32;
    if ((*s).arr1).is_null() || ((*s).arr2).is_null() || ((*s).ftab).is_null() {
        if !((*s).arr1).is_null() {
            ((*strm).bzfree).expect("non-null function pointer")(
                (*strm).opaque,
                (*s).arr1 as *mut std::os::raw::c_void,
            );
        }
        if !((*s).arr2).is_null() {
            ((*strm).bzfree).expect("non-null function pointer")(
                (*strm).opaque,
                (*s).arr2 as *mut std::os::raw::c_void,
            );
        }
        if !((*s).ftab).is_null() {
            ((*strm).bzfree).expect("non-null function pointer")(
                (*strm).opaque,
                (*s).ftab as *mut std::os::raw::c_void,
            );
        }
        if !s.is_null() {
            ((*strm).bzfree).expect("non-null function pointer")(
                (*strm).opaque,
                s as *mut std::os::raw::c_void,
            );
        }
        return -3;
    };
    (*s).blockNo = 0;
    (*s).state = 2;
    (*s).mode = 2;
    (*s).combinedCRC = 0;
    (*s).blockSize100k = blockSize100k;
    (*s).nblockMAX = 100000 * blockSize100k - 19;
    (*s).verbosity = verbosity;
    (*s).workFactor = workFactor;
    (*s).block = (*s).arr2 as *mut u8;
    (*s).mtfv = (*s).arr1 as *mut u16;
    (*s).zbits = 0 as *mut u8;
    (*s).ptr = (*s).arr1;
    (*strm).state = s as *mut std::os::raw::c_void;
    (*strm).total_in_lo32 = 0;
    (*strm).total_in_hi32 = 0;
    (*strm).total_out_lo32 = 0;
    (*strm).total_out_hi32 = 0;
    init_RL(s);
    prepare_new_block(s);
    return 0;
}

extern "C" fn add_pair_to_block(mut s: *mut EState) {
    let mut i: i32 = 0;
    let mut ch: u8 = (*s).state_in_ch as u8;
    i = 0;
    unsafe {
        while i < (*s).state_in_len {
            (*s).blockCRC =
                (*s).blockCRC << 8 ^ BZ2_crc32Table[((*s).blockCRC >> 24 ^ ch as u32) as usize];
            i += 1;
            i;
        }
        (*s).inUse[(*s).state_in_ch as usize] = 1;
        match (*s).state_in_len {
            1 => {
                *((*s).block).offset((*s).nblock as isize) = ch;
                (*s).nblock += 1;
                (*s).nblock;
            }
            2 => {
                *((*s).block).offset((*s).nblock as isize) = ch;
                (*s).nblock += 1;
                (*s).nblock;
                *((*s).block).offset((*s).nblock as isize) = ch;
                (*s).nblock += 1;
                (*s).nblock;
            }
            3 => {
                *((*s).block).offset((*s).nblock as isize) = ch;
                (*s).nblock += 1;
                (*s).nblock;
                *((*s).block).offset((*s).nblock as isize) = ch;
                (*s).nblock += 1;
                (*s).nblock;
                *((*s).block).offset((*s).nblock as isize) = ch;
                (*s).nblock += 1;
                (*s).nblock;
            }
            _ => {
                (*s).inUse[((*s).state_in_len - 4i32) as usize] = 1;
                *((*s).block).offset((*s).nblock as isize) = ch;
                (*s).nblock += 1;
                (*s).nblock;
                *((*s).block).offset((*s).nblock as isize) = ch;
                (*s).nblock += 1;
                (*s).nblock;
                *((*s).block).offset((*s).nblock as isize) = ch;
                (*s).nblock += 1;
                (*s).nblock;
                *((*s).block).offset((*s).nblock as isize) = ch;
                (*s).nblock += 1;
                (*s).nblock;
                *((*s).block).offset((*s).nblock as isize) = ((*s).state_in_len - 4i32) as u8;
                (*s).nblock += 1;
                (*s).nblock;
            }
        };
    }
}

extern "C" fn flush_RL(mut s: *mut EState) {
    if (*s).state_in_ch < 256 {
        add_pair_to_block(s);
    }
    init_RL(s);
}

extern "C" fn copy_input_until_stop(mut s: *mut EState) -> u8 {
    let mut progress_in: u8 = 0;
    unsafe {
        if (*s).mode == 2 {
            while 1 != 0 {
                if (*s).nblock >= (*s).nblockMAX {
                    break;
                }
                if (*(*s).strm).avail_in == 0 {
                    break;
                }
                progress_in = 1;
                let mut zchh: u32 = *((*(*s).strm).next_in as *mut u8) as u32;
                if zchh != (*s).state_in_ch && (*s).state_in_len == 1 {
                    let mut ch: u8 = (*s).state_in_ch as u8;
                    (*s).blockCRC = (*s).blockCRC << 8
                        ^ BZ2_crc32Table[((*s).blockCRC >> 24 ^ ch as u32) as usize];
                    (*s).inUse[(*s).state_in_ch as usize] = 1;
                    *((*s).block).offset((*s).nblock as isize) = ch;
                    (*s).nblock += 1;
                    (*s).nblock;
                    (*s).state_in_ch = zchh;
                } else if zchh != (*s).state_in_ch || (*s).state_in_len == 255 {
                    if (*s).state_in_ch < 256 {
                        add_pair_to_block(s);
                    };
                    (*s).state_in_ch = zchh;
                    (*s).state_in_len = 1;
                } else {
                    (*s).state_in_len += 1;
                    (*s).state_in_len;
                };
                (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                (*(*s).strm).next_in;
                (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                (*(*s).strm).avail_in;
                (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                (*(*s).strm).total_in_lo32;
                if (*(*s).strm).total_in_lo32 == 0 {
                    (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32).wrapping_add(1);
                    (*(*s).strm).total_in_hi32;
                }
            }
        } else {
            while 1 != 0 {
                if (*s).nblock >= (*s).nblockMAX {
                    break;
                }
                if (*(*s).strm).avail_in == 0 {
                    break;
                }
                if (*s).avail_in_expect == 0 {
                    break;
                }
                progress_in = 1;
                let mut zchh_0: u32 = *((*(*s).strm).next_in as *mut u8) as u32;
                if zchh_0 != (*s).state_in_ch && (*s).state_in_len == 1 {
                    let mut ch_0: u8 = (*s).state_in_ch as u8;
                    (*s).blockCRC = (*s).blockCRC << 8
                        ^ BZ2_crc32Table[((*s).blockCRC >> 24 ^ ch_0 as u32) as usize];
                    (*s).inUse[(*s).state_in_ch as usize] = 1;
                    *((*s).block).offset((*s).nblock as isize) = ch_0;
                    (*s).nblock += 1;
                    (*s).nblock;
                    (*s).state_in_ch = zchh_0;
                } else if zchh_0 != (*s).state_in_ch || (*s).state_in_len == 255 {
                    if (*s).state_in_ch < 256 {
                        add_pair_to_block(s);
                    };
                    (*s).state_in_ch = zchh_0;
                    (*s).state_in_len = 1;
                } else {
                    (*s).state_in_len += 1;
                    (*s).state_in_len;
                };
                (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                (*(*s).strm).next_in;
                (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                (*(*s).strm).avail_in;
                (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
                (*(*s).strm).total_in_lo32;
                if (*(*s).strm).total_in_lo32 == 0 {
                    (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32).wrapping_add(1);
                    (*(*s).strm).total_in_hi32;
                };
                (*s).avail_in_expect = ((*s).avail_in_expect).wrapping_sub(1);
                (*s).avail_in_expect;
            }
        }
    }
    return progress_in;
}

extern "C" fn copy_output_until_stop(mut s: *mut EState) -> u8 {
    let mut progress_out: u8 = 0;
    unsafe {
        while 1 != 0 {
            if (*(*s).strm).avail_out == 0 {
                break;
            }
            if (*s).state_out_pos >= (*s).numZ {
                break;
            }
            progress_out = 1;
            *(*(*s).strm).next_out = *((*s).zbits).offset((*s).state_out_pos as isize) as i8;
            (*s).state_out_pos += 1;
            (*s).state_out_pos;
            (*(*s).strm).avail_out = ((*(*s).strm).avail_out).wrapping_sub(1);
            (*(*s).strm).avail_out;
            (*(*s).strm).next_out = ((*(*s).strm).next_out).offset(1);
            (*(*s).strm).next_out;
            (*(*s).strm).total_out_lo32 = ((*(*s).strm).total_out_lo32).wrapping_add(1);
            (*(*s).strm).total_out_lo32;
            if (*(*s).strm).total_out_lo32 == 0 {
                (*(*s).strm).total_out_hi32 = ((*(*s).strm).total_out_hi32).wrapping_add(1);
                (*(*s).strm).total_out_hi32;
            }
        }
    }
    return progress_out;
}

extern "C" fn handle_compress(mut strm: *mut bz_stream) -> u8 {
    let mut progress_in: u8 = 0;
    let mut progress_out: u8 = 0;
    let mut s: *mut EState = (*strm).state as *mut EState;
    unsafe {
        while 1 != 0 {
            if (*s).state == 1 {
                progress_out = (progress_out as i32 | copy_output_until_stop(s) as i32) as u8;
                if (*s).state_out_pos < (*s).numZ {
                    break;
                }
                if (*s).mode == 4 && (*s).avail_in_expect == 0 && isempty_RL(s) as i32 != 0 {
                    break;
                }
                prepare_new_block(s);
                (*s).state = 2;
                if (*s).mode == 3 && (*s).avail_in_expect == 0 && isempty_RL(s) as i32 != 0 {
                    break;
                }
            }
            if !((*s).state == 2) {
                continue;
            }
            progress_in = (progress_in as i32 | copy_input_until_stop(s) as i32) as u8;
            if (*s).mode != 2 && (*s).avail_in_expect == 0 {
                flush_RL(s);
                BZ2_compressBlock(s, ((*s).mode == 4) as u8);
                (*s).state = 1;
            } else if (*s).nblock >= (*s).nblockMAX {
                BZ2_compressBlock(s, 0);
                (*s).state = 1;
            } else if (*(*s).strm).avail_in == 0 {
                break;
            }
        }
    }
    return (progress_in as i32 != 0 || progress_out as i32 != 0) as u8;
}

#[no_mangle]
pub extern "C" fn BZ2_bzCompress(mut strm: *mut bz_stream, mut action: i32) -> i32 {
    let mut progress: u8 = 0;
    let mut s: *mut EState = 0 as *mut EState;
    if strm.is_null() {
        return -2;
    }
    s = (*strm).state as *mut EState;
    if s.is_null() {
        return -2;
    }
    if (*s).strm != strm {
        return -2;
    }
    loop {
        match (*s).mode {
            1 => return -1,
            2 => {
                if action == 0 {
                    progress = handle_compress(strm);
                    return if progress as i32 != 0 { 1 } else { -2 };
                } else if action == 1 {
                    (*s).avail_in_expect = (*strm).avail_in;
                    (*s).mode = 3;
                } else if action == 2 {
                    (*s).avail_in_expect = (*strm).avail_in;
                    (*s).mode = 4;
                } else {
                    return -2;
                }
            }
            3 => {
                if action != 1 {
                    return -1;
                }
                if (*s).avail_in_expect != (*(*s).strm).avail_in {
                    return -1;
                }
                progress = handle_compress(strm);
                if (*s).avail_in_expect > 0 || isempty_RL(s) == 0 || (*s).state_out_pos < (*s).numZ
                {
                    return 2;
                };
                (*s).mode = 2;
                return 1;
            }
            4 => {
                if action != 2 {
                    return -1;
                }
                if (*s).avail_in_expect != (*(*s).strm).avail_in {
                    return -1;
                }
                progress = handle_compress(strm);
                if progress == 0 {
                    return -1;
                }
                if (*s).avail_in_expect > 0 || isempty_RL(s) == 0 || (*s).state_out_pos < (*s).numZ
                {
                    return 3;
                };
                (*s).mode = 1;
                return 4;
            }
            _ => return 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn BZ2_bzCompressEnd(mut strm: *mut bz_stream) -> i32 {
    let mut s: *mut EState = 0 as *mut EState;
    if strm.is_null() {
        return -2;
    }
    s = (*strm).state as *mut EState;
    if s.is_null() {
        return -2;
    }
    if (*s).strm != strm {
        return -2;
    }
    if !((*s).arr1).is_null() {
        ((*strm).bzfree).expect("non-null function pointer")(
            (*strm).opaque,
            (*s).arr1 as *mut std::os::raw::c_void,
        );
    }
    if !((*s).arr2).is_null() {
        ((*strm).bzfree).expect("non-null function pointer")(
            (*strm).opaque,
            (*s).arr2 as *mut std::os::raw::c_void,
        );
    }
    if !((*s).ftab).is_null() {
        ((*strm).bzfree).expect("non-null function pointer")(
            (*strm).opaque,
            (*s).ftab as *mut std::os::raw::c_void,
        );
    };
    ((*strm).bzfree).expect("non-null function pointer")((*strm).opaque, (*strm).state);
    (*strm).state = 0 as *mut std::os::raw::c_void;
    return 0;
}

#[no_mangle]
pub extern "C" fn BZ2_bzDecompressInit(
    mut strm: *mut bz_stream,
    mut verbosity: i32,
    mut small: i32,
) -> i32 {
    let mut s: *mut DState = 0 as *mut DState;
    if bz_config_ok() == 0 {
        return -9;
    }
    if strm.is_null() {
        return -2;
    }
    if small != 0 && small != 1 {
        return -2;
    }
    if verbosity < 0 || verbosity > 4 {
        return -2;
    }
    if ((*strm).bzalloc).is_none() {
        (*strm).bzalloc = Some(
            default_bzalloc
                as unsafe extern "C" fn(*mut std::os::raw::c_void, i32, i32) -> *mut std::os::raw::c_void,
        );
    }
    if ((*strm).bzfree).is_none() {
        (*strm).bzfree = Some(
            default_bzfree as unsafe extern "C" fn(*mut std::os::raw::c_void, *mut std::os::raw::c_void) -> (),
        );
    }
    s = ((*strm).bzalloc).expect("non-null function pointer")(
        (*strm).opaque,
        ::std::mem::size_of::<DState>() as i32,
        1,
    ) as *mut DState;
    if s.is_null() {
        return -3;
    };
    (*s).strm = strm;
    (*strm).state = s as *mut std::os::raw::c_void;
    (*s).state = 10;
    (*s).bsLive = 0;
    (*s).bsBuff = 0;
    (*s).calculatedCombinedCRC = 0;
    (*strm).total_in_lo32 = 0;
    (*strm).total_in_hi32 = 0;
    (*strm).total_out_lo32 = 0;
    (*strm).total_out_hi32 = 0;
    (*s).smallDecompress = small as u8;
    (*s).ll4 = 0 as *mut u8;
    (*s).ll16 = 0 as *mut u16;
    (*s).tt = 0 as *mut u32;
    (*s).currBlockNo = 0;
    (*s).verbosity = verbosity;
    return 0;
}

extern "C" fn unRLE_obuf_to_output_FAST(mut s: *mut DState) -> u8 {
    let mut current_block: u64;
    let mut k1: u8 = 0;
    unsafe {
        if (*s).blockRandomised != 0 {
            while 1 != 0 {
                while 1 != 0 {
                    if (*(*s).strm).avail_out == 0 {
                        return 0;
                    }
                    if (*s).state_out_len == 0 {
                        break;
                    }
                    *((*(*s).strm).next_out as *mut u8) = (*s).state_out_ch;
                    (*s).calculatedBlockCRC = (*s).calculatedBlockCRC << 8
                        ^ BZ2_crc32Table
                            [((*s).calculatedBlockCRC >> 24 ^ (*s).state_out_ch as u32) as usize];
                    (*s).state_out_len -= 1;
                    (*s).state_out_len;
                    (*(*s).strm).next_out = ((*(*s).strm).next_out).offset(1);
                    (*(*s).strm).next_out;
                    (*(*s).strm).avail_out = ((*(*s).strm).avail_out).wrapping_sub(1);
                    (*(*s).strm).avail_out;
                    (*(*s).strm).total_out_lo32 = ((*(*s).strm).total_out_lo32).wrapping_add(1);
                    (*(*s).strm).total_out_lo32;
                    if (*(*s).strm).total_out_lo32 == 0 {
                        (*(*s).strm).total_out_hi32 = ((*(*s).strm).total_out_hi32).wrapping_add(1);
                        (*(*s).strm).total_out_hi32;
                    }
                }
                if (*s).nblock_used == (*s).save_nblock + 1 {
                    return 0;
                }
                if (*s).nblock_used > (*s).save_nblock + 1 {
                    return 1;
                };
                (*s).state_out_len = 1;
                (*s).state_out_ch = (*s).k0 as u8;
                if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                    return 1;
                };
                (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                k1 = ((*s).tPos & 0xffu32) as u8;
                (*s).tPos >>= 8;
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
                k1 = (k1 as i32 ^ if (*s).rNToGo == 1i32 { 1 } else { 0 }) as u8;
                (*s).nblock_used += 1;
                (*s).nblock_used;
                if (*s).nblock_used == (*s).save_nblock + 1 {
                    continue;
                }
                if k1 as i32 != (*s).k0 {
                    (*s).k0 = k1 as i32;
                } else {
                    (*s).state_out_len = 2;
                    if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                        return 1;
                    };
                    (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                    k1 = ((*s).tPos & 0xffu32) as u8;
                    (*s).tPos >>= 8;
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
                    k1 = (k1 as i32 ^ if (*s).rNToGo == 1i32 { 1 } else { 0 }) as u8;
                    (*s).nblock_used += 1;
                    (*s).nblock_used;
                    if (*s).nblock_used == (*s).save_nblock + 1 {
                        continue;
                    }
                    if k1 as i32 != (*s).k0 {
                        (*s).k0 = k1 as i32;
                    } else {
                        (*s).state_out_len = 3;
                        if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                            return 1;
                        };
                        (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                        k1 = ((*s).tPos & 0xffu32) as u8;
                        (*s).tPos >>= 8;
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
                        k1 = (k1 as i32 ^ if (*s).rNToGo == 1i32 { 1 } else { 0 }) as u8;
                        (*s).nblock_used += 1;
                        (*s).nblock_used;
                        if (*s).nblock_used == (*s).save_nblock + 1 {
                            continue;
                        }
                        if k1 as i32 != (*s).k0 {
                            (*s).k0 = k1 as i32;
                        } else {
                            if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                                return 1;
                            };
                            (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                            k1 = ((*s).tPos & 0xffu32) as u8;
                            (*s).tPos >>= 8;
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
                            k1 = (k1 as i32 ^ if (*s).rNToGo == 1i32 { 1 } else { 0 }) as u8;
                            (*s).nblock_used += 1;
                            (*s).nblock_used;
                            (*s).state_out_len = k1 as i32 + 4;
                            if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                                return 1;
                            };
                            (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                            (*s).k0 = ((*s).tPos & 0xffu32) as i32;
                            (*s).tPos >>= 8;
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
                            (*s).nblock_used += 1;
                            (*s).nblock_used;
                        }
                    }
                }
            }
        } else {
            let mut c_calculatedBlockCRC: u32 = (*s).calculatedBlockCRC;
            let mut c_state_out_ch: u8 = (*s).state_out_ch;
            let mut c_state_out_len: i32 = (*s).state_out_len;
            let mut c_nblock_used: i32 = (*s).nblock_used;
            let mut c_k0: i32 = (*s).k0;
            let mut c_tt: *mut u32 = (*s).tt;
            let mut c_tPos: u32 = (*s).tPos;
            let mut cs_next_out: *mut i8 = (*(*s).strm).next_out;
            let mut cs_avail_out: u32 = (*(*s).strm).avail_out;
            let mut ro_blockSize100k: i32 = (*s).blockSize100k;
            let mut avail_out_INIT: u32 = cs_avail_out;
            let mut s_save_nblockPP: i32 = (*s).save_nblock + 1;
            let mut total_out_lo32_old: u32 = 0;
            's_453: while 1 != 0 {
                if c_state_out_len > 0 {
                    while 1 != 0 {
                        if cs_avail_out == 0 {
                            break 's_453;
                        }
                        if c_state_out_len == 1 {
                            break;
                        }
                        *(cs_next_out as *mut u8) = c_state_out_ch;
                        c_calculatedBlockCRC = c_calculatedBlockCRC << 8
                            ^ BZ2_crc32Table
                                [(c_calculatedBlockCRC >> 24 ^ c_state_out_ch as u32) as usize];
                        c_state_out_len -= 1;
                        c_state_out_len;
                        cs_next_out = cs_next_out.offset(1);
                        cs_next_out;
                        cs_avail_out = cs_avail_out.wrapping_sub(1);
                        cs_avail_out;
                    }
                    current_block = 1417769144978639029;
                } else {
                    current_block = 14483658890531361756;
                }
                loop {
                    match current_block {
                        1417769144978639029 => {
                            if cs_avail_out == 0 {
                                c_state_out_len = 1;
                                break 's_453;
                            } else {
                                *(cs_next_out as *mut u8) = c_state_out_ch;
                                c_calculatedBlockCRC = c_calculatedBlockCRC << 8
                                    ^ BZ2_crc32Table[(c_calculatedBlockCRC >> 24
                                        ^ c_state_out_ch as u32)
                                        as usize];
                                cs_next_out = cs_next_out.offset(1);
                                cs_next_out;
                                cs_avail_out = cs_avail_out.wrapping_sub(1);
                                cs_avail_out;
                                current_block = 14483658890531361756;
                            }
                        }
                        _ => {
                            if c_nblock_used > s_save_nblockPP {
                                return 1;
                            }
                            if c_nblock_used == s_save_nblockPP {
                                c_state_out_len = 0;
                                break 's_453;
                            } else {
                                c_state_out_ch = c_k0 as u8;
                                if c_tPos >= 100000 * ro_blockSize100k as u32 {
                                    return 1;
                                }
                                c_tPos = *c_tt.offset(c_tPos as isize);
                                k1 = (c_tPos & 0xffu32) as u8;
                                c_tPos >>= 8;
                                c_nblock_used += 1;
                                c_nblock_used;
                                if k1 as i32 != c_k0 {
                                    c_k0 = k1 as i32;
                                    current_block = 1417769144978639029;
                                } else {
                                    if c_nblock_used == s_save_nblockPP {
                                        current_block = 1417769144978639029;
                                        continue;
                                    }
                                    c_state_out_len = 2;
                                    if c_tPos >= 100000 * ro_blockSize100k as u32 {
                                        return 1;
                                    }
                                    c_tPos = *c_tt.offset(c_tPos as isize);
                                    k1 = (c_tPos & 0xffu32) as u8;
                                    c_tPos >>= 8;
                                    c_nblock_used += 1;
                                    c_nblock_used;
                                    if c_nblock_used == s_save_nblockPP {
                                        continue 's_453;
                                    }
                                    if k1 as i32 != c_k0 {
                                        current_block = 6897179874198677617;
                                        break;
                                    } else {
                                        current_block = 13256895345714485905;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                match current_block {
                    6897179874198677617 => {
                        c_k0 = k1 as i32;
                    }
                    _ => {
                        c_state_out_len = 3;
                        if c_tPos >= 100000 * ro_blockSize100k as u32 {
                            return 1;
                        }
                        c_tPos = *c_tt.offset(c_tPos as isize);
                        k1 = (c_tPos & 0xffu32) as u8;
                        c_tPos >>= 8;
                        c_nblock_used += 1;
                        c_nblock_used;
                        if c_nblock_used == s_save_nblockPP {
                            continue;
                        }
                        if k1 as i32 != c_k0 {
                            c_k0 = k1 as i32;
                        } else {
                            if c_tPos >= 100000 * ro_blockSize100k as u32 {
                                return 1;
                            }
                            c_tPos = *c_tt.offset(c_tPos as isize);
                            k1 = (c_tPos & 0xffu32) as u8;
                            c_tPos >>= 8;
                            c_nblock_used += 1;
                            c_nblock_used;
                            c_state_out_len = k1 as i32 + 4;
                            if c_tPos >= 100000 * ro_blockSize100k as u32 {
                                return 1;
                            }
                            c_tPos = *c_tt.offset(c_tPos as isize);
                            c_k0 = (c_tPos & 0xffu32) as i32;
                            c_tPos >>= 8;
                            c_nblock_used += 1;
                            c_nblock_used;
                        }
                    }
                }
            }
            total_out_lo32_old = (*(*s).strm).total_out_lo32;
            (*(*s).strm).total_out_lo32 = ((*(*s).strm).total_out_lo32)
                .wrapping_add(avail_out_INIT.wrapping_sub(cs_avail_out));
            if (*(*s).strm).total_out_lo32 < total_out_lo32_old {
                (*(*s).strm).total_out_hi32 = ((*(*s).strm).total_out_hi32).wrapping_add(1);
                (*(*s).strm).total_out_hi32;
            };
            (*s).calculatedBlockCRC = c_calculatedBlockCRC;
            (*s).state_out_ch = c_state_out_ch;
            (*s).state_out_len = c_state_out_len;
            (*s).nblock_used = c_nblock_used;
            (*s).k0 = c_k0;
            (*s).tt = c_tt;
            (*s).tPos = c_tPos;
            (*(*s).strm).next_out = cs_next_out;
            (*(*s).strm).avail_out = cs_avail_out;
        }
    }
    return 0;
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub extern "C" fn BZ2_indexIntoF(mut indx: i32, mut cftab: *mut i32) -> i32 {
    let mut nb: i32 = 0;
    let mut na: i32 = 0;
    let mut mid: i32 = 0;
    nb = 0;
    na = 256;
    unsafe {
        loop {
            mid = nb + na >> 1;
            if indx >= *cftab.offset(mid as isize) {
                nb = mid;
            } else {
                na = mid;
            }
            if !(na - nb != 1) {
                break;
            }
        }
    }
    return nb;
}

extern "C" fn unRLE_obuf_to_output_SMALL(mut s: *mut DState) -> u8 {
    let mut k1: u8 = 0;
    unsafe {
        if (*s).blockRandomised != 0 {
            while 1 != 0 {
                while 1 != 0 {
                    if (*(*s).strm).avail_out == 0 {
                        return 0;
                    }
                    if (*s).state_out_len == 0 {
                        break;
                    }
                    *((*(*s).strm).next_out as *mut u8) = (*s).state_out_ch;
                    (*s).calculatedBlockCRC = (*s).calculatedBlockCRC << 8
                        ^ BZ2_crc32Table
                            [((*s).calculatedBlockCRC >> 24 ^ (*s).state_out_ch as u32) as usize];
                    (*s).state_out_len -= 1;
                    (*s).state_out_len;
                    (*(*s).strm).next_out = ((*(*s).strm).next_out).offset(1);
                    (*(*s).strm).next_out;
                    (*(*s).strm).avail_out = ((*(*s).strm).avail_out).wrapping_sub(1);
                    (*(*s).strm).avail_out;
                    (*(*s).strm).total_out_lo32 = ((*(*s).strm).total_out_lo32).wrapping_add(1);
                    (*(*s).strm).total_out_lo32;
                    if (*(*s).strm).total_out_lo32 == 0 {
                        (*(*s).strm).total_out_hi32 = ((*(*s).strm).total_out_hi32).wrapping_add(1);
                        (*(*s).strm).total_out_hi32;
                    }
                }
                if (*s).nblock_used == (*s).save_nblock + 1 {
                    return 0;
                }
                if (*s).nblock_used > (*s).save_nblock + 1 {
                    return 1;
                };
                (*s).state_out_len = 1;
                (*s).state_out_ch = (*s).k0 as u8;
                if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                    return 1;
                }
                k1 = BZ2_indexIntoF((*s).tPos as i32, ((*s).cftab).as_mut_ptr()) as u8;
                (*s).tPos = *((*s).ll16).offset((*s).tPos as isize) as u32
                    | (*((*s).ll4).offset(((*s).tPos >> 1i32) as isize) as u32
                        >> ((*s).tPos << 2 & 0x4)
                        & 0xf)
                        << 16;
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
                k1 = (k1 as i32 ^ if (*s).rNToGo == 1i32 { 1 } else { 0 }) as u8;
                (*s).nblock_used += 1;
                (*s).nblock_used;
                if (*s).nblock_used == (*s).save_nblock + 1 {
                    continue;
                }
                if k1 as i32 != (*s).k0 {
                    (*s).k0 = k1 as i32;
                } else {
                    (*s).state_out_len = 2;
                    if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                        return 1;
                    }
                    k1 = BZ2_indexIntoF((*s).tPos as i32, ((*s).cftab).as_mut_ptr()) as u8;
                    (*s).tPos = *((*s).ll16).offset((*s).tPos as isize) as u32
                        | (*((*s).ll4).offset(((*s).tPos >> 1i32) as isize) as u32
                            >> ((*s).tPos << 2 & 0x4)
                            & 0xf)
                            << 16;
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
                    k1 = (k1 as i32 ^ if (*s).rNToGo == 1i32 { 1 } else { 0 }) as u8;
                    (*s).nblock_used += 1;
                    (*s).nblock_used;
                    if (*s).nblock_used == (*s).save_nblock + 1 {
                        continue;
                    }
                    if k1 as i32 != (*s).k0 {
                        (*s).k0 = k1 as i32;
                    } else {
                        (*s).state_out_len = 3;
                        if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                            return 1;
                        }
                        k1 = BZ2_indexIntoF((*s).tPos as i32, ((*s).cftab).as_mut_ptr()) as u8;
                        (*s).tPos = *((*s).ll16).offset((*s).tPos as isize) as u32
                            | (*((*s).ll4).offset(((*s).tPos >> 1i32) as isize) as u32
                                >> ((*s).tPos << 2 & 0x4)
                                & 0xf)
                                << 16;
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
                        k1 = (k1 as i32 ^ if (*s).rNToGo == 1i32 { 1 } else { 0 }) as u8;
                        (*s).nblock_used += 1;
                        (*s).nblock_used;
                        if (*s).nblock_used == (*s).save_nblock + 1 {
                            continue;
                        }
                        if k1 as i32 != (*s).k0 {
                            (*s).k0 = k1 as i32;
                        } else {
                            if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                                return 1;
                            }
                            k1 = BZ2_indexIntoF((*s).tPos as i32, ((*s).cftab).as_mut_ptr()) as u8;
                            (*s).tPos = *((*s).ll16).offset((*s).tPos as isize) as u32
                                | (*((*s).ll4).offset(((*s).tPos >> 1i32) as isize) as u32
                                    >> ((*s).tPos << 2 & 0x4)
                                    & 0xf)
                                    << 16;
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
                            k1 = (k1 as i32 ^ if (*s).rNToGo == 1i32 { 1 } else { 0 }) as u8;
                            (*s).nblock_used += 1;
                            (*s).nblock_used;
                            (*s).state_out_len = k1 as i32 + 4;
                            if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                                return 1;
                            };
                            (*s).k0 = BZ2_indexIntoF((*s).tPos as i32, ((*s).cftab).as_mut_ptr());
                            (*s).tPos = *((*s).ll16).offset((*s).tPos as isize) as u32
                                | (*((*s).ll4).offset(((*s).tPos >> 1i32) as isize) as u32
                                    >> ((*s).tPos << 2 & 0x4)
                                    & 0xf)
                                    << 16;
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
                            (*s).nblock_used += 1;
                            (*s).nblock_used;
                        }
                    }
                }
            }
        } else {
            while 1 != 0 {
                while 1 != 0 {
                    if (*(*s).strm).avail_out == 0 {
                        return 0;
                    }
                    if (*s).state_out_len == 0 {
                        break;
                    }
                    *((*(*s).strm).next_out as *mut u8) = (*s).state_out_ch;
                    (*s).calculatedBlockCRC = (*s).calculatedBlockCRC << 8
                        ^ BZ2_crc32Table
                            [((*s).calculatedBlockCRC >> 24 ^ (*s).state_out_ch as u32) as usize];
                    (*s).state_out_len -= 1;
                    (*s).state_out_len;
                    (*(*s).strm).next_out = ((*(*s).strm).next_out).offset(1);
                    (*(*s).strm).next_out;
                    (*(*s).strm).avail_out = ((*(*s).strm).avail_out).wrapping_sub(1);
                    (*(*s).strm).avail_out;
                    (*(*s).strm).total_out_lo32 = ((*(*s).strm).total_out_lo32).wrapping_add(1);
                    (*(*s).strm).total_out_lo32;
                    if (*(*s).strm).total_out_lo32 == 0 {
                        (*(*s).strm).total_out_hi32 = ((*(*s).strm).total_out_hi32).wrapping_add(1);
                        (*(*s).strm).total_out_hi32;
                    }
                }
                if (*s).nblock_used == (*s).save_nblock + 1 {
                    return 0;
                }
                if (*s).nblock_used > (*s).save_nblock + 1 {
                    return 1;
                };
                (*s).state_out_len = 1;
                (*s).state_out_ch = (*s).k0 as u8;
                if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                    return 1;
                }
                k1 = BZ2_indexIntoF((*s).tPos as i32, ((*s).cftab).as_mut_ptr()) as u8;
                (*s).tPos = *((*s).ll16).offset((*s).tPos as isize) as u32
                    | (*((*s).ll4).offset(((*s).tPos >> 1i32) as isize) as u32
                        >> ((*s).tPos << 2 & 0x4)
                        & 0xf)
                        << 16;
                (*s).nblock_used += 1;
                (*s).nblock_used;
                if (*s).nblock_used == (*s).save_nblock + 1 {
                    continue;
                }
                if k1 as i32 != (*s).k0 {
                    (*s).k0 = k1 as i32;
                } else {
                    (*s).state_out_len = 2;
                    if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                        return 1;
                    }
                    k1 = BZ2_indexIntoF((*s).tPos as i32, ((*s).cftab).as_mut_ptr()) as u8;
                    (*s).tPos = *((*s).ll16).offset((*s).tPos as isize) as u32
                        | (*((*s).ll4).offset(((*s).tPos >> 1i32) as isize) as u32
                            >> ((*s).tPos << 2 & 0x4)
                            & 0xf)
                            << 16;
                    (*s).nblock_used += 1;
                    (*s).nblock_used;
                    if (*s).nblock_used == (*s).save_nblock + 1 {
                        continue;
                    }
                    if k1 as i32 != (*s).k0 {
                        (*s).k0 = k1 as i32;
                    } else {
                        (*s).state_out_len = 3;
                        if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                            return 1;
                        }
                        k1 = BZ2_indexIntoF((*s).tPos as i32, ((*s).cftab).as_mut_ptr()) as u8;
                        (*s).tPos = *((*s).ll16).offset((*s).tPos as isize) as u32
                            | (*((*s).ll4).offset(((*s).tPos >> 1i32) as isize) as u32
                                >> ((*s).tPos << 2 & 0x4)
                                & 0xf)
                                << 16;
                        (*s).nblock_used += 1;
                        (*s).nblock_used;
                        if (*s).nblock_used == (*s).save_nblock + 1 {
                            continue;
                        }
                        if k1 as i32 != (*s).k0 {
                            (*s).k0 = k1 as i32;
                        } else {
                            if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                                return 1;
                            }
                            k1 = BZ2_indexIntoF((*s).tPos as i32, ((*s).cftab).as_mut_ptr()) as u8;
                            (*s).tPos = *((*s).ll16).offset((*s).tPos as isize) as u32
                                | (*((*s).ll4).offset(((*s).tPos >> 1i32) as isize) as u32
                                    >> ((*s).tPos << 2 & 0x4)
                                    & 0xf)
                                    << 16;
                            (*s).nblock_used += 1;
                            (*s).nblock_used;
                            (*s).state_out_len = k1 as i32 + 4;
                            if (*s).tPos >= 100000 * (*s).blockSize100k as u32 {
                                return 1;
                            };
                            (*s).k0 = BZ2_indexIntoF((*s).tPos as i32, ((*s).cftab).as_mut_ptr());
                            (*s).tPos = *((*s).ll16).offset((*s).tPos as isize) as u32
                                | (*((*s).ll4).offset(((*s).tPos >> 1i32) as isize) as u32
                                    >> ((*s).tPos << 2 & 0x4)
                                    & 0xf)
                                    << 16;
                            (*s).nblock_used += 1;
                            (*s).nblock_used;
                        }
                    }
                }
            }
        }
    }
    panic!("Reached end of non-void function without returning");
}

#[no_mangle]
pub extern "C" fn BZ2_bzDecompress(mut strm: *mut bz_stream) -> i32 {
    let mut corrupt: u8 = 0;
    let mut s: *mut DState = 0 as *mut DState;
    if strm.is_null() {
        return -2;
    }
    s = (*strm).state as *mut DState;
    if s.is_null() {
        return -2;
    }
    if (*s).strm != strm {
        return -2;
    }
    unsafe {
        while 1 != 0 {
            if (*s).state == 1 {
                return -1;
            }
            if (*s).state == 2 {
                if (*s).smallDecompress != 0 {
                    corrupt = unRLE_obuf_to_output_SMALL(s);
                } else {
                    corrupt = unRLE_obuf_to_output_FAST(s);
                }
                if corrupt != 0 {
                    return -4;
                }
                if (*s).nblock_used == (*s).save_nblock + 1 && (*s).state_out_len == 0 {
                    (*s).calculatedBlockCRC = !(*s).calculatedBlockCRC;
                    if (*s).verbosity >= 3 {
                        fprintf(
                            __stderrp,
                            b" {0x%08x, 0x%08x}\0" as *const u8 as *const i8,
                            (*s).storedBlockCRC,
                            (*s).calculatedBlockCRC,
                        );
                    }
                    if (*s).verbosity >= 2 {
                        fprintf(__stderrp, b"]\0" as *const u8 as *const i8);
                    }
                    if (*s).calculatedBlockCRC != (*s).storedBlockCRC {
                        return -4;
                    };
                    (*s).calculatedCombinedCRC =
                        (*s).calculatedCombinedCRC << 1 | (*s).calculatedCombinedCRC >> 31;
                    (*s).calculatedCombinedCRC ^= (*s).calculatedBlockCRC;
                    (*s).state = 14;
                } else {
                    return 0;
                }
            }
            if (*s).state >= 10 {
                let mut r: i32 = BZ2_decompress(s);
                if r == 4 {
                    if (*s).verbosity >= 3 {
                        fprintf(
                            __stderrp,
                            b"\n    combined CRCs: stored = 0x%08x, computed = 0x%08x\0"
                                as *const u8 as *const i8,
                            (*s).storedCombinedCRC,
                            (*s).calculatedCombinedCRC,
                        );
                    }
                    if (*s).calculatedCombinedCRC != (*s).storedCombinedCRC {
                        return -4;
                    }
                    return r;
                }
                if (*s).state != 2 {
                    return r;
                }
            }
        }
    }
    if 0 == 0 {
        BZ2_bz__AssertH__fail(6001);
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn BZ2_bzDecompressEnd(mut strm: *mut bz_stream) -> i32 {
    let mut s: *mut DState = 0 as *mut DState;
    if strm.is_null() {
        return -2;
    }
    s = (*strm).state as *mut DState;
    if s.is_null() {
        return -2;
    }
    if (*s).strm != strm {
        return -2;
    }
    if !((*s).tt).is_null() {
        ((*strm).bzfree).expect("non-null function pointer")(
            (*strm).opaque,
            (*s).tt as *mut std::os::raw::c_void,
        );
    }
    if !((*s).ll16).is_null() {
        ((*strm).bzfree).expect("non-null function pointer")(
            (*strm).opaque,
            (*s).ll16 as *mut std::os::raw::c_void,
        );
    }
    if !((*s).ll4).is_null() {
        ((*strm).bzfree).expect("non-null function pointer")(
            (*strm).opaque,
            (*s).ll4 as *mut std::os::raw::c_void,
        );
    };
    ((*strm).bzfree).expect("non-null function pointer")((*strm).opaque, (*strm).state);
    (*strm).state = 0 as *mut std::os::raw::c_void;
    return 0;
}

extern "C" fn myfeof(mut f: *mut FILE) -> u8 {
    unsafe {
        let mut c: i32 = fgetc(f);
        if c == -1 {
            return 1;
        }
        ungetc(c, f);
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn BZ2_bzWriteOpen(
    mut bzerror: *mut i32,
    mut f: *mut FILE,
    mut blockSize100k: i32,
    mut verbosity: i32,
    mut workFactor: i32,
) -> *mut std::os::raw::c_void {
    let mut ret: i32 = 0;
    let mut bzf: *mut bzFile = 0 as *mut bzFile;
    if !bzerror.is_null() {
        *bzerror = 0;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = 0;
    }
    if f.is_null()
        || (blockSize100k < 1 || blockSize100k > 9)
        || (workFactor < 0 || workFactor > 250)
        || (verbosity < 0 || verbosity > 4)
    {
        if !bzerror.is_null() {
            *bzerror = -2;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -2;
        }
        return 0 as *mut std::os::raw::c_void;
    }
    unsafe {
        if ferror(f) != 0 {
            if !bzerror.is_null() {
                *bzerror = -6;
            }
            if !bzf.is_null() {
                (*bzf).lastErr = -6;
            }
            return 0 as *mut std::os::raw::c_void;
        }
        bzf = malloc(::std::mem::size_of::<bzFile>() as u64) as *mut bzFile;
    }
    if bzf.is_null() {
        if !bzerror.is_null() {
            *bzerror = -3;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -3;
        }
        return 0 as *mut std::os::raw::c_void;
    }
    if !bzerror.is_null() {
        *bzerror = 0;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = 0;
    };
    (*bzf).initialisedOk = 0;
    (*bzf).bufN = 0;
    (*bzf).handle = f;
    (*bzf).writing = 1;
    (*bzf).strm.bzalloc = None;
    (*bzf).strm.bzfree = None;
    (*bzf).strm.opaque = 0 as *mut std::os::raw::c_void;
    if workFactor == 0 {
        workFactor = 30;
    }
    ret = BZ2_bzCompressInit(&mut (*bzf).strm, blockSize100k, verbosity, workFactor);
    unsafe {
        if ret != 0 {
            if !bzerror.is_null() {
                *bzerror = ret;
            }
            if !bzf.is_null() {
                (*bzf).lastErr = ret;
            }
            free(bzf as *mut std::os::raw::c_void);
            return 0 as *mut std::os::raw::c_void;
        };
    }
    (*bzf).strm.avail_in = 0;
    (*bzf).initialisedOk = 1;
    return bzf as *mut std::os::raw::c_void;
}

#[no_mangle]
pub extern "C" fn BZ2_bzWrite(
    mut bzerror: *mut i32,
    mut b: *mut std::os::raw::c_void,
    mut buf: *mut std::os::raw::c_void,
    mut len: i32,
) {
    let mut n: i32 = 0;
    let mut n2: i32 = 0;
    let mut ret: i32 = 0;
    let mut bzf: *mut bzFile = b as *mut bzFile;
    if !bzerror.is_null() {
        *bzerror = 0;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = 0;
    }
    if bzf.is_null() || buf.is_null() || len < 0 {
        if !bzerror.is_null() {
            *bzerror = -2;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -2;
        }
        return;
    }
    if (*bzf).writing == 0 {
        if !bzerror.is_null() {
            *bzerror = -1;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -1;
        }
        return;
    }
    unsafe {
        if ferror((*bzf).handle) != 0 {
            if !bzerror.is_null() {
                *bzerror = -6;
            }
            if !bzf.is_null() {
                (*bzf).lastErr = -6;
            }
            return;
        }
    }
    if len == 0 {
        if !bzerror.is_null() {
            *bzerror = 0;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = 0;
        }
        return;
    };
    (*bzf).strm.avail_in = len as u32;
    (*bzf).strm.next_in = buf as *mut i8;
    unsafe {
        while 1 != 0 {
            (*bzf).strm.avail_out = 5000;
            (*bzf).strm.next_out = ((*bzf).buf).as_mut_ptr();
            ret = BZ2_bzCompress(&mut (*bzf).strm, 0);
            if ret != 1 {
                if !bzerror.is_null() {
                    *bzerror = ret;
                }
                if !bzf.is_null() {
                    (*bzf).lastErr = ret;
                }
                return;
            }
            if (*bzf).strm.avail_out < 5000 {
                n = 5000u32.wrapping_sub((*bzf).strm.avail_out) as i32;
                n2 = fwrite(
                    ((*bzf).buf).as_mut_ptr() as *mut std::os::raw::c_void,
                    ::std::mem::size_of::<u8>() as u64,
                    n as u64,
                    (*bzf).handle,
                ) as i32;
                if n != n2 || ferror((*bzf).handle) != 0 {
                    if !bzerror.is_null() {
                        *bzerror = -6;
                    }
                    if !bzf.is_null() {
                        (*bzf).lastErr = -6;
                    }
                    return;
                }
            }
            if (*bzf).strm.avail_in == 0 {
                if !bzerror.is_null() {
                    *bzerror = 0;
                }
                if !bzf.is_null() {
                    (*bzf).lastErr = 0;
                }
                return;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn BZ2_bzWriteClose(
    mut bzerror: *mut i32,
    mut b: *mut std::os::raw::c_void,
    mut abandon: i32,
    mut nbytes_in: *mut u32,
    mut nbytes_out: *mut u32,
) {
    BZ2_bzWriteClose64(
        bzerror,
        b,
        abandon,
        nbytes_in,
        0 as *mut u32,
        nbytes_out,
        0 as *mut u32,
    );
}

#[no_mangle]
pub extern "C" fn BZ2_bzWriteClose64(
    mut bzerror: *mut i32,
    mut b: *mut std::os::raw::c_void,
    mut abandon: i32,
    mut nbytes_in_lo32: *mut u32,
    mut nbytes_in_hi32: *mut u32,
    mut nbytes_out_lo32: *mut u32,
    mut nbytes_out_hi32: *mut u32,
) {
    let mut n: i32 = 0;
    let mut n2: i32 = 0;
    let mut ret: i32 = 0;
    let mut bzf: *mut bzFile = b as *mut bzFile;
    if bzf.is_null() {
        if !bzerror.is_null() {
            *bzerror = 0;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = 0;
        }
        return;
    }
    if (*bzf).writing == 0 {
        if !bzerror.is_null() {
            *bzerror = -1;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -1;
        }
        return;
    }
    unsafe {
        if ferror((*bzf).handle) != 0 {
            if !bzerror.is_null() {
                *bzerror = -6;
            }
            if !bzf.is_null() {
                (*bzf).lastErr = -6;
            }
            return;
        }
    }
    if !nbytes_in_lo32.is_null() {
        *nbytes_in_lo32 = 0;
    }
    if !nbytes_in_hi32.is_null() {
        *nbytes_in_hi32 = 0;
    }
    if !nbytes_out_lo32.is_null() {
        *nbytes_out_lo32 = 0;
    }
    if !nbytes_out_hi32.is_null() {
        *nbytes_out_hi32 = 0;
    }
    unsafe {
        if abandon == 0 && (*bzf).lastErr == 0 {
            while 1 != 0 {
                (*bzf).strm.avail_out = 5000;
                (*bzf).strm.next_out = ((*bzf).buf).as_mut_ptr();
                ret = BZ2_bzCompress(&mut (*bzf).strm, 2);
                if ret != 3 && ret != 4 {
                    if !bzerror.is_null() {
                        *bzerror = ret;
                    }
                    if !bzf.is_null() {
                        (*bzf).lastErr = ret;
                    }
                    return;
                }
                if (*bzf).strm.avail_out < 5000 {
                    n = 5000u32.wrapping_sub((*bzf).strm.avail_out) as i32;
                    n2 = fwrite(
                        ((*bzf).buf).as_mut_ptr() as *mut std::os::raw::c_void,
                        ::std::mem::size_of::<u8>() as u64,
                        n as u64,
                        (*bzf).handle,
                    ) as i32;
                    if n != n2 || ferror((*bzf).handle) != 0 {
                        if !bzerror.is_null() {
                            *bzerror = -6;
                        }
                        if !bzf.is_null() {
                            (*bzf).lastErr = -6;
                        }
                        return;
                    }
                }
                if ret == 4 {
                    break;
                }
            }
        }
        if abandon == 0 && ferror((*bzf).handle) == 0 {
            fflush((*bzf).handle);
            if ferror((*bzf).handle) != 0 {
                if !bzerror.is_null() {
                    *bzerror = -6;
                }
                if !bzf.is_null() {
                    (*bzf).lastErr = -6;
                }
                return;
            }
        }
    }
    if !nbytes_in_lo32.is_null() {
        *nbytes_in_lo32 = (*bzf).strm.total_in_lo32;
    }
    if !nbytes_in_hi32.is_null() {
        *nbytes_in_hi32 = (*bzf).strm.total_in_hi32;
    }
    if !nbytes_out_lo32.is_null() {
        *nbytes_out_lo32 = (*bzf).strm.total_out_lo32;
    }
    if !nbytes_out_hi32.is_null() {
        *nbytes_out_hi32 = (*bzf).strm.total_out_hi32;
    }
    if !bzerror.is_null() {
        *bzerror = 0;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = 0;
    }
    BZ2_bzCompressEnd(&mut (*bzf).strm);
    unsafe {
        free(bzf as *mut std::os::raw::c_void);
    }
}

#[no_mangle]
pub extern "C" fn BZ2_bzReadOpen(
    mut bzerror: *mut i32,
    mut f: *mut FILE,
    mut verbosity: i32,
    mut small: i32,
    mut unused: *mut std::os::raw::c_void,
    mut nUnused: i32,
) -> *mut std::os::raw::c_void {
    let mut bzf: *mut bzFile = 0 as *mut bzFile;
    let mut ret: i32 = 0;
    if !bzerror.is_null() {
        *bzerror = 0;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = 0;
    }
    if f.is_null()
        || small != 0 && small != 1
        || (verbosity < 0 || verbosity > 4)
        || unused.is_null() && nUnused != 0
        || !unused.is_null() && (nUnused < 0 || nUnused > 5000)
    {
        if !bzerror.is_null() {
            *bzerror = -2;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -2;
        }
        return 0 as *mut std::os::raw::c_void;
    }
    unsafe {
        if ferror(f) != 0 {
            if !bzerror.is_null() {
                *bzerror = -6;
            }
            if !bzf.is_null() {
                (*bzf).lastErr = -6;
            }
            return 0 as *mut std::os::raw::c_void;
        }
        bzf = malloc(::std::mem::size_of::<bzFile>() as u64) as *mut bzFile;
    }
    if bzf.is_null() {
        if !bzerror.is_null() {
            *bzerror = -3;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -3;
        }
        return 0 as *mut std::os::raw::c_void;
    }
    if !bzerror.is_null() {
        *bzerror = 0;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = 0;
    };
    (*bzf).initialisedOk = 0;
    (*bzf).handle = f;
    (*bzf).bufN = 0;
    (*bzf).writing = 0;
    (*bzf).strm.bzalloc = None;
    (*bzf).strm.bzfree = None;
    (*bzf).strm.opaque = 0 as *mut std::os::raw::c_void;
    unsafe {
        while nUnused > 0 {
            (*bzf).buf[(*bzf).bufN as usize] = *(unused as *mut u8) as i8;
            (*bzf).bufN += 1;
            (*bzf).bufN;
            unused = (unused as *mut u8).offset(1 as isize) as *mut std::os::raw::c_void;
            nUnused -= 1;
            nUnused;
        }
    }
    ret = BZ2_bzDecompressInit(&mut (*bzf).strm, verbosity, small);
    unsafe {
        if ret != 0 {
            if !bzerror.is_null() {
                *bzerror = ret;
            }
            if !bzf.is_null() {
                (*bzf).lastErr = ret;
            }
            free(bzf as *mut std::os::raw::c_void);
            return 0 as *mut std::os::raw::c_void;
        };
    }
    (*bzf).strm.avail_in = (*bzf).bufN as u32;
    (*bzf).strm.next_in = ((*bzf).buf).as_mut_ptr();
    (*bzf).initialisedOk = 1;
    return bzf as *mut std::os::raw::c_void;
}

#[no_mangle]
pub extern "C" fn BZ2_bzReadClose(mut bzerror: *mut i32, mut b: *mut std::os::raw::c_void) {
    let mut bzf: *mut bzFile = b as *mut bzFile;
    if !bzerror.is_null() {
        *bzerror = 0;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = 0;
    }
    if bzf.is_null() {
        if !bzerror.is_null() {
            *bzerror = 0;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = 0;
        }
        return;
    }
    if (*bzf).writing != 0 {
        if !bzerror.is_null() {
            *bzerror = -1;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -1;
        }
        return;
    }
    if (*bzf).initialisedOk != 0 {
        BZ2_bzDecompressEnd(&mut (*bzf).strm);
    }
    unsafe {
        free(bzf as *mut std::os::raw::c_void);
    }
}

#[no_mangle]
pub extern "C" fn BZ2_bzRead(
    mut bzerror: *mut i32,
    mut b: *mut std::os::raw::c_void,
    mut buf: *mut std::os::raw::c_void,
    mut len: i32,
) -> i32 {
    let mut n: i32 = 0;
    let mut ret: i32 = 0;
    let mut bzf: *mut bzFile = b as *mut bzFile;
    if !bzerror.is_null() {
        *bzerror = 0;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = 0;
    }
    if bzf.is_null() || buf.is_null() || len < 0 {
        if !bzerror.is_null() {
            *bzerror = -2;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -2;
        }
        return 0;
    }
    if (*bzf).writing != 0 {
        if !bzerror.is_null() {
            *bzerror = -1;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -1;
        }
        return 0;
    }
    if len == 0 {
        if !bzerror.is_null() {
            *bzerror = 0;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = 0;
        }
        return 0;
    };
    (*bzf).strm.avail_out = len as u32;
    (*bzf).strm.next_out = buf as *mut i8;
    unsafe {
        while 1 != 0 {
            if ferror((*bzf).handle) != 0 {
                if !bzerror.is_null() {
                    *bzerror = -6;
                }
                if !bzf.is_null() {
                    (*bzf).lastErr = -6;
                }
                return 0;
            }
            if (*bzf).strm.avail_in == 0 && myfeof((*bzf).handle) == 0 {
                n = fread(
                    ((*bzf).buf).as_mut_ptr() as *mut std::os::raw::c_void,
                    ::std::mem::size_of::<u8>() as u64,
                    5000,
                    (*bzf).handle,
                ) as i32;
                if ferror((*bzf).handle) != 0 {
                    if !bzerror.is_null() {
                        *bzerror = -6;
                    }
                    if !bzf.is_null() {
                        (*bzf).lastErr = -6;
                    }
                    return 0;
                };
                (*bzf).bufN = n;
                (*bzf).strm.avail_in = (*bzf).bufN as u32;
                (*bzf).strm.next_in = ((*bzf).buf).as_mut_ptr();
            }
            ret = BZ2_bzDecompress(&mut (*bzf).strm);
            if ret != 0 && ret != 4 {
                if !bzerror.is_null() {
                    *bzerror = ret;
                }
                if !bzf.is_null() {
                    (*bzf).lastErr = ret;
                }
                return 0;
            }
            if ret == 0
                && myfeof((*bzf).handle) as i32 != 0
                && (*bzf).strm.avail_in == 0
                && (*bzf).strm.avail_out > 0
            {
                if !bzerror.is_null() {
                    *bzerror = -7;
                }
                if !bzf.is_null() {
                    (*bzf).lastErr = -7;
                }
                return 0;
            }
            if ret == 4 {
                if !bzerror.is_null() {
                    *bzerror = 4;
                }
                if !bzf.is_null() {
                    (*bzf).lastErr = 4;
                }
                return (len as u32).wrapping_sub((*bzf).strm.avail_out) as i32;
            }
            if (*bzf).strm.avail_out == 0 {
                if !bzerror.is_null() {
                    *bzerror = 0;
                }
                if !bzf.is_null() {
                    (*bzf).lastErr = 0;
                }
                return len;
            }
        }
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn BZ2_bzReadGetUnused(
    mut bzerror: *mut i32,
    mut b: *mut std::os::raw::c_void,
    mut unused: *mut *mut std::os::raw::c_void,
    mut nUnused: *mut i32,
) {
    let mut bzf: *mut bzFile = b as *mut bzFile;
    if bzf.is_null() {
        if !bzerror.is_null() {
            *bzerror = -2;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -2;
        }
        return;
    }
    if (*bzf).lastErr != 4 {
        if !bzerror.is_null() {
            *bzerror = -1;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -1;
        }
        return;
    }
    if unused.is_null() || nUnused.is_null() {
        if !bzerror.is_null() {
            *bzerror = -2;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -2;
        }
        return;
    }
    if !bzerror.is_null() {
        *bzerror = 0;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = 0;
    }
    *nUnused = (*bzf).strm.avail_in as i32;
    *unused = (*bzf).strm.next_in as *mut std::os::raw::c_void;
}

#[no_mangle]
pub extern "C" fn BZ2_bzBuffToBuffCompress(
    mut dest: *mut i8,
    mut destLen: *mut u32,
    mut source: *mut i8,
    mut sourceLen: u32,
    mut blockSize100k: i32,
    mut verbosity: i32,
    mut workFactor: i32,
) -> i32 {
    let mut strm: bz_stream = bz_stream {
        next_in: 0 as *mut i8,
        avail_in: 0,
        total_in_lo32: 0,
        total_in_hi32: 0,
        next_out: 0 as *mut i8,
        avail_out: 0,
        total_out_lo32: 0,
        total_out_hi32: 0,
        state: 0 as *mut std::os::raw::c_void,
        bzalloc: None,
        bzfree: None,
        opaque: 0 as *mut std::os::raw::c_void,
    };
    let mut ret: i32 = 0;
    if dest.is_null()
        || destLen.is_null()
        || source.is_null()
        || blockSize100k < 1
        || blockSize100k > 9
        || verbosity < 0
        || verbosity > 4
        || workFactor < 0
        || workFactor > 250
    {
        return -2;
    }
    if workFactor == 0 {
        workFactor = 30;
    }
    strm.bzalloc = None;
    strm.bzfree = None;
    strm.opaque = 0 as *mut std::os::raw::c_void;
    ret = BZ2_bzCompressInit(&mut strm, blockSize100k, verbosity, workFactor);
    if ret != 0 {
        return ret;
    }
    strm.next_in = source;
    strm.next_out = dest;
    strm.avail_in = sourceLen;
    strm.avail_out = *destLen;
    ret = BZ2_bzCompress(&mut strm, 2);
    if ret == 3 {
        BZ2_bzCompressEnd(&mut strm);
        return -8;
    } else if ret != 4 {
        BZ2_bzCompressEnd(&mut strm);
        return ret;
    } else {
        *destLen = (*destLen).wrapping_sub(strm.avail_out);
        BZ2_bzCompressEnd(&mut strm);
        return 0;
    };
}

#[no_mangle]
pub extern "C" fn BZ2_bzBuffToBuffDecompress(
    mut dest: *mut i8,
    mut destLen: *mut u32,
    mut source: *mut i8,
    mut sourceLen: u32,
    mut small: i32,
    mut verbosity: i32,
) -> i32 {
    let mut strm: bz_stream = bz_stream {
        next_in: 0 as *mut i8,
        avail_in: 0,
        total_in_lo32: 0,
        total_in_hi32: 0,
        next_out: 0 as *mut i8,
        avail_out: 0,
        total_out_lo32: 0,
        total_out_hi32: 0,
        state: 0 as *mut std::os::raw::c_void,
        bzalloc: None,
        bzfree: None,
        opaque: 0 as *mut std::os::raw::c_void,
    };
    let mut ret: i32 = 0;
    if dest.is_null()
        || destLen.is_null()
        || source.is_null()
        || small != 0 && small != 1
        || verbosity < 0
        || verbosity > 4
    {
        return -2;
    }
    strm.bzalloc = None;
    strm.bzfree = None;
    strm.opaque = 0 as *mut std::os::raw::c_void;
    ret = BZ2_bzDecompressInit(&mut strm, verbosity, small);
    if ret != 0 {
        return ret;
    }
    strm.next_in = source;
    strm.next_out = dest;
    strm.avail_in = sourceLen;
    strm.avail_out = *destLen;
    ret = BZ2_bzDecompress(&mut strm);
    if ret == 0 {
        if strm.avail_out > 0 {
            BZ2_bzDecompressEnd(&mut strm);
            return -7;
        } else {
            BZ2_bzDecompressEnd(&mut strm);
            return -8;
        }
    } else if ret != 4 {
        BZ2_bzDecompressEnd(&mut strm);
        return ret;
    } else {
        *destLen = (*destLen).wrapping_sub(strm.avail_out);
        BZ2_bzDecompressEnd(&mut strm);
        return 0;
    };
}

#[no_mangle]
pub extern "C" fn BZ2_bzlibVersion() -> *const i8 {
    return b"1.0.8, 13-Jul-2019\0" as *const u8 as *const i8;
}

extern "C" fn bzopen_or_bzdopen(
    mut path: *const i8,
    mut fd: i32,
    mut mode: *const i8,
    mut open_mode: i32,
) -> *mut std::os::raw::c_void {
    let mut bzerr: i32 = 0;
    let mut unused: [i8; 5000] = [0; 5000];
    let mut blockSize100k: i32 = 9;
    let mut writing: i32 = 0;
    unsafe {
        let mut mode2: [i8; 10] =
            *::std::mem::transmute::<&[u8; 10], &mut [i8; 10]>(b"\0\0\0\0\0\0\0\0\0\0");
        let mut fp: *mut FILE = 0 as *mut FILE;
        let mut bzfp: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
        let mut verbosity: i32 = 0;
        let mut workFactor: i32 = 30;
        let mut smallMode: i32 = 0;
        let mut nUnused: i32 = 0;
        if mode.is_null() {
            return 0 as *mut std::os::raw::c_void;
        }
        while *mode != 0 {
            match *mode as i32 {
                114 => {
                    writing = 0;
                }
                119 => {
                    writing = 1;
                }
                115 => {
                    smallMode = 1;
                }
                _ => {
                    if isdigit(*mode as i32) != 0 {
                        blockSize100k = *mode as i32 - 0x30;
                    }
                }
            }
            mode = mode.offset(1);
            mode;
        }
        strcat(
            mode2.as_mut_ptr(),
            if writing != 0 {
                b"w\0" as *const u8 as *const i8
            } else {
                b"r\0" as *const u8 as *const i8
            },
        );
        strcat(mode2.as_mut_ptr(), b"b\0" as *const u8 as *const i8);
        if open_mode == 0 {
            if path.is_null() || strcmp(path, b"\0" as *const u8 as *const i8) == 0 {
                fp = if writing != 0 { __stdoutp } else { __stdinp };
            } else {
                fp = fopen(path, mode2.as_mut_ptr());
            }
        } else {
            fp = fdopen(fd, mode2.as_mut_ptr());
        }
        if fp.is_null() {
            return 0 as *mut std::os::raw::c_void;
        }
        if writing != 0 {
            if blockSize100k < 1 {
                blockSize100k = 1;
            }
            if blockSize100k > 9 {
                blockSize100k = 9;
            }
            bzfp = BZ2_bzWriteOpen(&mut bzerr, fp, blockSize100k, verbosity, workFactor);
        } else {
            bzfp = BZ2_bzReadOpen(
                &mut bzerr,
                fp,
                verbosity,
                smallMode,
                unused.as_mut_ptr() as *mut std::os::raw::c_void,
                nUnused,
            );
        }
        if bzfp.is_null() {
            if fp != __stdinp && fp != __stdoutp {
                fclose(fp);
            }
            return 0 as *mut std::os::raw::c_void;
        }
        return bzfp;
    }
}

#[no_mangle]
pub extern "C" fn BZ2_bzopen(mut path: *const i8, mut mode: *const i8) -> *mut std::os::raw::c_void {
    return bzopen_or_bzdopen(path, -1, mode, 0);
}

#[no_mangle]
pub extern "C" fn BZ2_bzdopen(mut fd: i32, mut mode: *const i8) -> *mut std::os::raw::c_void {
    return bzopen_or_bzdopen(0 as *const i8, fd, mode, 1);
}

#[no_mangle]
pub extern "C" fn BZ2_bzread(
    mut b: *mut std::os::raw::c_void,
    mut buf: *mut std::os::raw::c_void,
    mut len: i32,
) -> i32 {
    let mut bzerr: i32 = 0;
    let mut nread: i32 = 0;
    if (*(b as *mut bzFile)).lastErr == 4 {
        return 0;
    }
    nread = BZ2_bzRead(&mut bzerr, b, buf, len);
    if bzerr == 0 || bzerr == 4 {
        return nread;
    } else {
        return -1;
    };
}

#[no_mangle]
pub extern "C" fn BZ2_bzwrite(
    mut b: *mut std::os::raw::c_void,
    mut buf: *mut std::os::raw::c_void,
    mut len: i32,
) -> i32 {
    let mut bzerr: i32 = 0;
    BZ2_bzWrite(&mut bzerr, b, buf, len);
    if bzerr == 0 {
        return len;
    } else {
        return -1;
    };
}

#[no_mangle]
pub extern "C" fn BZ2_bzflush(mut b: *mut std::os::raw::c_void) -> i32 {
    return 0;
}

#[no_mangle]
pub extern "C" fn BZ2_bzclose(mut b: *mut std::os::raw::c_void) {
    let mut bzerr: i32 = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    if b.is_null() {
        return;
    }
    fp = (*(b as *mut bzFile)).handle;
    if (*(b as *mut bzFile)).writing != 0 {
        BZ2_bzWriteClose(&mut bzerr, b, 0, 0 as *mut u32, 0 as *mut u32);
        if bzerr != 0 {
            BZ2_bzWriteClose(0 as *mut i32, b, 1, 0 as *mut u32, 0 as *mut u32);
        }
    } else {
        BZ2_bzReadClose(&mut bzerr, b);
    }
    unsafe {
        if fp != __stdinp && fp != __stdoutp {
            fclose(fp);
        }
    }
}

static mut bzerrorstrings: [*const i8; 16] = [
    b"OK\0" as *const u8 as *const i8,
    b"SEQUENCE_ERROR\0" as *const u8 as *const i8,
    b"PARAM_ERROR\0" as *const u8 as *const i8,
    b"MEM_ERROR\0" as *const u8 as *const i8,
    b"DATA_ERROR\0" as *const u8 as *const i8,
    b"DATA_ERROR_MAGIC\0" as *const u8 as *const i8,
    b"IO_ERROR\0" as *const u8 as *const i8,
    b"UNEXPECTED_EOF\0" as *const u8 as *const i8,
    b"OUTBUFF_FULL\0" as *const u8 as *const i8,
    b"CONFIG_ERROR\0" as *const u8 as *const i8,
    b"???\0" as *const u8 as *const i8,
    b"???\0" as *const u8 as *const i8,
    b"???\0" as *const u8 as *const i8,
    b"???\0" as *const u8 as *const i8,
    b"???\0" as *const u8 as *const i8,
    b"???\0" as *const u8 as *const i8,
];
#[no_mangle]
pub extern "C" fn BZ2_bzerror(mut b: *mut std::os::raw::c_void, mut errnum: *mut i32) -> *const i8 {
    let mut err: i32 = (*(b as *mut bzFile)).lastErr;
    if err > 0 {
        err = 0;
    }
    *errnum = err;
    unsafe {
        return bzerrorstrings[(err * -1i32) as usize];
    }
}
