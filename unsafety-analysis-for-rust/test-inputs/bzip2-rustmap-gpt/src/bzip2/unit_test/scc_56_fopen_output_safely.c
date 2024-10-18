#include "../typedef.h"

#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>
// 安全地打开一个用于输出的文件

static
FILE* fopen_output_safely ( Char* name, const char* mode )
{

   FILE* fp;
   IntNative fh;
   fh = open(name, 
                  01
                          |
                           0100
                                  |
                                   0200
                                         , 
                                           0200
                                                  |
                                                   0400
                                                          );
   if (fh == -1) return 
                       ((void *)0)
                           ;
   fp = fdopen(fh, mode);
   if (fp == 
            ((void *)0)
                ) close(fh);
   return fp;



}


// 测试fopen_output_safely函数
void test_fopen_output_safely() {
    FILE* fp;

    // 使用fopen_output_safely创建并打开文件
    fp = fopen_output_safely("test.txt", "w");
    CU_ASSERT_PTR_NOT_NULL(fp);  // 断言文件指针不为NULL，表示文件成功打开
    if (fp != NULL) fclose(fp);

    // 为确保文件确实被创建，我们再次使用fopen来打开它
    fp = fopen("test.txt", "r");
    CU_ASSERT_PTR_NOT_NULL(fp);  // 断言文件指针不为NULL，表示文件确实存在并被成功打开
    if (fp != NULL) fclose(fp);

    // 删除测试文件（可选，这样可以确保每次测试都是从一个清洁的状态开始）
    remove("test.txt");
}


// int main() {
//     CU_pSuite pSuite = NULL;

//     // 初始化CUnit库
//     if (CUE_SUCCESS != CU_initialize_registry())
//         return CU_get_error();

//     // 添加一个测试套件
//     pSuite = CU_add_suite("Suite_1", NULL, NULL);
//     if (NULL == pSuite) {
//         CU_cleanup_registry();
//         return CU_get_error();
//     }

//     // 在测试套件中添加测试用例
//     if (NULL == CU_add_test(pSuite, "test of fopen_output_safely", test_fopen_output_safely)) {
//         CU_cleanup_registry();
//         return CU_get_error();
//     }

//     // 运行所有测试
//     CU_basic_run_tests();

//     // 清理CUnit并退出
//     CU_cleanup_registry();
//     return CU_get_error();
// }