#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn memmove(
        _: *mut u8,
        _: *const u8,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
    fn memset(
        _: *mut u8,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
    fn memcmp(
        _: *const u8,
        _: *const u8,
        _: std::os::raw::c_ulong,
    ) -> std::os::raw::c_int;
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
}
unsafe extern "C" fn _parseDecimal<'a1>(
    mut pchCursor: Option<&'a1 mut * const std::os::raw::c_char>,
) -> std::os::raw::c_uint {
    let mut nVal: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut chNow: std::os::raw::c_char = 0;
    loop {
        chNow = **borrow_mut(&mut pchCursor).unwrap();
        if !(chNow as std::os::raw::c_int >= '0' as i32 && chNow as std::os::raw::c_int <= '9' as i32) {
            break;
        }
        nVal = nVal.wrapping_mul(10 as std::os::raw::c_int as std::os::raw::c_uint);
        nVal = nVal.wrapping_add((chNow as std::os::raw::c_int - '0' as i32) as std::os::raw::c_uint);
        *borrow_mut(&mut pchCursor).unwrap() = (*borrow_mut(&mut pchCursor).unwrap()).offset(1);
        *borrow(& pchCursor).unwrap();
    }
    return nVal;
}
unsafe extern "C" fn _parseHex<'a1>(mut pchCursor: Option<&'a1 mut * const std::os::raw::c_char>) -> std::os::raw::c_uint {
    let mut nVal: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut chNow: std::os::raw::c_char = 0;
    loop {
        chNow = (**borrow(& pchCursor).unwrap() as std::os::raw::c_int & 0x5f as std::os::raw::c_int) as std::os::raw::c_char;
        if !(chNow as std::os::raw::c_int >= '0' as i32 & 0x5f as std::os::raw::c_int
            && chNow as std::os::raw::c_int <= '9' as i32 & 0x5f as std::os::raw::c_int
            || chNow as std::os::raw::c_int >= 'A' as i32 && chNow as std::os::raw::c_int <= 'F' as i32)
        {
            break;
        }
        let mut nybbleValue: std::os::raw::c_uchar = 0;
        chNow = (chNow as std::os::raw::c_int - 0x10 as std::os::raw::c_int) as std::os::raw::c_char;
        nybbleValue = (if chNow as std::os::raw::c_int > 9 as std::os::raw::c_int {
            chNow as std::os::raw::c_int - (0x31 as std::os::raw::c_int - 0xa as std::os::raw::c_int)
        } else {
            chNow as std::os::raw::c_int
        }) as std::os::raw::c_uchar;
        nVal <<= 4 as std::os::raw::c_int;
        nVal = nVal.wrapping_add(nybbleValue as std::os::raw::c_uint);
        *borrow_mut(&mut pchCursor).unwrap() = (*borrow_mut(&mut pchCursor).unwrap()).offset(1);
        *borrow(& pchCursor).unwrap();
    }
    return nVal;
}
#[no_mangle]
pub unsafe extern "C" fn ParseIPv4OrIPv6<'a1, 'a2>(
    mut ppszText: Option<&'a1 mut * const std::os::raw::c_char>,
    mut abyAddr: * mut std::os::raw::c_uchar,
    mut pnPort: * mut std::os::raw::c_int,
    mut pbIsIPv6: Option<&'a2 mut std::os::raw::c_int>,
) -> std::os::raw::c_int {
    let mut abyAddrLocal: *mut std::os::raw::c_uchar = 0 as *mut std::os::raw::c_uchar;
    let mut abyDummyAddr: [std::os::raw::c_uchar; 16] = [0; 16];
    let mut pchColon: *const std::os::raw::c_char = strchr(*borrow(& ppszText).unwrap(), ':' as i32);
    let mut pchDot: *const std::os::raw::c_char = strchr(*borrow(& ppszText).unwrap(), '.' as i32);
    let mut pchOpenBracket: *const std::os::raw::c_char = strchr(*borrow(& ppszText).unwrap(), '[' as i32);
    let mut pchCloseBracket: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut bIsIPv6local: std::os::raw::c_int = (!pchOpenBracket.is_null() || pchDot.is_null()
        || !pchColon.is_null() && (pchDot.is_null() || pchColon < pchDot))
        as std::os::raw::c_int;
    if bIsIPv6local != 0 {
        pchCloseBracket = strchr(*borrow(& ppszText).unwrap(), ']' as i32);
        if !pchOpenBracket.is_null()
            && (pchCloseBracket.is_null() || pchCloseBracket < pchOpenBracket)
        {
            return 0 as std::os::raw::c_int;
        }
    } else if pchDot.is_null() || !pchColon.is_null() && pchColon < pchDot {
        return 0 as std::os::raw::c_int
    }
    if !borrow(& pbIsIPv6).is_none() {
        *borrow_mut(&mut pbIsIPv6).unwrap() = bIsIPv6local;
    }
    abyAddrLocal = abyAddr;
    if abyAddrLocal.is_null() {
        abyAddrLocal = abyDummyAddr.as_mut_ptr();
    }
    if bIsIPv6local == 0 {
        let mut pbyAddrCursor: *mut std::os::raw::c_uchar = abyAddrLocal;
        let mut nVal: std::os::raw::c_uint = 0;
        let mut pszTextBefore: *const std::os::raw::c_char = *borrow_mut(&mut ppszText).unwrap();
        nVal = _parseDecimal(borrow_mut(&mut ppszText));
        if '.' as i32 != **borrow(& ppszText).unwrap() as std::os::raw::c_int
            || nVal > 255 as std::os::raw::c_int as std::os::raw::c_uint || pszTextBefore == *borrow(& ppszText).unwrap()
        {
            return 0 as std::os::raw::c_int;
        }
        let fresh0 = pbyAddrCursor;
        pbyAddrCursor = pbyAddrCursor.offset(1);
        *fresh0 = nVal as std::os::raw::c_uchar;
        *borrow_mut(&mut ppszText).unwrap() = (*borrow_mut(&mut ppszText).unwrap()).offset(1);
        *borrow(& ppszText).unwrap();
        pszTextBefore = *borrow_mut(&mut ppszText).unwrap();
        nVal = _parseDecimal(borrow_mut(&mut ppszText));
        if '.' as i32 != **borrow(& ppszText).unwrap() as std::os::raw::c_int
            || nVal > 255 as std::os::raw::c_int as std::os::raw::c_uint || pszTextBefore == *borrow(& ppszText).unwrap()
        {
            return 0 as std::os::raw::c_int;
        }
        let fresh1 = pbyAddrCursor;
        pbyAddrCursor = pbyAddrCursor.offset(1);
        *fresh1 = nVal as std::os::raw::c_uchar;
        *borrow_mut(&mut ppszText).unwrap() = (*borrow_mut(&mut ppszText).unwrap()).offset(1);
        *borrow(& ppszText).unwrap();
        pszTextBefore = *borrow_mut(&mut ppszText).unwrap();
        nVal = _parseDecimal(borrow_mut(&mut ppszText));
        if '.' as i32 != **borrow(& ppszText).unwrap() as std::os::raw::c_int
            || nVal > 255 as std::os::raw::c_int as std::os::raw::c_uint || pszTextBefore == *borrow(& ppszText).unwrap()
        {
            return 0 as std::os::raw::c_int;
        }
        let fresh2 = pbyAddrCursor;
        pbyAddrCursor = pbyAddrCursor.offset(1);
        *fresh2 = nVal as std::os::raw::c_uchar;
        *borrow_mut(&mut ppszText).unwrap() = (*borrow_mut(&mut ppszText).unwrap()).offset(1);
        *borrow(& ppszText).unwrap();
        pszTextBefore = *borrow_mut(&mut ppszText).unwrap();
        nVal = _parseDecimal(borrow_mut(&mut ppszText));
        if nVal > 255 as std::os::raw::c_int as std::os::raw::c_uint || pszTextBefore == *borrow(& ppszText).unwrap() {
            return 0 as std::os::raw::c_int;
        }
        let fresh3 = pbyAddrCursor;
        pbyAddrCursor = pbyAddrCursor.offset(1);
        *fresh3 = nVal as std::os::raw::c_uchar;
        if ':' as i32 == **borrow(& ppszText).unwrap() as std::os::raw::c_int && !pnPort.is_null() {
            let mut usPortNetwork: std::os::raw::c_ushort = 0;
            *borrow_mut(&mut ppszText).unwrap() = (*borrow_mut(&mut ppszText).unwrap()).offset(1);
            *borrow(& ppszText).unwrap();
            pszTextBefore = *borrow_mut(&mut ppszText).unwrap();
            nVal = _parseDecimal(borrow_mut(&mut ppszText));
            if nVal > 65535 as std::os::raw::c_int as std::os::raw::c_uint || pszTextBefore == *borrow(& ppszText).unwrap()
            {
                return 0 as std::os::raw::c_int;
            }
            *(&mut usPortNetwork as *mut std::os::raw::c_ushort as *mut std::os::raw::c_uchar)
                .offset(
                    0 as std::os::raw::c_int as isize,
                ) = ((nVal & 0xff00 as std::os::raw::c_int as std::os::raw::c_uint) >> 8 as std::os::raw::c_int)
                as std::os::raw::c_uchar;
            *(&mut usPortNetwork as *mut std::os::raw::c_ushort as *mut std::os::raw::c_uchar)
                .offset(
                    1 as std::os::raw::c_int as isize,
                ) = (nVal & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_uchar;
            *pnPort = usPortNetwork as std::os::raw::c_int;
            return 1 as std::os::raw::c_int;
        } else {
            if !pnPort.is_null() {
                *pnPort = 0 as std::os::raw::c_int;
            }
            return 1 as std::os::raw::c_int;
        }
    } else {
        let mut pbyAddrCursor_0: *mut std::os::raw::c_uchar = 0 as *mut std::os::raw::c_uchar;
        let mut pbyZerosLoc: *mut std::os::raw::c_uchar = 0 as *mut std::os::raw::c_uchar;
        let mut bIPv4Detected: std::os::raw::c_int = 0;
        let mut nIdx: std::os::raw::c_int = 0;
        if !pchOpenBracket.is_null() {
            *borrow_mut(&mut ppszText).unwrap() = pchOpenBracket.offset(1 as std::os::raw::c_int as isize);
        }
        pbyAddrCursor_0 = abyAddrLocal;
        pbyZerosLoc = 0 as *mut std::os::raw::c_uchar;
        bIPv4Detected = 0 as std::os::raw::c_int;
        nIdx = 0 as std::os::raw::c_int;
        while nIdx < 8 as std::os::raw::c_int {
            let mut pszTextBefore_0: *const std::os::raw::c_char = *borrow_mut(&mut ppszText).unwrap();
            let mut nVal_0: std::os::raw::c_uint = _parseHex(borrow_mut(&mut ppszText));
            if pszTextBefore_0 == *borrow(& ppszText).unwrap() {
                if !pbyZerosLoc.is_null() {
                    if pbyZerosLoc == pbyAddrCursor_0 {
                        nIdx -= 1;
                        nIdx;
                        break;
                    } else {
                        return 0 as std::os::raw::c_int
                    }
                } else {
                    if ':' as i32 != **borrow(& ppszText).unwrap() as std::os::raw::c_int {
                        return 0 as std::os::raw::c_int;
                    }
                    if 0 as std::os::raw::c_int == nIdx {
                        *borrow_mut(&mut ppszText).unwrap() = (*borrow_mut(&mut ppszText).unwrap()).offset(1);
                        *borrow(& ppszText).unwrap();
                        if ':' as i32 != **borrow(& ppszText).unwrap() as std::os::raw::c_int {
                            return 0 as std::os::raw::c_int;
                        }
                    }
                    pbyZerosLoc = pbyAddrCursor_0;
                    *borrow_mut(&mut ppszText).unwrap() = (*borrow_mut(&mut ppszText).unwrap()).offset(1);
                    *borrow(& ppszText).unwrap();
                }
            } else if '.' as i32 == **borrow(& ppszText).unwrap() as std::os::raw::c_int {
                let mut pszTextlocal: *const std::os::raw::c_char = pszTextBefore_0;
                let mut abyAddrlocal: [std::os::raw::c_uchar; 16] = [0; 16];
                let mut bIsIPv6local_0: std::os::raw::c_int = 0;
                let mut bParseResultlocal: std::os::raw::c_int = ParseIPv4OrIPv6(
                    Some(&mut pszTextlocal),
                    abyAddrlocal.as_mut_ptr(),
                    core::ptr::null_mut(),
                    Some(&mut bIsIPv6local_0),
                );
                *borrow_mut(&mut ppszText).unwrap() = pszTextlocal;
                if bParseResultlocal == 0 || bIsIPv6local_0 != 0 {
                    return 0 as std::os::raw::c_int;
                }
                let fresh4 = pbyAddrCursor_0;
                pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                *fresh4 = abyAddrlocal[0 as std::os::raw::c_int as usize];
                let fresh5 = pbyAddrCursor_0;
                pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                *fresh5 = abyAddrlocal[1 as std::os::raw::c_int as usize];
                let fresh6 = pbyAddrCursor_0;
                pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                *fresh6 = abyAddrlocal[2 as std::os::raw::c_int as usize];
                let fresh7 = pbyAddrCursor_0;
                pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                *fresh7 = abyAddrlocal[3 as std::os::raw::c_int as usize];
                nIdx += 1;
                nIdx;
                bIPv4Detected = 1 as std::os::raw::c_int;
                break;
            } else {
                if nVal_0 > 65535 as std::os::raw::c_int as std::os::raw::c_uint {
                    return 0 as std::os::raw::c_int;
                }
                let fresh8 = pbyAddrCursor_0;
                pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                *fresh8 = (nVal_0 >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
                let fresh9 = pbyAddrCursor_0;
                pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                *fresh9 = (nVal_0 & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
                    as std::os::raw::c_uchar;
                if !(':' as i32 == **borrow(& ppszText).unwrap() as std::os::raw::c_int) {
                    break;
                }
                *borrow_mut(&mut ppszText).unwrap() = (*borrow_mut(&mut ppszText).unwrap()).offset(1);
                *borrow(& ppszText).unwrap();
            }
            nIdx += 1;
            nIdx;
        }
        if !pbyZerosLoc.is_null() {
            let mut nHead: std::os::raw::c_int = pbyZerosLoc.offset_from(abyAddrLocal)
                as std::os::raw::c_long as std::os::raw::c_int;
            let mut nTail: std::os::raw::c_int = nIdx * 2 as std::os::raw::c_int
                - pbyZerosLoc.offset_from(abyAddrLocal) as std::os::raw::c_long as std::os::raw::c_int;
            let mut nZeros: std::os::raw::c_int = 16 as std::os::raw::c_int - nTail - nHead;
            memmove(
                &mut *abyAddrLocal.offset((16 as std::os::raw::c_int - nTail) as isize)
                    as *mut std::os::raw::c_uchar as *mut u8,
                pbyZerosLoc as *const u8,
                nTail as std::os::raw::c_ulong,
            );
            memset(
                pbyZerosLoc as *mut u8,
                0 as std::os::raw::c_int,
                nZeros as std::os::raw::c_ulong,
            );
        }
        if bIPv4Detected != 0 {
            static mut abyPfx: [std::os::raw::c_uchar; 12] = [
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0xff as std::os::raw::c_int as std::os::raw::c_uchar,
                0xff as std::os::raw::c_int as std::os::raw::c_uchar,
            ];
            if 0 as std::os::raw::c_int
                != memcmp(
                    abyAddrLocal as *const u8,
                    abyPfx.as_ptr() as *const u8,
                    std::mem::size_of::<[std::os::raw::c_uchar; 12]>() as std::os::raw::c_ulong,
                )
            {
                return 0 as std::os::raw::c_int;
            }
        }
        if !pchOpenBracket.is_null() {
            if ']' as i32 != **borrow(& ppszText).unwrap() as std::os::raw::c_int {
                return 0 as std::os::raw::c_int;
            }
            *borrow_mut(&mut ppszText).unwrap() = (*borrow_mut(&mut ppszText).unwrap()).offset(1);
            *borrow(& ppszText).unwrap();
        }
        if ':' as i32 == **borrow(& ppszText).unwrap() as std::os::raw::c_int && !pnPort.is_null() {
            let mut pszTextBefore_1: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
            let mut nVal_1: std::os::raw::c_uint = 0;
            let mut usPortNetwork_0: std::os::raw::c_ushort = 0;
            *borrow_mut(&mut ppszText).unwrap() = (*borrow_mut(&mut ppszText).unwrap()).offset(1);
            *borrow(& ppszText).unwrap();
            pszTextBefore_1 = *borrow_mut(&mut ppszText).unwrap();
            pszTextBefore_1 = *borrow_mut(&mut ppszText).unwrap();
            nVal_1 = _parseDecimal(borrow_mut(&mut ppszText));
            if nVal_1 > 65535 as std::os::raw::c_int as std::os::raw::c_uint
                || pszTextBefore_1 == *borrow(& ppszText).unwrap()
            {
                return 0 as std::os::raw::c_int;
            }
            *(&mut usPortNetwork_0 as *mut std::os::raw::c_ushort as *mut std::os::raw::c_uchar)
                .offset(
                    0 as std::os::raw::c_int as isize,
                ) = ((nVal_1 & 0xff00 as std::os::raw::c_int as std::os::raw::c_uint)
                >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
            *(&mut usPortNetwork_0 as *mut std::os::raw::c_ushort as *mut std::os::raw::c_uchar)
                .offset(
                    1 as std::os::raw::c_int as isize,
                ) = (nVal_1 & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_uchar;
            *pnPort = usPortNetwork_0 as std::os::raw::c_int;
            return 1 as std::os::raw::c_int;
        } else {
            if !pnPort.is_null() {
                *pnPort = 0 as std::os::raw::c_int;
            }
            return 1 as std::os::raw::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ParseIPv4OrIPv6_2<'a1>(
    mut pszText: * const std::os::raw::c_char,
    mut abyAddr: * mut std::os::raw::c_uchar,
    mut pnPort: * mut std::os::raw::c_int,
    mut pbIsIPv6: Option<&'a1 mut std::os::raw::c_int>,
) -> std::os::raw::c_int {
    let mut pszTextLocal: *const std::os::raw::c_char = pszText;
    return ParseIPv4OrIPv6(Some(&mut pszTextLocal), abyAddr, pnPort, borrow_mut(&mut pbIsIPv6));
}
#[no_mangle]
pub unsafe extern "C" fn htons(mut us: std::os::raw::c_ushort) -> std::os::raw::c_ushort {
    return (((*(&mut us as *mut std::os::raw::c_ushort as *mut std::os::raw::c_uchar)
        .offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int) << 8 as std::os::raw::c_int)
        + *(&mut us as *mut std::os::raw::c_ushort as *mut std::os::raw::c_uchar)
            .offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int) as std::os::raw::c_ushort;
}
#[no_mangle]
pub unsafe extern "C" fn dumpbin(mut pbyBin: *mut std::os::raw::c_uchar, mut nLen: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < nLen {
        printf(
            b"%02x\0" as *const u8 as *const std::os::raw::c_char,
            *pbyBin.offset(i as isize) as std::os::raw::c_int,
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn testcase(mut pszTest: *const std::os::raw::c_char) {
    let mut abyAddr: [std::os::raw::c_uchar; 16] = [0; 16];
    let mut bIsIPv6: std::os::raw::c_int = 0;
    let mut nPort: std::os::raw::c_int = 0;
    let mut bSuccess: std::os::raw::c_int = 0;
    printf(b"Test case '%s'\n\0" as *const u8 as *const std::os::raw::c_char, pszTest);
    let mut pszTextCursor: *const std::os::raw::c_char = pszTest;
    bSuccess = ParseIPv4OrIPv6(
        Some(&mut pszTextCursor),
        abyAddr.as_mut_ptr(),
        &mut nPort,
        Some(&mut bIsIPv6),
    );
    if bSuccess == 0 {
        printf(
            b"parse failed, at about index %d; rest: '%s'\n\0" as *const u8
                as *const std::os::raw::c_char,
            pszTextCursor.offset_from(pszTest) as std::os::raw::c_long,
            pszTextCursor,
        );
        return;
    }
    printf(b"addr:  \0" as *const u8 as *const std::os::raw::c_char);
    dumpbin(
        abyAddr.as_mut_ptr(),
        if bIsIPv6 != 0 { 16 as std::os::raw::c_int } else { 4 as std::os::raw::c_int },
    );
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    if 0 as std::os::raw::c_int == nPort {
        printf(b"port absent\0" as *const u8 as *const std::os::raw::c_char);
    } else {
        printf(
            b"port:  %d\0" as *const u8 as *const std::os::raw::c_char,
            htons(nPort as std::os::raw::c_ushort) as std::os::raw::c_int,
        );
    }
    printf(b"\n\n\0" as *const u8 as *const std::os::raw::c_char);
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    testcase(b"127.0.0.1\0" as *const u8 as *const std::os::raw::c_char);
    testcase(b"127.0.0.1:80\0" as *const u8 as *const std::os::raw::c_char);
    testcase(b"::1\0" as *const u8 as *const std::os::raw::c_char);
    testcase(b"[::1]:80\0" as *const u8 as *const std::os::raw::c_char);
    testcase(b"2605:2700:0:3::4713:93e3\0" as *const u8 as *const std::os::raw::c_char);
    testcase(b"[2605:2700:0:3::4713:93e3]:80\0" as *const u8 as *const std::os::raw::c_char);
    testcase(b"::ffff:192.168.173.22\0" as *const u8 as *const std::os::raw::c_char);
    testcase(b"[::ffff:192.168.173.22]:80\0" as *const u8 as *const std::os::raw::c_char);
    testcase(b"1::\0" as *const u8 as *const std::os::raw::c_char);
    testcase(b"[1::]:80\0" as *const u8 as *const std::os::raw::c_char);
    testcase(b"::\0" as *const u8 as *const std::os::raw::c_char);
    testcase(b"[::]:80\0" as *const u8 as *const std::os::raw::c_char);
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
