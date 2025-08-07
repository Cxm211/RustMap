
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





pub fn bz2_bz_assert_h_fail(errcode: i32) {
  / * 省略 */
}


pub fn bz2_hb_make_code_lengths(len: &mut [u8], freq: &[i32], alpha_size: i32, max_len: i32) {
  / * 省略 */
}

pub fn bz2_hb_assign_codes(code: &mut [i32], length: &[u8], min_len: i32, max_len: i32, alpha_size: i32) {
  / * 省略 */
}

pub fn bs_w(state: &mut EState, n: i32, v: u32) {
   / * 省略 */
}