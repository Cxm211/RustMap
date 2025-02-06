

#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>

typedef int Int32;
void test_BZ2_indexIntoF(void) {
    Int32 cftab[257];
    for (Int32 i = 0; i < 257; i++) {
        cftab[i] = i * 10;  // 为简单起见，每个索引位置的值是其索引的10倍
    }

    CU_ASSERT_EQUAL(BZ2_indexIntoF(50, cftab), 5);
    CU_ASSERT_EQUAL(BZ2_indexIntoF(150, cftab), 15);
    CU_ASSERT_EQUAL(BZ2_indexIntoF(2550, cftab), 255);
}

int main(void) {
    CU_pSuite pSuite = NULL;

    /* 初始化CUnit库 */
    if (CUE_SUCCESS != CU_initialize_registry())
        return CU_get_error();

    /* 添加测试套件 */
    pSuite = CU_add_suite("Suite_1", NULL, NULL);
    if (NULL == pSuite) {
        CU_cleanup_registry();
        return CU_get_error();
    }

    /* 添加测试用例到套件 */
    if ((NULL == CU_add_test(pSuite, "test_BZ2_indexIntoF", test_BZ2_indexIntoF))) {
        CU_cleanup_registry();
        return CU_get_error();
    }

    /* 运行所有测试 */
    CU_basic_set_mode(CU_BRM_VERBOSE);
    CU_basic_run_tests();
    CU_cleanup_registry();
    return CU_get_error();
}

__inline__ Int32 BZ2_indexIntoF ( Int32 indx, Int32 *cftab )
{
   // 定义搜索范围的开始和结束
   Int32 nb, na, mid;
   nb = 0;
   na = 256;

   // 用二分查找在cftab中搜索indx
   do {
      // 计算中点
      mid = (nb + na) >> 1;

      // 根据中点位置更新搜索范围
      if (indx >= cftab[mid]) nb = mid; else na = mid;
   } 
   while (na - nb != 1); // 继续循环直到搜索范围缩小到1

   // 返回找到的位置
   return nb;
}

