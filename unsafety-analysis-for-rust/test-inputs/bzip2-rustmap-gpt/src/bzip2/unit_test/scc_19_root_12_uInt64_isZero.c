// use crate::global_vars::bzip2_c1::{UInt64};


// uInt64_isZero ( UInt64* n )

#include "../typedef.h"
// 检查UInt64是否为0的函数
static
Bool uInt64_isZero(UInt64* n) {
   Int32 i;
   // 遍历这8个字节
   for (i = 0; i < 8; i++) {
      // 如果任意一个字节不为0，返回0（假）
      if (n->b[i] != 0) return 0;
   }
   // 所有字节都是0，返回1（真）
   return 1;
}

#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>
// 包含你的头文件

void test_uInt64_isZero_True(void) {
   UInt64 n = {{0, 0, 0, 0, 0, 0, 0, 0}};
   CU_ASSERT_TRUE(uInt64_isZero(&n));
}

void test_uInt64_isZero_False(void) {
   UInt64 n = {{0, 0, 0, 0, 0, 1, 0, 0}};
   CU_ASSERT_FALSE(uInt64_isZero(&n));
}

// int main() {
//    CU_initialize_registry();
//    CU_pSuite suite = CU_add_suite("uInt64_isZero_suite", NULL, NULL);

//    CU_add_test(suite, "test_uInt64_isZero_True", test_uInt64_isZero_True);
//    CU_add_test(suite, "test_uInt64_isZero_False", test_uInt64_isZero_False);

//    CU_basic_run_tests();
//    CU_cleanup_registry();

//    return 0;
// }
