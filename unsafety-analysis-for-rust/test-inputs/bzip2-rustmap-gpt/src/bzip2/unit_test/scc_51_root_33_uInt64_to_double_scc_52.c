// use crate::global_vars::bzip2_c1::{UInt64};

#include "../typedef.h"
static
double uInt64_to_double ( UInt64* n )
{
   Int32 i;
   double base = 1.0;
   double sum = 0.0;
   for (i = 0; i < 8; i++) {
      sum += base * (double)(n->b[i]);
      base *= 256.0;
   }
   return sum;
}


// 将两个32位无符号整数合并成一个64位无符号整数
static void uInt64_from_UInt32s ( UInt64* n, UInt32 lo32, UInt32 hi32 )
{
   // 从高到低提取hi32的每个字节，并存放在UInt64的高4字节中
   n->b[7] = (UChar)((hi32 >> 24) & 0xFF);
   n->b[6] = (UChar)((hi32 >> 16) & 0xFF);
   n->b[5] = (UChar)((hi32 >> 8) & 0xFF);
   n->b[4] = (UChar) (hi32 & 0xFF);

   // 从高到低提取lo32的每个字节，并存放在UInt64的低4字节中
   n->b[3] = (UChar)((lo32 >> 24) & 0xFF);
   n->b[2] = (UChar)((lo32 >> 16) & 0xFF);
   n->b[1] = (UChar)((lo32 >> 8) & 0xFF);
   n->b[0] = (UChar) (lo32 & 0xFF);
}


#include <CUnit/Basic.h>
#include <stdio.h>


// 测试函数
static void test_uInt64_to_double(void) {
   UInt64 number;
   // 测试用例1：全部字节都为0
   for (int i = 0; i < 8; i++) {
      number.b[i] = 0;
   }
   CU_ASSERT_DOUBLE_EQUAL(uInt64_to_double(&number), 0.0, 0.000001);

   // 测试用例2：最低位字节为1，其余为0
   for (int i = 0; i < 8; i++) {
      number.b[i] = 0;
   }
   number.b[0] = 1;
   CU_ASSERT_DOUBLE_EQUAL(uInt64_to_double(&number), 1.0, 0.000001);

   // ... 您可以添加更多测试用例 ...
}
// 测试函数
static void test_uInt64_from_UInt32s(void) {
   UInt64 number;
   // 测试用例1
   uInt64_from_UInt32s(&number, 0xFFFFFFFF, 0xFFFFFFFF);
   CU_ASSERT(number.b[7] == 0xFF);
   CU_ASSERT(number.b[6] == 0xFF);
   CU_ASSERT(number.b[5] == 0xFF);
   CU_ASSERT(number.b[4] == 0xFF);
   CU_ASSERT(number.b[3] == 0xFF);
   CU_ASSERT(number.b[2] == 0xFF);
   CU_ASSERT(number.b[1] == 0xFF);
   CU_ASSERT(number.b[0] == 0xFF);

   // 测试用例2
   uInt64_from_UInt32s(&number, 0x12345678, 0x90ABCDEF);
   CU_ASSERT(number.b[7] == 0x90);
   CU_ASSERT(number.b[6] == 0xAB);
   CU_ASSERT(number.b[5] == 0xCD);
   CU_ASSERT(number.b[4] == 0xEF);
   CU_ASSERT(number.b[3] == 0x12);
   CU_ASSERT(number.b[2] == 0x34);
   CU_ASSERT(number.b[1] == 0x56);
   CU_ASSERT(number.b[0] == 0x78);

   // ... 您可以继续添加其他测试用例 ...
}

int main(void) {
   CU_pSuite pSuite = NULL;

   // 初始化CUnit库
   if (CUE_SUCCESS != CU_initialize_registry()) {
      return CU_get_error();
   }

   // 添加一个测试套件
   pSuite = CU_add_suite("uInt64_from_UInt32s Suite", NULL, NULL);
   if (NULL == pSuite) {
      CU_cleanup_registry();
      return CU_get_error();
   }


   // 添加一个测试套件
   pSuite = CU_add_suite("uInt64_to_double Suite", NULL, NULL);
   if (NULL == pSuite) {
      CU_cleanup_registry();
      return CU_get_error();
   }

   // 在这个套件中添加测试用例
   if (NULL == CU_add_test(pSuite, "test_uInt64_from_UInt32s", test_uInt64_from_UInt32s)) {
      CU_cleanup_registry();
      return CU_get_error();
   }

   // 在这个套件中添加测试用例
   if (NULL == CU_add_test(pSuite, "test_uInt64_to_double", test_uInt64_to_double)) {
      CU_cleanup_registry();
      return CU_get_error();
   }


   // 运行所有测试
   CU_basic_set_mode(CU_BRM_VERBOSE);
   CU_basic_run_tests();
   CU_cleanup_registry();
   return CU_get_error();
}
