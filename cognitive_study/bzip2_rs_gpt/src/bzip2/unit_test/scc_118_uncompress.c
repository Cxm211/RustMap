
#include "../typedef.h"
#include "./scc_56_fopen_output_safely.c"
#include "./scc_57_root_38_pad.c"
#include "./scc_59_root_40_notAStandardFile.c"
#include "./scc_60_root_41_containsDubiousChars.c"
#include "./scc_87_fileExists.c"
#include "./scc_90_root_52_setExit.c"
#include "./scc_91_cleanUpAndFail.c"
#include "./scc_93_crcError_ioError.c"
#include "./scc_95_applySavedTimeInfoToOutputFile_96_applySavedFileAttrToOutputFile_97_saveInputFileMetaInfo.c"
#include "./scc_98_panic.c"
#include "./scc_105_copyFileName.c"
#include "./scc_116_hasSuffix_107_mapSuffix.c"
static
void uncompress ( Char *name )
{
   FILE *inStr;
   FILE *outStr;
   Int32 n, i;
   Bool magicNumberOK;
   Bool cantGuess;
   struct stat statBuf;

   deleteOutputOnInterrupt = ((Bool)0);

   if (name == 
              ((void *)0) 
                   && srcMode != 1)
      panic ( "uncompress: bad modes\n" );

   cantGuess = ((Bool)0);
   switch (srcMode) {
      case 1:
         copyFileName ( inName, (Char*)"(stdin)" );
         copyFileName ( outName, (Char*)"(stdout)" );
         break;
      case 3:
         copyFileName ( inName, name );
         copyFileName ( outName, name );
         for (i = 0; i < 4; i++)
            if (mapSuffix(outName,zSuffix[i],unzSuffix[i]))
               goto zzz;
         cantGuess = ((Bool)1);
         strcat ( outName, ".out" );
         break;
      case 2:
         copyFileName ( inName, name );
         copyFileName ( outName, (Char*)"(stdout)" );
         break;
   }

   zzz:
   if ( srcMode != 1 && containsDubiousChars ( inName ) ) {
      if (noisy)
      fprintf ( 
               stderr
                     , "%s: There are no files matching `%s'.\n",
                progName, inName );
      setExit(1);
      return;
   }
   if ( srcMode != 1 && !fileExists ( inName ) ) {
      fprintf ( 
               stderr
                     , "%s: Can't open input file %s: %s.\n",
                progName, inName, strerror(
                                          (*__errno_location ())
                                               ) );
      setExit(1);
      return;
   }
   if ( srcMode == 3 || srcMode == 2 ) {
      stat(inName, &statBuf);
      if ( 
          ((((
          statBuf.st_mode
          )) & 0170000) == (0040000)) 
                                      ) {
         fprintf( 
                 stderr
                       ,
                  "%s: Input file %s is a directory.\n",
                  progName,inName);
         setExit(1);
         return;
      }
   }
   if ( srcMode == 3 && !forceOverwrite && notAStandardFile ( inName )) {
      if (noisy)
      fprintf ( 
               stderr
                     , "%s: Input file %s is not a normal file.\n",
                progName, inName );
      setExit(1);
      return;
   }
   if ( cantGuess ) {
      if (noisy)
      fprintf ( 
               stderr
                     ,
                "%s: Can't guess original name for %s -- using %s\n",
                progName, inName, outName );

   }
   if ( srcMode == 3 && fileExists ( outName ) ) {
      if (forceOverwrite) {
 remove(outName);
      } else {
        fprintf ( 
                 stderr
                       , "%s: Output file %s already exists.\n",
                  progName, outName );
        setExit(1);
        return;
      }
   }
   if ( srcMode == 3 && !forceOverwrite &&
        (n=countHardLinks ( inName ) ) > 0) {
      fprintf ( 
               stderr
                     , "%s: Input file %s has %d other link%s.\n",
                progName, inName, n, n > 1 ? "s" : "" );
      setExit(1);
      return;
   }

   if ( srcMode == 3 ) {


      saveInputFileMetaInfo ( inName );
   }

   switch ( srcMode ) {

      case 1:
         inStr = 
                stdin
                     ;
         outStr = 
                 stdout
                       ;
         if ( isatty ( fileno ( 
                               stdin 
                                     ) ) ) {
            fprintf ( 
                     stderr
                           ,
                      "%s: I won't read compressed data from a terminal.\n",
                      progName );
            fprintf ( 
                     stderr
                           , "%s: For help, type: `%s --help'.\n",
                              progName, progName );
            setExit(1);
            return;
         };
         break;

      case 2:
         inStr = fopen ( inName, "rb" );
         outStr = 
                 stdout
                       ;
         if ( inStr == 
                      ((void *)0) 
                           ) {
            fprintf ( 
                     stderr
                           , "%s: Can't open input file %s:%s.\n",
                      progName, inName, strerror(
                                                (*__errno_location ())
                                                     ) );
            if ( inStr != 
                         ((void *)0) 
                              ) fclose ( inStr );
            setExit(1);
            return;
         };
         break;

      case 3:
         inStr = fopen ( inName, "rb" );
         outStr = fopen_output_safely ( outName, "wb" );
         if ( outStr == 
                       ((void *)0)
                           ) {
            fprintf ( 
                     stderr
                           , "%s: Can't create output file %s: %s.\n",
                      progName, outName, strerror(
                                                 (*__errno_location ())
                                                      ) );
            if ( inStr != 
                         ((void *)0) 
                              ) fclose ( inStr );
            setExit(1);
            return;
         }
         if ( inStr == 
                      ((void *)0) 
                           ) {
            fprintf ( 
                     stderr
                           , "%s: Can't open input file %s: %s.\n",
                      progName, inName, strerror(
                                                (*__errno_location ())
                                                     ) );
            if ( outStr != 
                          ((void *)0) 
                               ) fclose ( outStr );
            setExit(1);
            return;
         };
         break;

      default:
         panic ( "uncompress: bad srcMode" );
         break;
   }

   if (verbosity >= 1) {
      fprintf ( 
               stderr
                     , "  %s: ", inName );
      pad ( inName );
      fflush ( 
              stderr 
                     );
   }


   outputHandleJustInCase = outStr;
   deleteOutputOnInterrupt = ((Bool)1);
   magicNumberOK = uncompressStream ( inStr, outStr );
   outputHandleJustInCase = 
                           ((void *)0)
                               ;


   if ( magicNumberOK ) {
      if ( srcMode == 3 ) {
         applySavedTimeInfoToOutputFile ( outName );
         deleteOutputOnInterrupt = ((Bool)0);
         if ( !keepInputFiles ) {
            IntNative retVal = remove ( inName );
            { if ((retVal) != 0) ioError(); };
         }
      }
   } else {
      unzFailsExist = ((Bool)1);
      deleteOutputOnInterrupt = ((Bool)0);
      if ( srcMode == 3 ) {
         IntNative retVal = remove ( outName );
         { if ((retVal) != 0) ioError(); };
      }
   }
   deleteOutputOnInterrupt = ((Bool)0);

   if ( magicNumberOK ) {
      if (verbosity >= 1)
         fprintf ( 
                  stderr
                        , "done\n" );
   } else {
      setExit(2);
      if (verbosity >= 1)
         fprintf ( 
                  stderr
                        , "not a bzip2 file.\n" ); else
         fprintf ( 
                  stderr
                        ,
                   "%s: %s is not a bzip2 file.\n",
                   progName, inName );
   }

}


void test_uncompress_valid_file(void)
{
    // 使用bash命令创建一个文本文件
    system("echo 'This is a test file.' > valid.txt");

    // 使用bzip2打包这个文本文件
    system("bzip2 valid.txt");

    // 额外删除valid.txt，虽然bzip2通常会自动做这件事
    system("rm -f valid.txt");

    // 使用uncompress函数解压
    uncompress("valid.txt.bz2");

    // 我们期望解压缩成功，所以输出文件valid应该存在
    // 这里只是一个示例，您可能需要添加更多的逻辑来验证文件内容等
    CU_ASSERT_TRUE(fileExists("valid"));
}

int main()
{
    // 初始化CUnit测试库
    if (CUE_SUCCESS != CU_initialize_registry())
    {
        return CU_get_error();
    }

    // 添加测试套件
    CU_pSuite pSuite = NULL;
    pSuite = CU_add_suite("Testing uncompress function", NULL, NULL);
    if (NULL == pSuite)
    {
        CU_cleanup_registry();
        return CU_get_error();
    }

    // 添加测试到测试套件
    if ((NULL == CU_add_test(pSuite, "test_uncompress_valid_file", test_uncompress_valid_file)))
    {
        CU_cleanup_registry();
        return CU_get_error();
    }

    // 使用CUnit Basic界面运行测试
    CU_basic_set_mode(CU_BRM_VERBOSE);
    CU_basic_run_tests();
    CU_cleanup_registry();
    return CU_get_error();
}
