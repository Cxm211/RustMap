use libc::stat;

pub type Char = u8; // Rust does not have `char` of size 1 byte, using `u8` as equivalent
pub type Bool = u8;
pub type UChar = u8;
pub type Int32 = i32;
pub type UInt32 = u32;
pub type Int16 = i16;
pub type UInt16 = u16;
pub type IntNative = i32;

#[repr(C)]
pub struct UInt64 {
    pub b: [UChar; 8],
}

static mut fileMetaInfo: stat = unsafe { std::mem::zeroed() };
pub static zSuffix: [&'static str; 4] = [".bz2", ".bz", ".tbz2", ".tbz"];
pub static unzSuffix: [&'static str; 4] = ["", "", ".tar", ".tar"];

pub const True: Bool = 1;
pub const False: Bool = 0;

pub static mut verbosity: Int32 = 0;
pub static mut keepInputFiles: Bool = False;
pub static mut smallMode: Bool = False;
pub static mut deleteOutputOnInterrupt: Bool = False;
pub static mut forceOverwrite: Bool = False;
pub static mut testFailsExist: Bool = False;
pub static mut unzFailsExist: Bool = False;
pub static mut noisy: Bool = False;
pub static mut numFileNames: Int32 = 0;
pub static mut numFilesProcessed: Int32 = 0;
pub static mut blockSize100k: Int32 = 0;
pub static mut exitValue: Int32 = 0;

pub const SM_I2O: Int32 = 1;
pub const SM_F2O: Int32 = 2;
pub const SM_F2F: Int32 = 3;

pub const OM_Z: Int32 = 1;
pub const OM_UNZ: Int32 = 2;
pub const OM_TEST: Int32 = 3;

pub static mut opMode: Int32 = 0;
pub static mut srcMode: Int32 = 0;

pub static mut longestFileName: Int32 = 0;
pub static mut inName: [Char; 1034] = [0; 1034];
pub static mut outName: [Char; 1034] = [0; 1034];
pub static mut tmpName: [Char; 1034] = [0; 1034];
pub static mut progName: *mut Char = std::ptr::null_mut();
pub static mut progNameReally: [Char; 1034] = [0; 1034];
pub static mut outputHandleJustInCase: *mut libc::FILE = std::ptr::null_mut();
pub static mut workFactor: Int32 = 0;
