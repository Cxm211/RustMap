use std::io::{Cursor, Write, Read};
use std::convert::TryInto;
use crate::global_vars::bzip2_c2::*;
use crate::global_vars::bzip2_c4::*;
use crate::bzip2recover::writeError::*;


pub fn bs_put_u_char(bitstream: &mut BitStream, c: u8) {
    for i in (0..8).rev() {
        bs_put_bit(bitstream, ((c as u32) >> i) as i32 & 0x1);
    }
}

pub fn bs_put_u_int32(bitstream: &mut BitStream, c: u32) {
    for i in (0..32).rev() {
        bs_put_bit(bitstream, (c >> i) as i32 & 0x1);
    }
}

pub fn bs_put_bit(bitstream: &mut BitStream, bit: i32) {
    if bitstream.buff_live == 8 {
        if bitstream.handle.write_all(&[bitstream.buffer as u8]).is_err() {
            write_error();
        }
        unsafe { BYTES_OUT += 1; }
        bitstream.buff_live = 0;
        bitstream.buffer = 0;
    }
    bitstream.buffer = ((bitstream.buffer << 1) | (bit & 0x1));
    bitstream.buff_live += 1;
}

pub fn flush(bitstream: &mut BitStream) {
    if bitstream.buff_live > 0 {
        bitstream.buffer <<= (8 - bitstream.buff_live);
        if bitstream.handle.write_all(&[bitstream.buffer as u8]).is_err() {
            write_error();
        }
        unsafe { BYTES_OUT += 1; }
        bitstream.buff_live = 0;
        bitstream.buffer = 0;
    }
}

pub fn reset(bitstream: &mut BitStream) {
    bitstream.buffer = 0;
    bitstream.buff_live = 0;
    bitstream.handle.set_position(0);
    unsafe { BYTES_OUT = 0; }
}










