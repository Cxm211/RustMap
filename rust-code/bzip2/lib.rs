pub mod blocksort {
    pub mod BZ2_blockSort;
    pub mod fallbackQSort3;
    pub mod fallbackSimpleSort;
    pub mod fallbackSort;
    pub mod mainGtU;
    pub mod mainQSort3;
    pub mod mainSimpleSort;
    pub mod mainSort;
}

pub mod bzlib {
    pub mod BZ2_bz__AssertH__fail;
    pub mod BZ2_bzDecompress;
    pub mod BZ2_bzWriteOpen;
    pub mod bz2_bzwrite;
    pub mod BZ2_bzWriteClose;
    pub mod BZ2_bzReadOpen;
    pub mod bz2_bzread;
    pub mod BZ2_bzReadGetUnused;
    pub mod BZ2_bzReadClose;
    pub mod BZ_SETERR;
    pub mod BZ2_bzCompressInit;
    pub mod BZ2_bzCompress;
    pub mod BZ2_bzCompressEnd;
    pub mod BZ2_bzCompressInit;
    pub mod BZ2_bzDecompressInit;
    pub mod BZ2_bzDecompressEnd;
    pub mod utils;
    pub mod flush_RL;
}

pub mod headers{
    pub mod global_vars;
}

pub mod huffman{
    pub mod BZ2_hbAssignCodes;
    pub mod BZ2_hbCreateDecodeTables;
    pub mod BZ2_hbMakeCodeLengths;
}

pub mod crctable {
    pub mod BZ2_crc32Table;
}

pub mod randtable {
    pub mod BZ2_rNums;
}

pub mod compress{
    pub mod BZ2_compressBlock;
    pub mod bsW;
    pub mod sendMTFValues;
}

pub mod decompress{
    pub mod BZ2_decompress;
}

pub mod bzip2{
    pub mod globals;
}


pub use crate::headers::global_vars::DState;
pub use crate::headers::global_vars::EState;
pub use crate::headers::global_vars::bz_stream;
pub use crate::headers::global_vars::bzFile;
pub use crate::bzlib::BZ2_bz__AssertH__fail::BZ2_bz__AssertH__fail;
pub use crate::crctable::BZ2_crc32Table::BZ2_CRC32_TABLE;
pub use crate::randtable::BZ2_rNums::BZ2_rNums;