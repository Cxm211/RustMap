

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt16 = u16;
pub type UChar = u8;
pub type Bool = bool;
pub static mut BZ2_rNums: [i32; 512] = [0; 512];
pub static mut BZ2_crc32Table: [u32; 256] = [0; 256];
use std::fs::File;


pub struct EState {
 pub   strm: Option<*mut bz_stream>,

 pub   mode: Int32,
  pub  state: Int32,
  pub  avail_in_expect: UInt32,

 pub   arr1: Option<*mut UInt32>,
 pub   arr2: Option<*mut UInt32>,
 pub   ftab: Option<*mut UInt32>,
 pub   origPtr: Int32,

 pub   ptr: Option<*mut UInt32>,
 pub   block: Option<*mut UChar>,
 pub   mtfv: Option<*mut UInt16>,
 pub   zbits: Option<*mut UChar>, //TODO: Vec<u8>

 pub   workFactor: Int32,

pub    state_in_ch: UInt32,
 pub   state_in_len: Int32,
 pub   rNToGo: Int32, 
 pub   rTPos: Int32,

 pub   nblock: Int32,
  pub  nblockMAX: Int32,
 pub   numZ: Int32,
 pub   state_out_pos: Int32,

 pub   nInUse: Int32,
 pub   inUse: [Bool; 256],
  pub  unseqToSeq: [UChar; 256],

pub    bsBuff: UInt32,
 pub   bsLive: Int32,

  pub  blockCRC: UInt32,
 pub  combinedCRC: UInt32,

 pub   verbosity: Int32,
 pub   blockNo: Int32,
 pub   blockSize100k: Int32,

 pub   nMTF: Int32,
 pub   mtfFreq: [Int32; 258],
 pub   selector: [UChar; 2 + (900000 / 50)],
 pub   selectorMtf: [UChar; 2 + (900000 / 50)],

  pub  len: [[UChar; 258]; 6],
  pub  code: [[Int32; 258]; 6],
  pub  rfreq: [[Int32; 258]; 6],

   pub len_pack: [[UInt32; 4]; 258],
}

impl Default for EState {
fn default() -> Self {
        EState {
            strm: None,
    mode: 0,
    state: 0,
    avail_in_expect: 0,
    arr1: None,
    arr2: None,
    ftab: None,
    origPtr: 0,
    ptr: None,
    block: None,
    mtfv: None,
    zbits: None,
    workFactor: 0,
    state_in_ch: 0,
    state_in_len: 0,
    rNToGo: 0,
    rTPos: 0,
    nblock: 0,
    nblockMAX: 0,
    numZ: 0,
    state_out_pos: 0,
    bsBuff: 0,
    bsLive: 0,
    blockCRC: 0,
    combinedCRC: 0,
    verbosity: 0,
    blockNo: 0,
    blockSize100k: 0,
    nMTF: 0,
    selector: [0; 2 + (900000 / 50)],
    selectorMtf: [0; 2 + (900000 / 50)],
    len: [[0; 258]; 6],
    code: [[0; 258]; 6],
    rfreq: [[0; 258]; 6],
    len_pack: [[0; 4]; 258],
    nInUse: 0,
    inUse: [false; 256],
    unseqToSeq: [0; 256],
    mtfFreq: [0; 258], 
        }
    }
}




#[derive(Copy, Clone)]
pub struct UInt64 {
    pub b: [u8; 8],  // UChar在C中通常是u8在Rust中
}
impl UInt64 {
    // Convert UInt64 to f64
    pub fn to_double(&self) -> f64 {
        let mut base = 1.0;
        let mut sum = 0.0;
        for i in 0..8 {
            sum += base * (self.b[i] as f64);
            base *= 256.0;
        }
        sum
    }

    // Populate UInt64 from two UInt32s
    pub fn from_u32s(&mut self, lo32: UInt32, hi32: UInt32) {
        self.b[7] = ((hi32 >> 24) & 0xFF) as UChar;
        self.b[6] = ((hi32 >> 16) & 0xFF) as UChar;
        self.b[5] = ((hi32 >> 8) & 0xFF) as UChar;
        self.b[4] = (hi32 & 0xFF) as UChar;
        self.b[3] = ((lo32 >> 24) & 0xFF) as UChar;
        self.b[2] = ((lo32 >> 16) & 0xFF) as UChar;
        self.b[1] = ((lo32 >> 8) & 0xFF) as UChar;
        self.b[0] = (lo32 & 0xFF) as UChar;
    }
}





pub struct bzFile {
 pub   handle: *mut File,
  pub  buf: [u8; 5000],
  pub  bufN: i32,
 pub   writing: bool,
  pub  strm: bz_stream,
 pub   lastErr: i32,
 pub   initialisedOk: bool,
}
use std::ptr;

impl Default for bzFile {
    fn default() -> Self {
        bzFile {
            handle: ptr::null_mut(),
            buf: [0; 5000],
            bufN: 0,
            writing: false,
            strm: bz_stream {
                next_in: ptr::null_mut(),
                avail_in: 0,
                total_in_lo32: 0,
                total_in_hi32: 0,
                next_out: ptr::null_mut(),
                avail_out: 0,
                total_out_lo32: 0,
                total_out_hi32: 0,
                state: ptr::null_mut(),
                bzalloc: None,
                bzfree: None,
                opaque: ptr::null_mut(),
                // ... [Other default initializations for your specific logic]
            },
            lastErr: 0,
            initialisedOk: false,
        }
    }
}


pub struct bz_stream {
    pub   next_in: *mut i8, // char in C is i8 in Rust.
    pub   avail_in: u32,
    pub   total_in_lo32: u32,
     pub  total_in_hi32: u32,
   
    pub   next_out: *mut i8,
     pub  avail_out: u32,
     pub  total_out_lo32: u32,
     pub  total_out_hi32: u32,
   
    pub   state: *mut DState, // I'm assuming DState here based on your earlier code.
       
    pub   bzalloc: Option<fn(*mut std::ffi::c_void, Int32, Int32) -> *mut std::ffi::c_void>,
    pub   bzfree: Option<fn(*mut std::ffi::c_void, *mut std::ffi::c_void)>,
    pub   opaque: *mut std::ffi::c_void,
}

   

pub struct DState {
    pub  strm: *mut bz_stream,   // 引用关联的 bz_stream // Reference to associated bz_stream
    pub  state: Int32,          // The current state
    pub  state_out_ch: usize,   // Output character state
    pub  state_out_len: Int32,  // Length of output state
    pub  blockRandomised: Bool, // 区块是否随机化 // Whether the block is randomised
    pub  rNToGo: Int32,         /// RLE 重复次数 / RLE repeat count
    pub  rTPos: Int32,          // 最后输出的RLE值 // RLE value last output
  
    pub  bsBuff: UInt32,        // Bitstream // 比特流缓冲区
    pub  bsLive: Int32,         // bsBuff 中剩余的位数 // Number of bits left in bsBuff
  
    pub  blockSize100k: Int32,  // Block size, units of 100k
    pub  smallDecompress: Bool, // Use the slower, smaller decompression routine
    pub  currBlockNo: Int32,    // The current block number
    pub  verbosity: Int32,      // 详细级别 // Verbosity level
  
   pub   origPtr: Int32,        // Pointer to original data
   pub   tPos: UInt32,          // 缓冲区中的当前位置 // Current position in buffer
   pub   k0: Int32,             // MTF 解码序列中的位置 // Position in sequence for MTF decoding
    pub    unzftab: [Int32; 256], // 字节值频率表（用于 MTF  Frequency table of byte values (used for MTF)
   pub   nblock_used: Int32,    // Number of blocks used
   pub   cftab: [Int32; 257],   // 累计频率表 // Cumulative frequency table
   pub   cftabCopy: [Int32; 257], // A copy of the cftab array
  
   pub   tt: *mut UInt32,       // Pointer to another array of unknown purpose
   pub   ll16: *mut UInt16,     // Pointer to another array of unknown purpose
   pub   ll4: *mut UChar,       // Pointer to another array of unknown purpose
  
    pub  storedBlockCRC: UInt32,  // CRC value of stored block
    pub  storedCombinedCRC: UInt32, // Combined CRC value
    pub  calculatedBlockCRC: UInt32, // Calculated CRC value of block
    pub  calculatedCombinedCRC: UInt32, // Calculated combined CRC value
  
    pub  nInUse: Int32,           // Number of bytes in use
    pub  inUse: [Bool; 256],      // Flags indicating which values are in use
    pub  inUse16: [Bool; 16],     // 16 位值的标志 Flags for 16-bit values
    pub  seqToUnseq: [UChar; 256], // MTF values
  
   pub   mtfa: [UChar; 4096],      // Another array for MTF values
    pub  mtfbase: [Int32; 256 / 16], // Base values for MTF encoding
  
     pub selector: [UChar; 2 + (900000 / 50)], // Selector values
    pub  selectorMtf: [UChar; 2 + (900000 / 50)], // MTF'd selector values
  
    pub  len: [[UChar; 258]; 6],  // Lengths of Huffman code tables
  
    pub  limit: [[Int32; 258]; 6], //哈夫曼编码表限制值 Huffman coding table limit values
    pub  base: [[Int32; 258]; 6],  // Huffman coding table base values
    pub  perm: [[Int32; 258]; 6],  // Huffman coding table permutation values
    pub  minLens: [Int32; 6],      // 霍夫曼编码表的最小长度 Minimum lengths of Huffman coding tables
  
      // State-saving variables (used for saving and restoring state)
   pub   save_i: Int32,
   pub   save_j: Int32,
   pub   save_t: Int32,
   pub   save_alphaSize: Int32,
   pub   save_nGroups: Int32,
   pub   save_nSelectors: Int32,
   pub   save_EOB: Int32,
   pub   save_groupNo: Int32,
   pub   save_groupPos: Int32,
   pub   save_nextSym: Int32,
   pub   save_nblockMAX: Int32,
   pub   save_nblock: Int32,
   pub   save_es: Int32,
   pub   save_N: Int32,
   pub   save_curr: Int32,
   pub   save_zt: Int32,
   pub   save_zn: Int32,
   pub   save_zvec: Int32,
   pub   save_zj: Int32,
    pub  save_gSel: Int32,
  pub    save_gMinlen: Int32,
   pub   save_gLimit: *mut Int32,
  pub    save_gBase: *mut Int32,
   pub   save_gPerm: *mut Int32,
  }

  impl Default for DState {
    fn default() -> Self {
        DState {
            strm: std::ptr::null_mut(),
            state: 0,
            state_out_ch: 0,
            state_out_len: 0,
            blockRandomised: false,
            rNToGo: 0,
            rTPos: 0,
            bsBuff: 0,
            bsLive: 0,
            blockSize100k: 0,
            smallDecompress: false,
            currBlockNo: 0,
            verbosity: 0,
            origPtr: 0,
            tPos: 0,
            k0: 0,
            unzftab: [0; 256],
            nblock_used: 0,
            cftab: [0; 257],
            cftabCopy: [0; 257],
            tt: std::ptr::null_mut(),
            ll16: std::ptr::null_mut(),
            ll4: std::ptr::null_mut(),
            storedBlockCRC: 0,
            storedCombinedCRC: 0,
            calculatedBlockCRC: 0,
            calculatedCombinedCRC: 0,
            nInUse: 0,
            inUse: [false; 256],
            inUse16: [false; 16],
            seqToUnseq: [0; 256],
            mtfa: [0; 4096],
            mtfbase: [0; 16],
            selector: [0; 2 + (900000 / 50)],
            selectorMtf: [0; 2 + (900000 / 50)],
            len: [[0; 258]; 6],
            limit: [[0; 258]; 6],
            base: [[0; 258]; 6],
            perm: [[0; 258]; 6],
            minLens: [0; 6],
            save_i: 0,
            save_j: 0,
            save_t: 0,
            save_alphaSize: 0,
            save_nGroups: 0,
            save_nSelectors: 0,
            save_EOB: 0,
            save_groupNo: 0,
            save_groupPos: 0,
            save_nextSym: 0,
            save_nblockMAX: 0,
            save_nblock: 0,
            save_es: 0,
            save_N: 0,
            save_curr: 0,
            save_zt: 0,
            save_zn: 0,
            save_zvec: 0,
            save_zj: 0,
            save_gSel: 0,
            save_gMinlen: 0,
            save_gLimit: std::ptr::null_mut(),
            save_gBase: std::ptr::null_mut(),
            save_gPerm: std::ptr::null_mut(),
        }
    }
}




use std::io::Cursor;
pub type Char = u8;
type UChar = u8;
type Int32 = i32; // 对应C的int类型
type MaybeUInt64 = u64;
pub const MAX_SIZE: usize = 2000;
pub static mut PROG_NAME: [u8; MAX_SIZE] = [b'\0'; MAX_SIZE];
pub static mut IN_FILE_NAME: [u8; MAX_SIZE] = [b'\0'; MAX_SIZE];

pub static mut BYTES_OUT: u64 = 0;
pub struct BitStream {
    pub handle: Cursor<Vec<u8>>,
    pub buffer: Int32,
    pub buff_live: Int32,
    pub mode: Char,
}


impl Default for BitStream {
    fn default() -> Self {
        BitStream {
            handle: Cursor::new(Vec::new()),
            buffer: 0,
            buff_live: 0,
            mode: b'R', // 选择一个合适的默认值
        }
    }
}






pub static INCS: [i32; 14] = [1, 4, 13, 40, 121, 364, 1093, 3280,
                          9841, 29524, 88573, 265720,
                          797161, 2391484];
