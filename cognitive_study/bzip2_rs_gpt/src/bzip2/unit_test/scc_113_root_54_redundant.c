#include "../typedef.h"
#include <stdio.h>
#include <stdbool.h>
#include <stdarg.h>  // 引入stdarg.h头文件
#include <string.h>
#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>

bool mock_fprintf_called = false;
char captured_msg[1024];

int mock_fprintf(FILE *stream, const char *format, ...) {
    mock_fprintf_called = true;
    va_list args;
    va_start(args, format);
    vsprintf(captured_msg, format, args);
    va_end(args);
    return 0;
}

#define fprintf mock_fprintf


static
void redundant ( Char* flag )
{
   fprintf (
     stderr,
      "%s: %s is redundant in versions 0.9.5 and above\n",
      progName, flag );
}

void test_redundant(void) {

    // char progName[1024] = "testProg";  // 初始化progName
    // 为progName赋一个测试值
    strcpy(progName, "testProg");

    // 调用函数
    redundant("testFlag");
    
    // 确保fprintf被调用
    CU_ASSERT_TRUE(mock_fprintf_called);
    
    // 确保打印的信息正确
    CU_ASSERT_STRING_EQUAL(captured_msg, "testProg: testFlag is redundant in versions 0.9.5 and above\n");
}

int main(void) {
    CU_initialize_registry();
    
    CU_pSuite suite1 = CU_add_suite("RedundantTests", 0, 0);
    CU_add_test(suite1, "test_redundant", test_redundant);
    
    CU_basic_set_mode(CU_BRM_VERBOSE);
    CU_basic_run_tests();
    CU_cleanup_registry();
    
    return 0;
}
