//use libc;
extern "C" {
    pub type __sFILEX;
    static mut __stderrp: *mut FILE;
    fn fclose(_: *mut FILE) -> i32;
    fn fflush(_: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn getc(_: *mut FILE) -> i32;
    fn perror(_: *const i8);
    fn putc(_: i32, _: *mut FILE) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn __error() -> *mut i32;
    fn malloc(_: u64) -> *mut std::os::raw::c_void;
    fn free(_: *mut std::os::raw::c_void);
    fn exit(_: i32) -> !;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
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
pub struct BitStream {
    pub handle: *mut FILE,
    pub buffer: i32,
    pub buffLive: i32,
    pub mode: i8,
}
#[no_mangle]
pub static mut inFileName: [i8; 2000] = [0; 2000];
#[no_mangle]
pub static mut outFileName: [i8; 2000] = [0; 2000];
#[no_mangle]
pub static mut progName: [i8; 2000] = [0; 2000];
#[no_mangle]
pub static mut bytesOut: u64 = 0;
#[no_mangle]
pub static mut bytesIn: u64 = 0;
extern "C" fn readError() {
    unsafe {
        fprintf(
            __stderrp,
            b"%s: I/O error reading `%s', possible reason follows.\n\0" as *const u8 as *const i8,
            progName.as_mut_ptr(),
            inFileName.as_mut_ptr(),
        );
        perror(progName.as_mut_ptr());
        fprintf(
            __stderrp,
            b"%s: warning: output file(s) may be incomplete.\n\0" as *const u8 as *const i8,
            progName.as_mut_ptr(),
        );
        exit(1);
    }
}

extern "C" fn writeError() {
    unsafe {
        fprintf(
            __stderrp,
            b"%s: I/O error reading `%s', possible reason follows.\n\0" as *const u8 as *const i8,
            progName.as_mut_ptr(),
            inFileName.as_mut_ptr(),
        );
        perror(progName.as_mut_ptr());
        fprintf(
            __stderrp,
            b"%s: warning: output file(s) may be incomplete.\n\0" as *const u8 as *const i8,
            progName.as_mut_ptr(),
        );
        exit(1);
    }
}

extern "C" fn mallocFail(mut n: i32) {
    unsafe {
        fprintf(
            __stderrp,
            b"%s: malloc failed on request for %d bytes.\n\0" as *const u8 as *const i8,
            progName.as_mut_ptr(),
            n,
        );
        fprintf(
            __stderrp,
            b"%s: warning: output file(s) may be incomplete.\n\0" as *const u8 as *const i8,
            progName.as_mut_ptr(),
        );
        exit(1);
    }
}

extern "C" fn tooManyBlocks(mut max_handled_blocks: i32) {
    unsafe {
        fprintf(
            __stderrp,
            b"%s: `%s' appears to contain more than %d blocks\n\0" as *const u8 as *const i8,
            progName.as_mut_ptr(),
            inFileName.as_mut_ptr(),
            max_handled_blocks,
        );
        fprintf(
            __stderrp,
            b"%s: and cannot be handled.  To fix, increase\n\0" as *const u8 as *const i8,
            progName.as_mut_ptr(),
        );
        fprintf(
            __stderrp,
            b"%s: BZ_MAX_HANDLED_BLOCKS in bzip2recover.c, and recompile.\n\0" as *const u8
                as *const i8,
            progName.as_mut_ptr(),
        );
        exit(1);
    }
}

extern "C" fn bsOpenReadStream(mut stream: *mut FILE) -> *mut BitStream {
    unsafe {
        let mut bs: *mut BitStream =
            malloc(::std::mem::size_of::<BitStream>() as u64) as *mut BitStream;
        if bs.is_null() {
            mallocFail(::std::mem::size_of::<BitStream>() as i32);
        };
        (*bs).handle = stream;
        (*bs).buffer = 0;
        (*bs).buffLive = 0;
        (*bs).mode = 'r' as i8;
        return bs;
    }
}

extern "C" fn bsOpenWriteStream(mut stream: *mut FILE) -> *mut BitStream {
    unsafe {
        let mut bs: *mut BitStream =
            malloc(::std::mem::size_of::<BitStream>() as u64) as *mut BitStream;
        if bs.is_null() {
            mallocFail(::std::mem::size_of::<BitStream>() as i32);
        };
        (*bs).handle = stream;
        (*bs).buffer = 0;
        (*bs).buffLive = 0;
        (*bs).mode = 'w' as i8;
        return bs;
    }
}

extern "C" fn bsPutBit(mut bs: *mut BitStream, mut bit: i32) {
    unsafe {
        if (*bs).buffLive == 8 {
            let mut retVal: i32 = putc((*bs).buffer as i32, (*bs).handle);
            if retVal == -1 {
                writeError();
            }
            bytesOut = bytesOut.wrapping_add(1);
            bytesOut;
            (*bs).buffLive = 1;
            (*bs).buffer = bit & 0x1;
        } else {
            (*bs).buffer = (*bs).buffer << 1 | bit & 0x1;
            (*bs).buffLive += 1;
            (*bs).buffLive;
        };
    }
}

extern "C" fn bsGetBit(mut bs: *mut BitStream) -> i32 {
    unsafe {
        if (*bs).buffLive > 0 {
            (*bs).buffLive -= 1;
            (*bs).buffLive;
            return (*bs).buffer >> (*bs).buffLive & 0x1;
        } else {
            let mut retVal: i32 = getc((*bs).handle);
            if retVal == -1 {
                if *__error() != 0 {
                    readError();
                }
                return 2;
            };
            (*bs).buffLive = 7;
            (*bs).buffer = retVal;
            return (*bs).buffer >> 7 & 0x1;
        };
    }
}

extern "C" fn bsClose(mut bs: *mut BitStream) {
    let mut retVal: i32 = 0;
    unsafe {
        if (*bs).mode as i32 == 'w' as i32 {
            while (*bs).buffLive < 8 {
                (*bs).buffLive += 1;
                (*bs).buffLive;
                (*bs).buffer <<= 1;
            }
            retVal = putc((*bs).buffer as i32, (*bs).handle);
            if retVal == -1 {
                writeError();
            }
            bytesOut = bytesOut.wrapping_add(1);
            bytesOut;
            retVal = fflush((*bs).handle);
            if retVal == -1 {
                writeError();
            }
        }
        retVal = fclose((*bs).handle);
    }
    if retVal == -1 {
        if (*bs).mode as i32 == 'w' as i32 {
            writeError();
        } else {
            readError();
        }
    }
    unsafe {
        free(bs as *mut std::os::raw::c_void);
    }
}

extern "C" fn bsPutUChar(mut bs: *mut BitStream, mut c: u8) {
    let mut i: i32 = 0;
    i = 7;
    while i >= 0 {
        bsPutBit(bs, (c as u32 >> i & 0x1u32) as i32);
        i -= 1;
        i;
    }
}

extern "C" fn bsPutUInt32(mut bs: *mut BitStream, mut c: u32) {
    let mut i: i32 = 0;
    i = 31;
    while i >= 0 {
        bsPutBit(bs, (c >> i & 0x1u32) as i32);
        i -= 1;
        i;
    }
}

extern "C" fn endsInBz2(mut name: *mut i8) -> u8 {
    unsafe {
        let mut n: i32 = strlen(name) as i32;
        if n <= 4 {
            return 0;
        }
        return (*name.offset((n - 4i32) as isize) as i32 == '.' as i32
            && *name.offset((n - 3i32) as isize) as i32 == 'b' as i32
            && *name.offset((n - 2i32) as isize) as i32 == 'z' as i32
            && *name.offset((n - 1i32) as isize) as i32 == '2' as i32) as u8;
    }
}

#[no_mangle]
pub static mut bStart: [u64; 50000] = [0; 50000];
#[no_mangle]
pub static mut bEnd: [u64; 50000] = [0; 50000];
#[no_mangle]
pub static mut rbStart: [u64; 50000] = [0; 50000];
#[no_mangle]
pub static mut rbEnd: [u64; 50000] = [0; 50000];
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut inFile: *mut FILE = 0 as *mut FILE;
    let mut outFile: *mut FILE = 0 as *mut FILE;
    let mut bsIn: *mut BitStream = 0 as *mut BitStream;
    let mut bsWr: *mut BitStream = 0 as *mut BitStream;
    let mut b: i32 = 0;
    let mut wrBlock: i32 = 0;
    let mut currBlock: i32 = 0;
    let mut rbCtr: i32 = 0;
    let mut bitsRead: u64 = 0;
    let mut buffHi: u32 = 0;
    let mut buffLo: u32 = 0;
    let mut blockCRC: u32 = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    unsafe {
        strncpy(
            progName.as_mut_ptr(),
            *argv.offset(0 as isize),
            (2000 - 1i32) as u64,
        );
        progName[(2000 - 1i32) as usize] = '\0' as i8;
        outFileName[0 as usize] = 0;
        inFileName[0 as usize] = outFileName[0 as usize];
        fprintf(
            __stderrp,
            b"bzip2recover 1.0.8: extracts blocks from damaged .bz2 files.\n\0" as *const u8
                as *const i8,
        );
        if argc != 2 {
            fprintf(
                __stderrp,
                b"%s: usage is `%s damaged_file_name'.\n\0" as *const u8 as *const i8,
                progName.as_mut_ptr(),
                progName.as_mut_ptr(),
            );
            match ::std::mem::size_of::<u64>() as u64 {
                8 => {
                    fprintf(
                        __stderrp,
                        b"\trestrictions on size of recovered file: None\n\0" as *const u8
                            as *const i8,
                    );
                }
                4 => {
                    fprintf(
                        __stderrp,
                        b"\trestrictions on size of recovered file: 512 MB\n\0" as *const u8
                            as *const i8,
                    );
                    fprintf (__stderrp, b"\tto circumvent, recompile with MaybeUInt64 as an\n\tunsigned 64-bit int.\n\0" as * const u8 as * const i8,);
                }
                _ => {
                    fprintf(
                        __stderrp,
                        b"\tsizeof(MaybeUInt64) is not 4 or 8 -- configuration error.\n\0"
                            as *const u8 as *const i8,
                    );
                }
            }
            exit(1);
        }
        if strlen(*argv.offset(1 as isize)) >= (2000 - 20i32) as u64 {
            fprintf(
                __stderrp,
                b"%s: supplied filename is suspiciously (>= %d chars) long.  Bye!\n\0" as *const u8
                    as *const i8,
                progName.as_mut_ptr(),
                strlen(*argv.offset(1 as isize)) as i32,
            );
            exit(1);
        }
        strcpy(inFileName.as_mut_ptr(), *argv.offset(1 as isize));
        inFile = fopen(inFileName.as_mut_ptr(), b"rb\0" as *const u8 as *const i8);
        if inFile.is_null() {
            fprintf(
                __stderrp,
                b"%s: can't read `%s'\n\0" as *const u8 as *const i8,
                progName.as_mut_ptr(),
                inFileName.as_mut_ptr(),
            );
            exit(1);
        }
    }
    bsIn = bsOpenReadStream(inFile);
    unsafe {
        fprintf(
            __stderrp,
            b"%s: searching for block boundaries ...\n\0" as *const u8 as *const i8,
            progName.as_mut_ptr(),
        );
    }
    bitsRead = 0;
    buffLo = 0;
    buffHi = buffLo;
    currBlock = 0;
    unsafe {
        bStart[currBlock as usize] = 0;
    }
    rbCtr = 0;
    unsafe {
        while 1 != 0 {
            b = bsGetBit(bsIn);
            bitsRead = bitsRead.wrapping_add(1);
            bitsRead;
            if b == 2 {
                if bitsRead >= bStart[currBlock as usize]
                    && bitsRead.wrapping_sub(bStart[currBlock as usize]) >= 40
                {
                    bEnd[currBlock as usize] = bitsRead.wrapping_sub(1);
                    if currBlock > 0 {
                        fprintf(
                            __stderrp,
                            b"   block %d runs from %Lu to %Lu (incomplete)\n\0" as *const u8
                                as *const i8,
                            currBlock,
                            bStart[currBlock as usize],
                            bEnd[currBlock as usize],
                        );
                    }
                } else {
                    currBlock -= 1;
                    currBlock;
                }
                break;
            } else {
                buffHi = buffHi << 1 | buffLo >> 31;
                buffLo = buffLo << 1 | (b & 1i32) as u32;
                if (buffHi & 0xffffu32) as u64 == 0x3141 && buffLo as u64 == 0x59265359
                    || (buffHi & 0xffffu32) as u64 == 0x1772 && buffLo as u64 == 0x45385090
                {
                    if bitsRead > 49 {
                        bEnd[currBlock as usize] = bitsRead.wrapping_sub(49);
                    } else {
                        bEnd[currBlock as usize] = 0;
                    }
                    if currBlock > 0
                        && (bEnd[currBlock as usize]).wrapping_sub(bStart[currBlock as usize])
                            >= 130
                    {
                        fprintf(
                            __stderrp,
                            b"   block %d runs from %Lu to %Lu\n\0" as *const u8 as *const i8,
                            rbCtr + 1,
                            bStart[currBlock as usize],
                            bEnd[currBlock as usize],
                        );
                        rbStart[rbCtr as usize] = bStart[currBlock as usize];
                        rbEnd[rbCtr as usize] = bEnd[currBlock as usize];
                        rbCtr += 1;
                        rbCtr;
                    }
                    if currBlock >= 50000 {
                        tooManyBlocks(50000);
                    }
                    currBlock += 1;
                    currBlock;
                    bStart[currBlock as usize] = bitsRead;
                }
            }
        }
    }
    bsClose(bsIn);
    unsafe {
        if rbCtr < 1 {
            fprintf(
                __stderrp,
                b"%s: sorry, I couldn't find any block boundaries.\n\0" as *const u8 as *const i8,
                progName.as_mut_ptr(),
            );
            exit(1);
        }
        fprintf(
            __stderrp,
            b"%s: splitting into blocks\n\0" as *const u8 as *const i8,
            progName.as_mut_ptr(),
        );
        inFile = fopen(inFileName.as_mut_ptr(), b"rb\0" as *const u8 as *const i8);
        if inFile.is_null() {
            fprintf(
                __stderrp,
                b"%s: can't open `%s'\n\0" as *const u8 as *const i8,
                progName.as_mut_ptr(),
                inFileName.as_mut_ptr(),
            );
            exit(1);
        }
    }
    bsIn = bsOpenReadStream(inFile);
    blockCRC = 0;
    bsWr = 0 as *mut BitStream;
    bitsRead = 0;
    outFile = 0 as *mut FILE;
    wrBlock = 0;
    unsafe {
        while 1 != 0 {
            b = bsGetBit(bsIn);
            if b == 2 {
                break;
            }
            buffHi = buffHi << 1 | buffLo >> 31;
            buffLo = buffLo << 1 | (b & 1i32) as u32;
            if bitsRead == 47u64.wrapping_add(rbStart[wrBlock as usize]) {
                blockCRC = buffHi << 16 | buffLo >> 16;
            }
            if !outFile.is_null()
                && bitsRead >= rbStart[wrBlock as usize]
                && bitsRead <= rbEnd[wrBlock as usize]
            {
                bsPutBit(bsWr, b);
            }
            bitsRead = bitsRead.wrapping_add(1);
            bitsRead;
            if bitsRead == (rbEnd[wrBlock as usize]).wrapping_add(1) {
                if !outFile.is_null() {
                    bsPutUChar(bsWr, 0x17);
                    bsPutUChar(bsWr, 0x72);
                    bsPutUChar(bsWr, 0x45);
                    bsPutUChar(bsWr, 0x38);
                    bsPutUChar(bsWr, 0x50);
                    bsPutUChar(bsWr, 0x90);
                    bsPutUInt32(bsWr, blockCRC);
                    bsClose(bsWr);
                    outFile = 0 as *mut FILE;
                }
                if wrBlock >= rbCtr {
                    break;
                }
                wrBlock += 1;
                wrBlock;
            } else if bitsRead == rbStart[wrBlock as usize] {
                let mut split: *mut i8 = 0 as *mut i8;
                let mut ofs: i32 = 0;
                let mut k: i32 = 0;
                k = 0;
                while k < 2000 {
                    outFileName[k as usize] = 0;
                    k += 1;
                    k;
                }
                strcpy(outFileName.as_mut_ptr(), inFileName.as_mut_ptr());
                split = strrchr(outFileName.as_mut_ptr(), '/' as i32);
                if split.is_null() {
                    split = outFileName.as_mut_ptr();
                } else {
                    split = split.offset(1);
                    split;
                }
                ofs = split.offset_from(outFileName.as_mut_ptr()) as i32;
                sprintf(split, b"rec%5d\0" as *const u8 as *const i8, wrBlock + 1);
                p = split;
                while *p as i32 != 0 {
                    if *p as i32 == ' ' as i32 {
                        *p = '0' as i8;
                    }
                    p = p.offset(1);
                    p;
                }
                strcat(
                    outFileName.as_mut_ptr(),
                    inFileName.as_mut_ptr().offset(ofs as isize),
                );
                if endsInBz2(outFileName.as_mut_ptr()) == 0 {
                    strcat(
                        outFileName.as_mut_ptr(),
                        b".bz2\0" as *const u8 as *const i8,
                    );
                }
                fprintf(
                    __stderrp,
                    b"   writing block %d to `%s' ...\n\0" as *const u8 as *const i8,
                    wrBlock + 1,
                    outFileName.as_mut_ptr(),
                );
                outFile = fopen(outFileName.as_mut_ptr(), b"wb\0" as *const u8 as *const i8);
                if outFile.is_null() {
                    fprintf(
                        __stderrp,
                        b"%s: can't write `%s'\n\0" as *const u8 as *const i8,
                        progName.as_mut_ptr(),
                        outFileName.as_mut_ptr(),
                    );
                    exit(1);
                }
                bsWr = bsOpenWriteStream(outFile);
                bsPutUChar(bsWr, 0x42);
                bsPutUChar(bsWr, 0x5a);
                bsPutUChar(bsWr, 0x68);
                bsPutUChar(bsWr, (0x30 + 9i32) as u8);
                bsPutUChar(bsWr, 0x31);
                bsPutUChar(bsWr, 0x41);
                bsPutUChar(bsWr, 0x59);
                bsPutUChar(bsWr, 0x26);
                bsPutUChar(bsWr, 0x53);
                bsPutUChar(bsWr, 0x59);
            }
        }
        fprintf(
            __stderrp,
            b"%s: finished\n\0" as *const u8 as *const i8,
            progName.as_mut_ptr(),
        );
    }
    return 0;
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
