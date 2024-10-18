#include "../typedef.h"
typedef
   struct {

      bz_stream* strm;



      Int32 mode;
      Int32 state;


      UInt32 avail_in_expect;


      UInt32* arr1;
      UInt32* arr2;
      UInt32* ftab;
      Int32 origPtr;


      UInt32* ptr;
      UChar* block;
      UInt16* mtfv;
      UChar* zbits;


      Int32 workFactor;


      UInt32 state_in_ch;
      Int32 state_in_len;
      Int32 rNToGo; Int32 rTPos;


      Int32 nblock;
      Int32 nblockMAX;
      Int32 numZ;
      Int32 state_out_pos;


      Int32 nInUse;
      Bool inUse[256];
      UChar unseqToSeq[256];


      UInt32 bsBuff;
      Int32 bsLive;


      UInt32 blockCRC;
      UInt32 combinedCRC;


      Int32 verbosity;
      Int32 blockNo;
      Int32 blockSize100k;


      Int32 nMTF;
      Int32 mtfFreq [258];
      UChar selector [(2 + (900000 / 50))];
      UChar selectorMtf[(2 + (900000 / 50))];

      UChar len [6][258];
      Int32 code [6][258];
      Int32 rfreq [6][258];

      UInt32 len_pack[258][4];

   }
   EState;

/**
 * 在写入二进制数据流结束时，处理并确保EState中的所有挂起的二进制数据都被写入到zbits中。
 * @param s 指向EState结构体实例的指针。
 */
static void bsFinishWrite ( EState* s ) {
   // 当还有挂起的位未写入时，继续处理
   while (s->bsLive > 0) {
      // 将bsBuff中的最高8位写入到zbits数组
      s->zbits[s->numZ] = (UChar)(s->bsBuff >> 24);
      // 更新zbits的写入位置
      s->numZ++;
      // 将bsBuff左移8位，以便处理下一个字节
      s->bsBuff <<= 8;
      // 减少未处理的挂起位的数量
      s->bsLive -= 8;
   }
}

#include <CUnit/Basic.h>
#include <stdio.h>

// 假设你已经定义了上述的EState结构体和bsFinishWrite函数...

void test_bsFinishWrite(void) {
   EState state;
   state.bsBuff = 0xA5A5A5A5; // 示例值
   state.bsLive = 16; // 示例值：表示还有16位未写入
   state.numZ = 0;
   bsFinishWrite(&state);
   
   CU_ASSERT(state.zbits[0] == 0xA5);
   CU_ASSERT(state.zbits[1] == 0xA5);
   CU_ASSERT(state.bsLive == 0); // 检查所有挂起的位是否都已写入
}

int main(void) {
   CU_pSuite pSuite = NULL;

   /* 初始化CUnit库 */
   if (CUE_SUCCESS != CU_initialize_registry())
      return CU_get_error();

   /* 添加测试套件到注册中心 */
   pSuite = CU_add_suite("bsFinishWrite_test_suite", NULL, NULL);
   if (NULL == pSuite) {
      CU_cleanup_registry();
      return CU_get_error();
   }

   /* 添加测试函数到套件 */
   if (NULL == CU_add_test(pSuite, "test_bsFinishWrite", test_bsFinishWrite)) {
      CU_cleanup_registry();
      return CU_get_error();
   }

   /* 运行所有测试 */
   CU_basic_set_mode(CU_BRM_VERBOSE);
   CU_basic_run_tests();
   CU_cleanup_registry();

   return CU_get_error();
}
