#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn cos(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sin(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn perror(__s: *const std::os::raw::c_char);
}
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type size_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::os::raw::c_int,
    pub _IO_read_ptr: *mut std::os::raw::c_char,
    pub _IO_read_end: *mut std::os::raw::c_char,
    pub _IO_read_base: *mut std::os::raw::c_char,
    pub _IO_write_base: *mut std::os::raw::c_char,
    pub _IO_write_ptr: *mut std::os::raw::c_char,
    pub _IO_write_end: *mut std::os::raw::c_char,
    pub _IO_buf_base: *mut std::os::raw::c_char,
    pub _IO_buf_end: *mut std::os::raw::c_char,
    pub _IO_save_base: *mut std::os::raw::c_char,
    pub _IO_backup_base: *mut std::os::raw::c_char,
    pub _IO_save_end: *mut std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::os::raw::c_int,
    pub _flags2: std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: std::os::raw::c_ushort,
    pub _vtable_offset: std::os::raw::c_schar,
    pub _shortbuf: [std::os::raw::c_char; 1],
    pub _lock: *mut u8,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut u8,
    pub __pad5: size_t,
    pub _mode: std::os::raw::c_int,
    pub _unused2: [std::os::raw::c_char; 20],
}
impl std::default::Default for _IO_FILE {
    fn default() -> Self {
        _IO_FILE {
        _flags: std::os::raw::c_int::default(),
        _IO_read_ptr: 0 as * mut std::os::raw::c_char,
        _IO_read_end: 0 as * mut std::os::raw::c_char,
        _IO_read_base: 0 as * mut std::os::raw::c_char,
        _IO_write_base: 0 as * mut std::os::raw::c_char,
        _IO_write_ptr: 0 as * mut std::os::raw::c_char,
        _IO_write_end: 0 as * mut std::os::raw::c_char,
        _IO_buf_base: 0 as * mut std::os::raw::c_char,
        _IO_buf_end: 0 as * mut std::os::raw::c_char,
        _IO_save_base: 0 as * mut std::os::raw::c_char,
        _IO_backup_base: 0 as * mut std::os::raw::c_char,
        _IO_save_end: 0 as * mut std::os::raw::c_char,
        _markers: 0 as * mut crate::_IO_marker,
        _chain: core::ptr::null_mut(),
        _fileno: std::os::raw::c_int::default(),
        _flags2: std::os::raw::c_int::default(),
        _old_offset: std::os::raw::c_long::default(),
        _cur_column: std::os::raw::c_ushort::default(),
        _vtable_offset: std::os::raw::c_schar::default(),
        _shortbuf: [std::os::raw::c_char::default(); 1],
        _lock: 0 as * mut u8,
        _offset: std::os::raw::c_long::default(),
        _codecvt: 0 as * mut crate::_IO_codecvt,
        _wide_data: 0 as * mut crate::_IO_wide_data,
        _freeres_list: core::ptr::null_mut(),
        _freeres_buf: 0 as * mut u8,
        __pad5: std::os::raw::c_ulong::default(),
        _mode: std::os::raw::c_int::default(),
        _unused2: [std::os::raw::c_char::default(); 20]
        }
    }
}

pub type _IO_lock_t = ();
pub type FILE = crate::_IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cursor_tag {
    pub x: std::os::raw::c_double,
    pub y: std::os::raw::c_double,
    pub angle: std::os::raw::c_int,
}
impl std::default::Default for cursor_tag {
    fn default() -> Self {
        cursor_tag {
        x: std::os::raw::c_double::default(),
        y: std::os::raw::c_double::default(),
        angle: std::os::raw::c_int::default()
        }
    }
}

pub type cursor_t = crate::cursor_tag;
#[no_mangle]
pub unsafe extern "C" fn turn<'a1>(mut cursor: Option<&'a1 mut crate::cursor_tag>, mut angle: std::os::raw::c_int) {
    (*borrow_mut(&mut cursor).unwrap()).angle = ((*borrow(& cursor).unwrap()).angle + angle) % 360 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn draw_line<'a1>(
    mut out: * mut crate::_IO_FILE,
    mut cursor: Option<&'a1 mut crate::cursor_tag>,
    mut length: std::os::raw::c_double,
) {
    let mut theta: std::os::raw::c_double = 3.14159265358979323846f64
        * (*borrow(& cursor).unwrap()).angle as std::os::raw::c_double / 180.0f64;
    (*borrow_mut(&mut cursor).unwrap()).x += length * cos(theta);
    (*borrow_mut(&mut cursor).unwrap()).y += length * sin(theta);
    fprintf(
        out,
        b"L%g,%g\n\0" as *const u8 as *const std::os::raw::c_char,
        (*borrow_mut(&mut cursor).unwrap()).x,
        (*borrow_mut(&mut cursor).unwrap()).y,
    );
}
#[no_mangle]
pub unsafe extern "C" fn curve<'a1>(
    mut out: * mut crate::_IO_FILE,
    mut order: std::os::raw::c_int,
    mut length: std::os::raw::c_double,
    mut cursor: Option<&'a1 mut crate::cursor_tag>,
    mut angle: std::os::raw::c_int,
) {
    if order == 0 as std::os::raw::c_int {
        draw_line(out, borrow_mut(&mut cursor), length);
    } else {
        curve(
            out,
            order - 1 as std::os::raw::c_int,
            length / 2 as std::os::raw::c_int as std::os::raw::c_double,
            borrow_mut(&mut cursor),
            -angle,
        );
        turn(borrow_mut(&mut cursor), angle);
        curve(
            out,
            order - 1 as std::os::raw::c_int,
            length / 2 as std::os::raw::c_int as std::os::raw::c_double,
            borrow_mut(&mut cursor),
            angle,
        );
        turn(borrow_mut(&mut cursor), angle);
        curve(
            out,
            order - 1 as std::os::raw::c_int,
            length / 2 as std::os::raw::c_int as std::os::raw::c_double,
            borrow_mut(&mut cursor),
            -angle,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn write_sierpinski_arrowhead(
    mut out: *mut FILE,
    mut size: std::os::raw::c_int,
    mut order: std::os::raw::c_int,
) {
    let margin: std::os::raw::c_double = 20.0f64;
    let side: std::os::raw::c_double = size as std::os::raw::c_double - 2.0f64 * margin;
    let mut cursor: cursor_t = cursor_t { x: 0., y: 0., angle: 0 };
    cursor.angle = 0 as std::os::raw::c_int;
    cursor.x = margin;
    cursor
        .y = 0.5f64 * size as std::os::raw::c_double
        + 0.25f64 * sqrt(3 as std::os::raw::c_int as std::os::raw::c_double) * side;
    if order & 1 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        turn(Some(&mut cursor), -(60 as std::os::raw::c_int));
    }
    fprintf(
        out,
        b"<svg xmlns='http://www.w3.org/2000/svg' width='%d' height='%d'>\n\0"
            as *const u8 as *const std::os::raw::c_char,
        size,
        size,
    );
    fprintf(
        out,
        b"<rect width='100%%' height='100%%' fill='white'/>\n\0" as *const u8
            as *const std::os::raw::c_char,
    );
    fprintf(
        out,
        b"<path stroke-width='1' stroke='black' fill='none' d='\0" as *const u8
            as *const std::os::raw::c_char,
    );
    fprintf(out, b"M%g,%g\n\0" as *const u8 as *const std::os::raw::c_char, cursor.x, cursor.y);
    curve(out, order, side, Some(&mut cursor), 60 as std::os::raw::c_int);
    fprintf(out, b"'/>\n</svg>\n\0" as *const u8 as *const std::os::raw::c_char);
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut filename: *const std::os::raw::c_char = b"sierpinski_arrowhead.svg\0" as *const u8
        as *const std::os::raw::c_char;
    if argc == 2 as std::os::raw::c_int {
        filename = *argv.offset(1 as std::os::raw::c_int as isize);
    }
    let mut out: *mut FILE = fopen(filename, b"w\0" as *const u8 as *const std::os::raw::c_char);
    if out.is_null() {
        perror(filename);
        return 1 as std::os::raw::c_int;
    }
    write_sierpinski_arrowhead(out, 600 as std::os::raw::c_int, 8 as std::os::raw::c_int);
    fclose(out);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as std::os::raw::c_int,
                args.as_mut_ptr() as *mut *mut std::os::raw::c_char,
            ) as i32,
        )
    }
}
