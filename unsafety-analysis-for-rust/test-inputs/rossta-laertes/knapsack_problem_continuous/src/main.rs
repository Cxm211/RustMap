#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn qsort(
        __base: *mut u8,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = std::os::raw::c_ulong;
pub type __compar_fn_t = Option<unsafe extern "C"  fn(_: * const u8,_: * const u8,) -> std::os::raw::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct item {
    pub w: std::os::raw::c_double,
    pub v: std::os::raw::c_double,
    pub name: * const std::os::raw::c_char,
}
impl std::default::Default for item {
    fn default() -> Self {
        item {
        w: std::os::raw::c_double::default(),
        v: std::os::raw::c_double::default(),
        name: 0 as * const std::os::raw::c_char
        }
    }
}

#[no_mangle]
pub static mut items: [item; 9] = [
    {
        let mut init = item {
            w: 3.8f64,
            v: 36 as std::os::raw::c_int as std::os::raw::c_double,
            name: b"beef\0" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = item {
            w: 5.4f64,
            v: 43 as std::os::raw::c_int as std::os::raw::c_double,
            name: b"pork\0" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = item {
            w: 3.6f64,
            v: 90 as std::os::raw::c_int as std::os::raw::c_double,
            name: b"ham\0" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = item {
            w: 2.4f64,
            v: 45 as std::os::raw::c_int as std::os::raw::c_double,
            name: b"greaves\0" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = item {
            w: 4.0f64,
            v: 30 as std::os::raw::c_int as std::os::raw::c_double,
            name: b"flitch\0" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = item {
            w: 2.5f64,
            v: 56 as std::os::raw::c_int as std::os::raw::c_double,
            name: b"brawn\0" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = item {
            w: 3.7f64,
            v: 67 as std::os::raw::c_int as std::os::raw::c_double,
            name: b"welt\0" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = item {
            w: 3.0f64,
            v: 95 as std::os::raw::c_int as std::os::raw::c_double,
            name: b"salami\0" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = item {
            w: 5.9f64,
            v: 98 as std::os::raw::c_int as std::os::raw::c_double,
            name: b"sausage\0" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn item_cmp(
    mut aa: *const u8,
    mut bb: *const u8,
) -> std::os::raw::c_int {
    let mut a: *const item = aa as *const item;
    let mut b: *const item = bb as *const item;
    let mut ua: std::os::raw::c_double = (*a).v / (*a).w;
    let mut ub: std::os::raw::c_double = (*b).v / (*b).w;
    return if ua < ub { -(1 as std::os::raw::c_int) } else { (ua > ub) as std::os::raw::c_int };
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut it: *mut item = 0 as *mut item;
    let mut space: std::os::raw::c_double = 15 as std::os::raw::c_int as std::os::raw::c_double;
    qsort(
        items.as_mut_ptr() as *mut u8,
        9 as std::os::raw::c_int as size_t,
        std::mem::size_of::<item>() as std::os::raw::c_ulong,
        Some(
            item_cmp
                as unsafe extern "C" fn(
                    *const u8,
                    *const u8,
                ) -> std::os::raw::c_int,
        ),
    );
    it = items.as_mut_ptr().offset(9 as std::os::raw::c_int as isize);
    loop {
        let fresh0 = it;
        it = it.offset(-1);
        if !(fresh0.offset_from(items.as_mut_ptr()) as std::os::raw::c_long != 0
            && space > 0 as std::os::raw::c_int as std::os::raw::c_double)
        {
            break;
        }
        if space >= (*it).w {
            printf(b"take all %s\n\0" as *const u8 as *const std::os::raw::c_char, (*it).name);
        } else {
            printf(
                b"take %gkg of %g kg of %s\n\0" as *const u8 as *const std::os::raw::c_char,
                space,
                (*it).w,
                (*it).name,
            );
        }
        space -= (*it).w;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
