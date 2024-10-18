// use std::io::{Write, Seek, Read, SeekFrom, Cursor};
use std::io::{Cursor, Write, Read};
use std::convert::TryInto;
use crate::global_vars::bzip2_c2::*;
use crate::global_vars::bzip2_c4::*;
use crate::bzip2recover::writeError::*;


pub fn bs_put_u_char(bitstream: &mut BitStream, c: u8) {
    for i in (0..8).rev() {
        bs_put_bit(bitstream, ((c as u32) >> i) as i32 & 0x1);
    }
}

pub fn bs_put_u_int32(bitstream: &mut BitStream, c: u32) {
    for i in (0..32).rev() {
        bs_put_bit(bitstream, (c >> i) as i32 & 0x1);
    }
}

pub fn bs_put_bit(bitstream: &mut BitStream, bit: i32) {
    if bitstream.buff_live == 8 {
        if bitstream.handle.write_all(&[bitstream.buffer as u8]).is_err() {
            write_error();
        }
        unsafe { BYTES_OUT += 1; }
        bitstream.buff_live = 0;
        bitstream.buffer = 0;
    }
    bitstream.buffer = ((bitstream.buffer << 1) | (bit & 0x1));
    bitstream.buff_live += 1;
}

pub fn flush(bitstream: &mut BitStream) {
    if bitstream.buff_live > 0 {
        bitstream.buffer <<= (8 - bitstream.buff_live);
        if bitstream.handle.write_all(&[bitstream.buffer as u8]).is_err() {
            write_error();
        }
        unsafe { BYTES_OUT += 1; }
        bitstream.buff_live = 0;
        bitstream.buffer = 0;
    }
}

pub fn reset(bitstream: &mut BitStream) {
    bitstream.buffer = 0;
    bitstream.buff_live = 0;
    bitstream.handle.set_position(0);
    unsafe { BYTES_OUT = 0; }
}

// impl BitStream {
//     pub fn bs_put_u_char(&mut self, c: u8) {
//         for i in (0..8).rev() {
//             self.bs_put_bit(((c as u32) >> i) as i32 & 0x1);
//         }
//     }

//     pub fn bs_put_u_int32(&mut self, c: u32) {
//         for i in (0..32).rev() {
//             self.bs_put_bit((c >> i) as i32 & 0x1);
//         }
//     }

//     pub fn reset(&mut self) {
//         self.buffer = 0;
//         self.buff_live = 0;
//         self.handle.set_position(0);
//         unsafe { BYTES_OUT = 0; }
//     }

//     // 建议在每次 bs_put_bit 调用后检查 buffer 和 buff_live 的状态，以确保数据已经被正确地写入缓存
//     /*
//     ，这些测试用例运行超过60秒通常意味着有无限循环或其他形式的资源耗尽。从给定的代码中，有一些可能的原因：

//     bs_put_bit函数在buff_live为8时，只重置buff_live和buffer，没有将数据写入文件。
//     这可能导致在某些情况下缓存被反复填充但从未清空。
//     在bs_put_bit的第一个if块中，确保在写入文件后正确地重置buffer。
//     重写flush函数，确保它不会陷入无限循环。
//     */
//     pub fn bs_put_bit(&mut self, bit: i32) {
//         if self.buff_live == 8 {
//             if self.handle.write_all(&[self.buffer as u8]).is_err() {
//                 write_error();
//             }
//             unsafe { BYTES_OUT += 1; }
//             self.buff_live = 0;
//             self.buffer = 0;
//         }
//         self.buffer = ((self.buffer << 1) | (bit & 0x1));
//         self.buff_live += 1;
//     }
//     /*
//         flush方法是一个可能的无限循环的地方。
//     如果bs_put_bit在某些情况下没有适当地修改buff_live，
//         那么flush方法会反复调用bs_put_bit(0)，但buff_live可能永远不会达到0。
//     解决这个问题的办法：

//      */
//     pub fn flush(&mut self) {
//         if self.buff_live > 0 {
//             self.buffer <<= (8 - self.buff_live);
//             if self.handle.write_all(&[self.buffer as u8]).is_err() {
//                 write_error();
//             }
//             unsafe { BYTES_OUT += 1; }
//             self.buff_live = 0;
//             self.buffer = 0;
//         }
//     }
// }

// Unit tests
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_bs_put_u_char() {
//         let mut bs = BitStream {
//             handle: Cursor::new(Vec::new()),
//             buffer: 0,
//             buff_live: 0,
//             mode: b'w',
//         };
//         let test_value = 0x5A; // Arbitrary test value
//         bs.bs_put_u_char(test_value);
//         bs.flush();
//         bs.handle.set_position(0);
//         let mut read_value = [0u8; 1];
//         bs.handle.read_exact(&mut read_value).unwrap();
//         assert_eq!(read_value[0], test_value);
//     }

//     #[test]
//     fn test_bs_put_bit() {
//         let mut bs = BitStream {
//             handle: Cursor::new(Vec::new()),
//             buffer: 0,
//             buff_live: 0,
//             mode: b'w',
//         };
//         bs.bs_put_bit(0);
//         bs.bs_put_bit(1);
//         bs.bs_put_bit(0);
//         bs.bs_put_bit(1);
//         bs.bs_put_bit(1);
//         bs.bs_put_bit(0);
//         bs.bs_put_bit(1);
//         bs.bs_put_bit(0);
//         bs.flush();
//         bs.handle.set_position(0);
//         let mut read_value = [0u8; 1];
//         bs.handle.read_exact(&mut read_value).unwrap();
//         assert_eq!(read_value[0], 0x5A);
//     }

//     #[test]
//     fn test_bs_put_u_int32() {
//         let mut bs = BitStream {
//             handle: Cursor::new(Vec::new()),
//             buffer: 0,
//             buff_live: 0,
//             mode: b'w',
//         };
//         let test_value = 0xDEADBEEF; // Arbitrary test value
//         bs.bs_put_u_int32(test_value);
//         bs.flush();
//         bs.handle.set_position(0);
//         let mut read_value = [0u8; 4];
//         bs.handle.read_exact(&mut read_value).unwrap();
//         assert_eq!(u32::from_be_bytes(read_value), test_value);
//     }
// }
