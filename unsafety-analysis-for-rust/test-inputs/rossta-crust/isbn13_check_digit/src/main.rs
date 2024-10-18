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
extern "C" {}
#[no_mangle]
pub extern "C" fn check_isbn13(mut isbn: *const i8) -> i32 {
    unsafe {
        let mut ch: i32 = *isbn as i32;
        let mut count: i32 = 0;
        let mut sum: i32 = 0;
        while ch != 0 {
            if ch == ' ' as i32 || ch == '-' as i32 {
                count -= 1;
                count;
            } else {
                if ch < '0' as i32 || ch > '9' as i32 {
                    return 0;
                }
                if count & 1 != 0 {
                    sum += 3 * (ch - '0' as i32);
                } else {
                    sum += ch - '0' as i32;
                }
            }
            isbn = isbn.offset(1);
            ch = *isbn as i32;
            count += 1;
            count;
        }
        if count != 13 {
            return 0;
        }
        return (sum % 10 == 0) as i32;
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut isbns: [*const i8; 4] = [
        b"978-1734314502\0" as *const u8 as *const i8,
        b"978-1734314509\0" as *const u8 as *const i8,
        b"978-1788399081\0" as *const u8 as *const i8,
        b"978-1788399083\0" as *const u8 as *const i8,
    ];
    i = 0;
    unsafe {
        while i < 4 {
            if check_isbn13(isbns[i as usize]) != 0 {
                print!(
                    "{}: {}\n",
                    build_str_from_raw_ptr(isbns[i as usize] as *mut u8),
                    "good\0"
                )
            } else {
                print!(
                    "{}: {}\n",
                    build_str_from_raw_ptr(isbns[i as usize] as *mut u8),
                    "bad\0"
                )
            };
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
