
#include "../typedef.h"
/**
 * 获取三个UChar中的中间值。
 * @param a 第一个UChar。
 * @param b 第二个UChar。
 * @param c 第三个UChar。
 * @return 返回三者中的中间值。
 */
static __inline__ UChar mmed3 ( UChar a, UChar b, UChar c ) {
   UChar t;  // 临时变量，用于交换
   if (a > b) { t = a; a = b; b = t; };  // 如果a>b，则交换它们
   if (b > c) {
      b = c;  // 如果b>c，则b的值变为c
      if (a > b) b = a;  // 如果a现在大于b，则b的值变为a
   }
   return b;  // 返回中间的值
}


#include <CUnit/Basic.h>
#include <stdio.h>

// 假设你已经有了上面的mmed3函数定义...

void test_mmed3(void) {
   CU_ASSERT(mmed3(1,2,3) == 2);  // 中间值是2
   CU_ASSERT(mmed3(3,1,2) == 2);  // 中间值是2
   CU_ASSERT(mmed3(2,3,1) == 2);  // 中间值是2
   CU_ASSERT(mmed3(5,5,5) == 5);  // 所有值都相同
   CU_ASSERT(mmed3(0,255,128) == 128);  // 测试极限值
}

// int main(void) {
//    CU_pSuite pSuite = NULL;

//    /* 初始化CUnit库 */
//    if (CUE_SUCCESS != CU_initialize_registry())
//       return CU_get_error();

//    /* 添加测试套件到注册中心 */
//    pSuite = CU_add_suite("mmed3_test_suite", NULL, NULL);
//    if (NULL == pSuite) {
//       CU_cleanup_registry();
//       return CU_get_error();
//    }

//    /* 添加测试函数到套件 */
//    if (NULL == CU_add_test(pSuite, "test_mmed3", test_mmed3)) {
//       CU_cleanup_registry();
//       return CU_get_error();
//    }

//    /* 运行所有测试 */
//    CU_basic_set_mode(CU_BRM_VERBOSE);
//    CU_basic_run_tests();
//    CU_cleanup_registry();

//    return CU_get_error();
// }
