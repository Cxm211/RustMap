#include "../typedef.h"
#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>

// 定义函数 setExit
// static
// void setExit ( Int32 v )
// {
//    // 如果传入的值 v 大于当前的 exitValue，则更新 exitValue
//    if (v > exitValue) exitValue = v;
// }


// void test_setExit_noUpdate() {
//     exitValue = 10;  // 设置初始值
//     setExit(5);      // 尝试更新

//     CU_ASSERT_EQUAL(exitValue, 10);  // 检查值仍然是10
// }

// int main() {
//     CU_initialize_registry();

//     CU_pSuite suite = CU_add_suite("Test setExit", NULL, NULL);

//     CU_add_test(suite, "test_setExit_updateValue", test_setExit_noUpdate);
//     CU_add_test(suite, "test_setExit_noUpdate", test_setExit_noUpdate);

//     CU_basic_set_mode(CU_BRM_VERBOSE);
//     CU_basic_run_tests();

//     CU_cleanup_registry();
//     return 0;
// }
