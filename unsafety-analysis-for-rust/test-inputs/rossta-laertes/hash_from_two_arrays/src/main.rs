#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn strdup(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sHme {
    pub key: * const std::os::raw::c_char,
    pub value: std::os::raw::c_int,
    pub link: * mut crate::sHme,
}
impl std::default::Default for sHme {
    fn default() -> Self {
        sHme {
        key: 0 as * const std::os::raw::c_char,
        value: std::os::raw::c_int::default(),
        link: core::ptr::null_mut()
        }
    }
}

pub type MapEntry = * mut crate::sHme;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct he {
    pub first: * mut crate::sHme,
    pub last: * mut crate::sHme,
}
impl std::default::Default for he {
    fn default() -> Self {
        he {
        first: core::ptr::null_mut(),
        last: core::ptr::null_mut()
        }
    }
}

pub type HashElement = crate::he;
pub type KeyCopyF = Option<unsafe extern "C"  fn(_: * mut * const std::os::raw::c_char,_: * const std::os::raw::c_char,) -> ()>;
pub type ValCopyF = Option<unsafe extern "C"  fn(_: * mut std::os::raw::c_int,_: std::os::raw::c_int,) -> ()>;
pub type KeyHashF = Option<unsafe extern "C"  fn(_: * const std::os::raw::c_char,_: std::os::raw::c_int,) -> std::os::raw::c_uint>;
pub type KeyCmprF = Option<unsafe extern "C"  fn(_: * const std::os::raw::c_char,_: * const std::os::raw::c_char,) -> std::os::raw::c_int>;
#[no_mangle]
pub unsafe extern "C" fn strhashkey(
    mut key: *const std::os::raw::c_char,
    mut max: std::os::raw::c_int,
) -> std::os::raw::c_uint {
    let mut h: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut hl: std::os::raw::c_uint = 0;
    let mut hr: std::os::raw::c_uint = 0;
    while *key != 0 {
        h = h.wrapping_add(*key as std::os::raw::c_uint);
        hl = 0x5c5 as std::os::raw::c_int as std::os::raw::c_uint
            ^ (h & 0xfff00000 as std::os::raw::c_uint) >> 18 as std::os::raw::c_int;
        hr = h & 0xfffff as std::os::raw::c_int as std::os::raw::c_uint;
        let fresh0 = key;
        key = key.offset(1);
        h = hl ^ hr ^ *fresh0 as std::os::raw::c_uint;
    }
    return h.wrapping_rem(max as std::os::raw::c_uint);
}
#[no_mangle]
pub static mut hash: [HashElement; 4096] = [HashElement {
    first: 0 as *const sHme as *mut sHme,
    last: 0 as *const sHme as *mut sHme,
}; 4096];
#[no_mangle]
pub unsafe extern "C" fn HashAddH(
    mut key: *const std::os::raw::c_char,
    mut value: std::os::raw::c_int,
    mut copyKey: KeyCopyF,
    mut copyVal: ValCopyF,
    mut hashKey: KeyHashF,
    mut keySame: KeyCmprF,
) {
    let mut hix: std::os::raw::c_uint = (Some(hashKey.expect("non-null function pointer")))
        .expect("non-null function pointer")(key, 4096 as std::os::raw::c_int);
    let mut m_ent: MapEntry = core::ptr::null_mut();
    m_ent = hash[hix as usize].first;
    while !m_ent.is_null()
        && (Some(keySame.expect("non-null function pointer")))
            .expect("non-null function pointer")((*m_ent).key, key) == 0
    {
        m_ent = (*m_ent).link;
    }
    if !m_ent.is_null() {
        (Some(copyVal.expect("non-null function pointer")))
            .expect("non-null function pointer")(&mut (*m_ent).value, value);
    } else {
        let mut last: MapEntry = core::ptr::null_mut();
        let mut hme: MapEntry = malloc(std::mem::size_of::<sHme>() as std::os::raw::c_ulong)
            as MapEntry;
        (Some(copyKey.expect("non-null function pointer")))
            .expect("non-null function pointer")(&mut (*hme).key, key);
        (Some(copyVal.expect("non-null function pointer")))
            .expect("non-null function pointer")(&mut (*hme).value, value);
        (*hme).link = core::ptr::null_mut();
        last = hash[hix as usize].last;
        if !last.is_null() {
            (*last).link = hme;
        } else {
            hash[hix as usize].first = hme;
        }
        hash[hix as usize].last = hme;
    };
}
#[no_mangle]
pub unsafe extern "C" fn HashGetH<'a1>(
    mut val: Option<&'a1 mut std::os::raw::c_int>,
    mut key: * const std::os::raw::c_char,
    mut hashKey: KeyHashF,
    mut keySame: KeyCmprF,
) -> std::os::raw::c_int {
    let mut hix: std::os::raw::c_uint = (Some(hashKey.expect("non-null function pointer")))
        .expect("non-null function pointer")(key, 4096 as std::os::raw::c_int);
    let mut m_ent: MapEntry = core::ptr::null_mut();
    m_ent = hash[hix as usize].first;
    while !m_ent.is_null()
        && (Some(keySame.expect("non-null function pointer")))
            .expect("non-null function pointer")((*m_ent).key, key) == 0
    {
        m_ent = (*m_ent).link;
    }
    if !m_ent.is_null() {
        *borrow_mut(&mut val).unwrap() = (*m_ent).value;
    }
    return (m_ent != core::ptr::null_mut()) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn copyStr(
    mut dest: *mut *const std::os::raw::c_char,
    mut src: *const std::os::raw::c_char,
) {
    *dest = strdup(src);
}
#[no_mangle]
pub unsafe extern "C" fn copyInt(mut dest: *mut std::os::raw::c_int, mut src: std::os::raw::c_int) {
    *dest = src;
}
#[no_mangle]
pub unsafe extern "C" fn strCompare(
    mut key1: *const std::os::raw::c_char,
    mut key2: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    return (strcmp(key1, key2) == 0 as std::os::raw::c_int) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn HashAdd(mut key: *const std::os::raw::c_char, mut value: std::os::raw::c_int) {
    HashAddH(
        key,
        value,
        Some(
            copyStr
                as unsafe extern "C" fn(
                    *mut *const std::os::raw::c_char,
                    *const std::os::raw::c_char,
                ) -> (),
        ),
        Some(copyInt as unsafe extern "C" fn(*mut std::os::raw::c_int, std::os::raw::c_int) -> ()),
        Some(
            strhashkey
                as unsafe extern "C" fn(*const std::os::raw::c_char, std::os::raw::c_int) -> std::os::raw::c_uint,
        ),
        Some(
            strCompare
                as unsafe extern "C" fn(
                    *const std::os::raw::c_char,
                    *const std::os::raw::c_char,
                ) -> std::os::raw::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn HashGet<'a1>(
    mut val: Option<&'a1 mut std::os::raw::c_int>,
    mut key: * const std::os::raw::c_char,
) -> std::os::raw::c_int {
    return HashGetH(
        borrow_mut(&mut val),
        key,
        Some(
            strhashkey
                as unsafe extern "C" fn(*const std::os::raw::c_char, std::os::raw::c_int) -> std::os::raw::c_uint,
        ),
        Some(
            strCompare
                as unsafe extern "C" fn(
                    *const std::os::raw::c_char,
                    *const std::os::raw::c_char,
                ) -> std::os::raw::c_int,
        ),
    );
}
unsafe fn main_0() -> std::os::raw::c_int {
    static mut keyList: [*const std::os::raw::c_char; 6] = [
        b"red\0" as *const u8 as *const std::os::raw::c_char,
        b"orange\0" as *const u8 as *const std::os::raw::c_char,
        b"yellow\0" as *const u8 as *const std::os::raw::c_char,
        b"green\0" as *const u8 as *const std::os::raw::c_char,
        b"blue\0" as *const u8 as *const std::os::raw::c_char,
        b"violet\0" as *const u8 as *const std::os::raw::c_char,
    ];
    static mut valuList: [std::os::raw::c_int; 6] = [
        1 as std::os::raw::c_int,
        43 as std::os::raw::c_int,
        640 as std::os::raw::c_int,
        747 as std::os::raw::c_int,
        42 as std::os::raw::c_int,
        42 as std::os::raw::c_int,
    ];
    let mut ix: std::os::raw::c_int = 0;
    ix = 0 as std::os::raw::c_int;
    while ix < 6 as std::os::raw::c_int {
        HashAdd(keyList[ix as usize], valuList[ix as usize]);
        ix += 1;
        ix;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
