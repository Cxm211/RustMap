#include "../typedef.h"
#include <CUnit/Basic.h>
#include <CUnit/CUnit.h>
// 该函数对fmap数组在指定范围[lo, hi]内进行排序。
// 排序是基于eclass数组中的值进行的，即fmap中的值用作eclass的索引。
void fallbackSimpleSort ( UInt32* fmap,    // 需要排序的数组
                          UInt32* eclass,  // 用于排序的辅助数组
                          Int32 lo,        // 排序范围的开始索引
                          Int32 hi )       // 排序范围的结束索引
{
   Int32 i, j, tmp;         // 临时变量
   UInt32 ec_tmp;           // 临时存储eclass的值

   // 如果lo和hi相等，即排序范围只有一个元素，则直接返回
   if (lo == hi) return;

   // 如果排序范围大于3个元素
   if (hi - lo > 3) {
      // 从倒数第四个元素开始，向前遍历
      for ( i = hi-4; i >= lo; i-- ) {
         tmp = fmap[i];
         ec_tmp = eclass[tmp];
         // 向后移动eclass值大于ec_tmp的所有元素
         for ( j = i+4; j <= hi && ec_tmp > eclass[fmap[j]]; j += 4 )
            fmap[j-4] = fmap[j];
         // 插入当前元素
         fmap[j-4] = tmp;
      }
   }

   // 对所有元素进行一般的插入排序
   for ( i = hi-1; i >= lo; i-- ) {
      tmp = fmap[i];
      ec_tmp = eclass[tmp];
      // 向后移动eclass值大于ec_tmp的所有元素
      for ( j = i+1; j <= hi && ec_tmp > eclass[fmap[j]]; j++ )
         fmap[j-1] = fmap[j];
      // 插入当前元素
      fmap[j-1] = tmp;
   }
}


/*
初始化fmap和eclass数组: 你可以选择一个你想要的fmap数组的大小，然后初始化这个数组。🌟
确保eclass数组的大小至少为fmap数组中的最大值 + 1。🌟
*/
void test_fallbackSimpleSort(void) {
    // 第一个测试用例
    UInt32 fmap1[] = {120243, 242, 184, 183, 177, 113, 54};
    UInt32 eclass1[120244] = {0}; // 初始化所有值为0
    eclass1[120243] = 30061;
    eclass1[242] = 0;
    eclass1[184] = 90183;
    eclass1[183] = 0;
    eclass1[177] = 60126;
    eclass1[113] = 30122;
    eclass1[54] = 90187;

    fallbackSimpleSort(fmap1, eclass1, 0, 6);

    UInt32 expected_fmap1[] = {242, 183, 120243, 113, 177, 184, 54};
    for(int i = 0; i < 7; i++) {
        CU_ASSERT_EQUAL(fmap1[i], expected_fmap1[i]);
    }
    // 第二个测试用例
    UInt32 fmap2[] = {210, 128, 220, 145, 159, 106};
    UInt32 eclass2[221] = {0}; // 初始化所有值为0
    eclass2[210] = 90200;
    eclass2[128] = 90187;
    eclass2[220] = 90187;
    eclass2[145] = 90200;
    eclass2[159] = 90200;
    eclass2[106] = 90200;

    fallbackSimpleSort(fmap2, eclass2, 0, 5);

    UInt32 expected_fmap2[] = {128, 220, 210, 145, 159, 106};
    for(int i = 0; i < 6; i++) {
        CU_ASSERT_EQUAL(fmap2[i], expected_fmap2[i]);
    }

}



int main() {
    // 初始化CUnit test registry
    if (CUE_SUCCESS != CU_initialize_registry())
        return CU_get_error();

    // 添加测试用例
    CU_pSuite pSuite = NULL;
    pSuite = CU_add_suite("TestSuite_fallbackSimpleSort", 0, 0);
    if (NULL == pSuite) {
        CU_cleanup_registry();
        return CU_get_error();
    }
    
    // 添加测试函数到测试集
    if ((NULL == CU_add_test(pSuite, "test_fallbackSimpleSort", test_fallbackSimpleSort))) {
        CU_cleanup_registry();
        return CU_get_error();
    }

    // 运行所有测试
    CU_basic_set_mode(CU_BRM_VERBOSE);
    CU_basic_run_tests();
    CU_cleanup_registry();
    return CU_get_error();
}
