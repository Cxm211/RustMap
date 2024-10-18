#![no_std]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
//use c2rust_out::*;
extern "C" {
    fn strlen(_: *const i8) -> u64;
}
#[no_mangle]
pub extern "C" fn rc_crc32(mut crc: u32, mut buf: *const i8, mut len: u64) -> u32 {
    unsafe {
        static mut table: [u32; 256] = [0; 256];
        static mut have_table: i32 = 0;
        let mut rem: u32 = 0;
        let mut octet: u8 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut p: *const i8 = 0 as *const i8;
        let mut q: *const i8 = 0 as *const i8;
        if have_table == 0 {
            i = 0;
            while i < 256 {
                rem = i as u32;
                j = 0;
                while j < 8 {
                    if rem & 1 != 0 {
                        rem >>= 1;
                        rem ^= 0xedb88320;
                    } else {
                        rem >>= 1;
                    }
                    j += 1;
                    j;
                }
                table[i as usize] = rem;
                i += 1;
                i;
            }
            have_table = 1;
        }
        crc = !crc;
        q = buf.offset(len as isize);
        p = buf;
        while p < q {
            octet = *p as u8;
            crc = crc >> 8 ^ table[(crc & 0xff ^ octet as u32) as usize];
            p = p.offset(1);
            p;
        }
        return !crc;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut s: *const i8 =
            b"The quick brown fox jumps over the lazy dog\0" as *const u8 as *const i8;
        print!("{:X}\n", rc_crc32(0, s, strlen(s)));
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
