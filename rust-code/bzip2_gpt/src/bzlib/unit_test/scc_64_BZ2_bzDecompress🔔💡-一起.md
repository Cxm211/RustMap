pub struct bzFile {
 pub   handle: *mut File,
  pub  buf: [u8; 5000],
  pub  bufN: i32,
 pub   writing: bool,
  pub  strm: bz_stream,
  pub  strmD: bz_stream_DState,
 pub   lastErr: i32,
 pub   initialisedOk: bool,
}
pub type BZFILE = *mut bzFile; // Using a raw pointer to represent the opaque BZFILE
#[derive(Copy, Clone)]
pub struct bz_stream {
    pub   next_in: *mut i8, // char in C is i8 in Rust.
    pub   avail_in: u32,
    pub   total_in_lo32: u32,
     pub  total_in_hi32: u32,
   
     pub   next_out: *mut i8,
     pub  avail_out: u32,
     pub  total_out_lo32: u32,
     pub  total_out_hi32: u32,
   
    pub   state: *mut EState, // I'm assuming DState here based on your earlier code.
       
    pub   bzalloc: Option<fn(*mut std::ffi::c_void, Int32, Int32) -> *mut std::ffi::c_void>,
    pub   bzfree: Option<fn(*mut std::ffi::c_void, *mut std::ffi::c_void)>,
    pub   opaque: *mut std::ffi::c_void,
}

pub struct DState {
    pub  strm: *mut bz_stream_DState,   // 引用关联的 bz_stream // Reference to associated bz_stream
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
   pub   unzftab: [Int32; 256], // 字节值频率表（用于 MTF  Frequency table of byte values (used for MTF)
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