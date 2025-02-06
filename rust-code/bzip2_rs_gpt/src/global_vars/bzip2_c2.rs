use std::io::Cursor;
pub type Char = u8;
type UChar = u8;
type Int32 = i32; // 对应C的int类型
pub const MAX_SIZE: usize = 2000;
pub static mut PROG_NAME: [u8; MAX_SIZE] = [b'\0'; MAX_SIZE];
pub static mut IN_FILE_NAME: [u8; MAX_SIZE] = [b'\0'; MAX_SIZE];
pub static mut BLOCK_SIZE100K: Int32 = 0; // c2pub static mut BYTES_OUT: u64 = 0;
pub struct BitStream {
    pub handle: Cursor<Vec<u8>>,
    pub buffer: Int32,
    pub buff_live: Int32,
    pub mode: Char,
}


type MaybeUInt64 = u64;

pub const ARRAY_SIZE: usize = 50000; // 明确标明usize类型，因为数组长度和索引通常是usize类型

pub static mut B_START: [MaybeUInt64; ARRAY_SIZE] = [0; ARRAY_SIZE];
pub static mut B_END: [MaybeUInt64; ARRAY_SIZE] = [0; ARRAY_SIZE];
pub static mut RB_START: [MaybeUInt64; ARRAY_SIZE] = [0; ARRAY_SIZE];
pub static mut RB_END: [MaybeUInt64; ARRAY_SIZE] = [0; ARRAY_SIZE];

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