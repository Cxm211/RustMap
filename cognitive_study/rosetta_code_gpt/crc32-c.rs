use std::fmt;

static mut TABLE: [u32; 256] = [0; 256];
static mut HAVE_TABLE: bool = false;

fn rc_crc32(mut crc: u32, buf: &[u8]) -> u32 {
    unsafe {
        if !HAVE_TABLE {
            let mut rem: u32;
            for i in 0..256 {
                rem = i;
                for _ in 0..8 {
                    if rem & 1 != 0 {
                        rem >>= 1;
                        rem ^= 0xedb88320;
                    } else {
                        rem >>= 1;
                    }
                }
                TABLE[i] = rem;
            }
            HAVE_TABLE = true;
        }
    }
    
    crc = !crc;
    for &octet in buf.iter() {
        crc = (crc >> 8) ^ unsafe { TABLE[((crc & 0xff) ^ octet as u32) as usize] };
    }
    return !crc;
}

fn main() {
    let s = "The quick brown fox jumps over the lazy dog";
    let crc_value = rc_crc32(0, s.as_bytes());
    println!("{:X}", crc_value);
}