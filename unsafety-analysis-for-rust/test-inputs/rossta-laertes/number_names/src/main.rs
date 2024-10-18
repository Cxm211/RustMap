#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
}
#[no_mangle]
pub static mut ones: [*const std::os::raw::c_char; 20] = [
    0 as *const std::os::raw::c_char,
    b"one\0" as *const u8 as *const std::os::raw::c_char,
    b"two\0" as *const u8 as *const std::os::raw::c_char,
    b"three\0" as *const u8 as *const std::os::raw::c_char,
    b"four\0" as *const u8 as *const std::os::raw::c_char,
    b"five\0" as *const u8 as *const std::os::raw::c_char,
    b"six\0" as *const u8 as *const std::os::raw::c_char,
    b"seven\0" as *const u8 as *const std::os::raw::c_char,
    b"eight\0" as *const u8 as *const std::os::raw::c_char,
    b"nine\0" as *const u8 as *const std::os::raw::c_char,
    b"ten\0" as *const u8 as *const std::os::raw::c_char,
    b"eleven\0" as *const u8 as *const std::os::raw::c_char,
    b"twelve\0" as *const u8 as *const std::os::raw::c_char,
    b"thirteen\0" as *const u8 as *const std::os::raw::c_char,
    b"fourteen\0" as *const u8 as *const std::os::raw::c_char,
    b"fifteen\0" as *const u8 as *const std::os::raw::c_char,
    b"sixteen\0" as *const u8 as *const std::os::raw::c_char,
    b"seventeen\0" as *const u8 as *const std::os::raw::c_char,
    b"eighteen\0" as *const u8 as *const std::os::raw::c_char,
    b"nineteen\0" as *const u8 as *const std::os::raw::c_char,
];
#[no_mangle]
pub static mut tens: [*const std::os::raw::c_char; 10] = [
    0 as *const std::os::raw::c_char,
    b"ten\0" as *const u8 as *const std::os::raw::c_char,
    b"twenty\0" as *const u8 as *const std::os::raw::c_char,
    b"thirty\0" as *const u8 as *const std::os::raw::c_char,
    b"forty\0" as *const u8 as *const std::os::raw::c_char,
    b"fifty\0" as *const u8 as *const std::os::raw::c_char,
    b"sixty\0" as *const u8 as *const std::os::raw::c_char,
    b"seventy\0" as *const u8 as *const std::os::raw::c_char,
    b"eighty\0" as *const u8 as *const std::os::raw::c_char,
    b"ninety\0" as *const u8 as *const std::os::raw::c_char,
];
#[no_mangle]
pub static mut llions: [*const std::os::raw::c_char; 5] = [
    0 as *const std::os::raw::c_char,
    b"thousand\0" as *const u8 as *const std::os::raw::c_char,
    b"million\0" as *const u8 as *const std::os::raw::c_char,
    b"billion\0" as *const u8 as *const std::os::raw::c_char,
    b"trillion\0" as *const u8 as *const std::os::raw::c_char,
];
#[no_mangle]
pub static mut maxillion: std::os::raw::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn say_hundred(
    mut s: *const std::os::raw::c_char,
    mut len: std::os::raw::c_int,
    mut depth: std::os::raw::c_int,
    mut has_lead: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut c: [std::os::raw::c_int; 3] = [0; 3];
    let mut i: std::os::raw::c_int = 0;
    i = -(3 as std::os::raw::c_int);
    while i < 0 as std::os::raw::c_int {
        if len + i >= 0 as std::os::raw::c_int {
            c[(i + 3 as std::os::raw::c_int)
                as usize] = *s.offset((len + i) as isize) as std::os::raw::c_int - '0' as i32;
        } else {
            c[(i + 3 as std::os::raw::c_int) as usize] = 0 as std::os::raw::c_int;
        }
        i += 1;
        i;
    }
    if c[0 as std::os::raw::c_int as usize] + c[1 as std::os::raw::c_int as usize]
        + c[2 as std::os::raw::c_int as usize] == 0
    {
        return 0 as std::os::raw::c_int;
    }
    if c[0 as std::os::raw::c_int as usize] != 0 {
        printf(
            b"%s hundred\0" as *const u8 as *const std::os::raw::c_char,
            ones[c[0 as std::os::raw::c_int as usize] as usize],
        );
        has_lead = 1 as std::os::raw::c_int;
    }
    if has_lead != 0
        && (c[1 as std::os::raw::c_int as usize] != 0 || c[2 as std::os::raw::c_int as usize] != 0)
    {
        printf(
            if (depth == 0 || c[0 as std::os::raw::c_int as usize] != 0)
                && (c[0 as std::os::raw::c_int as usize] == 0
                    || c[1 as std::os::raw::c_int as usize] == 0)
            {
                b"and \0" as *const u8 as *const std::os::raw::c_char
            } else if c[0 as std::os::raw::c_int as usize] != 0 {
                b" \0" as *const u8 as *const std::os::raw::c_char
            } else {
                b"\0" as *const u8 as *const std::os::raw::c_char
            },
        );
    }
    if c[1 as std::os::raw::c_int as usize] < 2 as std::os::raw::c_int {
        if c[1 as std::os::raw::c_int as usize] != 0 || c[2 as std::os::raw::c_int as usize] != 0 {
            printf(
                b"%s\0" as *const u8 as *const std::os::raw::c_char,
                ones[(c[1 as std::os::raw::c_int as usize] * 10 as std::os::raw::c_int
                    + c[2 as std::os::raw::c_int as usize]) as usize],
            );
        }
    } else {
        if c[1 as std::os::raw::c_int as usize] != 0 {
            printf(
                b"%s\0" as *const u8 as *const std::os::raw::c_char,
                tens[c[1 as std::os::raw::c_int as usize] as usize],
            );
            if c[2 as std::os::raw::c_int as usize] != 0 {
                putchar('-' as i32);
            }
        }
        if c[2 as std::os::raw::c_int as usize] != 0 {
            printf(
                b"%s\0" as *const u8 as *const std::os::raw::c_char,
                ones[c[2 as std::os::raw::c_int as usize] as usize],
            );
        }
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn say_maxillion(
    mut s: *const std::os::raw::c_char,
    mut len: std::os::raw::c_int,
    mut depth: std::os::raw::c_int,
    mut has_lead: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut n: std::os::raw::c_int = len / 3 as std::os::raw::c_int;
    let mut r: std::os::raw::c_int = len % 3 as std::os::raw::c_int;
    if r == 0 {
        n -= 1;
        n;
        r = 3 as std::os::raw::c_int;
    }
    let mut e: *const std::os::raw::c_char = s.offset(r as isize);
    loop {
        if say_hundred(s, r, n, has_lead) != 0 && n != 0 {
            has_lead = 1 as std::os::raw::c_int;
            printf(b" %s\0" as *const u8 as *const std::os::raw::c_char, llions[n as usize]);
            if depth == 0 {
                printf(b", \0" as *const u8 as *const std::os::raw::c_char);
            } else {
                printf(b" \0" as *const u8 as *const std::os::raw::c_char);
            }
        }
        s = e;
        e = e.offset(3 as std::os::raw::c_int as isize);
        r = 3 as std::os::raw::c_int;
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 != 0) {
            break;
        }
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn say_number(mut s: *const std::os::raw::c_char) {
    let mut n: std::os::raw::c_int = 0;
    let mut r: std::os::raw::c_int = 0;
    let mut end: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut has_lead: std::os::raw::c_int = 0;
    let mut current_block: u64;
    let mut len: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut got_sign: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while *s as std::os::raw::c_int == ' ' as i32 {
        s = s.offset(1);
        s;
    }
    if (*s as std::os::raw::c_int) < '0' as i32 || *s as std::os::raw::c_int > '9' as i32 {
        if *s as std::os::raw::c_int == '-' as i32 {
            got_sign = -(1 as std::os::raw::c_int);
            current_block = 15619007995458559411;
        } else if *s as std::os::raw::c_int == '+' as i32 {
            got_sign = 1 as std::os::raw::c_int;
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
        got_sign = 1 as std::os::raw::c_int;
        current_block = 14523784380283086299;
    }
    match current_block {
        14523784380283086299 => {
            while *s as std::os::raw::c_int == '0' as i32 {
                s = s.offset(1);
                s;
                if *s as std::os::raw::c_int == '\0' as i32 {
                    printf(b"zero\n\0" as *const u8 as *const std::os::raw::c_char);
                    return;
                }
            }
            len = strlen(s) as std::os::raw::c_int;
            if !(len == 0) {
                i = 0 as std::os::raw::c_int;
                while i < len {
                    if (*s.offset(i as isize) as std::os::raw::c_int) < '0' as i32
                        || *s.offset(i as isize) as std::os::raw::c_int > '9' as i32
                    {
                        printf(b"(not a number)\0" as *const u8 as *const std::os::raw::c_char);
                        return;
                    }
                    i += 1;
                    i;
                }
                if got_sign == -(1 as std::os::raw::c_int) {
                    printf(b"minus \0" as *const u8 as *const std::os::raw::c_char);
                }
                n = len / maxillion;
                r = len % maxillion;
                if r == 0 {
                    r = maxillion;
                    n -= 1;
                    n;
                }
                end = s.offset(len as isize).offset(-((n * maxillion) as isize));
                has_lead = 0 as std::os::raw::c_int;
                loop {
                    has_lead = say_maxillion(s, r, n, has_lead);
                    if has_lead != 0 {
                        i = 0 as std::os::raw::c_int;
                        while i < n {
                            printf(
                                b" %s\0" as *const u8 as *const std::os::raw::c_char,
                                llions[(maxillion / 3 as std::os::raw::c_int) as usize],
                            );
                            i += 1;
                            i;
                        }
                        if n != 0 {
                            printf(b", \0" as *const u8 as *const std::os::raw::c_char);
                        }
                    }
                    n -= 1;
                    n;
                    r = maxillion;
                    s = end;
                    end = end.offset(r as isize);
                    if !(n >= 0 as std::os::raw::c_int) {
                        break;
                    }
                }
                printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
                return;
            }
        }
        _ => {}
    }
    printf(b"not a number\n\0" as *const u8 as *const std::os::raw::c_char);
}
unsafe fn main_0() -> std::os::raw::c_int {
    say_number(b"-42\0" as *const u8 as *const std::os::raw::c_char);
    say_number(b"1984\0" as *const u8 as *const std::os::raw::c_char);
    say_number(b"10000\0" as *const u8 as *const std::os::raw::c_char);
    say_number(b"1024\0" as *const u8 as *const std::os::raw::c_char);
    say_number(b"1001001001001\0" as *const u8 as *const std::os::raw::c_char);
    say_number(
        b"123456789012345678901234567890123456789012345678900000001\0" as *const u8
            as *const std::os::raw::c_char,
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
unsafe extern "C" fn run_static_initializers() {
    maxillion = (std::mem::size_of::<[*const std::os::raw::c_char; 5]>() as std::os::raw::c_ulong)
        .wrapping_div(std::mem::size_of::<*const std::os::raw::c_char>() as std::os::raw::c_ulong)
        .wrapping_mul(3 as std::os::raw::c_int as std::os::raw::c_ulong)
        .wrapping_sub(3 as std::os::raw::c_int as std::os::raw::c_ulong) as std::os::raw::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
