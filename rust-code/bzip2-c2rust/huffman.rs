#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn BZ2_bz__AssertH__fail(errcode: libc::c_int);
}
pub type Bool = libc::c_uchar;
pub type UChar = libc::c_uchar;
pub type Int32 = libc::c_int;
#[inline]
unsafe extern "C" fn WEIGHTOF(mut zz0: Int32) -> Int32 {
    return (zz0 as libc::c_uint & 0xffffff00 as libc::c_uint) as Int32;
}
#[inline]
unsafe extern "C" fn DEPTHOF(mut zz1: Int32) -> Int32 {
    return zz1 & 0xff as libc::c_int;
}
#[inline]
unsafe extern "C" fn MYMAX(mut zz2: Int32, mut zz3: Int32) -> Int32 {
    return if zz2 > zz3 { zz2 } else { zz3 };
}
#[inline]
unsafe extern "C" fn ADDWEIGHTS(mut zw1: Int32, mut zw2: Int32) -> Int32 {
    return WEIGHTOF(zw1) + WEIGHTOF(zw2)
        | 1 as libc::c_int + MYMAX(DEPTHOF(zw1), DEPTHOF(zw2));
}
#[inline]
unsafe extern "C" fn UPHEAP(mut z: Int32, mut heap: *mut Int32, mut weight: *mut Int32) {
    let mut zz: Int32 = z;
    let mut tmp: Int32 = *heap.offset(zz as isize);
    while *weight.offset(tmp as isize)
        < *weight.offset(*heap.offset((zz >> 1 as libc::c_int) as isize) as isize)
    {
        *heap.offset(zz as isize) = *heap.offset((zz >> 1 as libc::c_int) as isize);
        zz >>= 1 as libc::c_int;
    }
    *heap.offset(zz as isize) = tmp;
}
#[inline]
unsafe extern "C" fn DOWNHEAP(
    mut z: Int32,
    mut heap: *mut Int32,
    mut weight: *mut Int32,
    mut nHeap: Int32,
) {
    let mut zz: Int32 = z;
    let mut yy: Int32 = 0;
    let mut tmp: Int32 = *heap.offset(zz as isize);
    while 1 as libc::c_int as Bool != 0 {
        yy = zz << 1 as libc::c_int;
        if yy > nHeap {
            break;
        }
        if yy < nHeap
            && *weight.offset(*heap.offset((yy + 1 as libc::c_int) as isize) as isize)
                < *weight.offset(*heap.offset(yy as isize) as isize)
        {
            yy += 1;
            yy;
        }
        if *weight.offset(tmp as isize)
            < *weight.offset(*heap.offset(yy as isize) as isize)
        {
            break;
        }
        *heap.offset(zz as isize) = *heap.offset(yy as isize);
        zz = yy;
    }
    *heap.offset(zz as isize) = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_hbMakeCodeLengths(
    mut len: *mut UChar,
    mut freq: *mut Int32,
    mut alphaSize: Int32,
    mut maxLen: Int32,
) {
    let mut nNodes: Int32 = 0;
    let mut nHeap: Int32 = 0;
    let mut n1: Int32 = 0;
    let mut n2: Int32 = 0;
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut k: Int32 = 0;
    let mut tooLong: Bool = 0;
    let mut heap: [Int32; 260] = [0; 260];
    let mut weight: [Int32; 516] = [0; 516];
    let mut parent: [Int32; 516] = [0; 516];
    i = 0 as libc::c_int;
    while i < alphaSize {
        weight[(i + 1 as libc::c_int)
            as usize] = (if *freq.offset(i as isize) == 0 as libc::c_int {
            1 as libc::c_int
        } else {
            *freq.offset(i as isize)
        }) << 8 as libc::c_int;
        i += 1;
        i;
    }
    while 1 as libc::c_int as Bool != 0 {
        nNodes = alphaSize;
        nHeap = 0 as libc::c_int;
        heap[0 as libc::c_int as usize] = 0 as libc::c_int;
        weight[0 as libc::c_int as usize] = 0 as libc::c_int;
        parent[0 as libc::c_int as usize] = -(2 as libc::c_int);
        i = 1 as libc::c_int;
        while i <= alphaSize {
            parent[i as usize] = -(1 as libc::c_int);
            nHeap += 1;
            nHeap;
            heap[nHeap as usize] = i;
            UPHEAP(nHeap, heap.as_mut_ptr(), weight.as_mut_ptr());
            i += 1;
            i;
        }
        if !(nHeap < 258 as libc::c_int + 2 as libc::c_int) {
            BZ2_bz__AssertH__fail(2001 as libc::c_int);
        }
        while nHeap > 1 as libc::c_int {
            n1 = heap[1 as libc::c_int as usize];
            heap[1 as libc::c_int as usize] = heap[nHeap as usize];
            nHeap -= 1;
            nHeap;
            DOWNHEAP(1 as libc::c_int, heap.as_mut_ptr(), weight.as_mut_ptr(), nHeap);
            n2 = heap[1 as libc::c_int as usize];
            heap[1 as libc::c_int as usize] = heap[nHeap as usize];
            nHeap -= 1;
            nHeap;
            DOWNHEAP(1 as libc::c_int, heap.as_mut_ptr(), weight.as_mut_ptr(), nHeap);
            nNodes += 1;
            nNodes;
            parent[n2 as usize] = nNodes;
            parent[n1 as usize] = parent[n2 as usize];
            weight[nNodes
                as usize] = ADDWEIGHTS(weight[n1 as usize], weight[n2 as usize]);
            parent[nNodes as usize] = -(1 as libc::c_int);
            nHeap += 1;
            nHeap;
            heap[nHeap as usize] = nNodes;
            UPHEAP(nHeap, heap.as_mut_ptr(), weight.as_mut_ptr());
        }
        if !(nNodes < 258 as libc::c_int * 2 as libc::c_int) {
            BZ2_bz__AssertH__fail(2002 as libc::c_int);
        }
        tooLong = 0 as libc::c_int as Bool;
        i = 1 as libc::c_int;
        while i <= alphaSize {
            j = 0 as libc::c_int;
            k = i;
            while parent[k as usize] >= 0 as libc::c_int {
                k = parent[k as usize];
                j += 1;
                j;
            }
            *len.offset((i - 1 as libc::c_int) as isize) = j as UChar;
            if j > maxLen {
                tooLong = 1 as libc::c_int as Bool;
            }
            i += 1;
            i;
        }
        if tooLong == 0 {
            break;
        }
        i = 1 as libc::c_int;
        while i <= alphaSize {
            j = weight[i as usize] >> 8 as libc::c_int;
            j = 1 as libc::c_int + j / 2 as libc::c_int;
            weight[i as usize] = j << 8 as libc::c_int;
            i += 1;
            i;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_hbAssignCodes(
    mut code: *mut Int32,
    mut length: *mut UChar,
    mut minLen: Int32,
    mut maxLen: Int32,
    mut alphaSize: Int32,
) {
    let mut n: Int32 = 0;
    let mut vec: Int32 = 0;
    let mut i: Int32 = 0;
    vec = 0 as libc::c_int;
    n = minLen;
    while n <= maxLen {
        i = 0 as libc::c_int;
        while i < alphaSize {
            if *length.offset(i as isize) as libc::c_int == n {
                *code.offset(i as isize) = vec;
                vec += 1;
                vec;
            }
            i += 1;
            i;
        }
        vec <<= 1 as libc::c_int;
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_hbCreateDecodeTables(
    mut limit: *mut Int32,
    mut base: *mut Int32,
    mut perm: *mut Int32,
    mut length: *mut UChar,
    mut minLen: Int32,
    mut maxLen: Int32,
    mut alphaSize: Int32,
) {
    let mut pp: Int32 = 0;
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut vec: Int32 = 0;
    pp = 0 as libc::c_int;
    i = minLen;
    while i <= maxLen {
        j = 0 as libc::c_int;
        while j < alphaSize {
            if *length.offset(j as isize) as libc::c_int == i {
                *perm.offset(pp as isize) = j;
                pp += 1;
                pp;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 23 as libc::c_int {
        *base.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < alphaSize {
        let ref mut fresh0 = *base
            .offset(
                (*length.offset(i as isize) as libc::c_int + 1 as libc::c_int) as isize,
            );
        *fresh0 += 1;
        *fresh0;
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i < 23 as libc::c_int {
        let ref mut fresh1 = *base.offset(i as isize);
        *fresh1 += *base.offset((i - 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 23 as libc::c_int {
        *limit.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    vec = 0 as libc::c_int;
    i = minLen;
    while i <= maxLen {
        vec += *base.offset((i + 1 as libc::c_int) as isize) - *base.offset(i as isize);
        *limit.offset(i as isize) = vec - 1 as libc::c_int;
        vec <<= 1 as libc::c_int;
        i += 1;
        i;
    }
    i = minLen + 1 as libc::c_int;
    while i <= maxLen {
        *base
            .offset(
                i as isize,
            ) = ((*limit.offset((i - 1 as libc::c_int) as isize) + 1 as libc::c_int)
            << 1 as libc::c_int) - *base.offset(i as isize);
        i += 1;
        i;
    }
}
