#include "../typedef.h"
#include <CUnit/Basic.h>
#include <CUnit/CUnit.h>
#include "./scc_90_root_52_setExit.c"
#include "./scc_105_copyFileName.c"
#include "./scc_87_fileExists.c"
void compress ( Char *name )
{
   FILE *inStr;
   FILE *outStr;
   Int32 n, i;
   struct stat statBuf;

   deleteOutputOnInterrupt = ((Bool)0);

   // 如果名称为null，并且源模式不为1，则抛出异常
   if (name == ((void *)0) && srcMode != 1)
      panic ( "compress: bad modes\n" );

   // 根据源模式设置输入、输出文件名
   switch (srcMode) {
      case 1:
         // 从标准输入读取，向标准输出写入
         copyFileName ( inName, (Char*)"(stdin)" );
         copyFileName ( outName, (Char*)"(stdout)" );
         break;
      case 3:
         // 使用给定的文件名，并将输出文件的扩展名设置为.bz2
         copyFileName ( inName, name );
         copyFileName ( outName, name );
         strcat ( outName, ".bz2" );
         break;
      case 2:
         // 使用给定的文件名，但将输出写入标准输出
         copyFileName ( inName, name );
         copyFileName ( outName, (Char*)"(stdout)" );
         break;
   }


     // 验证文件名、文件存在性、文件类型等条件，并进行相应的错误处理
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
   for (i = 0; i < 4; i++) {
      if (hasSuffix(inName, zSuffix[i])) {
         if (noisy)
         fprintf ( 
                  stderr
                        ,
                   "%s: Input file %s already has %s suffix.\n",
                   progName, inName, zSuffix[i] );
         setExit(1);
         return;
      }
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
        (n=countHardLinks ( inName )) > 0) {
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
                               stdout 
                                      ) ) ) {
            fprintf ( 
                     stderr
                           ,
                      "%s: I won't write compressed data to a terminal.\n",
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
         if ( isatty ( fileno ( 
                               stdout 
                                      ) ) ) {
            fprintf ( 
                     stderr
                           ,
                      "%s: I won't write compressed data to a terminal.\n",
                      progName );
            fprintf ( 
                     stderr
                           , "%s: For help, type: `%s --help'.\n",
                              progName, progName );
            if ( inStr != 
                         ((void *)0) 
                              ) fclose ( inStr );
            setExit(1);
            return;
         };
         if ( inStr == 
                      ((void *)0) 
                           ) {
            fprintf ( 
                     stderr
                           , "%s: Can't open input file %s: %s.\n",
                      progName, inName, strerror(
                                                (*__errno_location ())
                                                     ) );
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
         panic ( "compress: bad srcMode" );
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
   compressStream ( inStr, outStr );
   outputHandleJustInCase = 
                           ((void *)0)
                               ;

    // 如果在源模式3中，并且不保留输入文件，则删除输入文件
   if ( srcMode == 3 ) {
      applySavedTimeInfoToOutputFile ( outName );
      deleteOutputOnInterrupt = ((Bool)0);
      if ( !keepInputFiles ) {
         IntNative retVal = remove ( inName );
         { if ((retVal) != 0) ioError(); };
      }
   }

   deleteOutputOnInterrupt = ((Bool)0);
}


void test_basic_compression(void) {
    // Remove test.txt if it exists to ensure a fresh start
    remove("test.txt");

    // Create the test.txt file
    FILE* testFile = fopen("test.txt", "w");
    CU_ASSERT_PTR_NOT_NULL(testFile);  // Ensure file is opened successfully
    if (testFile) {
        fprintf(testFile, "This is a test file for the compress function.");
        fclose(testFile);
    }

    // Now compress the file
    compress("test.txt");

    // Check if the compressed file exists and its content is valid
    FILE* compressedFile = fopen("test.txt.bz2", "rb");
    CU_ASSERT_PTR_NOT_NULL(compressedFile);  // Ensure compressed file is opened successfully
    if (compressedFile) {
        // Check if the file is not empty (you might have more specific checks)
        fseek(compressedFile, 0, SEEK_END);
        long size = ftell(compressedFile);
        CU_ASSERT_TRUE(size > 0);
        fclose(compressedFile);
    }

    // Remove test.txt after testing
    remove("test.txt");
}

int main(void) {
    // Initialize the CUnit test registry
    if (CUE_SUCCESS != CU_initialize_registry())
        return CU_get_error();

    // Add a suite to the registry
    CU_pSuite suite = CU_add_suite("Compression Suite", NULL, NULL);
    if (NULL == suite) {
        CU_cleanup_registry();
        return CU_get_error();
    }

    // Add the test_basic_compression to the suite
    if (NULL == CU_add_test(suite, "test_basic_compression", test_basic_compression)) {
        CU_cleanup_registry();
        return CU_get_error();
    }

    // Run all tests using the basic interface
    CU_basic_set_mode(CU_BRM_VERBOSE);
    CU_basic_run_tests();
    CU_cleanup_registry();

    return CU_get_error();
}





















/*
对于CUnit的测试，你可能需要以下测试用例：

压缩标准输入到标准输出。
使用模式2压缩指定文件到标准输出。
使用模式3压缩指定文件到一个新的.bz2文件。
测试各种错误情况，例如：文件不存在、文件是目录、输出文件已存在等。
具体的CUnit代码格式取决于您的测试框架配置和结构。请告诉我您的具体需求，以便我能为您提供更多的帮助。
*/


/*

scc_119_compress.c:(.text+0x87): undefined reference to `panic'
/usr/bin/ld: scc_119_compress.c:(.text+0xe2): undefined reference to `copyFileName'
/usr/bin/ld: scc_119_compress.c:(.text+0x112): undefined reference to `copyFileName'
/usr/bin/ld: scc_119_compress.c:(.text+0x15a): undefined reference to `copyFileName'
/usr/bin/ld: scc_119_compress.c:(.text+0x18a): undefined reference to `copyFileName'
/usr/bin/ld: scc_119_compress.c:(.text+0x1f4): undefined reference to `copyFileName'
/usr/bin/ld: /tmp/cc3scAZM.o:scc_119_compress.c:(.text+0x224): more undefined references to `copyFileName' follow
/usr/bin/ld: /tmp/cc3scAZM.o: in function `compress':
scc_119_compress.c:(.text+0x26c): undefined reference to `containsDubiousChars'
/usr/bin/ld: scc_119_compress.c:(.text+0x2ff): undefined reference to `setExit'
/usr/bin/ld: scc_119_compress.c:(.text+0x34b): undefined reference to `fileExists'
/usr/bin/ld: scc_119_compress.c:(.text+0x3e7): undefined reference to `setExit'
/usr/bin/ld: scc_119_compress.c:(.text+0x463): undefined reference to `hasSuffix'
/usr/bin/ld: scc_119_compress.c:(.text+0x502): undefined reference to `setExit'
/usr/bin/ld: scc_119_compress.c:(.text+0x616): undefined reference to `setExit'
/usr/bin/ld: scc_119_compress.c:(.text+0x683): undefined reference to `notAStandardFile'
/usr/bin/ld: scc_119_compress.c:(.text+0x716): undefined reference to `setExit'
/usr/bin/ld: scc_119_compress.c:(.text+0x762): undefined reference to `fileExists'
/usr/bin/ld: scc_119_compress.c:(.text+0x818): undefined reference to `setExit'
/usr/bin/ld: scc_119_compress.c:(.text+0x885): undefined reference to `countHardLinks'
/usr/bin/ld: scc_119_compress.c:(.text+0x940): undefined reference to `setExit'
/usr/bin/ld: scc_119_compress.c:(.text+0x988): undefined reference to `saveInputFileMetaInfo'
/usr/bin/ld: scc_119_compress.c:(.text+0xaa9): undefined reference to `setExit'
/usr/bin/ld: scc_119_compress.c:(.text+0xc2e): undefined reference to `setExit'
/usr/bin/ld: scc_119_compress.c:(.text+0xcd5): undefined reference to `setExit'
/usr/bin/ld: scc_119_compress.c:(.text+0xd53): undefined reference to `fopen_output_safely'
/usr/bin/ld: scc_119_compress.c:(.text+0xe29): undefined reference to `setExit'
/usr/bin/ld: scc_119_compress.c:(.text+0xf0d): undefined reference to `setExit'
/usr/bin/ld: scc_119_compress.c:(.text+0xf5e): undefined reference to `panic'
/usr/bin/ld: scc_119_compress.c:(.text+0xfdc): undefined reference to `pad'
/usr/bin/ld: scc_119_compress.c:(.text+0x1042): undefined reference to `compressStream'
/usr/bin/ld: scc_119_compress.c:(.text+0x1094): undefined reference to `applySavedTimeInfoToOutputFile'
/usr/bin/ld: scc_119_compress.c:(.text+0x1116): undefined reference to `ioError'


*/