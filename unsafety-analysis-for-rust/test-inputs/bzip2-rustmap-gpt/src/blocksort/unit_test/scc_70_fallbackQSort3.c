// use crate::compress::scc_32_root_22_bsFinishWrite::*;
// use crate::decompress::scc_63_BZ2_decompress::*;
#include "../typedef.h"
#include "./scc_69_mainSort.c"
#include "../bzlib/scc_62_BZ2_bz__AssertH__fail.c"


/**
 * 使用基于三路划分的快速排序对fmap数组进行排序。
 *
 * @param fmap    待排序的数组
 * @param eclass  用作排序基准的数组
 * @param loSt    排序开始的位置
 * @param hiSt    排序结束的位置
 */
static
void fallbackQSort3 ( UInt32* fmap,
                      UInt32* eclass,
                      Int32 loSt,
                      Int32 hiSt )
{
   // 声明并初始化一些局部变量
   Int32 unLo, unHi, ltLo, gtHi, n, m;
   Int32 sp, lo, hi;
   UInt32 med, r, r3;
   Int32 stackLo[100];
   Int32 stackHi[100];

   r = 0;

   // 初始化栈
   sp = 0;
   { stackLo[sp] = loSt; stackHi[sp] = hiSt; sp++; };

   // 当栈不为空时，持续排序
   while (sp > 0) {

      // 确保栈不会溢出
      { if (!(sp < 100 - 1)) BZ2_bz__AssertH__fail ( 1004 ); };

      // 从栈中取出一个待处理的区间
      { sp--; lo = stackLo[sp]; hi = stackHi[sp]; };

      // 对于小的区间，使用简单排序
      if (hi - lo < 10) {
         fallbackSimpleSort ( fmap, eclass, lo, hi );
         continue;
      }
      // 计算用于划分数组的中值(med)
      r = ((r * 7621) + 1) % 32768;
      r3 = r % 3;
      if (r3 == 0) med = eclass[fmap[lo]];
      else if (r3 == 1) med = eclass[fmap[(lo+hi)>>1]];
      else med = eclass[fmap[hi]];

      unLo = ltLo = lo;
      unHi = gtHi = hi;

      // 使用三路划分的快速排序
      while (1) {
         // 将等于中值的元素移到左边
         while (1) {
            if (unLo > unHi) break;
            n = (Int32)eclass[fmap[unLo]] - (Int32)med;
            if (n == 0) {
               { Int32 zztmp = fmap[unLo]; fmap[unLo] = fmap[ltLo]; fmap[ltLo] = zztmp; };
               ltLo++; unLo++;
               continue;
            };
            if (n > 0) break;
            unLo++;
         }

         // 将等于中值的元素移到右边
         while (1) {
            if (unLo > unHi) break;
            n = (Int32)eclass[fmap[unHi]] - (Int32)med;
            if (n == 0) {
               { Int32 zztmp = fmap[unHi]; fmap[unHi] = fmap[gtHi]; fmap[gtHi] = zztmp; };
               gtHi--; unHi--;
               continue;
            };
            if (n < 0) break;
            unHi--;
         }

         // 当左右指针相遇时，结束当前的循环
         if (unLo > unHi) break;

         // 交换左右指针所指向的元素
         { Int32 zztmp = fmap[unLo]; fmap[unLo] = fmap[unHi]; fmap[unHi] = zztmp; }; unLo++; unHi--;
      }

      // 将等于中值的元素移回到中间
      n = ((ltLo-lo) < (unLo-ltLo)) ? (ltLo-lo) : (unLo-ltLo);
      m = ((hi-gtHi) < (gtHi-unHi)) ? (hi-gtHi) : (gtHi-unHi);
      // 下面是两个循环，用于完成元素的交换操作
      { Int32 yyp1 = (lo); Int32 yyp2 = (unLo-n); Int32 yyn = (n); while (yyn > 0) { { Int32 zztmp = fmap[yyp1]; fmap[yyp1] = fmap[yyp2]; fmap[yyp2] = zztmp; }; yyp1++; yyp2++; yyn--; } };
      { Int32 yyp1 = (unLo); Int32 yyp2 = (hi-m+1); Int32 yyn = (m); while (yyn > 0) { { Int32 zztmp = fmap[yyp1]; fmap[yyp1] = fmap[yyp2]; fmap[yyp2] = zztmp; }; yyp1++; yyp2++; yyn--; } };

      // 将子区间入栈，等待下一轮处理
      n = lo + unLo - ltLo - 1;
      m = hi - (gtHi - unHi) + 1;
      if (n - lo > hi - m) {
         { stackLo[sp] = lo; stackHi[sp] = n; sp++; };
         { stackLo[sp] = m; stackHi[sp] = hi; sp++; };
      } else {
         { stackLo[sp] = m; stackHi[sp] = hi; sp++; };
         { stackLo[sp] = lo; stackHi[sp] = n; sp++; };
      }
   }
}




#include <stdio.h>
#include <CUnit/Basic.h>

// 之前的代码和fallbackQSort3函数...

// 定义一个测试函数
void test_fallbackQSort3() {
   UInt32 fmap[] = {4, 3, 2, 10, 12, 1, 5, 6};
   UInt32 eclass[] = {4, 3, 2, 10, 12, 1, 5, 6};
   Int32 len = sizeof(fmap) / sizeof(fmap[0]);
   
   fallbackQSort3(fmap, eclass, 0, len-1);

   for (Int32 i = 1; i < len; i++) {
      CU_ASSERT(fmap[i-1] <= fmap[i]);
   }
}

int main() {
   if (CUE_SUCCESS != CU_initialize_registry()) {
      return CU_get_error();
   }

   CU_pSuite pSuite = NULL;
   pSuite = CU_add_suite("fallbackQSort3_test_suite", NULL, NULL);
   if (NULL == pSuite) {
      CU_cleanup_registry();
      return CU_get_error();
   }

   if (NULL == CU_add_test(pSuite, "test_fallbackQSort3", test_fallbackQSort3)) {
      CU_cleanup_registry();
      return CU_get_error();
   }

   CU_basic_set_mode(CU_BRM_VERBOSE);
   CU_basic_run_tests();
   CU_cleanup_registry();

   return CU_get_error();
}
