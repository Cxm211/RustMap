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
    fn fwrite(_: *const std::os::raw::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn perror(_: *const i8);
    fn remove(_: *const i8) -> i32;
    fn rewind(_: *mut FILE);
    fn ungetc(_: i32, _: *mut FILE) -> i32;
    fn fdopen(_: i32, _: *const i8) -> *mut FILE;
    fn fileno(_: *mut FILE) -> i32;
    fn exit(_: i32) -> !;
    fn getenv(_: *const i8) -> *mut i8;
    fn free(_: *mut std::os::raw::c_void);
    fn signal(
        _: i32,
        _: Option<unsafe extern "C" fn(i32) -> ()>,
    ) -> Option<unsafe extern "C" fn(i32) -> ()>;
    fn malloc(_: u64) -> *mut std::os::raw::c_void;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strerror(_: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    fn __error() -> *mut i32;
    fn __maskrune(_: i32, _: u64) -> i32;
    static mut _DefaultRuneLocale: _RuneLocale;
    fn BZ2_bzReadOpen(
        bzerror: *mut i32,
        f: *mut FILE,
        verbosity_0: i32,
        small: i32,
        unused: *mut std::os::raw::c_void,
        nUnused: i32,
    ) -> *mut std::os::raw::c_void;
    fn BZ2_bzReadClose(bzerror: *mut i32, b: *mut std::os::raw::c_void);
    fn BZ2_bzReadGetUnused(
        bzerror: *mut i32,
        b: *mut std::os::raw::c_void,
        unused: *mut *mut std::os::raw::c_void,
        nUnused: *mut i32,
    );
    fn BZ2_bzRead(bzerror: *mut i32, b: *mut std::os::raw::c_void, buf: *mut std::os::raw::c_void, len: i32)
        -> i32;
    fn BZ2_bzWriteOpen(
        bzerror: *mut i32,
        f: *mut FILE,
        blockSize100k_0: i32,
        verbosity_0: i32,
        workFactor_0: i32,
    ) -> *mut std::os::raw::c_void;
    fn BZ2_bzWrite(bzerror: *mut i32, b: *mut std::os::raw::c_void, buf: *mut std::os::raw::c_void, len: i32);
    fn BZ2_bzWriteClose64(
        bzerror: *mut i32,
        b: *mut std::os::raw::c_void,
        abandon: i32,
        nbytes_in_lo32: *mut u32,
        nbytes_in_hi32: *mut u32,
        nbytes_out_lo32: *mut u32,
        nbytes_out_hi32: *mut u32,
    );
    fn BZ2_bzlibVersion() -> *const i8;
    fn open(_: *const i8, _: i32, _: ...) -> i32;
    fn utime(_: *const i8, _: *const utimbuf) -> i32;
    fn _exit(_: i32) -> !;
    fn close(_: i32) -> i32;
    fn isatty(_: i32) -> i32;
    fn write(__fd: i32, __buf: *const std::os::raw::c_void, __nbyte: u64) -> i64;
    fn fchown(_: i32, _: u32, _: u32) -> i32;
    fn fchmod(_: i32, _: u16) -> i32;
    fn lstat(_: *const i8, _: *mut stat) -> i32;
    fn stat(_: *const i8, _: *mut stat) -> i32;
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
pub type BZFILE = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utimbuf {
    pub actime: i64,
    pub modtime: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: i32,
    pub st_mode: u16,
    pub st_nlink: u16,
    pub st_ino: u64,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: i32,
    pub st_atimespec: timespec,
    pub st_mtimespec: timespec,
    pub st_ctimespec: timespec,
    pub st_birthtimespec: timespec,
    pub st_size: i64,
    pub st_blocks: i64,
    pub st_blksize: i32,
    pub st_flags: u32,
    pub st_gen: u32,
    pub st_lspare: i32,
    pub st_qspare: [i64; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UInt64 {
    pub b: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zzzz {
    pub name: *mut i8,
    pub link: *mut zzzz,
}
pub type Cell = zzzz;
#[inline]
extern "C" fn isascii(mut _c: i32) -> i32 {
    return (_c & !0x7fi32 == 0) as i32;
}

#[inline]
extern "C" fn __istype(mut _c: i32, mut _f: u64) -> i32 {
    unsafe {
        return if isascii(_c) != 0 {
            (_DefaultRuneLocale.__runetype[_c as usize] as u64 & _f != 0) as i32
        } else {
            (__maskrune(_c, _f) != 0) as i32
        };
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub extern "C" fn isspace(mut _c: i32) -> i32 {
    return __istype(_c, 0x4000);
}

#[no_mangle]
pub static mut verbosity: i32 = 0;
#[no_mangle]
pub static mut keepInputFiles: u8 = 0;
#[no_mangle]
pub static mut smallMode: u8 = 0;
#[no_mangle]
pub static mut deleteOutputOnInterrupt: u8 = 0;
#[no_mangle]
pub static mut forceOverwrite: u8 = 0;
#[no_mangle]
pub static mut testFailsExist: u8 = 0;
#[no_mangle]
pub static mut unzFailsExist: u8 = 0;
#[no_mangle]
pub static mut noisy: u8 = 0;
#[no_mangle]
pub static mut numFileNames: i32 = 0;
#[no_mangle]
pub static mut numFilesProcessed: i32 = 0;
#[no_mangle]
pub static mut blockSize100k: i32 = 0;
#[no_mangle]
pub static mut exitValue: i32 = 0;
#[no_mangle]
pub static mut opMode: i32 = 0;
#[no_mangle]
pub static mut srcMode: i32 = 0;
#[no_mangle]
pub static mut longestFileName: i32 = 0;
#[no_mangle]
pub static mut inName: [i8; 1034] = [0; 1034];
#[no_mangle]
pub static mut outName: [i8; 1034] = [0; 1034];
#[no_mangle]
pub static mut tmpName: [i8; 1034] = [0; 1034];
#[no_mangle]
pub static mut progName: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut progNameReally: [i8; 1034] = [0; 1034];
#[no_mangle]
pub static mut outputHandleJustInCase: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut workFactor: i32 = 0;
extern "C" fn uInt64_from_UInt32s(mut n: *mut UInt64, mut lo32: u32, mut hi32: u32) {
    (*n).b[7 as usize] = (hi32 >> 24 & 0xffu32) as u8;
    (*n).b[6 as usize] = (hi32 >> 16 & 0xffu32) as u8;
    (*n).b[5 as usize] = (hi32 >> 8 & 0xffu32) as u8;
    (*n).b[4 as usize] = (hi32 & 0xffu32) as u8;
    (*n).b[3 as usize] = (lo32 >> 24 & 0xffu32) as u8;
    (*n).b[2 as usize] = (lo32 >> 16 & 0xffu32) as u8;
    (*n).b[1 as usize] = (lo32 >> 8 & 0xffu32) as u8;
    (*n).b[0 as usize] = (lo32 & 0xffu32) as u8;
}

extern "C" fn uInt64_to_double(mut n: *mut UInt64) -> f64 {
    let mut i: i32 = 0;
    let mut base: f64 = 1.0f64;
    let mut sum: f64 = 0.0f64;
    i = 0;
    while i < 8 {
        sum += base * (*n).b[i as usize] as f64;
        base *= 256.0f64;
        i += 1;
        i;
    }
    return sum;
}

extern "C" fn uInt64_isZero(mut n: *mut UInt64) -> u8 {
    let mut i: i32 = 0;
    i = 0;
    while i < 8 {
        if (*n).b[i as usize] as i32 != 0 {
            return 0;
        }
        i += 1;
        i;
    }
    return 1;
}

extern "C" fn uInt64_qrm10(mut n: *mut UInt64) -> i32 {
    let mut rem: u32 = 0;
    let mut tmp: u32 = 0;
    let mut i: i32 = 0;
    rem = 0;
    i = 7;
    while i >= 0 {
        tmp = (rem * 256u32).wrapping_add((*n).b[i as usize] as u32);
        (*n).b[i as usize] = (tmp / 10u32) as u8;
        rem = tmp % 10;
        i -= 1;
        i;
    }
    return rem as i32;
}

extern "C" fn uInt64_toAscii(mut outbuf: *mut i8, mut n: *mut UInt64) {
    let mut i: i32 = 0;
    let mut q: i32 = 0;
    let mut buf: [u8; 32] = [0; 32];
    let mut nBuf: i32 = 0;
    let mut n_copy: UInt64 = *n;
    unsafe {
        loop {
            q = uInt64_qrm10(&mut n_copy);
            buf[nBuf as usize] = (q + '0' as i32) as u8;
            nBuf += 1;
            nBuf;
            if !(uInt64_isZero(&mut n_copy) == 0) {
                break;
            }
        }
        *outbuf.offset(nBuf as isize) = 0;
    }
    i = 0;
    unsafe {
        while i < nBuf {
            *outbuf.offset(i as isize) = buf[(nBuf - i - 1i32) as usize] as i8;
            i += 1;
            i;
        }
    }
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

extern "C" fn compressStream(mut stream: *mut FILE, mut zStream: *mut FILE) {
    let mut current_block: u64;
    let mut bzf: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut ibuf: [u8; 5000] = [0; 5000];
    let mut nIbuf: i32 = 0;
    let mut nbytes_in_lo32: u32 = 0;
    let mut nbytes_in_hi32: u32 = 0;
    let mut nbytes_out_lo32: u32 = 0;
    let mut nbytes_out_hi32: u32 = 0;
    let mut bzerr: i32 = 0;
    let mut bzerr_dummy: i32 = 0;
    let mut ret: i32 = 0;
    unsafe {
        if !(ferror(stream) != 0) {
            if !(ferror(zStream) != 0) {
                bzf = BZ2_bzWriteOpen(&mut bzerr, zStream, blockSize100k, verbosity, workFactor);
                if bzerr != 0 {
                    current_block = 527161663175172342;
                } else {
                    if verbosity >= 2 {
                        fprintf(__stderrp, b"\n\0" as *const u8 as *const i8);
                    }
                    loop {
                        if !(1 != 0) {
                            current_block = 9606288038608642794;
                            break;
                        }
                        if myfeof(stream) != 0 {
                            current_block = 9606288038608642794;
                            break;
                        }
                        nIbuf = fread(
                            ibuf.as_mut_ptr() as *mut std::os::raw::c_void,
                            ::std::mem::size_of::<u8>() as u64,
                            5000,
                            stream,
                        ) as i32;
                        if ferror(stream) != 0 {
                            current_block = 12978808799519007784;
                            break;
                        }
                        if nIbuf > 0 {
                            BZ2_bzWrite(
                                &mut bzerr,
                                bzf,
                                ibuf.as_mut_ptr() as *mut std::os::raw::c_void,
                                nIbuf,
                            );
                        }
                        if bzerr != 0 {
                            current_block = 527161663175172342;
                            break;
                        }
                    }
                    match current_block {
                        12978808799519007784 => {}
                        527161663175172342 => {}
                        _ => {
                            BZ2_bzWriteClose64(
                                &mut bzerr,
                                bzf,
                                0,
                                &mut nbytes_in_lo32,
                                &mut nbytes_in_hi32,
                                &mut nbytes_out_lo32,
                                &mut nbytes_out_hi32,
                            );
                            if bzerr != 0 {
                                current_block = 527161663175172342;
                            } else if ferror(zStream) != 0 {
                                current_block = 12978808799519007784;
                            } else {
                                ret = fflush(zStream);
                                if ret == -1 {
                                    current_block = 12978808799519007784;
                                } else {
                                    if zStream != __stdoutp {
                                        let mut fd: i32 = fileno(zStream);
                                        if fd < 0 {
                                            current_block = 12978808799519007784;
                                        } else {
                                            applySavedFileAttrToOutputFile(fd);
                                            ret = fclose(zStream);
                                            outputHandleJustInCase = 0 as *mut FILE;
                                            if ret == -1 {
                                                current_block = 12978808799519007784;
                                            } else {
                                                current_block = 9828876828309294594;
                                            }
                                        }
                                    } else {
                                        current_block = 9828876828309294594;
                                    }
                                    match current_block {
                                        12978808799519007784 => {}
                                        _ => {
                                            outputHandleJustInCase = 0 as *mut FILE;
                                            if ferror(stream) != 0 {
                                                current_block = 12978808799519007784;
                                            } else {
                                                ret = fclose(stream);
                                                if ret == -1 {
                                                    current_block = 12978808799519007784;
                                                } else {
                                                    if verbosity >= 1 {
                                                        if nbytes_in_lo32 == 0
                                                            && nbytes_in_hi32 == 0
                                                        {
                                                            fprintf(
                                                                __stderrp,
                                                                b" no data compressed.\n\0"
                                                                    as *const u8
                                                                    as *const i8,
                                                            );
                                                        } else {
                                                            let mut buf_nin: [i8; 32] = [0; 32];
                                                            let mut buf_nout: [i8; 32] = [0; 32];
                                                            let mut nbytes_in: UInt64 =
                                                                UInt64 { b: [0; 8] };
                                                            let mut nbytes_out: UInt64 =
                                                                UInt64 { b: [0; 8] };
                                                            let mut nbytes_in_d: f64 = 0.;
                                                            let mut nbytes_out_d: f64 = 0.;
                                                            uInt64_from_UInt32s(
                                                                &mut nbytes_in,
                                                                nbytes_in_lo32,
                                                                nbytes_in_hi32,
                                                            );
                                                            uInt64_from_UInt32s(
                                                                &mut nbytes_out,
                                                                nbytes_out_lo32,
                                                                nbytes_out_hi32,
                                                            );
                                                            nbytes_in_d =
                                                                uInt64_to_double(&mut nbytes_in);
                                                            nbytes_out_d =
                                                                uInt64_to_double(&mut nbytes_out);
                                                            uInt64_toAscii(
                                                                buf_nin.as_mut_ptr(),
                                                                &mut nbytes_in,
                                                            );
                                                            uInt64_toAscii(
                                                                buf_nout.as_mut_ptr(),
                                                                &mut nbytes_out,
                                                            );
                                                            fprintf (__stderrp, b"%6.3f:1, %6.3f bits/byte, %5.2f%% saved, %s in, %s out.\n\0" as * const u8 as * const i8, nbytes_in_d / nbytes_out_d, 8.0f64 * nbytes_out_d / nbytes_in_d, 100.0f64 * (1.0f64
                                                              - nbytes_out_d / nbytes_in_d), buf_nin.as_mut_ptr (), buf_nout.as_mut_ptr (),);
                                                        }
                                                    }
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                match current_block {
                    12978808799519007784 => {}
                    _ => {
                        BZ2_bzWriteClose64(
                            &mut bzerr_dummy,
                            bzf,
                            1,
                            &mut nbytes_in_lo32,
                            &mut nbytes_in_hi32,
                            &mut nbytes_out_lo32,
                            &mut nbytes_out_hi32,
                        );
                        match bzerr {
                            -9 => {
                                current_block = 5473069587530966810;
                                match current_block {
                                    4135077742796430766 => {
                                        panic(
                                            b"compress:unexpected error\0" as *const u8
                                                as *const i8,
                                        );
                                    }
                                    14062563307227712503 => {
                                        outOfMemory();
                                    }
                                    _ => {
                                        configError();
                                    }
                                }
                            }
                            -3 => {
                                current_block = 14062563307227712503;
                                match current_block {
                                    4135077742796430766 => {
                                        panic(
                                            b"compress:unexpected error\0" as *const u8
                                                as *const i8,
                                        );
                                    }
                                    14062563307227712503 => {
                                        outOfMemory();
                                    }
                                    _ => {
                                        configError();
                                    }
                                }
                            }
                            -6 => {}
                            _ => {
                                current_block = 4135077742796430766;
                                match current_block {
                                    4135077742796430766 => {
                                        panic(
                                            b"compress:unexpected error\0" as *const u8
                                                as *const i8,
                                        );
                                    }
                                    14062563307227712503 => {
                                        outOfMemory();
                                    }
                                    _ => {
                                        configError();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ioError();
}

extern "C" fn uncompressStream(mut zStream: *mut FILE, mut stream: *mut FILE) -> u8 {
    let mut current_block: u64;
    let mut bzf: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut bzerr: i32 = 0;
    let mut bzerr_dummy: i32 = 0;
    let mut ret: i32 = 0;
    let mut nread: i32 = 0;
    let mut streamNo: i32 = 0;
    let mut i: i32 = 0;
    let mut obuf: [u8; 5000] = [0; 5000];
    let mut unused: [u8; 5000] = [0; 5000];
    let mut nUnused: i32 = 0;
    let mut unusedTmpV: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut unusedTmp: *mut u8 = 0 as *mut u8;
    nUnused = 0;
    streamNo = 0;
    unsafe {
        if !(ferror(stream) != 0) {
            if !(ferror(zStream) != 0) {
                's_37: loop {
                    if !(1 != 0) {
                        current_block = 6930817534769675471;
                        break;
                    }
                    bzf = BZ2_bzReadOpen(
                        &mut bzerr,
                        zStream,
                        verbosity,
                        smallMode as i32,
                        unused.as_mut_ptr() as *mut std::os::raw::c_void,
                        nUnused,
                    );
                    if bzf.is_null() || bzerr != 0 {
                        current_block = 11494384069833310849;
                        break;
                    }
                    streamNo += 1;
                    streamNo;
                    while bzerr == 0 {
                        nread = BZ2_bzRead(
                            &mut bzerr,
                            bzf,
                            obuf.as_mut_ptr() as *mut std::os::raw::c_void,
                            5000,
                        );
                        if bzerr == -5 {
                            current_block = 7959453693964252812;
                            break 's_37;
                        }
                        if (bzerr == 0 || bzerr == 4) && nread > 0 {
                            fwrite(
                                obuf.as_mut_ptr() as *const std::os::raw::c_void,
                                ::std::mem::size_of::<u8>() as u64,
                                nread as u64,
                                stream,
                            );
                        }
                        if ferror(stream) != 0 {
                            current_block = 11017706264856393887;
                            break 's_37;
                        }
                    }
                    if bzerr != 4 {
                        current_block = 11494384069833310849;
                        break;
                    }
                    BZ2_bzReadGetUnused(&mut bzerr, bzf, &mut unusedTmpV, &mut nUnused);
                    if bzerr != 0 {
                        panic(b"decompress:bzReadGetUnused\0" as *const u8 as *const i8);
                    }
                    unusedTmp = unusedTmpV as *mut u8;
                    i = 0;
                    while i < nUnused {
                        unused[i as usize] = *unusedTmp.offset(i as isize);
                        i += 1;
                        i;
                    }
                    BZ2_bzReadClose(&mut bzerr, bzf);
                    if bzerr != 0 {
                        panic(b"decompress:bzReadGetUnused\0" as *const u8 as *const i8);
                    }
                    if nUnused == 0 && myfeof(zStream) as i32 != 0 {
                        current_block = 6930817534769675471;
                        break;
                    }
                }
                match current_block {
                    11017706264856393887 => {}
                    _ => {
                        match current_block {
                            7959453693964252812 => {
                                if forceOverwrite != 0 {
                                    rewind(zStream);
                                    loop {
                                        if !(1 != 0) {
                                            current_block = 6930817534769675471;
                                            break;
                                        }
                                        if myfeof(zStream) != 0 {
                                            current_block = 6930817534769675471;
                                            break;
                                        }
                                        nread = fread(
                                            obuf.as_mut_ptr() as *mut std::os::raw::c_void,
                                            ::std::mem::size_of::<u8>() as u64,
                                            5000,
                                            zStream,
                                        ) as i32;
                                        if ferror(zStream) != 0 {
                                            current_block = 11017706264856393887;
                                            break;
                                        }
                                        if nread > 0 {
                                            fwrite(
                                                obuf.as_mut_ptr() as *const std::os::raw::c_void,
                                                ::std::mem::size_of::<u8>() as u64,
                                                nread as u64,
                                                stream,
                                            );
                                        }
                                        if ferror(stream) != 0 {
                                            current_block = 11017706264856393887;
                                            break;
                                        }
                                    }
                                } else {
                                    current_block = 11494384069833310849;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            11017706264856393887 => {}
                            _ => match current_block {
                                11494384069833310849 => {
                                    BZ2_bzReadClose(&mut bzerr_dummy, bzf);
                                    match bzerr {
                                        -9 => {
                                            current_block = 17471272622514765608;
                                            match current_block {
                                                12493785325868445109 => {
                                                    panic(
                                                        b"decompress:unexpected error\0"
                                                            as *const u8
                                                            as *const i8,
                                                    );
                                                }
                                                17471272622514765608 => {
                                                    configError();
                                                }
                                                7828606719164582517 => {
                                                    crcError();
                                                }
                                                11918817405682608102 => {
                                                    outOfMemory();
                                                }
                                                3036625737573344418 => {
                                                    compressedStreamEOF();
                                                }
                                                _ => {
                                                    if zStream != __stdinp {
                                                        fclose(zStream);
                                                    }
                                                    if stream != __stdoutp {
                                                        fclose(stream);
                                                    }
                                                    if streamNo == 1 {
                                                        return 0;
                                                    } else {
                                                        if noisy != 0 {
                                                            fprintf (__stderrp, b"\n%s: %s: trailing garbage after EOF ignored\n\0" as * const u8 as * const i8, progName, inName.as_mut_ptr (),);
                                                        }
                                                        return 1;
                                                    }
                                                }
                                            }
                                        }
                                        -6 => {}
                                        -4 => {
                                            current_block = 7828606719164582517;
                                            match current_block {
                                                12493785325868445109 => {
                                                    panic(
                                                        b"decompress:unexpected error\0"
                                                            as *const u8
                                                            as *const i8,
                                                    );
                                                }
                                                17471272622514765608 => {
                                                    configError();
                                                }
                                                7828606719164582517 => {
                                                    crcError();
                                                }
                                                11918817405682608102 => {
                                                    outOfMemory();
                                                }
                                                3036625737573344418 => {
                                                    compressedStreamEOF();
                                                }
                                                _ => {
                                                    if zStream != __stdinp {
                                                        fclose(zStream);
                                                    }
                                                    if stream != __stdoutp {
                                                        fclose(stream);
                                                    }
                                                    if streamNo == 1 {
                                                        return 0;
                                                    } else {
                                                        if noisy != 0 {
                                                            fprintf (__stderrp, b"\n%s: %s: trailing garbage after EOF ignored\n\0" as * const u8 as * const i8, progName, inName.as_mut_ptr (),);
                                                        }
                                                        return 1;
                                                    }
                                                }
                                            }
                                        }
                                        -3 => {
                                            current_block = 11918817405682608102;
                                            match current_block {
                                                12493785325868445109 => {
                                                    panic(
                                                        b"decompress:unexpected error\0"
                                                            as *const u8
                                                            as *const i8,
                                                    );
                                                }
                                                17471272622514765608 => {
                                                    configError();
                                                }
                                                7828606719164582517 => {
                                                    crcError();
                                                }
                                                11918817405682608102 => {
                                                    outOfMemory();
                                                }
                                                3036625737573344418 => {
                                                    compressedStreamEOF();
                                                }
                                                _ => {
                                                    if zStream != __stdinp {
                                                        fclose(zStream);
                                                    }
                                                    if stream != __stdoutp {
                                                        fclose(stream);
                                                    }
                                                    if streamNo == 1 {
                                                        return 0;
                                                    } else {
                                                        if noisy != 0 {
                                                            fprintf (__stderrp, b"\n%s: %s: trailing garbage after EOF ignored\n\0" as * const u8 as * const i8, progName, inName.as_mut_ptr (),);
                                                        }
                                                        return 1;
                                                    }
                                                }
                                            }
                                        }
                                        -7 => {
                                            current_block = 3036625737573344418;
                                            match current_block {
                                                12493785325868445109 => {
                                                    panic(
                                                        b"decompress:unexpected error\0"
                                                            as *const u8
                                                            as *const i8,
                                                    );
                                                }
                                                17471272622514765608 => {
                                                    configError();
                                                }
                                                7828606719164582517 => {
                                                    crcError();
                                                }
                                                11918817405682608102 => {
                                                    outOfMemory();
                                                }
                                                3036625737573344418 => {
                                                    compressedStreamEOF();
                                                }
                                                _ => {
                                                    if zStream != __stdinp {
                                                        fclose(zStream);
                                                    }
                                                    if stream != __stdoutp {
                                                        fclose(stream);
                                                    }
                                                    if streamNo == 1 {
                                                        return 0;
                                                    } else {
                                                        if noisy != 0 {
                                                            fprintf (__stderrp, b"\n%s: %s: trailing garbage after EOF ignored\n\0" as * const u8 as * const i8, progName, inName.as_mut_ptr (),);
                                                        }
                                                        return 1;
                                                    }
                                                }
                                            }
                                        }
                                        -5 => {
                                            current_block = 2023660579338530765;
                                            match current_block {
                                                12493785325868445109 => {
                                                    panic(
                                                        b"decompress:unexpected error\0"
                                                            as *const u8
                                                            as *const i8,
                                                    );
                                                }
                                                17471272622514765608 => {
                                                    configError();
                                                }
                                                7828606719164582517 => {
                                                    crcError();
                                                }
                                                11918817405682608102 => {
                                                    outOfMemory();
                                                }
                                                3036625737573344418 => {
                                                    compressedStreamEOF();
                                                }
                                                _ => {
                                                    if zStream != __stdinp {
                                                        fclose(zStream);
                                                    }
                                                    if stream != __stdoutp {
                                                        fclose(stream);
                                                    }
                                                    if streamNo == 1 {
                                                        return 0;
                                                    } else {
                                                        if noisy != 0 {
                                                            fprintf (__stderrp, b"\n%s: %s: trailing garbage after EOF ignored\n\0" as * const u8 as * const i8, progName, inName.as_mut_ptr (),);
                                                        }
                                                        return 1;
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            current_block = 12493785325868445109;
                                            match current_block {
                                                12493785325868445109 => {
                                                    panic(
                                                        b"decompress:unexpected error\0"
                                                            as *const u8
                                                            as *const i8,
                                                    );
                                                }
                                                17471272622514765608 => {
                                                    configError();
                                                }
                                                7828606719164582517 => {
                                                    crcError();
                                                }
                                                11918817405682608102 => {
                                                    outOfMemory();
                                                }
                                                3036625737573344418 => {
                                                    compressedStreamEOF();
                                                }
                                                _ => {
                                                    if zStream != __stdinp {
                                                        fclose(zStream);
                                                    }
                                                    if stream != __stdoutp {
                                                        fclose(stream);
                                                    }
                                                    if streamNo == 1 {
                                                        return 0;
                                                    } else {
                                                        if noisy != 0 {
                                                            fprintf (__stderrp, b"\n%s: %s: trailing garbage after EOF ignored\n\0" as * const u8 as * const i8, progName, inName.as_mut_ptr (),);
                                                        }
                                                        return 1;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    if !(ferror(zStream) != 0) {
                                        if stream != __stdoutp {
                                            let mut fd: i32 = fileno(stream);
                                            if fd < 0 {
                                                current_block = 11017706264856393887;
                                            } else {
                                                applySavedFileAttrToOutputFile(fd);
                                                current_block = 11459959175219260272;
                                            }
                                        } else {
                                            current_block = 11459959175219260272;
                                        }
                                        match current_block {
                                            11017706264856393887 => {}
                                            _ => {
                                                ret = fclose(zStream);
                                                if !(ret == -1) {
                                                    if !(ferror(stream) != 0) {
                                                        ret = fflush(stream);
                                                        if !(ret != 0) {
                                                            if stream != __stdoutp {
                                                                ret = fclose(stream);
                                                                outputHandleJustInCase =
                                                                    0 as *mut FILE;
                                                                if ret == -1 {
                                                                    current_block =
                                                                        11017706264856393887;
                                                                } else {
                                                                    current_block =
                                                                        3123434771885419771;
                                                                }
                                                            } else {
                                                                current_block = 3123434771885419771;
                                                            }
                                                            match current_block {
                                                                11017706264856393887 => {}
                                                                _ => {
                                                                    outputHandleJustInCase =
                                                                        0 as *mut FILE;
                                                                    if verbosity >= 2 {
                                                                        fprintf(
                                                                            __stderrp,
                                                                            b"\n    \0" as *const u8
                                                                                as *const i8,
                                                                        );
                                                                    }
                                                                    return 1;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
            }
        }
    }
    ioError();
}

extern "C" fn testStream(mut zStream: *mut FILE) -> u8 {
    let mut current_block: u64;
    let mut bzf: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut bzerr: i32 = 0;
    let mut bzerr_dummy: i32 = 0;
    let mut ret: i32 = 0;
    let mut streamNo: i32 = 0;
    let mut i: i32 = 0;
    let mut obuf: [u8; 5000] = [0; 5000];
    let mut unused: [u8; 5000] = [0; 5000];
    let mut nUnused: i32 = 0;
    let mut unusedTmpV: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut unusedTmp: *mut u8 = 0 as *mut u8;
    nUnused = 0;
    streamNo = 0;
    unsafe {
        if !(ferror(zStream) != 0) {
            's_29: loop {
                if !(1 != 0) {
                    current_block = 5783071609795492627;
                    break;
                }
                bzf = BZ2_bzReadOpen(
                    &mut bzerr,
                    zStream,
                    verbosity,
                    smallMode as i32,
                    unused.as_mut_ptr() as *mut std::os::raw::c_void,
                    nUnused,
                );
                if bzf.is_null() || bzerr != 0 {
                    current_block = 6558334290726384385;
                    break;
                }
                streamNo += 1;
                streamNo;
                while bzerr == 0 {
                    BZ2_bzRead(
                        &mut bzerr,
                        bzf,
                        obuf.as_mut_ptr() as *mut std::os::raw::c_void,
                        5000,
                    );
                    if bzerr == -5 {
                        current_block = 6558334290726384385;
                        break 's_29;
                    }
                }
                if bzerr != 4 {
                    current_block = 6558334290726384385;
                    break;
                }
                BZ2_bzReadGetUnused(&mut bzerr, bzf, &mut unusedTmpV, &mut nUnused);
                if bzerr != 0 {
                    panic(b"test:bzReadGetUnused\0" as *const u8 as *const i8);
                }
                unusedTmp = unusedTmpV as *mut u8;
                i = 0;
                while i < nUnused {
                    unused[i as usize] = *unusedTmp.offset(i as isize);
                    i += 1;
                    i;
                }
                BZ2_bzReadClose(&mut bzerr, bzf);
                if bzerr != 0 {
                    panic(b"test:bzReadGetUnused\0" as *const u8 as *const i8);
                }
                if nUnused == 0 && myfeof(zStream) as i32 != 0 {
                    current_block = 5783071609795492627;
                    break;
                }
            }
            match current_block {
                5783071609795492627 => {
                    if !(ferror(zStream) != 0) {
                        ret = fclose(zStream);
                        if !(ret == -1) {
                            if verbosity >= 2 {
                                fprintf(__stderrp, b"\n    \0" as *const u8 as *const i8);
                            }
                            return 1;
                        }
                    }
                }
                _ => {
                    BZ2_bzReadClose(&mut bzerr_dummy, bzf);
                    if verbosity == 0 {
                        fprintf(
                            __stderrp,
                            b"%s: %s: \0" as *const u8 as *const i8,
                            progName,
                            inName.as_mut_ptr(),
                        );
                    }
                    match bzerr {
                        -9 => {
                            current_block = 11077065360137928378;
                            match current_block {
                                11378899144307935806 => {
                                    panic(b"test:unexpected error\0" as *const u8 as *const i8);
                                }
                                4262344653356995236 => {
                                    fprintf(
                                        __stderrp,
                                        b"file ends unexpectedly\n\0" as *const u8 as *const i8,
                                    );
                                    return 0;
                                }
                                14669664700600221311 => {
                                    if zStream != __stdinp {
                                        fclose(zStream);
                                    }
                                    if streamNo == 1 {
                                        fprintf(
                                            __stderrp,
                                            b"bad magic number (file not created by bzip2)\n\0"
                                                as *const u8
                                                as *const i8,
                                        );
                                        return 0;
                                    } else {
                                        if noisy != 0 {
                                            fprintf(
                                                __stderrp,
                                                b"trailing garbage after EOF ignored\n\0"
                                                    as *const u8
                                                    as *const i8,
                                            );
                                        }
                                        return 1;
                                    }
                                }
                                11077065360137928378 => {
                                    configError();
                                }
                                11454755363491939990 => {
                                    outOfMemory();
                                }
                                _ => {
                                    fprintf(
                                        __stderrp,
                                        b"data integrity (CRC) error in data\n\0" as *const u8
                                            as *const i8,
                                    );
                                    return 0;
                                }
                            }
                        }
                        -6 => {}
                        -4 => {
                            current_block = 1061549160212432160;
                            match current_block {
                                11378899144307935806 => {
                                    panic(b"test:unexpected error\0" as *const u8 as *const i8);
                                }
                                4262344653356995236 => {
                                    fprintf(
                                        __stderrp,
                                        b"file ends unexpectedly\n\0" as *const u8 as *const i8,
                                    );
                                    return 0;
                                }
                                14669664700600221311 => {
                                    if zStream != __stdinp {
                                        fclose(zStream);
                                    }
                                    if streamNo == 1 {
                                        fprintf(
                                            __stderrp,
                                            b"bad magic number (file not created by bzip2)\n\0"
                                                as *const u8
                                                as *const i8,
                                        );
                                        return 0;
                                    } else {
                                        if noisy != 0 {
                                            fprintf(
                                                __stderrp,
                                                b"trailing garbage after EOF ignored\n\0"
                                                    as *const u8
                                                    as *const i8,
                                            );
                                        }
                                        return 1;
                                    }
                                }
                                11077065360137928378 => {
                                    configError();
                                }
                                11454755363491939990 => {
                                    outOfMemory();
                                }
                                _ => {
                                    fprintf(
                                        __stderrp,
                                        b"data integrity (CRC) error in data\n\0" as *const u8
                                            as *const i8,
                                    );
                                    return 0;
                                }
                            }
                        }
                        -3 => {
                            current_block = 11454755363491939990;
                            match current_block {
                                11378899144307935806 => {
                                    panic(b"test:unexpected error\0" as *const u8 as *const i8);
                                }
                                4262344653356995236 => {
                                    fprintf(
                                        __stderrp,
                                        b"file ends unexpectedly\n\0" as *const u8 as *const i8,
                                    );
                                    return 0;
                                }
                                14669664700600221311 => {
                                    if zStream != __stdinp {
                                        fclose(zStream);
                                    }
                                    if streamNo == 1 {
                                        fprintf(
                                            __stderrp,
                                            b"bad magic number (file not created by bzip2)\n\0"
                                                as *const u8
                                                as *const i8,
                                        );
                                        return 0;
                                    } else {
                                        if noisy != 0 {
                                            fprintf(
                                                __stderrp,
                                                b"trailing garbage after EOF ignored\n\0"
                                                    as *const u8
                                                    as *const i8,
                                            );
                                        }
                                        return 1;
                                    }
                                }
                                11077065360137928378 => {
                                    configError();
                                }
                                11454755363491939990 => {
                                    outOfMemory();
                                }
                                _ => {
                                    fprintf(
                                        __stderrp,
                                        b"data integrity (CRC) error in data\n\0" as *const u8
                                            as *const i8,
                                    );
                                    return 0;
                                }
                            }
                        }
                        -7 => {
                            current_block = 4262344653356995236;
                            match current_block {
                                11378899144307935806 => {
                                    panic(b"test:unexpected error\0" as *const u8 as *const i8);
                                }
                                4262344653356995236 => {
                                    fprintf(
                                        __stderrp,
                                        b"file ends unexpectedly\n\0" as *const u8 as *const i8,
                                    );
                                    return 0;
                                }
                                14669664700600221311 => {
                                    if zStream != __stdinp {
                                        fclose(zStream);
                                    }
                                    if streamNo == 1 {
                                        fprintf(
                                            __stderrp,
                                            b"bad magic number (file not created by bzip2)\n\0"
                                                as *const u8
                                                as *const i8,
                                        );
                                        return 0;
                                    } else {
                                        if noisy != 0 {
                                            fprintf(
                                                __stderrp,
                                                b"trailing garbage after EOF ignored\n\0"
                                                    as *const u8
                                                    as *const i8,
                                            );
                                        }
                                        return 1;
                                    }
                                }
                                11077065360137928378 => {
                                    configError();
                                }
                                11454755363491939990 => {
                                    outOfMemory();
                                }
                                _ => {
                                    fprintf(
                                        __stderrp,
                                        b"data integrity (CRC) error in data\n\0" as *const u8
                                            as *const i8,
                                    );
                                    return 0;
                                }
                            }
                        }
                        -5 => {
                            current_block = 14669664700600221311;
                            match current_block {
                                11378899144307935806 => {
                                    panic(b"test:unexpected error\0" as *const u8 as *const i8);
                                }
                                4262344653356995236 => {
                                    fprintf(
                                        __stderrp,
                                        b"file ends unexpectedly\n\0" as *const u8 as *const i8,
                                    );
                                    return 0;
                                }
                                14669664700600221311 => {
                                    if zStream != __stdinp {
                                        fclose(zStream);
                                    }
                                    if streamNo == 1 {
                                        fprintf(
                                            __stderrp,
                                            b"bad magic number (file not created by bzip2)\n\0"
                                                as *const u8
                                                as *const i8,
                                        );
                                        return 0;
                                    } else {
                                        if noisy != 0 {
                                            fprintf(
                                                __stderrp,
                                                b"trailing garbage after EOF ignored\n\0"
                                                    as *const u8
                                                    as *const i8,
                                            );
                                        }
                                        return 1;
                                    }
                                }
                                11077065360137928378 => {
                                    configError();
                                }
                                11454755363491939990 => {
                                    outOfMemory();
                                }
                                _ => {
                                    fprintf(
                                        __stderrp,
                                        b"data integrity (CRC) error in data\n\0" as *const u8
                                            as *const i8,
                                    );
                                    return 0;
                                }
                            }
                        }
                        _ => {
                            current_block = 11378899144307935806;
                            match current_block {
                                11378899144307935806 => {
                                    panic(b"test:unexpected error\0" as *const u8 as *const i8);
                                }
                                4262344653356995236 => {
                                    fprintf(
                                        __stderrp,
                                        b"file ends unexpectedly\n\0" as *const u8 as *const i8,
                                    );
                                    return 0;
                                }
                                14669664700600221311 => {
                                    if zStream != __stdinp {
                                        fclose(zStream);
                                    }
                                    if streamNo == 1 {
                                        fprintf(
                                            __stderrp,
                                            b"bad magic number (file not created by bzip2)\n\0"
                                                as *const u8
                                                as *const i8,
                                        );
                                        return 0;
                                    } else {
                                        if noisy != 0 {
                                            fprintf(
                                                __stderrp,
                                                b"trailing garbage after EOF ignored\n\0"
                                                    as *const u8
                                                    as *const i8,
                                            );
                                        }
                                        return 1;
                                    }
                                }
                                11077065360137928378 => {
                                    configError();
                                }
                                11454755363491939990 => {
                                    outOfMemory();
                                }
                                _ => {
                                    fprintf(
                                        __stderrp,
                                        b"data integrity (CRC) error in data\n\0" as *const u8
                                            as *const i8,
                                    );
                                    return 0;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ioError();
}

extern "C" fn setExit(mut v: i32) {
    unsafe {
        if v > exitValue {
            exitValue = v;
        }
    }
}

extern "C" fn cadvise() {
    unsafe {
        if noisy != 0 {
            fprintf (__stderrp,
    b"\nIt is possible that the compressed file(s) have become corrupted.\nYou can use the -tvv option to test integrity of such files.\n\nYou can use the `bzip2recover' program to attempt to recover\ndata from undamaged sections of corrupted files.\n\n\0"
              as * const u8 as * const i8,);
        }
    }
}

extern "C" fn showFileNames() {
    unsafe {
        if noisy != 0 {
            fprintf(
                __stderrp,
                b"\tInput file = %s, output file = %s\n\0" as *const u8 as *const i8,
                inName.as_mut_ptr(),
                outName.as_mut_ptr(),
            );
        }
    }
}

extern "C" fn cleanUpAndFail(mut ec: i32) -> ! {
    let mut retVal: i32 = 0;
    unsafe {
        let mut statBuf: stat = stat {
            st_dev: 0,
            st_mode: 0,
            st_nlink: 0,
            st_ino: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_atimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_birthtimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_size: 0,
            st_blocks: 0,
            st_blksize: 0,
            st_flags: 0,
            st_gen: 0,
            st_lspare: 0,
            st_qspare: [0; 2],
        };
        if srcMode == 3 && opMode != 3 && deleteOutputOnInterrupt as i32 != 0 {
            retVal = stat(inName.as_mut_ptr(), &mut statBuf);
            if retVal == 0 {
                if noisy != 0 {
                    fprintf(
                        __stderrp,
                        b"%s: Deleting output file %s, if it exists.\n\0" as *const u8 as *const i8,
                        progName,
                        outName.as_mut_ptr(),
                    );
                }
                if !outputHandleJustInCase.is_null() {
                    fclose(outputHandleJustInCase);
                }
                retVal = remove(outName.as_mut_ptr());
                if retVal != 0 {
                    fprintf(
                        __stderrp,
                        b"%s: WARNING: deletion of output file (apparently) failed.\n\0"
                            as *const u8 as *const i8,
                        progName,
                    );
                }
            } else {
                fprintf(
                    __stderrp,
                    b"%s: WARNING: deletion of output file suppressed\n\0" as *const u8
                        as *const i8,
                    progName,
                );
                fprintf(
                    __stderrp,
                    b"%s:    since input file no longer exists.  Output file\n\0" as *const u8
                        as *const i8,
                    progName,
                );
                fprintf(
                    __stderrp,
                    b"%s:    `%s' may be incomplete.\n\0" as *const u8 as *const i8,
                    progName,
                    outName.as_mut_ptr(),
                );
                fprintf(
                    __stderrp,
                    b"%s:    I suggest doing an integrity test (bzip2 -tv) of it.\n\0" as *const u8
                        as *const i8,
                    progName,
                );
            }
        }
        if noisy as i32 != 0 && numFileNames > 0 && numFilesProcessed < numFileNames {
            fprintf (__stderrp, b"%s: WARNING: some files have not been processed:\n%s:    %d specified on command line, %d not processed yet.\n\n\0" as * const u8 as * const i8, progName, progName, numFileNames, numFileNames - numFilesProcessed,);
        }
    }
    setExit(ec);
    unsafe {
        exit(exitValue);
    }
}

extern "C" fn panic(mut s: *const i8) -> ! {
    unsafe {
        fprintf (__stderrp, b"\n%s: PANIC -- internal consistency error:\n\t%s\n\tThis is a BUG.  Please report it to:\n\tbzip2-devel@sourceware.org\n\0" as * const u8 as * const i8, progName, s,);
    }
    showFileNames();
    cleanUpAndFail(3);
}

extern "C" fn crcError() -> ! {
    unsafe {
        fprintf(
            __stderrp,
            b"\n%s: Data integrity error when decompressing.\n\0" as *const u8 as *const i8,
            progName,
        );
    }
    showFileNames();
    cadvise();
    cleanUpAndFail(2);
}

extern "C" fn compressedStreamEOF() -> ! {
    unsafe {
        if noisy != 0 {
            fprintf (__stderrp, b"\n%s: Compressed file ends unexpectedly;\n\tperhaps it is corrupted?  *Possible* reason follows.\n\0" as * const u8 as * const i8, progName,);
            perror(progName);
            showFileNames();
            cadvise();
        }
    }
    cleanUpAndFail(2);
}

extern "C" fn ioError() -> ! {
    unsafe {
        fprintf(
            __stderrp,
            b"\n%s: I/O or other error, bailing out.  Possible reason follows.\n\0" as *const u8
                as *const i8,
            progName,
        );
        perror(progName);
    }
    showFileNames();
    cleanUpAndFail(1);
}

extern "C" fn mySignalCatcher(mut n: i32) {
    unsafe {
        fprintf(
            __stderrp,
            b"\n%s: Control-C or similar caught, quitting.\n\0" as *const u8 as *const i8,
            progName,
        );
    }
    cleanUpAndFail(1);
}

extern "C" fn mySIGSEGVorSIGBUScatcher(mut n: i32) {
    let mut msg: *const i8 = 0 as *const i8;
    unsafe {
        if opMode == 1 {
            msg =
b": Caught a SIGSEGV or SIGBUS whilst compressing.\n\n   Possible causes are (most likely first):\n   (1) This computer has unreliable memory or cache hardware\n       (a surprisingly common problem; try a different machine.)\n   (2) A bug in the compiler used to create this executable\n       (unlikely, if you didn't compile bzip2 yourself.)\n   (3) A real bug in bzip2 -- I hope this should never be the case.\n   The user's manual, Section 4.3, has more info on (1) and (2).\n   \n   If you suspect this is a bug in bzip2, or are unsure about (1)\n   or (2), feel free to report it to: bzip2-devel@sourceware.org.\n   Section 4.3 of the user's manual describes the info a useful\n   bug report should have.  If the manual is available on your\n   system, please try and read it before mailing me.  If you don't\n   have the manual or can't be bothered to read it, mail me anyway.\n\n\0"
             as * const u8 as * const i8;
        } else {
            msg =
b": Caught a SIGSEGV or SIGBUS whilst decompressing.\n\n   Possible causes are (most likely first):\n   (1) The compressed data is corrupted, and bzip2's usual checks\n       failed to detect this.  Try bzip2 -tvv my_file.bz2.\n   (2) This computer has unreliable memory or cache hardware\n       (a surprisingly common problem; try a different machine.)\n   (3) A bug in the compiler used to create this executable\n       (unlikely, if you didn't compile bzip2 yourself.)\n   (4) A real bug in bzip2 -- I hope this should never be the case.\n   The user's manual, Section 4.3, has more info on (2) and (3).\n   \n   If you suspect this is a bug in bzip2, or are unsure about (2)\n   or (3), feel free to report it to: bzip2-devel@sourceware.org.\n   Section 4.3 of the user's manual describes the info a useful\n   bug report should have.  If the manual is available on your\n   system, please try and read it before mailing me.  If you don't\n   have the manual or can't be bothered to read it, mail me anyway.\n\n\0"
             as * const u8 as * const i8;
        }
        write(
            2,
            b"\n\0" as *const u8 as *const i8 as *const std::os::raw::c_void,
            1,
        );
        write(2, progName as *const std::os::raw::c_void, strlen(progName));
        write(2, msg as *const std::os::raw::c_void, strlen(msg));
    }
    msg = b"\tInput file = \0" as *const u8 as *const i8;
    unsafe {
        write(2, msg as *const std::os::raw::c_void, strlen(msg));
        write(
            2,
            inName.as_mut_ptr() as *const std::os::raw::c_void,
            strlen(inName.as_mut_ptr()),
        );
        write(
            2,
            b"\n\0" as *const u8 as *const i8 as *const std::os::raw::c_void,
            1,
        );
    }
    msg = b"\tOutput file = \0" as *const u8 as *const i8;
    unsafe {
        write(2, msg as *const std::os::raw::c_void, strlen(msg));
        write(
            2,
            outName.as_mut_ptr() as *const std::os::raw::c_void,
            strlen(outName.as_mut_ptr()),
        );
        write(
            2,
            b"\n\0" as *const u8 as *const i8 as *const std::os::raw::c_void,
            1,
        );
        if opMode == 1 {
            setExit(3);
        } else {
            setExit(2);
        }
        _exit(exitValue);
    }
}

extern "C" fn outOfMemory() -> ! {
    unsafe {
        fprintf(
            __stderrp,
            b"\n%s: couldn't allocate enough memory\n\0" as *const u8 as *const i8,
            progName,
        );
    }
    showFileNames();
    cleanUpAndFail(1);
}

extern "C" fn configError() -> ! {
    unsafe {
        fprintf (__stderrp,
       b"bzip2: I'm not configured correctly for this platform!\n\tI require Int32, Int16 and Char to have sizes\n\tof 4, 2 and 1 bytes to run properly, and they don't.\n\tProbably you can fix this by defining them correctly,\n\tand recompiling.  Bye!\n\0"
          as * const u8 as * const i8,);
    }
    setExit(3);
    unsafe {
        exit(exitValue);
    }
}

extern "C" fn pad(mut s: *mut i8) {
    let mut i: i32 = 0;
    unsafe {
        if strlen(s) as i32 >= longestFileName {
            return;
        }
    }
    i = 1;
    unsafe {
        while i <= longestFileName - strlen(s) as i32 {
            fprintf(__stderrp, b" \0" as *const u8 as *const i8);
            i += 1;
            i;
        }
    }
}

extern "C" fn copyFileName(mut to: *mut i8, mut from: *mut i8) {
    unsafe {
        if strlen(from) > (1034 - 10i32) as u64 {
            fprintf (__stderrp, b"bzip2: file name\n`%s'\nis suspiciously (more than %d chars) long.\nTry using a reasonable file name instead.  Sorry! :-)\n\0" as * const u8 as * const i8, from, 1034 - 10,);
            setExit(1);
            exit(exitValue);
        }
        strncpy(to, from, (1034 - 10i32) as u64);
        *to.offset((1034 - 10i32) as isize) = '\0' as i8;
    }
}

extern "C" fn fileExists(mut name: *mut i8) -> u8 {
    unsafe {
        let mut tmp: *mut FILE = fopen(name, b"rb\0" as *const u8 as *const i8);
        let mut exists: u8 = (tmp != 0 as *mut std::os::raw::c_void as *mut FILE) as u8;
        if !tmp.is_null() {
            fclose(tmp);
        }
        return exists;
    }
}

extern "C" fn fopen_output_safely(mut name: *mut i8, mut mode: *const i8) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut fh: i32 = 0;
    unsafe {
        fh = open(name, 0x1 | 0x200 | 0x800, 0o200 | 0o400);
    }
    if fh == -1 {
        return 0 as *mut FILE;
    }
    unsafe {
        fp = fdopen(fh, mode);
        if fp.is_null() {
            close(fh);
        }
    }
    return fp;
}

extern "C" fn notAStandardFile(mut name: *mut i8) -> u8 {
    let mut i: i32 = 0;
    unsafe {
        let mut statBuf: stat = stat {
            st_dev: 0,
            st_mode: 0,
            st_nlink: 0,
            st_ino: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_atimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_birthtimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_size: 0,
            st_blocks: 0,
            st_blksize: 0,
            st_flags: 0,
            st_gen: 0,
            st_lspare: 0,
            st_qspare: [0; 2],
        };
        i = lstat(name, &mut statBuf);
        if i != 0 {
            return 1;
        }
        if statBuf.st_mode as i32 & 0o170000 == 0o100000 {
            return 0;
        }
    }
    return 1;
}

extern "C" fn countHardLinks(mut name: *mut i8) -> i32 {
    let mut i: i32 = 0;
    unsafe {
        let mut statBuf: stat = stat {
            st_dev: 0,
            st_mode: 0,
            st_nlink: 0,
            st_ino: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_atimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_birthtimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_size: 0,
            st_blocks: 0,
            st_blksize: 0,
            st_flags: 0,
            st_gen: 0,
            st_lspare: 0,
            st_qspare: [0; 2],
        };
        i = lstat(name, &mut statBuf);
        if i != 0 {
            return 0;
        }
        return statBuf.st_nlink as i32 - 1;
    }
}

static mut fileMetaInfo: stat = stat {
    st_dev: 0,
    st_mode: 0,
    st_nlink: 0,
    st_ino: 0,
    st_uid: 0,
    st_gid: 0,
    st_rdev: 0,
    st_atimespec: timespec {
        tv_sec: 0,
        tv_nsec: 0,
    },
    st_mtimespec: timespec {
        tv_sec: 0,
        tv_nsec: 0,
    },
    st_ctimespec: timespec {
        tv_sec: 0,
        tv_nsec: 0,
    },
    st_birthtimespec: timespec {
        tv_sec: 0,
        tv_nsec: 0,
    },
    st_size: 0,
    st_blocks: 0,
    st_blksize: 0,
    st_flags: 0,
    st_gen: 0,
    st_lspare: 0,
    st_qspare: [0; 2],
};
extern "C" fn saveInputFileMetaInfo(mut srcName: *mut i8) {
    let mut retVal: i32 = 0;
    unsafe {
        retVal = stat(srcName, &mut fileMetaInfo);
    }
    if retVal != 0 {
        ioError();
    }
}

extern "C" fn applySavedTimeInfoToOutputFile(mut dstName: *mut i8) {
    let mut retVal: i32 = 0;
    let mut uTimBuf: utimbuf = utimbuf {
        actime: 0,
        modtime: 0,
    };
    unsafe {
        uTimBuf.actime = fileMetaInfo.st_atimespec.tv_sec;
        uTimBuf.modtime = fileMetaInfo.st_mtimespec.tv_sec;
        retVal = utime(dstName, &mut uTimBuf);
    }
    if retVal != 0 {
        ioError();
    }
}

extern "C" fn applySavedFileAttrToOutputFile(mut fd: i32) {
    let mut retVal: i32 = 0;
    unsafe {
        retVal = fchmod(fd, fileMetaInfo.st_mode);
    }
    if retVal != 0 {
        ioError();
    }
    unsafe {
        fchown(fd, fileMetaInfo.st_uid, fileMetaInfo.st_gid);
    }
}

extern "C" fn containsDubiousChars(mut name: *mut i8) -> u8 {
    return 0;
}

#[no_mangle]
pub static mut zSuffix: [*const i8; 4] = [
    b".bz2\0" as *const u8 as *const i8,
    b".bz\0" as *const u8 as *const i8,
    b".tbz2\0" as *const u8 as *const i8,
    b".tbz\0" as *const u8 as *const i8,
];
#[no_mangle]
pub static mut unzSuffix: [*const i8; 4] = [
    b"\0" as *const u8 as *const i8,
    b"\0" as *const u8 as *const i8,
    b".tar\0" as *const u8 as *const i8,
    b".tar\0" as *const u8 as *const i8,
];
extern "C" fn hasSuffix(mut s: *mut i8, mut suffix: *const i8) -> u8 {
    unsafe {
        let mut ns: i32 = strlen(s) as i32;
        let mut nx: i32 = strlen(suffix) as i32;
        if ns < nx {
            return 0;
        }
        if strcmp(s.offset(ns as isize).offset(-(nx as isize)), suffix) == 0 {
            return 1;
        }
    }
    return 0;
}

extern "C" fn mapSuffix(
    mut name: *mut i8,
    mut oldSuffix: *const i8,
    mut newSuffix: *const i8,
) -> u8 {
    if hasSuffix(name, oldSuffix) == 0 {
        return 0;
    };
    unsafe {
        *name.offset((strlen(name)).wrapping_sub(strlen(oldSuffix)) as isize) = 0;
        strcat(name, newSuffix);
    }
    return 1;
}

extern "C" fn compress(mut name: *mut i8) {
    let mut inStr: *mut FILE = 0 as *mut FILE;
    let mut outStr: *mut FILE = 0 as *mut FILE;
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    unsafe {
        let mut statBuf: stat = stat {
            st_dev: 0,
            st_mode: 0,
            st_nlink: 0,
            st_ino: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_atimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_birthtimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_size: 0,
            st_blocks: 0,
            st_blksize: 0,
            st_flags: 0,
            st_gen: 0,
            st_lspare: 0,
            st_qspare: [0; 2],
        };
        deleteOutputOnInterrupt = 0;
        if name.is_null() && srcMode != 1 {
            panic(b"compress: bad modes\n\0" as *const u8 as *const i8);
        }
        match srcMode {
            1 => {
                copyFileName(
                    inName.as_mut_ptr(),
                    b"(stdin)\0" as *const u8 as *const i8 as *mut i8,
                );
                copyFileName(
                    outName.as_mut_ptr(),
                    b"(stdout)\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            3 => {
                copyFileName(inName.as_mut_ptr(), name);
                copyFileName(outName.as_mut_ptr(), name);
                strcat(outName.as_mut_ptr(), b".bz2\0" as *const u8 as *const i8);
            }
            2 => {
                copyFileName(inName.as_mut_ptr(), name);
                copyFileName(
                    outName.as_mut_ptr(),
                    b"(stdout)\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            _ => {}
        }
        if srcMode != 1 && containsDubiousChars(inName.as_mut_ptr()) as i32 != 0 {
            if noisy != 0 {
                fprintf(
                    __stderrp,
                    b"%s: There are no files matching `%s'.\n\0" as *const u8 as *const i8,
                    progName,
                    inName.as_mut_ptr(),
                );
            }
            setExit(1);
            return;
        }
        if srcMode != 1 && fileExists(inName.as_mut_ptr()) == 0 {
            fprintf(
                __stderrp,
                b"%s: Can't open input file %s: %s.\n\0" as *const u8 as *const i8,
                progName,
                inName.as_mut_ptr(),
                strerror(*__error()),
            );
            setExit(1);
            return;
        }
        i = 0;
        while i < 4 {
            if hasSuffix(inName.as_mut_ptr(), zSuffix[i as usize]) != 0 {
                if noisy != 0 {
                    fprintf(
                        __stderrp,
                        b"%s: Input file %s already has %s suffix.\n\0" as *const u8 as *const i8,
                        progName,
                        inName.as_mut_ptr(),
                        zSuffix[i as usize],
                    );
                }
                setExit(1);
                return;
            }
            i += 1;
            i;
        }
        if srcMode == 3 || srcMode == 2 {
            stat(inName.as_mut_ptr(), &mut statBuf);
            if statBuf.st_mode as i32 & 0o170000 == 0o40000 {
                fprintf(
                    __stderrp,
                    b"%s: Input file %s is a directory.\n\0" as *const u8 as *const i8,
                    progName,
                    inName.as_mut_ptr(),
                );
                setExit(1);
                return;
            }
        }
        if srcMode == 3 && forceOverwrite == 0 && notAStandardFile(inName.as_mut_ptr()) as i32 != 0
        {
            if noisy != 0 {
                fprintf(
                    __stderrp,
                    b"%s: Input file %s is not a normal file.\n\0" as *const u8 as *const i8,
                    progName,
                    inName.as_mut_ptr(),
                );
            }
            setExit(1);
            return;
        }
        if srcMode == 3 && fileExists(outName.as_mut_ptr()) as i32 != 0 {
            if forceOverwrite != 0 {
                remove(outName.as_mut_ptr());
            } else {
                fprintf(
                    __stderrp,
                    b"%s: Output file %s already exists.\n\0" as *const u8 as *const i8,
                    progName,
                    outName.as_mut_ptr(),
                );
                setExit(1);
                return;
            }
        }
        if srcMode == 3 && forceOverwrite == 0 && {
            n = countHardLinks(inName.as_mut_ptr());
            n > 0
        } {
            fprintf(
                __stderrp,
                b"%s: Input file %s has %d other link%s.\n\0" as *const u8 as *const i8,
                progName,
                inName.as_mut_ptr(),
                n,
                if n > 1 {
                    b"s\0" as *const u8 as *const i8
                } else {
                    b"\0" as *const u8 as *const i8
                },
            );
            setExit(1);
            return;
        }
        if srcMode == 3 {
            saveInputFileMetaInfo(inName.as_mut_ptr());
        }
        match srcMode {
            1 => {
                inStr = __stdinp;
                outStr = __stdoutp;
                if isatty(fileno(__stdoutp)) != 0 {
                    fprintf(
                        __stderrp,
                        b"%s: I won't write compressed data to a terminal.\n\0" as *const u8
                            as *const i8,
                        progName,
                    );
                    fprintf(
                        __stderrp,
                        b"%s: For help, type: `%s --help'.\n\0" as *const u8 as *const i8,
                        progName,
                        progName,
                    );
                    setExit(1);
                    return;
                }
            }
            2 => {
                inStr = fopen(inName.as_mut_ptr(), b"rb\0" as *const u8 as *const i8);
                outStr = __stdoutp;
                if isatty(fileno(__stdoutp)) != 0 {
                    fprintf(
                        __stderrp,
                        b"%s: I won't write compressed data to a terminal.\n\0" as *const u8
                            as *const i8,
                        progName,
                    );
                    fprintf(
                        __stderrp,
                        b"%s: For help, type: `%s --help'.\n\0" as *const u8 as *const i8,
                        progName,
                        progName,
                    );
                    if !inStr.is_null() {
                        fclose(inStr);
                    }
                    setExit(1);
                    return;
                }
                if inStr.is_null() {
                    fprintf(
                        __stderrp,
                        b"%s: Can't open input file %s: %s.\n\0" as *const u8 as *const i8,
                        progName,
                        inName.as_mut_ptr(),
                        strerror(*__error()),
                    );
                    setExit(1);
                    return;
                }
            }
            3 => {
                inStr = fopen(inName.as_mut_ptr(), b"rb\0" as *const u8 as *const i8);
                outStr =
                    fopen_output_safely(outName.as_mut_ptr(), b"wb\0" as *const u8 as *const i8);
                if outStr.is_null() {
                    fprintf(
                        __stderrp,
                        b"%s: Can't create output file %s: %s.\n\0" as *const u8 as *const i8,
                        progName,
                        outName.as_mut_ptr(),
                        strerror(*__error()),
                    );
                    if !inStr.is_null() {
                        fclose(inStr);
                    }
                    setExit(1);
                    return;
                }
                if inStr.is_null() {
                    fprintf(
                        __stderrp,
                        b"%s: Can't open input file %s: %s.\n\0" as *const u8 as *const i8,
                        progName,
                        inName.as_mut_ptr(),
                        strerror(*__error()),
                    );
                    if !outStr.is_null() {
                        fclose(outStr);
                    }
                    setExit(1);
                    return;
                }
            }
            _ => {
                panic(b"compress: bad srcMode\0" as *const u8 as *const i8);
            }
        }
        if verbosity >= 1 {
            fprintf(
                __stderrp,
                b"  %s: \0" as *const u8 as *const i8,
                inName.as_mut_ptr(),
            );
            pad(inName.as_mut_ptr());
            fflush(__stderrp);
        }
        outputHandleJustInCase = outStr;
        deleteOutputOnInterrupt = 1;
    }
    compressStream(inStr, outStr);
    unsafe {
        outputHandleJustInCase = 0 as *mut FILE;
        if srcMode == 3 {
            applySavedTimeInfoToOutputFile(outName.as_mut_ptr());
            deleteOutputOnInterrupt = 0;
            if keepInputFiles == 0 {
                let mut retVal: i32 = remove(inName.as_mut_ptr());
                if retVal != 0 {
                    ioError();
                }
            }
        }
        deleteOutputOnInterrupt = 0;
    }
}

extern "C" fn uncompress(mut name: *mut i8) {
    let mut current_block: u64;
    let mut inStr: *mut FILE = 0 as *mut FILE;
    let mut outStr: *mut FILE = 0 as *mut FILE;
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut magicNumberOK: u8 = 0;
    let mut cantGuess: u8 = 0;
    unsafe {
        let mut statBuf: stat = stat {
            st_dev: 0,
            st_mode: 0,
            st_nlink: 0,
            st_ino: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_atimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_birthtimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_size: 0,
            st_blocks: 0,
            st_blksize: 0,
            st_flags: 0,
            st_gen: 0,
            st_lspare: 0,
            st_qspare: [0; 2],
        };
        deleteOutputOnInterrupt = 0;
        if name.is_null() && srcMode != 1 {
            panic(b"uncompress: bad modes\n\0" as *const u8 as *const i8);
        }
        cantGuess = 0;
        match srcMode {
            1 => {
                copyFileName(
                    inName.as_mut_ptr(),
                    b"(stdin)\0" as *const u8 as *const i8 as *mut i8,
                );
                copyFileName(
                    outName.as_mut_ptr(),
                    b"(stdout)\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            3 => {
                copyFileName(inName.as_mut_ptr(), name);
                copyFileName(outName.as_mut_ptr(), name);
                i = 0;
                loop {
                    if !(i < 4) {
                        current_block = 7651349459974463963;
                        break;
                    }
                    if mapSuffix(
                        outName.as_mut_ptr(),
                        zSuffix[i as usize],
                        unzSuffix[i as usize],
                    ) != 0
                    {
                        current_block = 16331460921565405547;
                        break;
                    }
                    i += 1;
                    i;
                }
                match current_block {
                    16331460921565405547 => {}
                    _ => {
                        cantGuess = 1;
                        strcat(outName.as_mut_ptr(), b".out\0" as *const u8 as *const i8);
                    }
                }
            }
            2 => {
                copyFileName(inName.as_mut_ptr(), name);
                copyFileName(
                    outName.as_mut_ptr(),
                    b"(stdout)\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            _ => {}
        }
        if srcMode != 1 && containsDubiousChars(inName.as_mut_ptr()) as i32 != 0 {
            if noisy != 0 {
                fprintf(
                    __stderrp,
                    b"%s: There are no files matching `%s'.\n\0" as *const u8 as *const i8,
                    progName,
                    inName.as_mut_ptr(),
                );
            }
            setExit(1);
            return;
        }
        if srcMode != 1 && fileExists(inName.as_mut_ptr()) == 0 {
            fprintf(
                __stderrp,
                b"%s: Can't open input file %s: %s.\n\0" as *const u8 as *const i8,
                progName,
                inName.as_mut_ptr(),
                strerror(*__error()),
            );
            setExit(1);
            return;
        }
        if srcMode == 3 || srcMode == 2 {
            stat(inName.as_mut_ptr(), &mut statBuf);
            if statBuf.st_mode as i32 & 0o170000 == 0o40000 {
                fprintf(
                    __stderrp,
                    b"%s: Input file %s is a directory.\n\0" as *const u8 as *const i8,
                    progName,
                    inName.as_mut_ptr(),
                );
                setExit(1);
                return;
            }
        }
        if srcMode == 3 && forceOverwrite == 0 && notAStandardFile(inName.as_mut_ptr()) as i32 != 0
        {
            if noisy != 0 {
                fprintf(
                    __stderrp,
                    b"%s: Input file %s is not a normal file.\n\0" as *const u8 as *const i8,
                    progName,
                    inName.as_mut_ptr(),
                );
            }
            setExit(1);
            return;
        }
        if cantGuess != 0 {
            if noisy != 0 {
                fprintf(
                    __stderrp,
                    b"%s: Can't guess original name for %s -- using %s\n\0" as *const u8
                        as *const i8,
                    progName,
                    inName.as_mut_ptr(),
                    outName.as_mut_ptr(),
                );
            }
        }
        if srcMode == 3 && fileExists(outName.as_mut_ptr()) as i32 != 0 {
            if forceOverwrite != 0 {
                remove(outName.as_mut_ptr());
            } else {
                fprintf(
                    __stderrp,
                    b"%s: Output file %s already exists.\n\0" as *const u8 as *const i8,
                    progName,
                    outName.as_mut_ptr(),
                );
                setExit(1);
                return;
            }
        }
        if srcMode == 3 && forceOverwrite == 0 && {
            n = countHardLinks(inName.as_mut_ptr());
            n > 0
        } {
            fprintf(
                __stderrp,
                b"%s: Input file %s has %d other link%s.\n\0" as *const u8 as *const i8,
                progName,
                inName.as_mut_ptr(),
                n,
                if n > 1 {
                    b"s\0" as *const u8 as *const i8
                } else {
                    b"\0" as *const u8 as *const i8
                },
            );
            setExit(1);
            return;
        }
        if srcMode == 3 {
            saveInputFileMetaInfo(inName.as_mut_ptr());
        }
        match srcMode {
            1 => {
                inStr = __stdinp;
                outStr = __stdoutp;
                if isatty(fileno(__stdinp)) != 0 {
                    fprintf(
                        __stderrp,
                        b"%s: I won't read compressed data from a terminal.\n\0" as *const u8
                            as *const i8,
                        progName,
                    );
                    fprintf(
                        __stderrp,
                        b"%s: For help, type: `%s --help'.\n\0" as *const u8 as *const i8,
                        progName,
                        progName,
                    );
                    setExit(1);
                    return;
                }
            }
            2 => {
                inStr = fopen(inName.as_mut_ptr(), b"rb\0" as *const u8 as *const i8);
                outStr = __stdoutp;
                if inStr.is_null() {
                    fprintf(
                        __stderrp,
                        b"%s: Can't open input file %s:%s.\n\0" as *const u8 as *const i8,
                        progName,
                        inName.as_mut_ptr(),
                        strerror(*__error()),
                    );
                    if !inStr.is_null() {
                        fclose(inStr);
                    }
                    setExit(1);
                    return;
                }
            }
            3 => {
                inStr = fopen(inName.as_mut_ptr(), b"rb\0" as *const u8 as *const i8);
                outStr =
                    fopen_output_safely(outName.as_mut_ptr(), b"wb\0" as *const u8 as *const i8);
                if outStr.is_null() {
                    fprintf(
                        __stderrp,
                        b"%s: Can't create output file %s: %s.\n\0" as *const u8 as *const i8,
                        progName,
                        outName.as_mut_ptr(),
                        strerror(*__error()),
                    );
                    if !inStr.is_null() {
                        fclose(inStr);
                    }
                    setExit(1);
                    return;
                }
                if inStr.is_null() {
                    fprintf(
                        __stderrp,
                        b"%s: Can't open input file %s: %s.\n\0" as *const u8 as *const i8,
                        progName,
                        inName.as_mut_ptr(),
                        strerror(*__error()),
                    );
                    if !outStr.is_null() {
                        fclose(outStr);
                    }
                    setExit(1);
                    return;
                }
            }
            _ => {
                panic(b"uncompress: bad srcMode\0" as *const u8 as *const i8);
            }
        }
        if verbosity >= 1 {
            fprintf(
                __stderrp,
                b"  %s: \0" as *const u8 as *const i8,
                inName.as_mut_ptr(),
            );
            pad(inName.as_mut_ptr());
            fflush(__stderrp);
        }
        outputHandleJustInCase = outStr;
        deleteOutputOnInterrupt = 1;
    }
    magicNumberOK = uncompressStream(inStr, outStr);
    unsafe {
        outputHandleJustInCase = 0 as *mut FILE;
        if magicNumberOK != 0 {
            if srcMode == 3 {
                applySavedTimeInfoToOutputFile(outName.as_mut_ptr());
                deleteOutputOnInterrupt = 0;
                if keepInputFiles == 0 {
                    let mut retVal: i32 = remove(inName.as_mut_ptr());
                    if retVal != 0 {
                        ioError();
                    }
                }
            }
        } else {
            unzFailsExist = 1;
            deleteOutputOnInterrupt = 0;
            if srcMode == 3 {
                let mut retVal_0: i32 = remove(outName.as_mut_ptr());
                if retVal_0 != 0 {
                    ioError();
                }
            }
        }
        deleteOutputOnInterrupt = 0;
        if magicNumberOK != 0 {
            if verbosity >= 1 {
                fprintf(__stderrp, b"done\n\0" as *const u8 as *const i8);
            }
        } else {
            setExit(2);
            if verbosity >= 1 {
                fprintf(
                    __stderrp,
                    b"not a bzip2 file.\n\0" as *const u8 as *const i8,
                );
            } else {
                fprintf(
                    __stderrp,
                    b"%s: %s is not a bzip2 file.\n\0" as *const u8 as *const i8,
                    progName,
                    inName.as_mut_ptr(),
                );
            }
        };
    }
}

extern "C" fn testf(mut name: *mut i8) {
    let mut inStr: *mut FILE = 0 as *mut FILE;
    let mut allOK: u8 = 0;
    unsafe {
        let mut statBuf: stat = stat {
            st_dev: 0,
            st_mode: 0,
            st_nlink: 0,
            st_ino: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_atimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_birthtimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_size: 0,
            st_blocks: 0,
            st_blksize: 0,
            st_flags: 0,
            st_gen: 0,
            st_lspare: 0,
            st_qspare: [0; 2],
        };
        deleteOutputOnInterrupt = 0;
        if name.is_null() && srcMode != 1 {
            panic(b"testf: bad modes\n\0" as *const u8 as *const i8);
        }
        copyFileName(
            outName.as_mut_ptr(),
            b"(none)\0" as *const u8 as *const i8 as *mut i8,
        );
        match srcMode {
            1 => {
                copyFileName(
                    inName.as_mut_ptr(),
                    b"(stdin)\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            3 => {
                copyFileName(inName.as_mut_ptr(), name);
            }
            2 => {
                copyFileName(inName.as_mut_ptr(), name);
            }
            _ => {}
        }
        if srcMode != 1 && containsDubiousChars(inName.as_mut_ptr()) as i32 != 0 {
            if noisy != 0 {
                fprintf(
                    __stderrp,
                    b"%s: There are no files matching `%s'.\n\0" as *const u8 as *const i8,
                    progName,
                    inName.as_mut_ptr(),
                );
            }
            setExit(1);
            return;
        }
        if srcMode != 1 && fileExists(inName.as_mut_ptr()) == 0 {
            fprintf(
                __stderrp,
                b"%s: Can't open input %s: %s.\n\0" as *const u8 as *const i8,
                progName,
                inName.as_mut_ptr(),
                strerror(*__error()),
            );
            setExit(1);
            return;
        }
        if srcMode != 1 {
            stat(inName.as_mut_ptr(), &mut statBuf);
            if statBuf.st_mode as i32 & 0o170000 == 0o40000 {
                fprintf(
                    __stderrp,
                    b"%s: Input file %s is a directory.\n\0" as *const u8 as *const i8,
                    progName,
                    inName.as_mut_ptr(),
                );
                setExit(1);
                return;
            }
        }
        match srcMode {
            1 => {
                if isatty(fileno(__stdinp)) != 0 {
                    fprintf(
                        __stderrp,
                        b"%s: I won't read compressed data from a terminal.\n\0" as *const u8
                            as *const i8,
                        progName,
                    );
                    fprintf(
                        __stderrp,
                        b"%s: For help, type: `%s --help'.\n\0" as *const u8 as *const i8,
                        progName,
                        progName,
                    );
                    setExit(1);
                    return;
                }
                inStr = __stdinp;
            }
            2 | 3 => {
                inStr = fopen(inName.as_mut_ptr(), b"rb\0" as *const u8 as *const i8);
                if inStr.is_null() {
                    fprintf(
                        __stderrp,
                        b"%s: Can't open input file %s:%s.\n\0" as *const u8 as *const i8,
                        progName,
                        inName.as_mut_ptr(),
                        strerror(*__error()),
                    );
                    setExit(1);
                    return;
                }
            }
            _ => {
                panic(b"testf: bad srcMode\0" as *const u8 as *const i8);
            }
        }
        if verbosity >= 1 {
            fprintf(
                __stderrp,
                b"  %s: \0" as *const u8 as *const i8,
                inName.as_mut_ptr(),
            );
            pad(inName.as_mut_ptr());
            fflush(__stderrp);
        }
        outputHandleJustInCase = 0 as *mut FILE;
    }
    allOK = testStream(inStr);
    unsafe {
        if allOK as i32 != 0 && verbosity >= 1 {
            fprintf(__stderrp, b"ok\n\0" as *const u8 as *const i8);
        }
        if allOK == 0 {
            testFailsExist = 1;
        }
    }
}

extern "C" fn license() {
    unsafe {
        fprintf (__stderrp,
b"bzip2, a block-sorting file compressor.  Version %s.\n   \n   Copyright (C) 1996-2019 by Julian Seward.\n   \n   This program is free software; you can redistribute it and/or modify\n   it under the terms set out in the LICENSE file, which is included\n   in the bzip2 source distribution.\n   \n   This program is distributed in the hope that it will be useful,\n   but WITHOUT ANY WARRANTY; without even the implied warranty of\n   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n   LICENSE file for more details.\n   \n\0"
         as * const u8 as * const i8, BZ2_bzlibVersion (),);
    }
}

extern "C" fn usage(mut fullProgName: *mut i8) {
    unsafe {
        fprintf (__stderrp,
b"bzip2, a block-sorting file compressor.  Version %s.\n\n   usage: %s [flags and input files in any order]\n\n   -h --help           print this message\n   -d --decompress     force decompression\n   -z --compress       force compression\n   -k --keep           keep (don't delete) input files\n   -f --force          overwrite existing output files\n   -t --test           test compressed file integrity\n   -c --stdout         output to standard out\n   -q --quiet          suppress noncritical error messages\n   -v --verbose        be verbose (a 2nd -v gives more)\n   -L --license        display software version & license\n   -V --version        display software version & license\n   -s --small          use less memory (at most 2500k)\n   -1 .. -9            set block size to 100k .. 900k\n   --fast              alias for -1\n   --best              alias for -9\n\n   If invoked as `bzip2', default action is to compress.\n              as `bunzip2',  default action is to decompress.\n              as `bzcat', default action is to decompress to stdout.\n\n   If no file names are given, bzip2 compresses or decompresses\n   from standard input to standard output.  You can combine\n   short flags, so `-v -4' means the same as -v4 or -4v, &c.\n\n\0"
         as * const u8 as * const i8, BZ2_bzlibVersion (), fullProgName,);
    }
}

extern "C" fn redundant(mut flag: *mut i8) {
    unsafe {
        fprintf(
            __stderrp,
            b"%s: %s is redundant in versions 0.9.5 and above\n\0" as *const u8 as *const i8,
            progName,
            flag,
        );
    }
}

extern "C" fn myMalloc(mut n: i32) -> *mut std::os::raw::c_void {
    let mut p: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    unsafe {
        p = malloc(n as u64);
    }
    if p.is_null() {
        outOfMemory();
    }
    return p;
}

extern "C" fn mkCell() -> *mut Cell {
    let mut c: *mut Cell = 0 as *mut Cell;
    c = myMalloc(::std::mem::size_of::<Cell>() as i32) as *mut Cell;
    (*c).name = 0 as *mut i8;
    (*c).link = 0 as *mut zzzz;
    return c;
}

extern "C" fn snocString(mut root: *mut Cell, mut name: *mut i8) -> *mut Cell {
    unsafe {
        if root.is_null() {
            let mut tmp: *mut Cell = mkCell();
            (*tmp).name = myMalloc(5u64.wrapping_add(strlen(name)) as i32) as *mut i8;
            strcpy((*tmp).name, name);
            return tmp;
        } else {
            let mut tmp_0: *mut Cell = root;
            while !((*tmp_0).link).is_null() {
                tmp_0 = (*tmp_0).link;
            }
            (*tmp_0).link = snocString((*tmp_0).link, name);
            return root;
        };
    }
}

extern "C" fn addFlagsFromEnvVar(mut argList: *mut *mut Cell, mut varName: *mut i8) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut envbase: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    unsafe {
        envbase = getenv(varName);
        if !envbase.is_null() {
            p = envbase;
            i = 0;
            while 1 != 0 {
                if *p.offset(i as isize) as i32 == 0 {
                    break;
                }
                p = p.offset(i as isize);
                i = 0;
                while isspace(*p.offset(0 as isize) as i32) != 0 {
                    p = p.offset(1);
                    p;
                }
                while *p.offset(i as isize) as i32 != 0
                    && isspace(*p.offset(i as isize) as i32) == 0
                {
                    i += 1;
                    i;
                }
                if i > 0 {
                    k = i;
                    if k > 1034 - 10 {
                        k = 1034 - 10;
                    }
                    j = 0;
                    while j < k {
                        tmpName[j as usize] = *p.offset(j as isize);
                        j += 1;
                        j;
                    }
                    tmpName[k as usize] = 0;
                    *argList = snocString(*argList, tmpName.as_mut_ptr());
                }
            }
        }
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut tmp: *mut i8 = 0 as *mut i8;
    let mut argList: *mut Cell = 0 as *mut Cell;
    let mut aa: *mut Cell = 0 as *mut Cell;
    let mut decode: u8 = 0;
    if ::std::mem::size_of::<i32>() as u64 != 4
        || ::std::mem::size_of::<u32>() as u64 != 4
        || ::std::mem::size_of::<i16>() as u64 != 2
        || ::std::mem::size_of::<u16>() as u64 != 2
        || ::std::mem::size_of::<i8>() as u64 != 1
        || ::std::mem::size_of::<u8>() as u64 != 1
    {
        configError();
    }
    unsafe {
        outputHandleJustInCase = 0 as *mut FILE;
        smallMode = 0;
        keepInputFiles = 0;
        forceOverwrite = 0;
        noisy = 1;
        verbosity = 0;
        blockSize100k = 9;
        testFailsExist = 0;
        unzFailsExist = 0;
        numFileNames = 0;
        numFilesProcessed = 0;
        workFactor = 30;
        deleteOutputOnInterrupt = 0;
        exitValue = 0;
    }
    j = 0;
    i = j;
    unsafe {
        signal(
            11,
            Some(mySIGSEGVorSIGBUScatcher as unsafe extern "C" fn(i32) -> ()),
        );
        signal(
            10,
            Some(mySIGSEGVorSIGBUScatcher as unsafe extern "C" fn(i32) -> ()),
        );
        copyFileName(
            inName.as_mut_ptr(),
            b"(none)\0" as *const u8 as *const i8 as *mut i8,
        );
        copyFileName(
            outName.as_mut_ptr(),
            b"(none)\0" as *const u8 as *const i8 as *mut i8,
        );
        copyFileName(progNameReally.as_mut_ptr(), *argv.offset(0 as isize));
        progName = &mut *progNameReally.as_mut_ptr().offset(0 as isize) as *mut i8;
        tmp = &mut *progNameReally.as_mut_ptr().offset(0 as isize) as *mut i8;
        while *tmp as i32 != '\0' as i32 {
            if *tmp as i32 == '/' as i32 {
                progName = tmp.offset(1 as isize);
            }
            tmp = tmp.offset(1);
            tmp;
        }
    }
    argList = 0 as *mut Cell;
    addFlagsFromEnvVar(
        &mut argList,
        b"BZIP2\0" as *const u8 as *const i8 as *mut i8,
    );
    addFlagsFromEnvVar(&mut argList, b"BZIP\0" as *const u8 as *const i8 as *mut i8);
    i = 1;
    unsafe {
        while i <= argc - 1 {
            argList = snocString(argList, *argv.offset(i as isize));
            i += 1;
            i;
        }
        longestFileName = 7;
        numFileNames = 0;
    }
    decode = 1;
    aa = argList;
    unsafe {
        while !aa.is_null() {
            if strcmp((*aa).name, b"--\0" as *const u8 as *const i8) == 0 {
                decode = 0;
            } else if !(*((*aa).name).offset(0 as isize) as i32 == '-' as i32 && decode as i32 != 0)
            {
                numFileNames += 1;
                numFileNames;
                if longestFileName < strlen((*aa).name) as i32 {
                    longestFileName = strlen((*aa).name) as i32;
                }
            }
            aa = (*aa).link;
        }
        if numFileNames == 0 {
            srcMode = 1;
        } else {
            srcMode = 3;
        }
        opMode = 1;
        if !(strstr(progName, b"unzip\0" as *const u8 as *const i8)).is_null()
            || !(strstr(progName, b"UNZIP\0" as *const u8 as *const i8)).is_null()
        {
            opMode = 2;
        }
        if !(strstr(progName, b"z2cat\0" as *const u8 as *const i8)).is_null()
            || !(strstr(progName, b"Z2CAT\0" as *const u8 as *const i8)).is_null()
            || !(strstr(progName, b"zcat\0" as *const u8 as *const i8)).is_null()
            || !(strstr(progName, b"ZCAT\0" as *const u8 as *const i8)).is_null()
        {
            opMode = 2;
            srcMode = if numFileNames == 0 { 1 } else { 2 };
        }
    }
    aa = argList;
    unsafe {
        while !aa.is_null() {
            if strcmp((*aa).name, b"--\0" as *const u8 as *const i8) == 0 {
                break;
            }
            if *((*aa).name).offset(0 as isize) as i32 == '-' as i32
                && *((*aa).name).offset(1 as isize) as i32 != '-' as i32
            {
                j = 1;
                while *((*aa).name).offset(j as isize) as i32 != '\0' as i32 {
                    match *((*aa).name).offset(j as isize) as i32 {
                        99 => {
                            srcMode = 2;
                        }
                        100 => {
                            opMode = 2;
                        }
                        122 => {
                            opMode = 1;
                        }
                        102 => {
                            forceOverwrite = 1;
                        }
                        116 => {
                            opMode = 3;
                        }
                        107 => {
                            keepInputFiles = 1;
                        }
                        115 => {
                            smallMode = 1;
                        }
                        113 => {
                            noisy = 0;
                        }
                        49 => {
                            blockSize100k = 1;
                        }
                        50 => {
                            blockSize100k = 2;
                        }
                        51 => {
                            blockSize100k = 3;
                        }
                        52 => {
                            blockSize100k = 4;
                        }
                        53 => {
                            blockSize100k = 5;
                        }
                        54 => {
                            blockSize100k = 6;
                        }
                        55 => {
                            blockSize100k = 7;
                        }
                        56 => {
                            blockSize100k = 8;
                        }
                        57 => {
                            blockSize100k = 9;
                        }
                        86 | 76 => {
                            license();
                        }
                        118 => {
                            verbosity += 1;
                            verbosity;
                        }
                        104 => {
                            usage(progName);
                            exit(0);
                        }
                        _ => {
                            fprintf(
                                __stderrp,
                                b"%s: Bad flag `%s'\n\0" as *const u8 as *const i8,
                                progName,
                                (*aa).name,
                            );
                            usage(progName);
                            exit(1);
                        }
                    }
                    j += 1;
                    j;
                }
            }
            aa = (*aa).link;
        }
    }
    aa = argList;
    unsafe {
        while !aa.is_null() {
            if strcmp((*aa).name, b"--\0" as *const u8 as *const i8) == 0 {
                break;
            }
            if strcmp((*aa).name, b"--stdout\0" as *const u8 as *const i8) == 0 {
                srcMode = 2;
            } else if strcmp((*aa).name, b"--decompress\0" as *const u8 as *const i8) == 0 {
                opMode = 2;
            } else if strcmp((*aa).name, b"--compress\0" as *const u8 as *const i8) == 0 {
                opMode = 1;
            } else if strcmp((*aa).name, b"--force\0" as *const u8 as *const i8) == 0 {
                forceOverwrite = 1;
            } else if strcmp((*aa).name, b"--test\0" as *const u8 as *const i8) == 0 {
                opMode = 3;
            } else if strcmp((*aa).name, b"--keep\0" as *const u8 as *const i8) == 0 {
                keepInputFiles = 1;
            } else if strcmp((*aa).name, b"--small\0" as *const u8 as *const i8) == 0 {
                smallMode = 1;
            } else if strcmp((*aa).name, b"--quiet\0" as *const u8 as *const i8) == 0 {
                noisy = 0;
            } else if strcmp((*aa).name, b"--version\0" as *const u8 as *const i8) == 0 {
                license();
            } else if strcmp((*aa).name, b"--license\0" as *const u8 as *const i8) == 0 {
                license();
            } else if strcmp((*aa).name, b"--exponential\0" as *const u8 as *const i8) == 0 {
                workFactor = 1;
            } else if strcmp((*aa).name, b"--repetitive-best\0" as *const u8 as *const i8) == 0 {
                redundant((*aa).name);
            } else if strcmp((*aa).name, b"--repetitive-fast\0" as *const u8 as *const i8) == 0 {
                redundant((*aa).name);
            } else if strcmp((*aa).name, b"--fast\0" as *const u8 as *const i8) == 0 {
                blockSize100k = 1;
            } else if strcmp((*aa).name, b"--best\0" as *const u8 as *const i8) == 0 {
                blockSize100k = 9;
            } else if strcmp((*aa).name, b"--verbose\0" as *const u8 as *const i8) == 0 {
                verbosity += 1;
                verbosity;
            } else if strcmp((*aa).name, b"--help\0" as *const u8 as *const i8) == 0 {
                usage(progName);
                exit(0);
            } else if strncmp((*aa).name, b"--\0" as *const u8 as *const i8, 2) == 0 {
                fprintf(
                    __stderrp,
                    b"%s: Bad flag `%s'\n\0" as *const u8 as *const i8,
                    progName,
                    (*aa).name,
                );
                usage(progName);
                exit(1);
            }
            aa = (*aa).link;
        }
        if verbosity > 4 {
            verbosity = 4;
        }
        if opMode == 1 && smallMode as i32 != 0 && blockSize100k > 2 {
            blockSize100k = 2;
        }
        if opMode == 3 && srcMode == 2 {
            fprintf(
                __stderrp,
                b"%s: -c and -t cannot be used together.\n\0" as *const u8 as *const i8,
                progName,
            );
            exit(1);
        }
        if srcMode == 2 && numFileNames == 0 {
            srcMode = 1;
        }
        if opMode != 1 {
            blockSize100k = 0;
        }
        if srcMode == 3 {
            signal(2, Some(mySignalCatcher as unsafe extern "C" fn(i32) -> ()));
            signal(15, Some(mySignalCatcher as unsafe extern "C" fn(i32) -> ()));
            signal(1, Some(mySignalCatcher as unsafe extern "C" fn(i32) -> ()));
        }
        if opMode == 1 {
            if srcMode == 1 {
                compress(0 as *mut i8);
            } else {
                decode = 1;
                aa = argList;
                while !aa.is_null() {
                    if strcmp((*aa).name, b"--\0" as *const u8 as *const i8) == 0 {
                        decode = 0;
                    } else if !(*((*aa).name).offset(0 as isize) as i32 == '-' as i32
                        && decode as i32 != 0)
                    {
                        numFilesProcessed += 1;
                        numFilesProcessed;
                        compress((*aa).name);
                    }
                    aa = (*aa).link;
                }
            }
        } else if opMode == 2 {
            unzFailsExist = 0;
            if srcMode == 1 {
                uncompress(0 as *mut i8);
            } else {
                decode = 1;
                aa = argList;
                while !aa.is_null() {
                    if strcmp((*aa).name, b"--\0" as *const u8 as *const i8) == 0 {
                        decode = 0;
                    } else if !(*((*aa).name).offset(0 as isize) as i32 == '-' as i32
                        && decode as i32 != 0)
                    {
                        numFilesProcessed += 1;
                        numFilesProcessed;
                        uncompress((*aa).name);
                    }
                    aa = (*aa).link;
                }
            }
            if unzFailsExist != 0 {
                setExit(2);
                exit(exitValue);
            }
        } else {
            testFailsExist = 0;
            if srcMode == 1 {
                testf(0 as *mut i8);
            } else {
                decode = 1;
                aa = argList;
                while !aa.is_null() {
                    if strcmp((*aa).name, b"--\0" as *const u8 as *const i8) == 0 {
                        decode = 0;
                    } else if !(*((*aa).name).offset(0 as isize) as i32 == '-' as i32
                        && decode as i32 != 0)
                    {
                        numFilesProcessed += 1;
                        numFilesProcessed;
                        testf((*aa).name);
                    }
                    aa = (*aa).link;
                }
            }
            if testFailsExist != 0 {
                if noisy != 0 {
                    fprintf (__stderrp, b"\nYou can use the `bzip2recover' program to attempt to recover\ndata from undamaged sections of corrupted files.\n\n\0" as * const u8 as * const i8,);
                }
                setExit(2);
                exit(exitValue);
            }
        }
    }
    aa = argList;
    unsafe {
        while !aa.is_null() {
            let mut aa2: *mut Cell = (*aa).link;
            if !((*aa).name).is_null() {
                free((*aa).name as *mut std::os::raw::c_void);
            }
            free(aa as *mut std::os::raw::c_void);
            aa = aa2;
        }
        return exitValue;
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        );
    }
}
