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
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
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
pub struct squareMtxStruct {
    pub dim: std::os::raw::c_int,
    pub cells: *mut std::os::raw::c_double,
    pub m: *mut *mut std::os::raw::c_double,
}
impl std::default::Default for squareMtxStruct {
    fn default() -> Self {
        squareMtxStruct {
        dim: std::os::raw::c_int::default(),
        cells: 0 as * mut std::os::raw::c_double,
        m: core::ptr::null_mut()
        }
    }
}

pub type SquareMtx = * mut crate::squareMtxStruct;
pub type FillFunc = Option<unsafe extern "C"  fn(_: * mut std::os::raw::c_double,_: std::os::raw::c_int,_: std::os::raw::c_int,_: * mut u8,) -> ()>;
#[no_mangle]
pub unsafe extern "C" fn NewSquareMtx(
    mut dim: std::os::raw::c_int,
    mut fillFunc: FillFunc,
    mut ff_data: *mut u8,
) -> SquareMtx {
    let mut sm: SquareMtx = malloc(
        std::mem::size_of::<squareMtxStruct>() as std::os::raw::c_ulong,
    ) as SquareMtx;
    if !sm.is_null() {
        let mut rw: std::os::raw::c_int = 0;
        (*sm).dim = dim;
        (*sm)
            .cells = malloc(
            ((dim * dim) as std::os::raw::c_ulong)
                .wrapping_mul(std::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong),
        ) as *mut std::os::raw::c_double;
        (*sm)
            .m = malloc(
            (dim as std::os::raw::c_ulong)
                .wrapping_mul(
                    std::mem::size_of::<*mut std::os::raw::c_double>() as std::os::raw::c_ulong,
                ),
        ) as *mut *mut std::os::raw::c_double;
        if !((*sm).cells).is_null() && !((*sm).m).is_null() {
            rw = 0 as std::os::raw::c_int;
            while rw < dim {
                let ref mut fresh0 = *((*sm).m).offset(rw as isize);
                *fresh0 = ((*sm).cells).offset((dim * rw) as isize);
                fillFunc
                    .expect(
                        "non-null function pointer",
                    )(*((*sm).m).offset(rw as isize), rw, dim, ff_data);
                rw += 1;
                rw;
            }
        } else {
            free((*sm).m as *mut u8);
            free((*sm).cells as *mut u8);
            free(sm as *mut u8);
            printf(
                b"Square Matrix allocation failure\n\0" as *const u8
                    as *const std::os::raw::c_char,
            );
            return 0 as SquareMtx;
        }
    } else {
        printf(
            b"Malloc failed for square matrix\n\0" as *const u8 as *const std::os::raw::c_char,
        );
    }
    return sm;
}
#[no_mangle]
pub unsafe extern "C" fn ffMatxSquare(
    mut cells: *mut std::os::raw::c_double,
    mut rw: std::os::raw::c_int,
    mut dim: std::os::raw::c_int,
    mut m0: SquareMtx,
) {
    let mut col: std::os::raw::c_int = 0;
    let mut ix: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_double = 0.;
    let mut m0rw: *mut std::os::raw::c_double = *((*m0).m).offset(rw as isize);
    col = 0 as std::os::raw::c_int;
    while col < dim {
        sum = 0.0f64;
        ix = 0 as std::os::raw::c_int;
        while ix < dim {
            sum
                += *m0rw.offset(ix as isize)
                    * *(*((*m0).m).offset(ix as isize)).offset(col as isize);
            ix += 1;
            ix;
        }
        *cells.offset(col as isize) = sum;
        col += 1;
        col;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ffMatxMulply(
    mut cells: *mut std::os::raw::c_double,
    mut rw: std::os::raw::c_int,
    mut dim: std::os::raw::c_int,
    mut mplcnds: *mut SquareMtx,
) {
    let mut mleft: SquareMtx = *mplcnds.offset(0 as std::os::raw::c_int as isize);
    let mut mrigt: SquareMtx = *mplcnds.offset(1 as std::os::raw::c_int as isize);
    let mut sum: std::os::raw::c_double = 0.;
    let mut m0rw: *mut std::os::raw::c_double = *((*mleft).m).offset(rw as isize);
    let mut col: std::os::raw::c_int = 0;
    let mut ix: std::os::raw::c_int = 0;
    col = 0 as std::os::raw::c_int;
    while col < dim {
        sum = 0.0f64;
        ix = 0 as std::os::raw::c_int;
        while ix < dim {
            sum
                += *m0rw.offset(ix as isize)
                    * *(*((*mrigt).m).offset(ix as isize)).offset(col as isize);
            ix += 1;
            ix;
        }
        *cells.offset(col as isize) = sum;
        col += 1;
        col;
    }
}
#[no_mangle]
pub unsafe extern "C" fn MatxMul(
    mut mr: SquareMtx,
    mut left: SquareMtx,
    mut rigt: SquareMtx,
) {
    let mut rw: std::os::raw::c_int = 0;
    let mut mplcnds: [SquareMtx; 2] = [core::ptr::null_mut(); 2];
    mplcnds[0 as std::os::raw::c_int as usize] = left;
    mplcnds[1 as std::os::raw::c_int as usize] = rigt;
    rw = 0 as std::os::raw::c_int;
    while rw < (*left).dim {
        ffMatxMulply(
            *((*mr).m).offset(rw as isize),
            rw,
            (*left).dim,
            mplcnds.as_mut_ptr(),
        );
        rw += 1;
        rw;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ffIdentity(
    mut cells: *mut std::os::raw::c_double,
    mut rw: std::os::raw::c_int,
    mut dim: std::os::raw::c_int,
    mut v: *mut u8,
) {
    let mut col: std::os::raw::c_int = 0;
    col = 0 as std::os::raw::c_int;
    while col < dim {
        *cells.offset(col as isize) = 0.0f64;
        col += 1;
        col;
    }
    *cells.offset(rw as isize) = 1.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn ffCopy(
    mut cells: *mut std::os::raw::c_double,
    mut rw: std::os::raw::c_int,
    mut dim: std::os::raw::c_int,
    mut m1: SquareMtx,
) {
    let mut col: std::os::raw::c_int = 0;
    col = 0 as std::os::raw::c_int;
    while col < dim {
        *cells
            .offset(
                col as isize,
            ) = *(*((*m1).m).offset(rw as isize)).offset(col as isize);
        col += 1;
        col;
    }
}
#[no_mangle]
pub unsafe extern "C" fn FreeSquareMtx(mut m: SquareMtx) {
    free((*m).m as *mut u8);
    free((*m).cells as *mut u8);
    free(m as *mut u8);
}
#[no_mangle]
pub unsafe extern "C" fn SquareMtxPow(
    mut m0: SquareMtx,
    mut exp: std::os::raw::c_int,
) -> SquareMtx {
    let mut v0: SquareMtx = NewSquareMtx(
        (*m0).dim,
        Some(
            ffIdentity
                as unsafe extern "C" fn(
                    *mut std::os::raw::c_double,
                    std::os::raw::c_int,
                    std::os::raw::c_int,
                    *mut u8,
                ) -> (),
        ),
        0 as *mut u8,
    );
    let mut v1: SquareMtx = 0 as SquareMtx;
    let mut base0: SquareMtx = NewSquareMtx(
        (*m0).dim,
        std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut std::os::raw::c_double,
                    std::os::raw::c_int,
                    std::os::raw::c_int,
                    SquareMtx,
                ) -> (),
            >,
            FillFunc,
        >(
            Some(
                ffCopy
                    as unsafe extern "C" fn(
                        *mut std::os::raw::c_double,
                        std::os::raw::c_int,
                        std::os::raw::c_int,
                        SquareMtx,
                    ) -> (),
            ),
        ),
        m0 as *mut u8,
    );
    let mut base1: SquareMtx = 0 as SquareMtx;
    let mut mplcnds: [SquareMtx; 2] = [core::ptr::null_mut(); 2];
    let mut t: SquareMtx = 0 as *mut squareMtxStruct;
    while exp != 0 {
        if exp % 2 as std::os::raw::c_int != 0 {
            if !v1.is_null() {
                MatxMul(v1, v0, base0);
            } else {
                mplcnds[0 as std::os::raw::c_int as usize] = v0;
                mplcnds[1 as std::os::raw::c_int as usize] = base0;
                v1 = NewSquareMtx(
                    (*m0).dim,
                    std::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *mut std::os::raw::c_double,
                                std::os::raw::c_int,
                                std::os::raw::c_int,
                                *mut SquareMtx,
                            ) -> (),
                        >,
                        FillFunc,
                    >(
                        Some(
                            ffMatxMulply
                                as unsafe extern "C" fn(
                                    *mut std::os::raw::c_double,
                                    std::os::raw::c_int,
                                    std::os::raw::c_int,
                                    *mut SquareMtx,
                                ) -> (),
                        ),
                    ),
                    mplcnds.as_mut_ptr() as *mut u8,
                );
            }
            t = v0;
            v0 = v1;
            v1 = t;
        }
        if !base1.is_null() {
            MatxMul(base1, base0, base0);
        } else {
            base1 = NewSquareMtx(
                (*m0).dim,
                std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut std::os::raw::c_double,
                            std::os::raw::c_int,
                            std::os::raw::c_int,
                            SquareMtx,
                        ) -> (),
                    >,
                    FillFunc,
                >(
                    Some(
                        ffMatxSquare
                            as unsafe extern "C" fn(
                                *mut std::os::raw::c_double,
                                std::os::raw::c_int,
                                std::os::raw::c_int,
                                SquareMtx,
                            ) -> (),
                    ),
                ),
                base0 as *mut u8,
            );
        }
        t = base0;
        base0 = base1;
        base1 = t;
        exp = exp / 2 as std::os::raw::c_int;
    }
    if !base0.is_null() {
        FreeSquareMtx(base0);
    }
    if !base1.is_null() {
        FreeSquareMtx(base1);
    }
    if !v1.is_null() {
        FreeSquareMtx(v1);
    }
    return v0;
}
#[no_mangle]
pub static mut fout: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub unsafe extern "C" fn SquareMtxPrint(
    mut mtx: SquareMtx,
    mut mn: *const std::os::raw::c_char,
) {
    let mut rw: std::os::raw::c_int = 0;
    let mut col: std::os::raw::c_int = 0;
    let mut d: std::os::raw::c_int = (*mtx).dim;
    fprintf(
        fout,
        b"%s dim:%d =\n\0" as *const u8 as *const std::os::raw::c_char,
        mn,
        (*mtx).dim,
    );
    rw = 0 as std::os::raw::c_int;
    while rw < d {
        fprintf(fout, b" |\0" as *const u8 as *const std::os::raw::c_char);
        col = 0 as std::os::raw::c_int;
        while col < d {
            fprintf(
                fout,
                b"%8.5f \0" as *const u8 as *const std::os::raw::c_char,
                *(*((*mtx).m).offset(rw as isize)).offset(col as isize),
            );
            col += 1;
            col;
        }
        fprintf(fout, b" |\n\0" as *const u8 as *const std::os::raw::c_char);
        rw += 1;
        rw;
    }
    fprintf(fout, b"\n\0" as *const u8 as *const std::os::raw::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn fillInit(
    mut cells: *mut std::os::raw::c_double,
    mut rw: std::os::raw::c_int,
    mut dim: std::os::raw::c_int,
    mut data: *mut u8,
) {
    let mut theta: std::os::raw::c_double = 3.1415926536f64 / 6.0f64;
    let mut c1: std::os::raw::c_double = cos(theta);
    let mut s1: std::os::raw::c_double = sin(theta);
    match rw {
        0 => {
            *cells.offset(0 as std::os::raw::c_int as isize) = c1;
            *cells.offset(1 as std::os::raw::c_int as isize) = s1;
            *cells.offset(2 as std::os::raw::c_int as isize) = 0.0f64;
        }
        1 => {
            *cells.offset(0 as std::os::raw::c_int as isize) = -s1;
            *cells.offset(1 as std::os::raw::c_int as isize) = c1;
            *cells
                .offset(2 as std::os::raw::c_int as isize) = 0 as std::os::raw::c_int as std::os::raw::c_double;
        }
        2 => {
            *cells.offset(0 as std::os::raw::c_int as isize) = 0.0f64;
            *cells.offset(1 as std::os::raw::c_int as isize) = 0.0f64;
            *cells.offset(2 as std::os::raw::c_int as isize) = 1.0f64;
        }
        _ => {}
    };
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut m0: SquareMtx = NewSquareMtx(
        3 as std::os::raw::c_int,
        Some(
            fillInit
                as unsafe extern "C" fn(
                    *mut std::os::raw::c_double,
                    std::os::raw::c_int,
                    std::os::raw::c_int,
                    *mut u8,
                ) -> (),
        ),
        0 as *mut u8,
    );
    let mut m1: SquareMtx = SquareMtxPow(m0, 5 as std::os::raw::c_int);
    let mut m2: SquareMtx = SquareMtxPow(m0, 9 as std::os::raw::c_int);
    let mut m3: SquareMtx = SquareMtxPow(m0, 2 as std::os::raw::c_int);
    fout = fopen(
        b"matrx_exp.txt\0" as *const u8 as *const std::os::raw::c_char,
        b"w\0" as *const u8 as *const std::os::raw::c_char,
    );
    SquareMtxPrint(m0, b"m0\0" as *const u8 as *const std::os::raw::c_char);
    FreeSquareMtx(m0);
    SquareMtxPrint(m1, b"m0^5\0" as *const u8 as *const std::os::raw::c_char);
    FreeSquareMtx(m1);
    SquareMtxPrint(m2, b"m0^9\0" as *const u8 as *const std::os::raw::c_char);
    FreeSquareMtx(m2);
    SquareMtxPrint(m3, b"m0^2\0" as *const u8 as *const std::os::raw::c_char);
    FreeSquareMtx(m3);
    fclose(fout);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
