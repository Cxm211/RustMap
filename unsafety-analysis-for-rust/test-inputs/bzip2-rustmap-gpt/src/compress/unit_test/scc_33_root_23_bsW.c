// use crate::global_vars::bzip2_c1::{EState};


#include "../typedef.h"
/**
 * 向BZip2的二进制数据流中写入指定数量的位。
 * @param s 指向EState结构体实例的指针。
 * @param n 要写入的位的数量。
 * @param v 要写入的值。
 */
static __inline__ void bsW ( EState* s, Int32 n, UInt32 v ) {
   // 如果bsBuff中的挂起位数量超过或等于8位，则将高8位写入到zbits数组，并更新bsBuff和bsLive。
   while (s->bsLive >= 8) {
      s->zbits[s->numZ] = (UChar)(s->bsBuff >> 24);
      s->numZ++;
      s->bsBuff <<= 8;
      s->bsLive -= 8;
   }

   // 将值v写入到bsBuff的适当位置。
   s->bsBuff |= (v << (32 - s->bsLive - n));

   // 更新bsLive，表示现在有多少挂起的位在bsBuff中。
   s->bsLive += n;
}

#include <CUnit/Basic.h>
#include <stdio.h>

// 假设你已经定义了上述的EState结构体和bsW函数...

void test_bsW(void) {
   EState state;
   state.bsBuff = 0x0;  // 初始值
   state.bsLive = 0;    // 初始值
   state.numZ = 0;
   
   bsW(&state, 4, 0xA); // 写入4位，值为0xA
   CU_ASSERT(state.bsBuff == 0xA0000000);
   CU_ASSERT(state.bsLive == 4);
   
   bsW(&state, 4, 0xB); // 再写入4位，值为0xB
   CU_ASSERT(state.bsBuff == 0xAB000000);
   CU_ASSERT(state.bsLive == 8);
   
   bsW(&state, 4, 0xC); // 再写入4位，值为0xC
   CU_ASSERT(state.bsBuff == 0xABC00000);
   CU_ASSERT(state.bsLive == 12);
   CU_ASSERT(state.numZ == 0);  // 确保没有数据被写入到zbits中

   bsW(&state, 4, 0xD); // 再写入4位，值为0xD
   CU_ASSERT(state.bsBuff == 0xABCD0000);
   CU_ASSERT(state.bsLive == 16);
   CU_ASSERT(state.numZ == 0);  // 确保没有数据被写入到zbits中

   bsW(&state, 4, 0xE); // 再写入4位，值为0xE
   CU_ASSERT(state.bsBuff == 0x0); // bsBuff被清空
   CU_ASSERT(state.bsLive == 8);
   CU_ASSERT(state.zbits[0] == 0xAB);  // 确保数据被正确地写入到zbits中
}

int main(void) {
   CU_pSuite pSuite = NULL;

   /* 初始化CUnit库 */
   if (CUE_SUCCESS != CU_initialize_registry())
      return CU_get_error();

   /* 添加测试套件到注册中心 */
   pSuite = CU_add_suite("bsW_test_suite", NULL, NULL);
   if (NULL == pSuite) {
      CU_cleanup_registry();
      return CU_get_error();
   }
}
