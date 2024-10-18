// use crate::global_vars::bzip2_c1::{DState};


// makeMaps_d ( DState* s )
#include <stdio.h>
#include <string.h>
#include <CUnit/Basic.h>
typedef char Char;
typedef unsigned char Bool;
typedef unsigned char UChar;
typedef int Int32;
typedef unsigned int UInt32;
typedef short Int16;
typedef unsigned short UInt16;

typedef
   struct {
      char *next_in;
      unsigned int avail_in;
      unsigned int total_in_lo32;
      unsigned int total_in_hi32;

      char *next_out;
      unsigned int avail_out;
      unsigned int total_out_lo32;
      unsigned int total_out_hi32;

      void *state;

      void *(*bzalloc)(void *,int,int);
      void (*bzfree)(void *,void *);
      void *opaque;
   }
   bz_stream;

typedef
   struct {

      bz_stream* strm;


      Int32 state;


      UChar state_out_ch;
      Int32 state_out_len;
      Bool blockRandomised;
      Int32 rNToGo; Int32 rTPos;


      UInt32 bsBuff;
      Int32 bsLive;


      Int32 blockSize100k;
      Bool smallDecompress;
      Int32 currBlockNo;
      Int32 verbosity;


      Int32 origPtr;
      UInt32 tPos;
      Int32 k0;
      Int32 unzftab[256];
      Int32 nblock_used;
      Int32 cftab[257];
      Int32 cftabCopy[257];


      UInt32 *tt;// 这也是一个裸指针，需要特别小心处理


      UInt16 *ll16; // 同上
      UChar *ll4; // 同上


      UInt32 storedBlockCRC;
      UInt32 storedCombinedCRC;
      UInt32 calculatedBlockCRC;
      UInt32 calculatedCombinedCRC;


      Int32 nInUse;
      Bool inUse[256];
      Bool inUse16[16];
      UChar seqToUnseq[256];


      UChar mtfa [4096];
      Int32 mtfbase[256 / 16];
      UChar selector [(2 + (900000 / 50))];
      UChar selectorMtf[(2 + (900000 / 50))];
      UChar len [6][258];

      Int32 limit [6][258];
      Int32 base [6][258];
      Int32 perm [6][258];
      Int32 minLens[6];


      Int32 save_i;
      Int32 save_j;
      Int32 save_t;
      Int32 save_alphaSize;
      Int32 save_nGroups;
      Int32 save_nSelectors;
      Int32 save_EOB;
      Int32 save_groupNo;
      Int32 save_groupPos;
      Int32 save_nextSym;
      Int32 save_nblockMAX;
      Int32 save_nblock;
      Int32 save_es;
      Int32 save_N;
      Int32 save_curr;
      Int32 save_zt;
      Int32 save_zn;
      Int32 save_zvec;
      Int32 save_zj;
      Int32 save_gSel;
      Int32 save_gMinlen;
      Int32* save_gLimit;
      Int32* save_gBase;
      Int32* save_gPerm;

   }
   DState;

//    函数的主要目的是对数组 inUse 进行遍历，每当遇到 true 值时，就将当前的索引值存储到 seqToUnseq 数组中，并更新 nInUse 的计数。
static
void makeMaps_d ( DState* s )
{
   Int32 i;
   s->nInUse = 0;
   for (i = 0; i < 256; i++)
      if (s->inUse[i]) {
         s->seqToUnseq[s->nInUse] = i;
         s->nInUse++;
      }
}

void test_makeMaps_d(void) {
    DState ds;

    // 使用测试值初始化结构
    for (int i = 0; i < 256; i++) {
        ds.inUse[i] = (i % 2 == 0) ? 1 : 0;  // 假设偶数索引正在使用
        ds.seqToUnseq[i] = 0;  // 清除先前的值
    }

    makeMaps_d(&ds);

    // 断言
    CU_ASSERT_EQUAL(ds.nInUse, 128);  // 256的一半（偶数）正在使用中

    for (int i = 0; i < ds.nInUse; i++) {
        CU_ASSERT_EQUAL(ds.seqToUnseq[i], 2 * i);  // 验证我们存储了偶数
    }
}

int main() {
    // 初始化CUnit测试注册表
    if (CUE_SUCCESS != CU_initialize_registry())
        return CU_get_error();

    // 向注册表添加套件
    CU_pSuite pSuite = NULL;
    pSuite = CU_add_suite("makeMaps_d_TestSuite", NULL, NULL);
    if (NULL == pSuite) {
        CU_cleanup_registry();
        return CU_get_error();
    }

    // 在套件中添加测试函数
    if ((NULL == CU_add_test(pSuite, "test_makeMaps_d", test_makeMaps_d))) {
        CU_cleanup_registry();
        return CU_get_error();
    }

    // 使用CUnit基本接口运行所有测试
    CU_basic_set_mode(CU_BRM_VERBOSE);
    CU_basic_run_tests();
    CU_cleanup_registry();
    
    return CU_get_error();
}
