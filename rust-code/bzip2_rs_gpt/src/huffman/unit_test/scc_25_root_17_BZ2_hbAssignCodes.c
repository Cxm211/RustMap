




#include "../typedef.h"
void BZ2_hbAssignCodes ( Int32 *code,
                         UChar *length,
                         Int32 minLen,
                         Int32 maxLen,
                         Int32 alphaSize )
{
   Int32 n, vec, i;

   vec = 0;  // 初始化编码向量
   // 遍历所有可能的霍夫曼编码长度
   for (n = minLen; n <= maxLen; n++) {
      // 检查字母表中的每个字符
      for (i = 0; i < alphaSize; i++)
         // 如果字符的编码长度与当前长度匹配
         if (length[i] == n) {
            // 为该字符分配一个代码
            code[i] = vec;
            // 为下一个字符递增编码向量
            vec++;
         };
      // 为下一个编码长度准备，左移编码向量
      vec <<= 1;
   }
}

#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>

void test_BZ2_hbAssignCodes(void) {
    Int32 code[5];
    UChar length[5] = {2, 2, 3, 3, 4};
    BZ2_hbAssignCodes(code, length, 2, 4, 5);

    CU_ASSERT_EQUAL(code[0], 0);  // 00
    CU_ASSERT_EQUAL(code[1], 1);  // 01
    CU_ASSERT_EQUAL(code[2], 2);  // 010
    CU_ASSERT_EQUAL(code[3], 3);  // 011
    CU_ASSERT_EQUAL(code[4], 4);  // 0100
}

int main(void) {
    CU_initialize_registry();

    CU_pSuite suite = CU_add_suite("BZ2_hbAssignCodes_suite", NULL, NULL);
    CU_add_test(suite, "test_BZ2_hbAssignCodes", test_BZ2_hbAssignCodes);

    CU_basic_set_mode(CU_BRM_VERBOSE);
    CU_basic_run_tests();
    CU_cleanup_registry();
    return CU_get_error();
}
