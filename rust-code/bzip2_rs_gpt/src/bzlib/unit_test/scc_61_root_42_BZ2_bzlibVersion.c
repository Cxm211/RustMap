

const char * BZ2_bzlibVersion(void)
{
   return "1.0.8, 13-Jul-2019";
}

#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>
#include <string.h>  // 为了使用strcmp函数


void test_BZ2_bzlibVersion(void) {
    // 调用BZ2_bzlibVersion函数并检查返回值是否与预期相符
    const char * version = BZ2_bzlibVersion();
    CU_ASSERT_STRING_EQUAL(version, "1.0.8, 13-Jul-2019");
}





