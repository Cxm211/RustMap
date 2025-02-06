

#include "../typedef.h"
#include "./scc_20_root_13_uInt64_qrm10.c"
/* 将 UInt64 类型的数字转换为其 ASCII 表示 */
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
static
void uInt64_toAscii ( char* outbuf, UInt64* n )
{
   Int32 i, q;
   UChar buf[32]; // 用于临时存储数字的每一位
   Int32 nBuf = 0; // 记录已经存放的数字位数
   UInt64 n_copy = *n; // 复制 n 的值，以防止修改原始数据
   
   // 从个位开始，不断对 n_copy 进行 10 的除法，并保存余数
   do {
      q = uInt64_qrm10 ( &n_copy ); 
      buf[nBuf] = q + '0'; // 将余数转换为字符并保存
      nBuf++;
   } while (!uInt64_isZero(&n_copy)); // 直到 n_copy 为 0 为止
   
   outbuf[nBuf] = 0; // 设置字符串结束标志
   // 将 buf 中的数字反转到 outbuf 中
   for (i = 0; i < nBuf; i++)
      outbuf[i] = buf[nBuf-i-1];
}

#include "CUnit/Basic.h"

void test_uInt64_toAscii(void) {
   UInt64 number = {{12345, 0, 0, 0, 0, 0, 0, 0}}; // 假设这是12345的UInt64表示
   char output[32];
   
   uInt64_toAscii(output, &number);

   CU_ASSERT_STRING_EQUAL(output, "12345");
}

int main() {
   CU_pSuite pSuite = NULL;

   /* 初始化 CUnit 库 */
   if (CUE_SUCCESS != CU_initialize_registry())
      return CU_get_error();

   /* 添加一个测试组 */
   pSuite = CU_add_suite("uInt64_toAscii_test_suite", NULL, NULL);
   if (NULL == pSuite) {
      CU_cleanup_registry();
      return CU_get_error();
   }

   /* 添加测试案例到测试组 */
   if ((NULL == CU_add_test(pSuite, "test_uInt64_toAscii", test_uInt64_toAscii))) {
      CU_cleanup_registry();
      return CU_get_error();
   }

   /* 运行所有测试 */
   CU_basic_set_mode(CU_BRM_VERBOSE);
   CU_basic_run_tests();
   CU_cleanup_registry();
   return CU_get_error();
}
