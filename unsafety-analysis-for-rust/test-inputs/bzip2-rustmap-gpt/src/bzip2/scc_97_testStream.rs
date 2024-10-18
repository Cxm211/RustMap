use crate::bzlib::scc_39_BZ2_bzDecompressInit_40_bzReadOpen::*;
use crate::bzlib::scc_64_BZ2_bzDecompress_65_BZ2_bzRead::*;
use crate::bzlib::scc_9_root_7_BZ2_bzReadGetUnused::*;
// use crate::huffman::scc_66_BZ2_hbMakeCodeLengths::*;
// use crate::bzip2::scc_18_root_11_utime::*;
// use crate::bzlib::scc_17_root_10_BZ2_bzReadGetUnused::*;
// use crate::bzlib::scc_50_BZ2_bzWriteOpen::*;
// use crate::bzip2::scc_108_testStream::*;
// use crate::bzip2::scc_100_myMalloc::*;
// use crate::bzip2::scc_99_outOfMemory::*;
// use crate::bzip2::scc_95_applySavedTimeInfoToOutputFile::*;
use crate::global_vars::bzip2_c5::*;
use crate::global_vars::bzip2_c2::{PROG_NAME};
use crate::global_vars::bzip2_c1::{verbosity,bzFile};


// testStream ( FILE *zStream )
use std::fs::File;
use std::io::{self, ErrorKind};

// Assuming bzFile and other required types are defined elsewhere.
// Please provide the definitions if they are different from standard implementations.

pub fn test_stream(z_stream: &mut File) {
//     let mut bzf: Option<Box<bzFile>> = None;
//     let mut bzerr = 0;
//     let mut n_unused = 0;
//     let mut stream_no = 0;
//     let mut obuf = [0u8; 5000];
//     let mut unused = [0u8; 5000];

//     if z_stream.metadata()?.len() == 0 {
//         return Err(io::Error::new(ErrorKind::Other, "Stream error"));
//     }

//     loop {
//         bzf = bz2_bzread_open(&mut bzerr, z_stream, unsafe { verbosity }, unsafe { SMALL_MODE } as i32, None);

//         if bzf.is_none() || bzerr != 0 {
//             return Err(io::Error::new(ErrorKind::Other, "Read error"));
//         }

//         stream_no += 1;

//         while bzerr == 0 {
//             let _ = BZ2_bzRead(&mut bzerr, bzf.as_mut().unwrap(), &mut obuf);
//             if bzerr == -5 {
//                 return Err(io::Error::new(ErrorKind::Other, "Read error"));
//             }
//         }

//         if bzerr != 4 {
//             return Err(io::Error::new(ErrorKind::Other, "Read error"));
//         }

//         let mut unused_tmp: Option<*mut i8> = None;
//         bz2_bz_read_get_unused(&mut Some(bzerr), bzf.as_mut().unwrap(), &mut unused_tmp, &mut Some(n_unused));

//         if bzerr != 0 {
//             panic!("test:bzReadGetUnused");
//         }

//         if let Some(ptr) = unused_tmp {
//             let unused_tmp_slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, n_unused as usize) };
//             unused[..n_unused as usize].copy_from_slice(unused_tmp_slice);
//         }

//         BZ2_bzReadClose(&mut Some(bzerr), bzf.take().unwrap());

//         if bzerr != 0 {
//             panic!("test:bzReadClose error");
//         }

//         if n_unused == 0 && z_stream.metadata()?.len() == 0 {
//             break;
//         }
//     }

//     if z_stream.metadata()?.len() == 0 {
//         return Err(io::Error::new(ErrorKind::Other, "IO error"));
//     }

//     z_stream.sync_all()?;

//     if unsafe { verbosity } >= 2 {
//         eprintln!("\n    ");
//     }

//     Ok(true)
// Ok(true) // Normal execution path ends here

// // Error handling
// errhandler: {
//     BZ2_bzReadClose(&mut Some(bzerr_dummy), bzf.take().unwrap());
//     if unsafe { verbosity } == 0 {
//         eprintln!("{}: {}: ", prog_name, in_name);
//     }
//     match bzerr {
//         -9 => config_error(),
//         -6 => {
//             io_error();
//             Err(io::Error::new(ErrorKind::Other, "IO Error"))
//         },
//         -4 => {
//             eprintln!("data integrity (CRC) error in data\n");
//             Err(io::Error::new(ErrorKind::Other, "Data integrity error"))
//         },
//         -3 => out_of_memory(),
//         -7 => {
//             eprintln!("file ends unexpectedly\n");
//             Err(io::Error::new(ErrorKind::Other, "Unexpected end of file"))
//         },
//         -5 => {
//             if z_stream != std::io::stdin() {
//                 z_stream.sync_all()?;
//             }
//             if stream_no == 1 {
//                 eprintln!("bad magic number (file not created by bzip2)\n");
//                 Err(io::Error::new(ErrorKind::Other, "Bad magic number"))
//             } else {
//                 if unsafe { noisy } {
//                     eprintln!("trailing garbage after EOF ignored\n");
//                 }
//                 Ok(true)
//             }
//         },
//         _ => panic!("test:unexpected error"),
//     }
// }

// panic!("test:end")
}