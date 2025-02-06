
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


void test_mmed3(void) {
   CU_ASSERT(mmed3(1,2,3) == 2);  // 中间值是2
   CU_ASSERT(mmed3(3,1,2) == 2);  // 中间值是2
   CU_ASSERT(mmed3(2,3,1) == 2);  // 中间值是2
   CU_ASSERT(mmed3(5,5,5) == 5);  // 所有值都相同
   CU_ASSERT(mmed3(0,255,128) == 128);  // 测试极限值
}






