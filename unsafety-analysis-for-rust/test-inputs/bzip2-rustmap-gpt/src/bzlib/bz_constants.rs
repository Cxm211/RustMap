// bz_constants.rs
pub const BZ_M_IDLE: i32 = 1;
pub const BZ_M_RUNNING: i32 = 2;
pub const BZ_M_FLUSHING: i32 = 3;
pub const BZ_M_FINISHING: i32 = 4;

pub const BZ_RUN: i32 = 0;
pub const BZ_FLUSH: i32 = 1;
pub const BZ_FINISH: i32 = 2;

pub const BZ_SEQUENCE_ERROR: i32 = -1;
pub const BZ_PARAM_ERROR: i32 = -2;
pub const BZ_RUN_OK: i32 = 1;
pub const BZ_FLUSH_OK: i32 = 2;
pub const BZ_FINISH_OK: i32 = 3;
pub const BZ_STREAM_END: i32 = 4;
pub const BZ_OK: i32 = 0;

#[derive(PartialEq)]
pub enum SrcMode {
    SmI2O = 1,
    SmF2O = 2,
    SmF2F = 3,
}
