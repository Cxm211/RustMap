pub static BZ_ERROR_STRINGS: [&str; 16] = [
    "OK",
    "SEQUENCE_ERROR",
    "PARAM_ERROR",
    "MEM_ERROR",
    "DATA_ERROR",
    "DATA_ERROR_MAGIC",
    "IO_ERROR",
    "UNEXPECTED_EOF",
    "OUTBUFF_FULL",
    "CONFIG_ERROR",
    "???",
    "???",
    "???",
    "???",
    "???",
    "???",
];

pub fn bz2_bzerror(b: Option<&bzFile>, errnum: &mut i32) -> &str {
    if let Some(bzf) = b {
        let mut err = bzf.lastErr;
        if err > 0 {
            err = 0;
        }
        *errnum = err;
        return BZ_ERROR_STRINGS[(-err) as usize];
    }

    *errnum = -2; // Corresponding to PARAM_ERROR in the list
    BZ_ERROR_STRINGS[2]
}
