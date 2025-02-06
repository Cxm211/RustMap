#include "../typedef.h"
#include "../bzlib/scc_61_root_42_BZ2_bzlibVersion.c"
static
void usage ( Char *fullProgName )  // 定义一个名为usage的静态函数，接受一个Char型指针fullProgName作为参数
{
   fprintf (
      
     stderr
           ,
      "bzip2, a block-sorting file compressor.  "
      "Version %s.\n"
      "\n   usage: %s [flags and input files in any order]\n"
      "\n"
      "   -h --help           print this message\n"
      "   -d --decompress     force decompression\n"
      "   -z --compress       force compression\n"
      "   -k --keep           keep (don't delete) input files\n"
      "   -f --force          overwrite existing output files\n"
      "   -t --test           test compressed file integrity\n"
      "   -c --stdout         output to standard out\n"
      "   -q --quiet          suppress noncritical error messages\n"
      "   -v --verbose        be verbose (a 2nd -v gives more)\n"
      "   -L --license        display software version & license\n"
      "   -V --version        display software version & license\n"
      "   -s --small          use less memory (at most 2500k)\n"
      "   -1 .. -9            set block size to 100k .. 900k\n"
      "   --fast              alias for -1\n"
      "   --best              alias for -9\n"
      "\n"
      "   If invoked as `bzip2', default action is to compress.\n"
      "              as `bunzip2',  default action is to decompress.\n"
      "              as `bzcat', default action is to decompress to stdout.\n"
      "\n"
      "   If no file names are given, bzip2 compresses or decompresses\n"
      "   from standard input to standard output.  You can combine\n"
      "   short flags, so `-v -4' means the same as -v4 or -4v, &c.\n"

      "\n"

      ,

      BZ2_bzlibVersion(),
      fullProgName
   );
}

#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>
#include <stdio.h>
#include <string.h>

void test_usage_output(void) {
    char* testProgName = "bzip2test";
    freopen("/tmp/test_output.txt", "w", stderr); // 重定向stderr到一个临时文件
    usage(testProgName); // 调用usage函数
    freopen("/dev/stderr", "w", stderr); // 恢复stderr

    FILE *fp = fopen("/tmp/test_output.txt", "r"); 
    CU_ASSERT_PTR_NOT_NULL(fp); // 检查文件是否成功打开

    char buffer[2048];
    fread(buffer, sizeof(char), 2048, fp);

    CU_ASSERT_PTR_NOT_NULL(strstr(buffer, testProgName)); // 检查输出是否包含提供的程序名称

    fclose(fp);
}


int main() {
    CU_initialize_registry();
    CU_pSuite suite = CU_add_suite("Test Usage Function", 0, 0);

    CU_add_test(suite, "test_usage_output", test_usage_output);

    CU_basic_set_mode(CU_BRM_VERBOSE);
    CU_basic_run_tests();
    CU_cleanup_registry();

    return 0;
}
