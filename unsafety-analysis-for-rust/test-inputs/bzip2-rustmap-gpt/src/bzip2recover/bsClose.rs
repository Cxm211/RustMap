use crate::bzip2recover::readError::*;
use crate::bzip2recover::writeError::*;
// use crate::bzip2recover::scc_6_root_3_mallocFail::*;
// use crate::global_vars::bzip2_c2::{bytesOut, BitStream};
use std::io::Cursor;

// bsClose ( BitStream* bs )

// bsPutBit ( BitStream* bs, Int32 bit )
type Char = u8;
type UChar = u8;
type Int32 = i32; // 对应C的int类型
type MaybeUInt64 = u64;

// 这是一个模拟的全局变量, 在实际Rust代码中，你可能想要用其他方式来管理
static mut BYTES_OUT: MaybeUInt64 = 0;
pub struct BitStream {
    handle: Cursor<Vec<u8>>,
    buffer: Int32,
    buff_live: Int32,
    mode: Char,
}

use std::io::{Read, Write};

impl BitStream {
    // ... [put_bit和其他函数保持不变]

    pub fn get_bit(&mut self) -> Option<Int32> {
        if self.buff_live > 0 {
            self.buff_live -= 1;
            return Some((self.buffer >> self.buff_live) & 0x1);
        } else {
            let mut byte = [0u8; 1];
            match self.handle.read(&mut byte) {
                Ok(1) => {
                    self.buffer = byte[0] as Int32;
                    self.buff_live = 7;
                    return Some((self.buffer >> 7) & 0x1);
                }
                Ok(0) => return None, // End of stream
                Ok(_) => unreachable!(),
                Err(_) => {
                    read_error();
                    return None;
                }
            }
        }
    }

    pub fn close(&mut self) {
        if self.mode == 'w' as Char {
            while self.buff_live < 8 {
                self.buff_live += 1;
                self.buffer <<= 1;
            }
            if self.handle.write_all(&[self.buffer as UChar]).is_err() {
                write_error();
                return;
            }
            unsafe {
                BYTES_OUT += 1;
            }
            if self.handle.flush().is_err() {
                write_error();
            }
        }
        // 我们在这里没有进行关闭操作，因为Cursor不需要关闭，但在实际文件I/O中可能需要
        // 对于免费操作，我们也不需要明确地释放BitStream，因为Rust有一个内置的垃圾回收器。
    }


    // 为读取模式打开BitStream
    pub fn open_read_stream(stream: Cursor<Vec<u8>>) -> Self {
        BitStream {
            handle: stream,
            buffer: 0,
            buff_live: 0,
            mode: 'r' as Char,
        }
    }

    // 为写入模式打开BitStream
    pub fn open_write_stream(stream: Cursor<Vec<u8>>) -> Self {
        BitStream {
            handle: stream,
            buffer: 0,
            buff_live: 0,
            mode: 'w' as Char,
        }
    }
    
}


mod tests {
    // ... [保持原先的测试不变]
    use std::io::Cursor;
    use super::*;
    #[test]
    fn test_get_bit() {
        let data = vec![0x55]; // 01010101
        let fake_file = Cursor::new(data);

        let mut bit_stream = BitStream {
            handle: fake_file,
            buffer: 0,
            buff_live: 0,
            mode: 0,
        };

        let bits: Vec<_> = (0..8).map(|_| bit_stream.get_bit()).collect();
        assert_eq!(bits, vec![Some(0), Some(1), Some(0), Some(1), Some(0), Some(1), Some(0), Some(1)]);
    }
    
    #[test]
    fn test_open_read_stream() {
        // 创建一个模拟文件
        let stream = Cursor::new(Vec::new());
        let bit_stream_read = BitStream::open_read_stream(stream);

        // 验证BitStream的属性
        assert_eq!(bit_stream_read.buffer, 0);
        assert_eq!(bit_stream_read.buff_live, 0);
        assert_eq!(bit_stream_read.mode, 'r' as Char);

        // 如果你想进一步验证handle的状态，你可以对Cursor<Vec<u8>>进行操作并检查其状态。
        // 但是在这里，我们主要是验证BitStream的初始化。
    }

    #[test]
    fn test_open_write_stream() {
        // 创建一个模拟文件
        let stream = Cursor::new(Vec::new());
        let bit_stream_write = BitStream::open_write_stream(stream);

        // 验证BitStream的属性
        assert_eq!(bit_stream_write.buffer, 0);
        assert_eq!(bit_stream_write.buff_live, 0);
        assert_eq!(bit_stream_write.mode, 'w' as Char);

        // 与上面的测试类似，你可以对handle进行额外的验证。
    }
}


