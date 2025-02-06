#include "../typedef.h"
#include "../bzlib/scc_11_BZ2_bzReadClose.c"
#include "../bzlib/scc_17_root_10_BZ2_bzReadGetUnused.c"
#include "../bzlib/scc_53_root_35_myfeof.c"
#include "../bzlib/scc_47_BZ2_bzDecompressInit_48_bzReadOpen.c"
#include "./scc_98_panic.c"
#include "./scc_107_configError.c"
#include "./scc_93_crcError_ioError.c"
#include "./scc_99_outOfMemory_scc_100_myMalloc.c"
static Bool testStream ( FILE *zStream ) {
   BZFILE* bzf = NULL;
   Int32 bzerr, bzerr_dummy, ret, streamNo, i;
   UChar obuf[5000];  // 输出缓冲区
   UChar unused[5000];  // 未使用的缓冲区
   Int32 nUnused;  // 未使用的字节数
   void* unusedTmpV;  // 临时指针，用于转换
   UChar* unusedTmp;  // 临时UChar指针，用于保存未使用的数据

   nUnused = 0;  // 初始未使用字节数为0
   streamNo = 0;  // 初始流号为0

   if (ferror(zStream)) goto errhandler_io;  // 检查文件流是否有错误

   while (1) {  // 无限循环，直到流结束或出错

      // 尝试打开一个bzip2压缩的读取流
      bzf = BZ2_bzReadOpen (&bzerr, zStream, verbosity, (int)smallMode, unused, nUnused);
      if (bzf == NULL || bzerr != 0) goto errhandler;  // 如果出错则跳转到错误处理
      streamNo++;  // 流号递增

      // 读取数据，直到出错或达到文件结束
      while (bzerr == 0) {
         BZ2_bzRead(&bzerr, bzf, obuf, 5000);
         if (bzerr == (-5)) goto errhandler;  // 如果出错则跳转到错误处理
      }
      if (bzerr != 4) goto errhandler;  // 如果错误不是EOF，跳转到错误处理

      // 获取未使用的数据
      BZ2_bzReadGetUnused(&bzerr, bzf, &unusedTmpV, &nUnused);
      if (bzerr != 0) panic("test:bzReadGetUnused");  // 如果出错则停止程序

      // 复制未使用的数据到unused缓冲区
      unusedTmp = (UChar*)unusedTmpV;
      for (i = 0; i < nUnused; i++) unused[i] = unusedTmp[i];

      // 关闭bzip2读取流
      BZ2_bzReadClose(&bzerr, bzf);
      if (bzerr != 0) panic("test:bzReadGetUnused");  // 如果出错则停止程序
      if (nUnused == 0 && myfeof(zStream)) break;  // 如果没有未使用的数据并且达到文件流结束，则跳出循环

   }

   // 检查文件流是否有错误
   if (ferror(zStream)) goto errhandler_io;
   ret = fclose(zStream);  // 关闭文件流
   if (ret == (-1)) goto errhandler_io;  // 如果关闭失败，跳转到错误处理

   if (verbosity >= 2) fprintf(stderr, "\n    ");  // 如果verbosity大于或等于2，则输出空行
   return 1;  // 返回真

   // 错误处理部分
   errhandler:
   BZ2_bzReadClose(&bzerr_dummy, bzf);
   if (verbosity == 0) fprintf(stderr, "%s: %s: ", progName, inName);
   switch (bzerr) {
      case (-9):
         configError(); break;
      case (-6):
         errhandler_io:
         ioError(); break;
      case (-4):
         fprintf(stderr, "data integrity (CRC) error in data\n");
         return 0;
      case (-3):
         outOfMemory();
      case (-7):
         fprintf(stderr, "file ends unexpectedly\n");
         return 0;
      case (-5):
         if (zStream != stdin) fclose(zStream);
         if (streamNo == 1) {
            fprintf(stderr, "bad magic number (file not created by bzip2)\n");
            return 0;
         } else {
            if (noisy) fprintf(stderr, "trailing garbage after EOF ignored\n");
            return 1;
         }
      default:
         panic("test:unexpected error");
   }

   panic("test:end");
   return 1;
}

#include <CUnit/Basic.h>

void test_testStream(void) {
    FILE* zStreamValid = fopen("108_tobe_compressed.txt.bz2 ", "r");  // 打开一个有效的bzip2文件
    FILE* zStreamInvalid = fopen("invalid_file.txt", "r");  // 打开一个无效的文件

    CU_ASSERT_TRUE(testStream(zStreamValid));  // 测试有效的bzip2文件，应该返回真
    CU_ASSERT_FALSE(testStream(zStreamInvalid));  // 测试无效的文件，应该返回假

    fclose(zStreamValid);
    fclose(zStreamInvalid);
}

int main() {
    CU_pSuite pSuite = NULL;

    if (CUE_SUCCESS != CU_initialize_registry()) {
        return CU_get_error();
    }

    pSuite = CU_add_suite("TestStreamTestSuite", NULL, NULL);
    if (NULL == pSuite) {
        CU_cleanup_registry();
        return CU_get_error();
    }

    if (NULL == CU_add_test(pSuite, "test_testStream", test_testStream)) {
        CU_cleanup_registry();
        return CU_get_error();
    }

    CU_basic_set_mode(CU_BRM_VERBOSE);
    CU_basic_run_tests();
    CU_cleanup_registry();
    return CU_get_error();
}
