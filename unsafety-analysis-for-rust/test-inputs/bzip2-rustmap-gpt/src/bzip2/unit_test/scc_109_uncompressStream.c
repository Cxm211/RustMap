// use crate::bzip2::scc_94_ioError::*;
// use crate::bzip2::scc_93_crcError::*;
// use crate::bzlib::scc_49_BZ2_bzCompressInit::*;
// use crate::huffman::scc_66_BZ2_hbMakeCodeLengths::*;
// use crate::bzip2::scc_18_root_11_utime::*;
// use crate::bzlib::scc_17_root_10_BZ2_bzReadGetUnused::*;
// use crate::bzip2::scc_97_saveInputFileMetaInfo::*;
// use crate::bzlib::scc_50_BZ2_bzWriteOpen::*;
// use crate::bzip2::scc_108_testStream::*;
// use crate::bzip2::scc_100_myMalloc::*;
// use crate::bzip2::scc_99_outOfMemory::*;
// use crate::bzip2::scc_95_applySavedTimeInfoToOutputFile::*;
// use crate::global_vars::bzip2_c5::{forceOverwrite, inName, noisy, outputHandleJustInCase, smallMode};
// use crate::global_vars::bzip2_c2::{progName, FILE};
// use crate::global_vars::bzip2_c1::{verbosity};

#include "../typedef.h"
static
Bool uncompressStream ( FILE *zStream, FILE *stream )
{
   BZFILE* bzf = 
                ((void *)0)
                    ;
   Int32 bzerr, bzerr_dummy, ret, nread, streamNo, i;
   UChar obuf[5000];
   UChar unused[5000];
   Int32 nUnused;
   void* unusedTmpV;
   UChar* unusedTmp;

   nUnused = 0;
   streamNo = 0;

   ;
   ;

   if (ferror(stream)) goto errhandler_io;
   if (ferror(zStream)) goto errhandler_io;

   while (((Bool)1)) {

      bzf = BZ2_bzReadOpen (
               &bzerr, zStream, verbosity,
               (int)smallMode, unused, nUnused
            );
      if (bzf == 
                ((void *)0) 
                     || bzerr != 0) goto errhandler;
      streamNo++;

      while (bzerr == 0) {
         nread = BZ2_bzRead ( &bzerr, bzf, obuf, 5000 );
         if (bzerr == (-5)) goto trycat;
         if ((bzerr == 0 || bzerr == 4) && nread > 0)
            fwrite ( obuf, sizeof(UChar), nread, stream );
         if (ferror(stream)) goto errhandler_io;
      }
      if (bzerr != 4) goto errhandler;

      BZ2_bzReadGetUnused ( &bzerr, bzf, &unusedTmpV, &nUnused );
      if (bzerr != 0) panic ( "decompress:bzReadGetUnused" );

      unusedTmp = (UChar*)unusedTmpV;
      for (i = 0; i < nUnused; i++) unused[i] = unusedTmp[i];

      BZ2_bzReadClose ( &bzerr, bzf );
      if (bzerr != 0) panic ( "decompress:bzReadGetUnused" );

      if (nUnused == 0 && myfeof(zStream)) break;
   }

   closeok:
   if (ferror(zStream)) goto errhandler_io;
   if (stream != 
                stdout
                      ) {
      Int32 fd = fileno ( stream );
      if (fd < 0) goto errhandler_io;
      applySavedFileAttrToOutputFile ( fd );
   }
   ret = fclose ( zStream );
   if (ret == 
             (-1)
                ) goto errhandler_io;

   if (ferror(stream)) goto errhandler_io;
   ret = fflush ( stream );
   if (ret != 0) goto errhandler_io;
   if (stream != 
                stdout
                      ) {
      ret = fclose ( stream );
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
   if (verbosity >= 2) fprintf ( 
                                stderr
                                      , "\n    " );
   return ((Bool)1);

   trycat:
   if (forceOverwrite) {
      rewind(zStream);
      while (((Bool)1)) {
        if (myfeof(zStream)) break;
        nread = fread ( obuf, sizeof(UChar), 5000, zStream );
        if (ferror(zStream)) goto errhandler_io;
        if (nread > 0) fwrite ( obuf, sizeof(UChar), nread, stream );
        if (ferror(stream)) goto errhandler_io;
      }
      goto closeok;
   }

   errhandler:
   BZ2_bzReadClose ( &bzerr_dummy, bzf );
   switch (bzerr) {
      case (-9):
         configError(); break;
      case (-6):
         errhandler_io:
         ioError(); break;
      case (-4):
         crcError();
      case (-3):
         outOfMemory();
      case (-7):
         compressedStreamEOF();
      case (-5):
         if (zStream != 
                       stdin
                            ) fclose(zStream);
         if (stream != 
                      stdout
                            ) fclose(stream);
         if (streamNo == 1) {
            return ((Bool)0);
         } else {
            if (noisy)
            fprintf ( 
                     stderr
                           ,
                      "\n%s: %s: trailing garbage after EOF ignored\n",
                      progName, inName );
            return ((Bool)1);
         }
      default:
         panic ( "decompress:unexpected error" );
   }

   panic ( "decompress:end" );
   return ((Bool)1);
}