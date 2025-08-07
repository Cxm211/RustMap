pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt16 = u16;
pub type UChar = u8;
pub type Bool = bool;
pub const BZ_HDR_B: u8 = 0x42; // 'B'
pub const BZ_HDR_Z: u8 = 0x5a; // 'Z'
pub const BZ_HDR_h: u8 = 0x68; // 'h'
pub const BZ_HDR_0: u8 = 0x30; // '0'
pub static mut verbosity: Int32 = 0;
pub static mut LONGEST_FILE_NAME: Int32 = 0;
pub static mut KEEP_INPUT_FILES: Bool = false;
pub static mut SMALL_MODE: Bool = false;
pub static mut DELETE_OUTPUT_ON_INTERRUPT: Bool = false;
pub static mut FORCE_OVERWRITE: Bool = false;
pub static mut UNZ_FAILS_EXIST: Bool = false;
pub static mut NOISY: Bool = false;
pub static mut NUM_FILE_NAMES: Int32 = 0;
pub static mut NUM_FILES_PROCESSED: Int32 = 0;
pub static mut EXIT_VALUE: Int32 = 0;
pub static mut blockSize100k: Int32 = 9;
pub static mut workFactor: Int32 = 0;


pub fn rebuild_arr2_from_block(block: &[u8]) -> Vec<u32> {
    block.chunks(4).map(|chunk| {
        let mut padded = [0u8; 4];
        padded[..chunk.len()].copy_from_slice(chunk);
        u32::from_ne_bytes(padded)
    }).collect()
}
pub fn build_block_from_arr2(arr2: &[u32]) -> Vec<u8> {
    arr2.iter().flat_map(|&x| {
        let bytes = x.to_ne_bytes();
        bytes.to_vec()
    }).collect()
}



pub fn build_mtfv_from_arr1(arr1: &[u32]) -> Vec<u16> {
    arr1.iter().flat_map(|&x| {
        let high = (x >> 16) as u16;
        let low = x as u16;
        vec![low, high] // 先低位，后高位
    }).collect()
}

pub fn rebuild_arr1_from_mtfv(arr1: &[u16]) -> Vec<u32> {
    arr1.chunks(2).map(|chunk| {
        let low = chunk[0] as u32;
        let high = chunk[1] as u32;
        (high << 16) | low
    }).collect()
}





pub static mut BZ2_rNums: [i32; 512] = [0; 512];
pub static mut BZ2_crc32Table: [u32; 256] = [
   0x00000000, 0x04c11db7, 0x09823b6e, 0x0d4326d9,
   0x130476dc, 0x17c56b6b, 0x1a864db2, 0x1e475005,
   0x2608edb8, 0x22c9f00f, 0x2f8ad6d6, 0x2b4bcb61,
   0x350c9b64, 0x31cd86d3, 0x3c8ea00a, 0x384fbdbd,
   0x4c11db70, 0x48d0c6c7, 0x4593e01e, 0x4152fda9,
   0x5f15adac, 0x5bd4b01b, 0x569796c2, 0x52568b75,
   0x6a1936c8, 0x6ed82b7f, 0x639b0da6, 0x675a1011,
   0x791d4014, 0x7ddc5da3, 0x709f7b7a, 0x745e66cd,
   0x9823b6e0, 0x9ce2ab57, 0x91a18d8e, 0x95609039,
   0x8b27c03c, 0x8fe6dd8b, 0x82a5fb52, 0x8664e6e5,
   0xbe2b5b58, 0xbaea46ef, 0xb7a96036, 0xb3687d81,
   0xad2f2d84, 0xa9ee3033, 0xa4ad16ea, 0xa06c0b5d,
   0xd4326d90, 0xd0f37027, 0xddb056fe, 0xd9714b49,
   0xc7361b4c, 0xc3f706fb, 0xceb42022, 0xca753d95,
   0xf23a8028, 0xf6fb9d9f, 0xfbb8bb46, 0xff79a6f1,
   0xe13ef6f4, 0xe5ffeb43, 0xe8bccd9a, 0xec7dd02d,
   0x34867077, 0x30476dc0, 0x3d044b19, 0x39c556ae,
   0x278206ab, 0x23431b1c, 0x2e003dc5, 0x2ac12072,
   0x128e9dcf, 0x164f8078, 0x1b0ca6a1, 0x1fcdbb16,
   0x018aeb13, 0x054bf6a4, 0x0808d07d, 0x0cc9cdca,
   0x7897ab07, 0x7c56b6b0, 0x71159069, 0x75d48dde,
   0x6b93dddb, 0x6f52c06c, 0x6211e6b5, 0x66d0fb02,
   0x5e9f46bf, 0x5a5e5b08, 0x571d7dd1, 0x53dc6066,
   0x4d9b3063, 0x495a2dd4, 0x44190b0d, 0x40d816ba,
   0xaca5c697, 0xa864db20, 0xa527fdf9, 0xa1e6e04e,
   0xbfa1b04b, 0xbb60adfc, 0xb6238b25, 0xb2e29692,
   0x8aad2b2f, 0x8e6c3698, 0x832f1041, 0x87ee0df6,
   0x99a95df3, 0x9d684044, 0x902b669d, 0x94ea7b2a,
   0xe0b41de7, 0xe4750050, 0xe9362689, 0xedf73b3e,
   0xf3b06b3b, 0xf771768c, 0xfa325055, 0xfef34de2,
   0xc6bcf05f, 0xc27dede8, 0xcf3ecb31, 0xcbffd686,
   0xd5b88683, 0xd1799b34, 0xdc3abded, 0xd8fba05a,
   0x690ce0ee, 0x6dcdfd59, 0x608edb80, 0x644fc637,
   0x7a089632, 0x7ec98b85, 0x738aad5c, 0x774bb0eb,
   0x4f040d56, 0x4bc510e1, 0x46863638, 0x42472b8f,
   0x5c007b8a, 0x58c1663d, 0x558240e4, 0x51435d53,
   0x251d3b9e, 0x21dc2629, 0x2c9f00f0, 0x285e1d47,
   0x36194d42, 0x32d850f5, 0x3f9b762c, 0x3b5a6b9b,
   0x0315d626, 0x07d4cb91, 0x0a97ed48, 0x0e56f0ff,
   0x1011a0fa, 0x14d0bd4d, 0x19939b94, 0x1d528623,
   0xf12f560e, 0xf5ee4bb9, 0xf8ad6d60, 0xfc6c70d7,
   0xe22b20d2, 0xe6ea3d65, 0xeba91bbc, 0xef68060b,
   0xd727bbb6, 0xd3e6a601, 0xdea580d8, 0xda649d6f,
   0xc423cd6a, 0xc0e2d0dd, 0xcda1f604, 0xc960ebb3,
   0xbd3e8d7e, 0xb9ff90c9, 0xb4bcb610, 0xb07daba7,
   0xae3afba2, 0xaafbe615, 0xa7b8c0cc, 0xa379dd7b,
   0x9b3660c6, 0x9ff77d71, 0x92b45ba8, 0x9675461f,
   0x8832161a, 0x8cf30bad, 0x81b02d74, 0x857130c3,
   0x5d8a9099, 0x594b8d2e, 0x5408abf7, 0x50c9b640,
   0x4e8ee645, 0x4a4ffbf2, 0x470cdd2b, 0x43cdc09c,
   0x7b827d21, 0x7f436096, 0x7200464f, 0x76c15bf8,
   0x68860bfd, 0x6c47164a, 0x61043093, 0x65c52d24,
   0x119b4be9, 0x155a565e, 0x18197087, 0x1cd86d30,
   0x029f3d35, 0x065e2082, 0x0b1d065b, 0x0fdc1bec,
   0x3793a651, 0x3352bbe6, 0x3e119d3f, 0x3ad08088,
   0x2497d08d, 0x2056cd3a, 0x2d15ebe3, 0x29d4f654,
   0xc5a92679, 0xc1683bce, 0xcc2b1d17, 0xc8ea00a0,
   0xd6ad50a5, 0xd26c4d12, 0xdf2f6bcb, 0xdbee767c,
   0xe3a1cbc1, 0xe760d676, 0xea23f0af, 0xeee2ed18,
   0xf0a5bd1d, 0xf464a0aa, 0xf9278673, 0xfde69bc4,
   0x89b8fd09, 0x8d79e0be, 0x803ac667, 0x84fbdbd0,
   0x9abc8bd5, 0x9e7d9662, 0x933eb0bb, 0x97ffad0c,
   0xafb010b1, 0xab710d06, 0xa6322bdf, 0xa2f33668,
   0xbcb4666d, 0xb8757bda, 0xb5365d03, 0xb1f740b4,
];
use std::fs::File;
use serde::{Serialize, Deserialize};
use std::io::{self, Read, BufRead};
use std::path::Path;
use std::str::FromStr;

fn convert_to_vec<T>(ptr_option: Option<*mut T>, num_elements: usize) -> Vec<T> 
where
    T: Copy, // T 需要实现 Copy trait，因为我们将从裸指针复制数据
{
    let mut vec = Vec::with_capacity(num_elements);
    if let Some(ptr) = ptr_option {
        unsafe {
            // 从 ptr 指向的内存地址复制 num_elements 个 T 类型元素到 vec 中
            let slice = std::slice::from_raw_parts(ptr, num_elements);
            vec.extend_from_slice(slice);
        }
    }
    vec
}

#[derive(Clone)]
pub struct EState {
 pub   strm: Option<*mut bz_stream>,

 pub   mode: Int32,
 pub  state: Int32,
 pub  avail_in_expect: UInt32,


pub   ftab: Vec<u32>,
pub   origPtr: Int32,


pub   ptr: Vec<u32>,
pub   block: Vec<u8>,
pub   mtfv: Vec<u16>,
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

pub fn print_estate (estate: &EState, file_name: &str){
    let mut file = match File::create(file_name) {
        Ok(file) => file,
        Err(e) => {
            // println!("创建文件时出错: {}", e);
            return;
        }
    };
    let mut write_line = |line: &str| {
        if let Err(e) = writeln!(file, "{}", line) {
            // println!("写入文件时出错: {}", e);
        }
    };
    write_line("EState {{");
    let arr1 = rebuild_arr1_from_mtfv(&estate.mtfv);
    let arr2 = rebuild_arr2_from_block(&estate.block);
    write_line(&format!("  \"arr1\": {:?},", arr1));
    write_line(&format!("  \"arr2\": {:?},", arr2));
    write_line(&format!("  \"zbits\": {:?},", estate.zbits));

    // write_line(&format!("  strm: {:?}", estate.strm));
    
    write_line(&format!("  \"blockSize100k\": {},", estate.blockSize100k));
    write_line(&format!("  \"mode\": {},", estate.mode));
    write_line(&format!("  \"state_in_ch\": {},", estate.state_in_ch));
    write_line(&format!("  \"state\": {},", estate.state));
    write_line(&format!("  \"avail_in_expect\": {},", estate.avail_in_expect));

    // 逐个打印指针类型字段，查看是否会导致崩溃
    // let arr1_vec = convert_to_vec(estate.arr1, (estate.nblock + 34) as usize);
    // write_line(&format!("  \"arr1\": {:?},", arr1_vec));
    // let arr2_vec = convert_to_vec(estate.arr2, (estate.nblock + 34) as usize);
    // write_line(&format!("  \"arr2\": {:?},", arr2_vec));
    // let ftab_vec = convert_to_vec(estate.ftab, 65537);
    // write_line(&format!("  \"ftab\": {:?},", ftab_vec));
    // write_line(&format!("  \"arr1\": {:?},", estate.arr1));
    // write_line(&format!("  \"arr2\": {:?},", estate.arr2));
    write_line(&format!("  \"ftab\": {:?},", estate.ftab));
    write_line(&format!("  \"origPtr\": {},", estate.origPtr));

    // let ptr_vec = convert_to_vec(estate.ptr, (estate.nblock + 34) as usize);
    // write_line(&format!("  \"ptr\": {:?},", ptr_vec));
    // let block_vec = convert_to_vec(estate.block, (estate.nblock + 34) as usize);
    // write_line(&format!("  \"block\": {:?},", block_vec));
    // let mtfv_vec = convert_to_vec(estate.mtfv, (estate.nblock + 34) as usize);
    // write_line(&format!("  \"mtfv\": {:?},", mtfv_vec));
    write_line(&format!("  \"ptr\": {:?},", estate.ptr));
    write_line(&format!("  \"block\": {:?},", estate.block));
    write_line(&format!("  \"mtfv\": {:?},", estate.mtfv));

    
    write_line(&format!("  \"zbits\": {:?},", estate.zbits));

    write_line(&format!("  \"workFactor\": {},", estate.workFactor));
    write_line(&format!("  \"state_in_ch\": {},", estate.state_in_ch));
    write_line(&format!("  \"state_in_len\": {},", estate.state_in_len));
    write_line(&format!("  \"rNToGo\": {},", estate.rNToGo));
    write_line(&format!("  \"rTPos\": {},", estate.rTPos));

    write_line(&format!("  \"nblock\": {},", estate.nblock));
    write_line(&format!("  \"nblockMAX\": {},", estate.nblockMAX));
    write_line(&format!("  \"numZ\": {},", estate.numZ));
    write_line(&format!("  \"state_out_pos\": {},", estate.state_out_pos));
    write_line(&format!("  \"nInUse\": {},", estate.nInUse));

    write_line(&format!("  \"inUse\": {:?},", estate.inUse));
    write_line(&format!("  \"unseqToSeq\": {:?},", estate.unseqToSeq));

    write_line(&format!("  \"bsBuff\": {},", estate.bsBuff));
    write_line(&format!("  \"bsLive\": {},", estate.bsLive));
    write_line(&format!("  \"blockCRC\": {},", estate.blockCRC));
    write_line(&format!("  \"combinedCRC\": {},", estate.combinedCRC));
    write_line(&format!("  \"verbosity\": {},", estate.verbosity));
    write_line(&format!("  \"blockNo\": {},", estate.blockNo));
    write_line(&format!("  \"blockSize100k\": {},", estate.blockSize100k));
    write_line(&format!("  \"nMTF\": {},", estate.nMTF));

    write_line(&format!("  \"mtfFreq\": {:?},", estate.mtfFreq));
    write_line(&format!("  \"selector\": {:?},", estate.selector));
    write_line(&format!("  \"selectorMtf\": {:?},", estate.selectorMtf));

    write_line(&format!("  \"len\": {:?},", estate.len));
    write_line(&format!("  \"code\": {:?},", estate.code));
    write_line(&format!("  \"rfreq\": {:?},", estate.rfreq));

    write_line(&format!("  \"len_pack\": {:?}", estate.len_pack));

    write_line("}}");
}

fn print_ptr_u32(name: &str, ptr: Option<*mut u32>) {
    match ptr {
        Some(p) if !p.is_null() =>  println!("  {}: {:?}", name, p),
        _ => println!("  {}: NULL or None", name),
    }
}


impl EState {  
    // pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<EState> {
    //     let file = File::open(path)?;
    //     let reader = io::BufReader::new(file);

    //     let mut state = EState::new();

    //     for line in reader.lines() {
    //         let line = line?;
    //         if let Some((key, value)) = parse_line(&line) {
    //             match key.as_str() {
    //                 "avail_in" => state.avail_in = UInt32::from_str(&value).unwrap_or_default(),
    //                 "total_in_lo32" => state.total_in_lo32 = UInt32::from_str(&value).unwrap_or_default(),
    //                 "total_in_hi32" => state.total_in_hi32 = UInt32::from_str(&value).unwrap_or_default(),
    //                 "avail_out" => state.avail_out = UInt32::from_str(&value).unwrap_or_default(),
    //                 "total_out_lo32" => state.total_out_lo32 = UInt32::from_str(&value).unwrap_or_default(),
    //                 "total_out_hi32" => state.total_out_hi32 = UInt32::from_str(&value).unwrap_or_default(),

    //                 "mode" => state.mode = Int32::from_str(&value).unwrap_or_default(),
    //                 "state" => state.state = Int32::from_str(&value).unwrap_or_default(),
    //                 "avail_in_expect" => state.avail_in_expect = UInt32::from_str(&value).unwrap_or_default(),
    //                 "origPtr" => state.origPtr = Int32::from_str(&value).unwrap_or_default(),
    //                 "state_in_ch" => state.state_in_ch = UInt32::from_str(&value).unwrap_or_default(),
    //                 "state_in_len" => state.state_in_len = Int32::from_str(&value).unwrap_or_default(),

    //                 "rNToGo" => state.rNToGo = Int32::from_str(&value).unwrap_or_default(),
    //                 "rTPos" => state.rTPos = Int32::from_str(&value).unwrap_or_default(),
    //                 "nblock" => state.nblock = Int32::from_str(&value).unwrap_or_default(),
    //                 "nblockMAX" => state.nblockMAX = Int32::from_str(&value).unwrap_or_default(),
    //                 "numZ" => state.numZ = Int32::from_str(&value).unwrap_or_default(),
    //                 "state_out_pos" => state.state_out_pos = Int32::from_str(&value).unwrap_or_default(),
    //                 "nInUse" => state.nInUse = Int32::from_str(&value).unwrap_or_default(),                }
    //         }
    //     }
    // }
    // 将deserialize_estate_from_file函数作为EState的一个关联函数
    pub fn deserialize_from_file<P: AsRef<Path>>(path: P) -> io::Result<EState> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        let mut estate = EState::default(); // 假设EState实现了Default trait

        for line in reader.lines() {
            let line = line?;
            
            // 根据行的内容更新estate的字段
            // 💡💡💡
            // if line.starts_with("mode: ") {
            //     estate.mode = EState::parse_value(&line, "mode: ")?;
            // } else if line.starts_with("state->arr1: ") {
            //     let arr = EState::parse_array(&line, "state->arr1: [")?;
            //     estate.arr1 = Some(EState::vec_to_raw_pointer(arr));
            // } else if line.starts_with("state->arr2: ") {
            //     let arr = EState::parse_array(&line, "state->arr2: [")?;
            //     estate.arr2 = Some(EState::vec_to_raw_pointer(arr));
            // }
            // 处理其他字段和数组
            // ...
        }

        Ok(estate)
    }
    fn vec_to_raw_pointer<T>(mut v: Vec<T>) -> *mut T {
        let ptr = v.as_mut_ptr();
        std::mem::forget(v); // Prevent Rust from freeing the memory
        ptr
    }
    fn parse_value<T: FromStr>(line: &str, prefix: &str) -> io::Result<T> {
        line.strip_prefix(prefix)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid line format"))
            .and_then(|s| s.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse value")))
    }

    // 私有辅助函数用于解析数组
    fn parse_array<T: FromStr>(line: &str, prefix: &str) -> io::Result<Vec<T>> {
        let mut values = Vec::new();
        let data = line.strip_prefix(prefix)
                       .and_then(|s| s.strip_suffix(']'))
                       .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid array format"))?;

        for item in data.split(", ") {
            let value = item.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse array item"))?;
            values.push(value);
        }

        Ok(values)
    }
}
impl Default for EState {
fn default() -> Self {
        EState {
            strm: None,
    mode: 0,
    state: 0,
    avail_in_expect: 0,
    // arr1: None,
    // arr2: None,
    // ftab: None,
    // arr1:  Vec::new(),
    // arr2:  Vec::new(),
    ftab:  Vec::new(),
    origPtr: 0,
    // ptr: None,
    // block: None,
    // mtfv: None,
    ptr: Vec::new(),
    block: Vec::new(),
    mtfv: Vec::new(),
    zbits: Vec::new(),
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




use crate::global_vars::bzip2_c1::serde_json::json;

use std::fmt::Write as FmtWrite; // 用于使用 write! 宏

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

use serde_json::{Value};
use std::os::raw::{c_int, c_void};
use base64::{encode, decode};  // Import specific functions if needed
pub fn serialize_bzfile(bz_file: &bzFile, filename: &str) -> std::io::Result<()> {
    // 创建 JSON 对象
    let serialized_data = json!({
        "handle": format!("{:p}", bz_file.handle),
        "buf": base64::encode(&bz_file.buf[..bz_file.bufN as usize]), // 使用 base64 编码 buf 数组
        "bufN": bz_file.bufN,
        "writing": bz_file.writing,
        "strm": format!("{:p}", &bz_file.strm as *const _), // 保存 strm 的地址
        "strmD": format!("{:p}", &bz_file.strmD as *const _), // 保存 strmD 的地址
        "lastErr": bz_file.lastErr,
        "initialisedOk": bz_file.initialisedOk,
    });

    // 将 JSON 对象转换为格式化的字符串
    let serialized_string = serde_json::to_string_pretty(&serialized_data)?;
    let mut file = File::create(filename)?;
    file.write_all(serialized_string.as_bytes())?;
    Ok(())
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
            strmD: bz_stream_DState {
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

use serde_json;
use std::io::{Write};

#[derive(Serialize, Deserialize)]
pub struct SerializableBzStream {
    pub avail_in: u32,
    pub total_in_lo32: u32,
    pub total_in_hi32: u32,
    pub avail_out: u32,
    pub total_out_lo32: u32,
    pub total_out_hi32: u32,
    pub next_in_ptr: usize,
    pub next_out_ptr: usize,
    pub state_ptr: usize,
    pub opaque_ptr: usize,
    pub bzalloc_ptr: usize,  // 新增字段
    pub bzfree_ptr: usize,   // 新增字段
}

impl From<&bz_stream> for SerializableBzStream {
    fn from(stream: &bz_stream) -> Self {
        SerializableBzStream {
            avail_in: stream.avail_in,
            total_in_lo32: stream.total_in_lo32,
            total_in_hi32: stream.total_in_hi32,
            avail_out: stream.avail_out,
            total_out_lo32: stream.total_out_lo32,
            total_out_hi32: stream.total_out_hi32,
            next_in_ptr: stream.next_in as usize,
            next_out_ptr: stream.next_out as usize,
            state_ptr: stream.state as usize,
            opaque_ptr: stream.opaque as usize,
            bzalloc_ptr: stream.bzalloc.map_or(0, |f| f as usize),  // 获取函数指针地址
            bzfree_ptr: stream.bzfree.map_or(0, |f| f as usize),   // 获取函数指针地址
        }
    }
}

pub fn serialize_bz_stream(stream: &bz_stream, file_path: &str) -> io::Result<()> {
    let serializable_stream = SerializableBzStream::from(stream);
    let json = serde_json::to_string_pretty(&serializable_stream)?;
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub struct bz_stream_DState {
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


