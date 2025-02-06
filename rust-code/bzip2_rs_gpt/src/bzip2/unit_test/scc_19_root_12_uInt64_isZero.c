


#include "../typedef.h"
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

void test_uInt64_isZero_True(void) {
   UInt64 n = {{0, 0, 0, 0, 0, 0, 0, 0}};
   CU_ASSERT_TRUE(uInt64_isZero(&n));
}

void test_uInt64_isZero_False(void) {
   UInt64 n = {{0, 0, 0, 0, 0, 1, 0, 0}};
   CU_ASSERT_FALSE(uInt64_isZero(&n));
}




