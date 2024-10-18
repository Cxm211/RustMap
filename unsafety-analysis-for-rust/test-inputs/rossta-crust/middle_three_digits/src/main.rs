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
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

//use c2rust_out::*;
extern "C" {
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn strlen(_: *const i8) -> u64;
}
#[no_mangle]
pub extern "C" fn mid3(mut n: i32) -> *mut i8 {
    unsafe {
        static mut buf: [i8; 32] = [0; 32];
        let mut l: i32 = 0;
        sprintf(
            buf.as_mut_ptr(),
            b"%d\0" as *const u8 as *const i8,
            if n > 0 { n } else { -n },
        );
        l = strlen(buf.as_mut_ptr()) as i32;
        if l < 3 || l & 1 == 0 {
            return 0 as *mut i8;
        }
        l = l / 2 - 1;
        buf[(l + 3i32) as usize] = 0;
        return buf.as_mut_ptr().offset(l as isize);
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut x: [i32; 18] = [
            123, 12345, 1234567, 987654321, 10001, -10001, -123, -100, 100, -12345, 1, 2, -1, -10,
            2002, -2002, 0, 1234567890,
        ];
        let mut i: i32 = 0;
        let mut m: *mut i8 = 0 as *mut i8;
        i = 0;
        while (i as u64)
            < (::core::mem::size_of::<[i32; 18]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64)
        {
            m = mid3(x[i as usize]);
            if m.is_null() {
                m = b"error\0" as *const u8 as *const i8 as *mut i8;
            }
            print!(
                "{}: {}\n",
                x[i as usize],
                build_str_from_raw_ptr(m as *mut u8)
            );
            i += 1;
            i;
        }
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
