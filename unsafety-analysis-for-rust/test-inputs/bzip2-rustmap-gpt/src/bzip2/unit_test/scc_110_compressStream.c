// use crate::bzlib::scc_54_root_36_fdopen::*;
// use crate::bzip2recover::scc_4_bsClose::*;
// use crate::bzlib::scc_77_BZ2_bzWrite::*;
// use crate::bzlib::scc_53_root_35_myfeof::*;
// use crate::bzip2::scc_52_root_34_uInt64_from_UInt32s::*;
// use crate::bzip2::scc_51_root_33_uInt64_to_double::*;
// use crate::bzip2::scc_97_saveInputFileMetaInfo::*;
// use crate::bzlib::scc_50_BZ2_bzWriteOpen::*;
// use crate::bzip2::scc_108_testStream::*;
// use crate::bzip2::scc_100_myMalloc::*;
// use crate::bzip2::scc_99_outOfMemory::*;
// use crate::bzip2::scc_95_applySavedTimeInfoToOutputFile::*;
// use crate::global_vars::bzip2_c1::{blockSize100k, verbosity, workFactor, UInt64};
// use crate::global_vars::bzip2_c5::{outputHandleJustInCase};
// use crate::global_vars::bzip2_c2::{FILE};

#include "../typedef.h"
static
void compressStream ( FILE *stream, FILE *zStream )
{
   BZFILE* bzf = 
                ((void *)0)
                    ;
   UChar ibuf[5000];
   Int32 nIbuf;
   UInt32 nbytes_in_lo32, nbytes_in_hi32;
   UInt32 nbytes_out_lo32, nbytes_out_hi32;
   Int32 bzerr, bzerr_dummy, ret;

   ;
   ;

   if (ferror(stream)) goto errhandler_io;
   if (ferror(zStream)) goto errhandler_io;

   bzf = BZ2_bzWriteOpen ( &bzerr, zStream,
                           blockSize100k, verbosity, workFactor );
   if (bzerr != 0) goto errhandler;

   if (verbosity >= 2) fprintf ( 
                                stderr
                                      , "\n" );

   while (((Bool)1)) {

      if (myfeof(stream)) break;
      nIbuf = fread ( ibuf, sizeof(UChar), 5000, stream );
      if (ferror(stream)) goto errhandler_io;
      if (nIbuf > 0) BZ2_bzWrite ( &bzerr, bzf, (void*)ibuf, nIbuf );
      if (bzerr != 0) goto errhandler;

   }

   BZ2_bzWriteClose64 ( &bzerr, bzf, 0,
                        &nbytes_in_lo32, &nbytes_in_hi32,
                        &nbytes_out_lo32, &nbytes_out_hi32 );
   if (bzerr != 0) goto errhandler;

   if (ferror(zStream)) goto errhandler_io;
   ret = fflush ( zStream );
   if (ret == 
             (-1)
                ) goto errhandler_io;
   if (zStream != 
                 stdout
                       ) {
      Int32 fd = fileno ( zStream );
      if (fd < 0) goto errhandler_io;
      applySavedFileAttrToOutputFile ( fd );
      ret = fclose ( zStream );
      outputHandleJustInCase = 
                              ((void *)0)
                                  ;
      if (ret == 
                (-1)
                   ) goto errhandler_io;
   }
   outputHandleJustInCase = 
                           ((void *)0)
                               ;
   if (ferror(stream)) goto errhandler_io;
   ret = fclose ( stream );
   if (ret == 
             (-1)
                ) goto errhandler_io;

   if (verbosity >= 1) {
      if (nbytes_in_lo32 == 0 && nbytes_in_hi32 == 0) {
  fprintf ( 
           stderr
                 , " no data compressed.\n");
      } else {
  Char buf_nin[32], buf_nout[32];
  UInt64 nbytes_in, nbytes_out;
  double nbytes_in_d, nbytes_out_d;
  uInt64_from_UInt32s ( &nbytes_in,
          nbytes_in_lo32, nbytes_in_hi32 );
  uInt64_from_UInt32s ( &nbytes_out,
          nbytes_out_lo32, nbytes_out_hi32 );
  nbytes_in_d = uInt64_to_double ( &nbytes_in );
  nbytes_out_d = uInt64_to_double ( &nbytes_out );
  uInt64_toAscii ( buf_nin, &nbytes_in );
  uInt64_toAscii ( buf_nout, &nbytes_out );
  fprintf ( 
           stderr
                 , "%6.3f:1, %6.3f bits/byte, "
     "%5.2f%% saved, %s in, %s out.\n",
     nbytes_in_d / nbytes_out_d,
     (8.0 * nbytes_out_d) / nbytes_in_d,
     100.0 * (1.0 - nbytes_out_d / nbytes_in_d),
     buf_nin,
     buf_nout
   );
      }
   }

   return;

   errhandler:
   BZ2_bzWriteClose64 ( &bzerr_dummy, bzf, 1,
                        &nbytes_in_lo32, &nbytes_in_hi32,
                        &nbytes_out_lo32, &nbytes_out_hi32 );
   switch (bzerr) {
      case (-9):
         configError(); break;
      case (-3):
         outOfMemory (); break;
      case (-6):
         errhandler_io:
         ioError(); break;
      default:
         panic ( "compress:unexpected error" );
   }

   panic ( "compress:end" );

}
