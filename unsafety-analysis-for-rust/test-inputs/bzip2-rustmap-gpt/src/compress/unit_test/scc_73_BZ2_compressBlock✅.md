pub fn bsW(estate: &mut EState)  n: i32, v: u32) {
        /* 省略 */
}

pub fn bs_finish_write(estate: &mut EState) {
    /* 省略 */
}


// Modified as a standalone function
pub fn bz2_bs_init_write(estate: &mut EState) {
    estate.bsLive = 0;
    estate.bsBuff = 0;
}

pub fn bs_put_u_char(bitstream: &mut BitStream, c: u8) {
     /* 省略 */
}

pub fn bs_put_u_int32(bitstream: &mut BitStream, c: u32) {
     /* 省略 */
}

pub fn bs_put_bit(bitstream: &mut BitStream, bit: i32) {    
    /* 省略 */
}


#define BZ_FINALISE_CRC(crcVar)                \
{                                              \
   crcVar = ~(crcVar);                         \
}



pub fn block_sort(s: &mut EState) 

pub fn generateMTFValues(s: &mut EState) 
pub fn sendMTFValues(s: &mut EState) 

serializeEState忽略，不用翻译到Rust

UChar可以表示为u8