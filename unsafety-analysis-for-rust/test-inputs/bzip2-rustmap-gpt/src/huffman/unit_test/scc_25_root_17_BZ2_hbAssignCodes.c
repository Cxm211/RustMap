
// 为给定的字母表（或字符集）分配霍夫曼编码。

// 参数解释：

// code：这是一个整数数组，用于存储由此函数为每个字母表中的字符计算出的霍夫曼编码。
// length：这是一个字符数组，表示字母表中每个字符的霍夫曼编码长度。
// minLen和maxLen：表示霍夫曼编码的最小和最大长度。
// alphaSize：表示字母表的大小或字符集的大小。
// 函数的工作机制如下：

// 对于霍夫曼编码的每个可能的长度（从minLen到maxLen），检查字母表中的每个字符是否具有该长度的编码。
// 如果字符的编码长度与当前长度匹配，则为该字符分配一个代码，并递增vec变量以在下次迭代中为下一个字符生成新的编码。
// 在处理完给定长度的所有字符后，vec值左移一位，为下一个编码长度准备

#include "../typedef.h"
// 为给定的字母表分配霍夫曼编码
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
