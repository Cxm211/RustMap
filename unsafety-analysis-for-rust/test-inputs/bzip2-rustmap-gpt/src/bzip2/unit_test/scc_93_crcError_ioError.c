#include "../typedef.h"

// 数据完整性错误处理函数
static
void crcError ( void )
{
   // 在标准错误流上输出数据完整性错误的信息
   fprintf(stderr, "\n%s: 数据完整性在解压缩时出错。\n", progName);
   
   // 显示文件名信息
   showFileNames();
   
   // 提供可能的建议
   cadvise();
   
   // 清理资源并退出程序，返回值为2
   cleanUpAndFail( 2 );
}

// I/O错误或其他类型的错误处理函数
static
void ioError ( void )
{
   // 在标准错误流上输出I/O或其他错误的信息
   fprintf(stderr, "\n%s: I/O或其他错误，退出程序。下面可能是具体原因。\n", progName);
   
   // 输出具体的系统错误信息
   perror(progName);
   
   // 显示文件名信息
   showFileNames();
   
   // 清理资源并退出程序，返回值为1
   cleanUpAndFail( 1 );
}


/*
对于这两个函数的CUnit测试，由于它们都涉及到退出程序的操作，所以直接测试会导致测试终止。
但为了满足您的需求，我们可以使用子进程或mocking技术来测试它们的行为。以下是一个简单的CUnit测试示例：
*/

#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>

// 测试crcError函数
void test_crcError(void) {
    // 由于crcError会导致程序退出，所以在实际测试中应避免直接调用它。
    // 一个可能的做法是使用mocking技术来拦截并验证其输出或行为，或使用子进程执行它并检查返回值。
    // 这里只提供一个简单的示例。
    CU_FAIL("This test is just a placeholder and shouldn't be executed directly.");
}

// 测试ioError函数
void test_ioError(void) {
    // 由于ioError也会导致程序退出，所以同样应避免直接调用它。
    CU_FAIL("This test is just a placeholder and shouldn't be executed directly.");
}

// int main(void) {
//     CU_initialize_registry();
    
//     CU_pSuite suite1 = CU_add_suite("ErrorHandlingTests", 0, 0);
//     CU_add_test(suite1, "test_crcError", test_crcError);
//     CU_add_test(suite1, "test_ioError", test_ioError);
    
//     CU_basic_set_mode(CU_BRM_VERBOSE);
//     CU_basic_run_tests();
//     CU_cleanup_registry();
    
//     return 0;
// }
