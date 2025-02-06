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
//  pub   zbits: Option<*mut UChar>, //TODO: Vec<u8>
    pub zbits: Vec<u8>,

 pub   workFactor: Int32,
 pub   state_in_ch: UInt32,
 pub   state_in_len: Int32,
 pub   rNToGo: Int32, 
 pub   rTPos: Int32,

 pub   nblock: Int32,
 pub   nblockMAX: Int32,
 pub   numZ: Int32,
 pub   state_out_pos: Int32,
 pub   nInUse: Int32,

 pub   inUse: [Bool; 256],
 pub   unseqToSeq: [UChar; 256],

 pub   bsBuff: UInt32,
 pub   bsLive: Int32,
 pub   blockCRC: UInt32,
 pub   combinedCRC: UInt32,
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

 pub  len_pack: [[UInt32; 4]; 258],
}

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


UInt32 BZ2_crc32Table[256];

pub fn add_pair_to_block(s: &mut EState) {
 /* 省略 */
}

UChar为u8