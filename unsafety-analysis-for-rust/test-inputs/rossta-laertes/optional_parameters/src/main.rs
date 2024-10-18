#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic, extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn qsort(
        __base: *mut u8,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    static mut stdout: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn __ctype_b_loc() -> *mut *const std::os::raw::c_ushort;
    fn tolower(_: std::os::raw::c_int) -> std::os::raw::c_int;
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: std::os::raw::c_uint,
    pub fp_offset: std::os::raw::c_uint,
    pub overflow_arg_area: *mut u8,
    pub reg_save_area: *mut u8,
}
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const u8, *const u8) -> std::os::raw::c_int,
>;
pub type va_list = __builtin_va_list;
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
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type String_0 = *const std::os::raw::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sTable {
    pub rows: *mut *mut String_0,
    pub n_rows: std::os::raw::c_int,
    pub n_cols: std::os::raw::c_int,
}
pub type Table = *mut sTable;
pub type CompareFctn = Option::<unsafe extern "C" fn(String_0, String_0) -> std::os::raw::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub compare: CompareFctn,
    pub column: std::os::raw::c_int,
    pub reversed: std::os::raw::c_int,
}
#[no_mangle]
pub static mut sortSpec: C2RustUnnamed_0 = C2RustUnnamed_0 {
    compare: None,
    column: 0,
    reversed: 0,
};
#[no_mangle]
pub unsafe extern "C" fn CmprRows(
    mut aa: *const u8,
    mut bb: *const u8,
) -> std::os::raw::c_int {
    let mut rA: *mut String_0 = *(aa as *const *mut String_0);
    let mut rB: *mut String_0 = *(bb as *const *mut String_0);
    let mut sortCol: std::os::raw::c_int = sortSpec.column;
    let mut left: String_0 = if sortSpec.reversed != 0 {
        *rB.offset(sortCol as isize)
    } else {
        *rA.offset(sortCol as isize)
    };
    let mut right: String_0 = if sortSpec.reversed != 0 {
        *rA.offset(sortCol as isize)
    } else {
        *rB.offset(sortCol as isize)
    };
    return (sortSpec.compare).expect("non-null function pointer")(left, right);
}
#[no_mangle]
pub unsafe extern "C" fn sortTable(
    mut tbl: Table,
    mut argSpec: *const std::os::raw::c_char,
    mut args: ...
) -> std::os::raw::c_int {
    let mut vl: std::ffi::VaListImpl;
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut c: std::os::raw::c_int = 0;
    sortSpec
        .compare = Some(
        strcmp
            as unsafe extern "C" fn(
                *const std::os::raw::c_char,
                *const std::os::raw::c_char,
            ) -> std::os::raw::c_int,
    );
    sortSpec.column = 0 as std::os::raw::c_int;
    sortSpec.reversed = 0 as std::os::raw::c_int;
    vl = args.clone();
    if !argSpec.is_null() {
        p = argSpec;
        while *p != 0 {
            match *p as std::os::raw::c_int {
                111 => {
                    sortSpec
                        .compare = std::mem::transmute(
                        vl
                            .arg::<
                                *mut unsafe extern "C" fn(String_0, String_0) -> std::os::raw::c_int,
                            >(),
                    );
                }
                99 => {
                    c = vl.arg::<std::os::raw::c_int>();
                    if 0 as std::os::raw::c_int <= c && c < (*tbl).n_cols {
                        sortSpec.column = c;
                    }
                }
                114 => {
                    sortSpec
                        .reversed = (0 as std::os::raw::c_int != vl.arg::<std::os::raw::c_int>())
                        as std::os::raw::c_int;
                }
                _ => {}
            }
            p = p.offset(1);
            p;
        }
    }
    qsort(
        (*tbl).rows as *mut u8,
        (*tbl).n_rows as size_t,
        std::mem::size_of::<*mut String_0>() as std::os::raw::c_ulong,
        Some(
            CmprRows
                as unsafe extern "C" fn(
                    *const u8,
                    *const u8,
                ) -> std::os::raw::c_int,
        ),
    );
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn printTable(
    mut tbl: Table,
    mut fout: *mut FILE,
    mut colFmts: *mut *const std::os::raw::c_char,
) {
    let mut row: std::os::raw::c_int = 0;
    let mut col: std::os::raw::c_int = 0;
    row = 0 as std::os::raw::c_int;
    while row < (*tbl).n_rows {
        fprintf(fout, b"   \0" as *const u8 as *const std::os::raw::c_char);
        col = 0 as std::os::raw::c_int;
        while col < (*tbl).n_cols {
            fprintf(
                fout,
                *colFmts.offset(col as isize),
                *(*((*tbl).rows).offset(row as isize)).offset(col as isize),
            );
            col += 1;
            col;
        }
        fprintf(fout, b"\n\0" as *const u8 as *const std::os::raw::c_char);
        row += 1;
        row;
    }
    fprintf(fout, b"\n\0" as *const u8 as *const std::os::raw::c_char);
}
#[no_mangle]
pub extern "C" fn ord(mut v: std::os::raw::c_char) -> std::os::raw::c_int {
    return v as std::os::raw::c_int - '0' as i32;
}
#[no_mangle]
pub unsafe extern "C" fn cmprStrgs(mut s1: String_0, mut s2: String_0) -> std::os::raw::c_int {
    let mut p1: *const std::os::raw::c_char = s1;
    let mut p2: *const std::os::raw::c_char = s2;
    let mut mrk1: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut mrk2: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    while tolower(*p1 as std::os::raw::c_int) == tolower(*p2 as std::os::raw::c_int)
        && *p1 as std::os::raw::c_int != 0
    {
        p1 = p1.offset(1);
        p1;
        p2 = p2.offset(1);
        p2;
    }
    if *(*__ctype_b_loc()).offset(*p1 as std::os::raw::c_int as isize) as std::os::raw::c_int
        & _ISdigit as std::os::raw::c_int as std::os::raw::c_ushort as std::os::raw::c_int != 0
        && *(*__ctype_b_loc()).offset(*p2 as std::os::raw::c_int as isize) as std::os::raw::c_int
            & _ISdigit as std::os::raw::c_int as std::os::raw::c_ushort as std::os::raw::c_int != 0
    {
        let mut v1: std::os::raw::c_long = 0;
        let mut v2: std::os::raw::c_long = 0;
        if *p1 as std::os::raw::c_int == '0' as i32 || *p2 as std::os::raw::c_int == '0' as i32 {
            while p1 > s1 {
                p1 = p1.offset(-1);
                p1;
                p2 = p2.offset(-1);
                p2;
                if *p1 as std::os::raw::c_int != '0' as i32 {
                    break;
                }
            }
            if *(*__ctype_b_loc()).offset(*p1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                & _ISdigit as std::os::raw::c_int as std::os::raw::c_ushort as std::os::raw::c_int == 0
            {
                p1 = p1.offset(1);
                p1;
                p2 = p2.offset(1);
                p2;
            }
        }
        mrk1 = p1;
        mrk2 = p2;
        v1 = 0 as std::os::raw::c_int as std::os::raw::c_long;
        while *(*__ctype_b_loc()).offset(*p1 as std::os::raw::c_int as isize) as std::os::raw::c_int
            & _ISdigit as std::os::raw::c_int as std::os::raw::c_ushort as std::os::raw::c_int != 0
        {
            v1 = 10 as std::os::raw::c_int as std::os::raw::c_long * v1 + ord(*p1) as std::os::raw::c_long;
            p1 = p1.offset(1);
            p1;
        }
        v2 = 0 as std::os::raw::c_int as std::os::raw::c_long;
        while *(*__ctype_b_loc()).offset(*p2 as std::os::raw::c_int as isize) as std::os::raw::c_int
            & _ISdigit as std::os::raw::c_int as std::os::raw::c_ushort as std::os::raw::c_int != 0
        {
            v2 = 10 as std::os::raw::c_int as std::os::raw::c_long * v2 + ord(*p2) as std::os::raw::c_long;
            p2 = p2.offset(1);
            p2;
        }
        if v1 == v2 {
            return (p2.offset_from(mrk2) as std::os::raw::c_long
                - p1.offset_from(mrk1) as std::os::raw::c_long) as std::os::raw::c_int;
        }
        return (v1 - v2) as std::os::raw::c_int;
    }
    if tolower(*p1 as std::os::raw::c_int) != tolower(*p2 as std::os::raw::c_int) {
        return tolower(*p1 as std::os::raw::c_int) - tolower(*p2 as std::os::raw::c_int);
    }
    p1 = s1;
    p2 = s2;
    while *p1 as std::os::raw::c_int == *p2 as std::os::raw::c_int && *p1 as std::os::raw::c_int != 0 {
        p1 = p1.offset(1);
        p1;
        p2 = p2.offset(1);
        p2;
    }
    return *p1 as std::os::raw::c_int - *p2 as std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut colFmts: [*const std::os::raw::c_char; 3] = [
        b" %-5.5s\0" as *const u8 as *const std::os::raw::c_char,
        b" %-5.5s\0" as *const u8 as *const std::os::raw::c_char,
        b" %-9.9s\0" as *const u8 as *const std::os::raw::c_char,
    ];
    let mut r1: [String_0; 3] = [
        b"a101\0" as *const u8 as *const std::os::raw::c_char,
        b"red\0" as *const u8 as *const std::os::raw::c_char,
        b"Java\0" as *const u8 as *const std::os::raw::c_char,
    ];
    let mut r2: [String_0; 3] = [
        b"ab40\0" as *const u8 as *const std::os::raw::c_char,
        b"gren\0" as *const u8 as *const std::os::raw::c_char,
        b"Smalltalk\0" as *const u8 as *const std::os::raw::c_char,
    ];
    let mut r3: [String_0; 3] = [
        b"ab9\0" as *const u8 as *const std::os::raw::c_char,
        b"blue\0" as *const u8 as *const std::os::raw::c_char,
        b"Fortran\0" as *const u8 as *const std::os::raw::c_char,
    ];
    let mut r4: [String_0; 3] = [
        b"ab09\0" as *const u8 as *const std::os::raw::c_char,
        b"ylow\0" as *const u8 as *const std::os::raw::c_char,
        b"Python\0" as *const u8 as *const std::os::raw::c_char,
    ];
    let mut r5: [String_0; 3] = [
        b"ab1a\0" as *const u8 as *const std::os::raw::c_char,
        b"blak\0" as *const u8 as *const std::os::raw::c_char,
        b"Factor\0" as *const u8 as *const std::os::raw::c_char,
    ];
    let mut r6: [String_0; 3] = [
        b"ab1b\0" as *const u8 as *const std::os::raw::c_char,
        b"brwn\0" as *const u8 as *const std::os::raw::c_char,
        b"C Sharp\0" as *const u8 as *const std::os::raw::c_char,
    ];
    let mut r7: [String_0; 3] = [
        b"Ab1b\0" as *const u8 as *const std::os::raw::c_char,
        b"pink\0" as *const u8 as *const std::os::raw::c_char,
        b"Ruby\0" as *const u8 as *const std::os::raw::c_char,
    ];
    let mut r8: [String_0; 3] = [
        b"ab1\0" as *const u8 as *const std::os::raw::c_char,
        b"orng\0" as *const u8 as *const std::os::raw::c_char,
        b"Scheme\0" as *const u8 as *const std::os::raw::c_char,
    ];
    let mut rows: [*mut String_0; 8] = [
        r1.as_mut_ptr(),
        r2.as_mut_ptr(),
        r3.as_mut_ptr(),
        r4.as_mut_ptr(),
        r5.as_mut_ptr(),
        r6.as_mut_ptr(),
        r7.as_mut_ptr(),
        r8.as_mut_ptr(),
    ];
    let mut table: sTable = sTable {
        rows: 0 as *mut *mut String_0,
        n_rows: 0,
        n_cols: 0,
    };
    table.rows = rows.as_mut_ptr();
    table.n_rows = 8 as std::os::raw::c_int;
    table.n_cols = 3 as std::os::raw::c_int;
    sortTable(&mut table as *mut sTable, b"\0" as *const u8 as *const std::os::raw::c_char);
    printf(b"sort on col 0, ascending\n\0" as *const u8 as *const std::os::raw::c_char);
    printTable(&mut table, stdout, colFmts.as_mut_ptr());
    sortTable(
        &mut table as *mut sTable,
        b"ro\0" as *const u8 as *const std::os::raw::c_char,
        1 as std::os::raw::c_int,
        Some(cmprStrgs as unsafe extern "C" fn(String_0, String_0) -> std::os::raw::c_int),
    );
    printf(b"sort on col 0, reverse.special\n\0" as *const u8 as *const std::os::raw::c_char);
    printTable(&mut table, stdout, colFmts.as_mut_ptr());
    sortTable(
        &mut table as *mut sTable,
        b"c\0" as *const u8 as *const std::os::raw::c_char,
        1 as std::os::raw::c_int,
    );
    printf(b"sort on col 1, ascending\n\0" as *const u8 as *const std::os::raw::c_char);
    printTable(&mut table, stdout, colFmts.as_mut_ptr());
    sortTable(
        &mut table as *mut sTable,
        b"cr\0" as *const u8 as *const std::os::raw::c_char,
        2 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
    );
    printf(b"sort on col 2, reverse\n\0" as *const u8 as *const std::os::raw::c_char);
    printTable(&mut table, stdout, colFmts.as_mut_ptr());
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
