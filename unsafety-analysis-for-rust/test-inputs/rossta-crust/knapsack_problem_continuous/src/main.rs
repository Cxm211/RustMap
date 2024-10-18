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
    fn qsort(__base: *mut libc::c_void, __nmemb: u64, __size: u64, __compar: __compar_fn_t);
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct item {
    pub w: f64,
    pub v: f64,
    pub name: *const i8,
}
#[no_mangle]
pub static mut items: [item; 9] = [
    {
        let mut init = item {
            w: 3.8f64,
            v: 36 as f64,
            name: b"beef\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = item {
            w: 5.4f64,
            v: 43 as f64,
            name: b"pork\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = item {
            w: 3.6f64,
            v: 90 as f64,
            name: b"ham\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = item {
            w: 2.4f64,
            v: 45 as f64,
            name: b"greaves\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = item {
            w: 4.0f64,
            v: 30 as f64,
            name: b"flitch\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = item {
            w: 2.5f64,
            v: 56 as f64,
            name: b"brawn\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = item {
            w: 3.7f64,
            v: 67 as f64,
            name: b"welt\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = item {
            w: 3.0f64,
            v: 95 as f64,
            name: b"salami\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = item {
            w: 5.9f64,
            v: 98 as f64,
            name: b"sausage\0" as *const u8 as *const i8,
        };
        init
    },
];
#[no_mangle]
pub extern "C" fn item_cmp(mut aa: *const libc::c_void, mut bb: *const libc::c_void) -> i32 {
    unsafe {
        let mut a: *const item = aa as *const item;
        let mut b: *const item = bb as *const item;
        let mut ua: f64 = (*a).v / (*a).w;
        let mut ub: f64 = (*b).v / (*b).w;
        return if ua < ub { -1 } else { (ua > ub) as i32 };
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut it: *mut item = 0 as *mut item;
        let mut space: f64 = 15 as f64;
        qsort(
            items.as_mut_ptr() as *mut libc::c_void,
            9,
            ::core::mem::size_of::<item>() as u64,
            Some(item_cmp as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32),
        );
        it = items.as_mut_ptr().offset(9 as isize);
        loop {
            let fresh0 = it;
            it = it.offset(-1);
            if !(fresh0.offset_from(items.as_mut_ptr()) as i64 != 0 && space > 0 as f64) {
                break;
            }
            if space >= (*it).w {
                print!(
                    "take all {}\n",
                    build_str_from_raw_ptr((*it).name as *mut u8)
                );
            } else {
                print!(
                    "take {}kg of {} kg of {}\n",
                    space,
                    (*it).w,
                    build_str_from_raw_ptr((*it).name as *mut u8)
                );
            }
            space -= (*it).w;
        }
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
