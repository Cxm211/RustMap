
// BZ2_bzlibVersion (
//       void
//    )

const char * BZ2_bzlibVersion(void)
{
   return "1.0.8, 13-Jul-2019";
}

#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>
#include <string.h>  // 为了使用strcmp函数

// 假设您已经包含了必要的头文件和定义

void test_BZ2_bzlibVersion(void) {
    // 调用BZ2_bzlibVersion函数并检查返回值是否与预期相符
    const char * version = BZ2_bzlibVersion();
    CU_ASSERT_STRING_EQUAL(version, "1.0.8, 13-Jul-2019");
}

// int main() {
//     CU_initialize_registry();

//     CU_pSuite suite = CU_add_suite("Test BZ2_bzlibVersion", NULL, NULL);

//     CU_add_test(suite, "test_BZ2_bzlibVersion", test_BZ2_bzlibVersion);

//     CU_basic_set_mode(CU_BRM_VERBOSE);
//     CU_basic_run_tests();

//     CU_cleanup_registry();
//     return 0;
// }
