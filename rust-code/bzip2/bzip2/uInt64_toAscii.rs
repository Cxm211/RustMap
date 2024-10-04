mod globals;

use globals::*;

pub fn u_int64_from_uint32s(n: &mut UInt64, lo32: UInt32, hi32: UInt32) {
    n.b[7] = ((hi32 >> 24) & 0xFF) as UChar;
    n.b[6] = ((hi32 >> 16) & 0xFF) as UChar;
    n.b[5] = ((hi32 >> 8) & 0xFF) as UChar;
    n.b[4] = (hi32 & 0xFF) as UChar;
    n.b[3] = ((lo32 >> 24) & 0xFF) as UChar;
    n.b[2] = ((lo32 >> 16) & 0xFF) as UChar;
    n.b[1] = ((lo32 >> 8) & 0xFF) as UChar;
    n.b[0] = (lo32 & 0xFF) as UChar;
}

pub fn u_int64_to_double(n: &UInt64) -> f64 {
    let mut base = 1.0;
    let mut sum = 0.0;

    for &byte in n.b.iter() {
        sum += base * (byte as f64);
        base *= 256.0;
    }

    sum
}

pub fn u_int64_is_zero(n: &UInt64) -> Bool {
    for &byte in n.b.iter() {
        if byte != 0 {
            return False;
        }
    }
    True
}

pub fn u_int64_qrm10(n: &mut UInt64) -> UInt32 {
    let mut rem: UInt32 = 0;

    for i in (0..8).rev() {
        let tmp = rem * 256 + n.b[i] as UInt32;
        n.b[i] = (tmp / 10) as UChar;
        rem = tmp % 10;
    }

    rem
}

pub fn u_int64_to_ascii(outbuf: &mut [u8], n: &UInt64) {
    let mut buf = [0u8; 32];
    let mut n_buf: usize = 0;
    let mut n_copy = *n;

    loop {
        let q = u_int64_qrm10(&mut n_copy);
        buf[n_buf] = q as u8 + b'0';
        n_buf += 1;

        if u_int64_is_zero(&n_copy) == True {
            break;
        }
    }

    outbuf[n_buf] = 0;

    for i in 0..n_buf {
        outbuf[i] = buf[n_buf - i - 1];
    }
}
