#include "../typedef.h"
// 检查给定名称是否包含可疑字符。
// 目前的实现总是返回False，表示不包含任何可疑字符。
static
Bool containsDubiousChars ( Char* name )
{
   // 目前直接返回False
   return ((Bool)0);
}

#include <CUnit/Basic.h>
void test_containsDubiousChars(void) {
    // 因为函数总是返回False，所以任何输入都应该返回False
    CU_ASSERT_EQUAL(containsDubiousChars("someName"), 0);
    CU_ASSERT_EQUAL(containsDubiousChars("anotherName"), 0);
}

// int main() {
//     CU_pSuite pSuite = NULL;

//     /* 初始化CUnit库 */
//     if (CUE_SUCCESS != CU_initialize_registry())
//         return CU_get_error();

//     /* 添加一个Suite到测试注册表 */
//     pSuite = CU_add_suite("containsDubiousChars_test_suite", NULL, NULL);
//     if (NULL == pSuite) {
//         CU_cleanup_registry();
//         return CU_get_error();
//     }

//     /* 在Suite中添加测试用例 */
//     if ((NULL == CU_add_test(pSuite, "test_containsDubiousChars", test_containsDubiousChars))) {
//         CU_cleanup_registry();
//         return CU_get_error();
//     }

//     /* 运行所有测试用CUnit Basic接口 */
//     CU_basic_set_mode(CU_BRM_VERBOSE);
//     CU_basic_run_tests();
//     CU_cleanup_registry();
//     return CU_get_error();
// }
