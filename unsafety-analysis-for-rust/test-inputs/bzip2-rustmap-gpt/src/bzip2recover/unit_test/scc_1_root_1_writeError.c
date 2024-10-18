// use crate::global_vars::bzip2_c2::{inFileName, progName};
// writeError ( void )
#include <stdio.h>
#include <string.h>
#include <CUnit/Basic.h>
typedef char Char;
// 你的代码定义...
Char inFileName[2000];
Char outFileName[2000];
Char progName[2000];
static void writeError ( void )
{
   fprintf ( 
            stderr
                  ,
             "%s: I/O error reading `%s', possible reason follows.\n",
            progName, inFileName );
   perror ( progName );
   fprintf ( 
            stderr
                  , "%s: warning: output file(s) may be incomplete.\n",
             progName );
   exit ( 1 );
}
// 测试函数
void test_writeError(void) {
    // 重定向stderr到一个文件以捕获输出
    freopen("stderr_output.txt", "w", stderr);
    strcpy(progName, "testProgram");
    strcpy(inFileName, "testFile.txt");
    
    writeError();
    // 重置stderr
    freopen("/dev/tty", "w", stderr);
    // 从文件中读取输出并验证
    FILE *fp = fopen("stderr_output.txt", "r");
    char line[4000];
    fgets(line, sizeof(line), fp);
    CU_ASSERT_STRING_EQUAL(line, "testProgram: I/O error reading `testFile.txt', possible reason follows.\n");
    fgets(line, sizeof(line), fp);
    // 由于perror产生的消息是基于系统的，所以我们只检查progName是否存在
    CU_ASSERT_PTR_NOT_NULL(strstr(line, "testProgram"));
    fgets(line, sizeof(line), fp);
    CU_ASSERT_STRING_EQUAL(line, "testProgram: warning: output file(s) may be incomplete.\n");
    fclose(fp);
}
// int main() {
//     // 初始化CUnit测试框架
//     CU_initialize_registry();
//     // 添加测试
//     CU_pSuite suite = CU_add_suite("writeError_suite", 0, 0);
//     CU_add_test(suite, "test_writeError", test_writeError);
//     // 运行所有测试
//     CU_basic_set_mode(CU_BRM_VERBOSE);
//     CU_basic_run_tests();
//     CU_cleanup_registry();
//     return 0;
// }
