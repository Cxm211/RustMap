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
    fn strlen(_: *const i8) -> u64;
}
#[no_mangle]
pub static mut ones: [*const i8; 20] = [
    0 as *const i8,
    b"one\0" as *const u8 as *const i8,
    b"two\0" as *const u8 as *const i8,
    b"three\0" as *const u8 as *const i8,
    b"four\0" as *const u8 as *const i8,
    b"five\0" as *const u8 as *const i8,
    b"six\0" as *const u8 as *const i8,
    b"seven\0" as *const u8 as *const i8,
    b"eight\0" as *const u8 as *const i8,
    b"nine\0" as *const u8 as *const i8,
    b"ten\0" as *const u8 as *const i8,
    b"eleven\0" as *const u8 as *const i8,
    b"twelve\0" as *const u8 as *const i8,
    b"thirteen\0" as *const u8 as *const i8,
    b"fourteen\0" as *const u8 as *const i8,
    b"fifteen\0" as *const u8 as *const i8,
    b"sixteen\0" as *const u8 as *const i8,
    b"seventeen\0" as *const u8 as *const i8,
    b"eighteen\0" as *const u8 as *const i8,
    b"nineteen\0" as *const u8 as *const i8,
];
#[no_mangle]
pub static mut tens: [*const i8; 10] = [
    0 as *const i8,
    b"ten\0" as *const u8 as *const i8,
    b"twenty\0" as *const u8 as *const i8,
    b"thirty\0" as *const u8 as *const i8,
    b"forty\0" as *const u8 as *const i8,
    b"fifty\0" as *const u8 as *const i8,
    b"sixty\0" as *const u8 as *const i8,
    b"seventy\0" as *const u8 as *const i8,
    b"eighty\0" as *const u8 as *const i8,
    b"ninety\0" as *const u8 as *const i8,
];
#[no_mangle]
pub static mut llions: [*const i8; 5] = [
    0 as *const i8,
    b"thousand\0" as *const u8 as *const i8,
    b"million\0" as *const u8 as *const i8,
    b"billion\0" as *const u8 as *const i8,
    b"trillion\0" as *const u8 as *const i8,
];
#[no_mangle]
pub static mut maxillion: i32 = 0;
#[no_mangle]
pub extern "C" fn say_hundred(
    mut s: *const i8,
    mut len: i32,
    mut depth: i32,
    mut has_lead: i32,
) -> i32 {
    unsafe {
        let mut c: [i32; 3] = [0; 3];
        let mut i: i32 = 0;
        i = -3;
        while i < 0 {
            if len + i >= 0 {
                c[(i + 3i32) as usize] = *s.offset((len + i) as isize) as i32 - '0' as i32;
            } else {
                c[(i + 3i32) as usize] = 0;
            }
            i += 1;
            i;
        }
        if c[0 as usize] + c[1 as usize] + c[2 as usize] == 0 {
            return 0;
        }
        if c[0 as usize] != 0 {
            print!(
                "{} hundred",
                build_str_from_raw_ptr(ones[c[0 as usize] as usize] as *mut u8)
            );
            has_lead = 1;
        }
        if has_lead != 0 && (c[1 as usize] != 0 || c[2 as usize] != 0) {
            if (depth == 0 || c[0 as usize] != 0) && (c[0 as usize] == 0 || c[1 as usize] == 0) {
                print!("and ")
            } else {
                if c[0 as usize] != 0 {
                    print!(" ")
                } else {
                    print!("")
                }
            };
        }
        if c[1 as usize] < 2 {
            if c[1 as usize] != 0 || c[2 as usize] != 0 {
                print!(
                    "{}",
                    build_str_from_raw_ptr(
                        ones[(c[1 as usize] * 10 + c[2 as usize]) as usize] as *mut u8
                    )
                );
            }
        } else {
            if c[1 as usize] != 0 {
                print!(
                    "{}",
                    build_str_from_raw_ptr(tens[c[1 as usize] as usize] as *mut u8)
                );
                if c[2 as usize] != 0 {
                    print!("{}", '-' as i32);
                }
            }
            if c[2 as usize] != 0 {
                print!(
                    "{}",
                    build_str_from_raw_ptr(ones[c[2 as usize] as usize] as *mut u8)
                );
            }
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn say_maxillion(
    mut s: *const i8,
    mut len: i32,
    mut depth: i32,
    mut has_lead: i32,
) -> i32 {
    unsafe {
        let mut n: i32 = len / 3;
        let mut r: i32 = len % 3;
        if r == 0 {
            n -= 1;
            n;
            r = 3;
        }
        let mut e: *const i8 = s.offset(r as isize);
        loop {
            if say_hundred(s, r, n, has_lead) != 0 && n != 0 {
                has_lead = 1;
                print!(" {}", build_str_from_raw_ptr(llions[n as usize] as *mut u8));
                if depth == 0 {
                    print!(", ");
                } else {
                    print!(" ");
                }
            }
            s = e;
            e = e.offset(3 as isize);
            r = 3;
            let fresh0 = n;
            n = n - 1;
            if !(fresh0 != 0) {
                break;
            }
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn say_number(mut s: *const i8) {
    unsafe {
        let mut n: i32 = 0;
        let mut r: i32 = 0;
        let mut end: *const i8 = 0 as *const i8;
        let mut has_lead: i32 = 0;
        let mut current_block: u64;
        let mut len: i32 = 0;
        let mut i: i32 = 0;
        let mut got_sign: i32 = 0;
        while *s as i32 == ' ' as i32 {
            s = s.offset(1);
            s;
        }
        if (*s as i32) < '0' as i32 || *s as i32 > '9' as i32 {
            if *s as i32 == '-' as i32 {
                got_sign = -1;
                current_block = 15619007995458559411;
            } else if *s as i32 == '+' as i32 {
                got_sign = 1;
                current_block = 15619007995458559411;
            } else {
                current_block = 1693057788147685417;
            }
            match current_block {
                1693057788147685417 => {}
                _ => {
                    s = s.offset(1);
                    s;
                    current_block = 14523784380283086299;
                }
            }
        } else {
            got_sign = 1;
            current_block = 14523784380283086299;
        }
        match current_block {
            14523784380283086299 => {
                while *s as i32 == '0' as i32 {
                    s = s.offset(1);
                    s;
                    if *s as i32 == '\0' as i32 {
                        print!("zero\n");
                        return;
                    }
                }
                len = strlen(s) as i32;
                if !(len == 0) {
                    i = 0;
                    while i < len {
                        if (*s.offset(i as isize) as i32) < '0' as i32
                            || *s.offset(i as isize) as i32 > '9' as i32
                        {
                            print!("(not a number)");
                            return;
                        }
                        i += 1;
                        i;
                    }
                    if got_sign == -1 {
                        print!("minus ");
                    }
                    n = len / maxillion;
                    r = len % maxillion;
                    if r == 0 {
                        r = maxillion;
                        n -= 1;
                        n;
                    }
                    end = s.offset(len as isize).offset(-((n * maxillion) as isize));
                    has_lead = 0;
                    loop {
                        has_lead = say_maxillion(s, r, n, has_lead);
                        if has_lead != 0 {
                            i = 0;
                            while i < n {
                                print!(
                                    " {}",
                                    build_str_from_raw_ptr(
                                        llions[(maxillion / 3i32) as usize] as *mut u8
                                    )
                                );
                                i += 1;
                                i;
                            }
                            if n != 0 {
                                print!(", ");
                            }
                        }
                        n -= 1;
                        n;
                        r = maxillion;
                        s = end;
                        end = end.offset(r as isize);
                        if !(n >= 0) {
                            break;
                        }
                    }
                    print!("\n");
                    return;
                }
            }
            _ => {}
        }
        print!("not a number\n");
    }
}

fn main_0() -> i32 {
    say_number(b"-42\0" as *const u8 as *const i8);
    say_number(b"1984\0" as *const u8 as *const i8);
    say_number(b"10000\0" as *const u8 as *const i8);
    say_number(b"1024\0" as *const u8 as *const i8);
    say_number(b"1001001001001\0" as *const u8 as *const i8);
    say_number(
        b"123456789012345678901234567890123456789012345678900000001\0" as *const u8 as *const i8,
    );
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

extern "C" fn run_static_initializers() {
    unsafe {
        maxillion = (::core::mem::size_of::<[*const i8; 5]>() as u64)
            .wrapping_div(::core::mem::size_of::<*const i8>() as u64)
            .wrapping_mul(3)
            .wrapping_sub(3) as i32;
    }
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
