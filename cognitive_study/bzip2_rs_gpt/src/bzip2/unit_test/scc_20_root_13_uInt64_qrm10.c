


#include "../typedef.h"
static
Int32 uInt64_qrm10(UInt64* n) {
   UInt32 rem, tmp;  // 定义余数和临时变量
   Int32 i;
   rem = 0;  // 初始化余数为0
   // 从最低位开始，往最高位遍历
   for (i = 7; i >= 0; i--) {
      tmp = rem * 256 + n->b[i];  // 通过乘256将余数左移一个字节，然后加上当前字节的值
      n->b[i] = tmp / 10;        // 用tmp除以10，将得到的商存回当前字节
      rem = tmp % 10;            // 更新余数为tmp除以10的余数
   }
   return rem;  // 返回最终的余数
}

#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>


void test_uInt64_qrm10_zero_remainder(void) {
   UInt64 n = {{10, 0, 0, 0, 0, 0, 0, 0}};
   Int32 rem = uInt64_qrm10(&n);
   CU_ASSERT_EQUAL(rem, 0);
   CU_ASSERT_EQUAL(n.b[7], 1);
   for (int i = 0; i < 7; i++) {
      CU_ASSERT_EQUAL(n.b[i], 0);
   }
}

void test_uInt64_qrm10_non_zero_remainder(void) {
   UInt64 n = {{255, 0, 0, 0, 0, 0, 0, 0}};
   Int32 rem = uInt64_qrm10(&n);
   CU_ASSERT_EQUAL(rem, 5);
   CU_ASSERT_EQUAL(n.b[7], 25);
   for (int i = 0; i < 7; i++) {
      CU_ASSERT_EQUAL(n.b[i], 0);
   }
}




