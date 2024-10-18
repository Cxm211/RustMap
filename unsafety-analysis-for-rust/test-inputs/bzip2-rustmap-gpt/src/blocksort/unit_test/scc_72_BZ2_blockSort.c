#include "../typedef.h"
#include "./scc_71_fallbackSort.c"
#include "./scc_69_mainSort.c"
void BZ2_blockSort ( EState* s )
{
   UInt32* ptr = s->ptr;  // 指向ptr数组的指针
   UChar* block = s->block; // 指向block数组的指针
   UInt32* ftab = s->ftab; // 指向ftab数组的指针
   Int32 nblock = s->nblock;  // block的大小
   Int32 verb = s->verbosity;  // 输出详细级别
   Int32 wfact = s->workFactor; // 工作因子，影响排序预算
   UInt16* quadrant;  
   Int32 budget;   // 排序预算
   Int32 budgetInit; // 初始预算
   Int32 i;  // 循环变量

   // 如果block小于10000，使用fallbackSort进行排序
   if (nblock < 10000) {
      fallbackSort ( s->arr1, s->arr2, ftab, nblock, verb );
   } else {
      i = nblock+(2 + 12 + 18 + 2);
      if (i & 1) i++;
      quadrant = (UInt16*)(&(block[i]));
      if (wfact < 1 ) wfact = 1;
      if (wfact > 100) wfact = 100;
      budgetInit = nblock * ((wfact-1) / 3);
      budget = budgetInit;

      mainSort ( ptr, block, quadrant, ftab, nblock, verb, &budget );
      // 如果详细级别>=3，则输出排序信息
      if (verb >= 3)
         fprintf(
        stderr
        ,"      %d work, %d block, ratio %5.2f\n",budgetInit - budget,nblock,(float)(budgetInit - budget) / (float)(nblock==0 ? 1 : nblock))



                                                     ;
    // 如果预算小于0，使用fallbackSort进行排序
      if (budget < 0) {
         if (verb >= 2)
            fprintf(
           stderr
           ,"    too repetitive; using fallback" " sorting algorithm\n")
                                               ;
         fallbackSort ( s->arr1, s->arr2, ftab, nblock, verb );
      }
   }

   s->origPtr = -1; // 初始化origPtr为-1
    // 寻找ptr数组中值为0的元素
   for (i = 0; i < s->nblock; i++)
      if (ptr[i] == 0)
         { s->origPtr = i; break; };
// 确保找到了值为0的元素，否则触发断言
   { if (!(s->origPtr != -1)) BZ2_bz__AssertH__fail ( 1003 ); };
}

#include <CUnit/Basic.h>

void test_BZ2_blockSort(void) {
   EState state;
   // 初始化state的内容...
   BZ2_blockSort(&state);
   // 在此可以添加断言来验证函数的结果
   CU_ASSERT_EQUAL(state.origPtr, expected_value);
}

int main() {
   CU_pSuite pSuite = NULL;

   /* 初始化CUnit测试注册表 */
   if (CUE_SUCCESS != CU_initialize_registry())
      return CU_get_error();

   /* 添加测试套件到注册表 */
   pSuite = CU_add_suite("TestSuite_BZ2_blockSort", init_suite, clean_suite);
   if (NULL == pSuite) {
      CU_cleanup_registry();
      return CU_get_error();
   }

   /* 添加测试用例到套件 */
   if ((NULL == CU_add_test(pSuite, "test_BZ2_blockSort", test_BZ2_blockSort))) {
      CU_cleanup_registry();
      return CU_get_error();
   }

   // 运行所有测试用例
   CU_basic_set_mode(CU_BRM_VERBOSE);
   CU_basic_run_tests();
   CU_cleanup_registry();
   return CU_get_error();
}
