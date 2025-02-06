

#include "../typedef.h"


static
void makeMaps_e ( EState* s )
{
   Int32 i;
   s->nInUse = 0;  // 初始化nInUse为0
   for (i = 0; i < 256; i++)  // 遍历inUse数组
      if (s->inUse[i]) {  // 如果当前位置为true
         s->unseqToSeq[i] = s->nInUse;  // 将nInUse的值映射到unseqToSeq数组中
         s->nInUse++;  // 增加nInUse的值
      }
}

#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>

void test_makeMaps_e(void) {
    EState s;

    // 初始化inUse数组
    for (int i = 0; i < 256; i++) {
        s.inUse[i] = (i % 2 == 0) ? 1 : 0;  // 假设偶数位置为true
    }

    makeMaps_e(&s);

    // 验证unseqToSeq数组的值
    int expected_nInUse = 128;
    CU_ASSERT_EQUAL(s.nInUse, expected_nInUse);
    for (int i = 0; i < 256; i++) {
        if (i % 2 == 0) {
            CU_ASSERT_EQUAL(s.unseqToSeq[i], i / 2);
        }
    }
}

static void generateMTFValues ( EState* s ) {
   UChar yy[256]; // 用于维护MTF变换中的字符列表
   Int32 i, j; // 循环变量
   Int32 zPend; // 用于记录连续出现的字符yy[0]的次数
   Int32 wr; // 写入索引，用于在mtfv数组中记录MTF值
   Int32 EOB; // 表示块结束（End Of Block）的特殊值
   UInt32* ptr = s->ptr; // ptr数组，指向当前块的各个位置
   UChar* block = s->block; // 当前数据块
   UInt16* mtfv = s->mtfv; // 用于存储生成的MTF值

   makeMaps_e ( s ); // 构造字符映射表
   EOB = s->nInUse + 1; // 设置EOB的值

   // 初始化字符频率数组
   for (i = 0; i <= EOB; i++) s->mtfFreq[i] = 0;

   wr = 0; // 初始化写入索引
   zPend = 0; // 初始化连续匹配计数
   // 初始化yy数组，表示MTF变换中的字符顺序
   for (i = 0; i < s->nInUse; i++) yy[i] = (UChar) i;

   // 遍历当前块的每个位置
   for (i = 0; i < s->nblock; i++) {
      UChar ll_i;
      // 计算j，表示当前字符在block数组中的位置
      j = ptr[i] - 1; if (j < 0) j += s->nblock;
      // 获取当前字符的序列值
      ll_i = s->unseqToSeq[block[j]];

      // 如果当前字符与yy[0]相同，增加zPend计数
      if (yy[0] == ll_i) {
         zPend++;
      } else {
         // 处理之前连续匹配的字符
         if (zPend > 0) {
            zPend--;
            while (1) {
               if (zPend & 1) {
                  mtfv[wr] = 1; wr++; s->mtfFreq[1]++;
               } else {
                  mtfv[wr] = 0; wr++; s->mtfFreq[0]++;
               }
               if (zPend < 2) break;
               zPend = (zPend - 2) / 2;
            };
            zPend = 0;
         }
         // 执行MTF变换
         {
            register UChar rtmp;
            register UChar* ryy_j;
            register UChar rll_i;
            rtmp = yy[1];
            yy[1] = yy[0];
            ryy_j = &(yy[1]);
            rll_i = ll_i;
            // 移动字符，直到找到相应的字符位置
            while ( rll_i != rtmp ) {
               register UChar rtmp2;
               ryy_j++;
               rtmp2 = rtmp;
               rtmp = *ryy_j;
               *ryy_j = rtmp2;
            };
            // 更新yy数组
            yy[0] = rtmp;
            // 计算当前字符在yy数组中的位置
            j = ryy_j - &(yy[0]);
            // 记录MTF值
            mtfv[wr] = j + 1; wr++; s->mtfFreq[j+1]++;
         }
      }
   }

   // 处理最后剩下的连续匹配字符
   if (zPend > 0) {
      zPend--;
      while (1) {
         if (zPend & 1) {
            mtfv[wr] = 1; wr++; s->mtfFreq[1]++;
         } else {
            mtfv[wr] = 0; wr++; s->mtfFreq[0]++;
         }
         if (zPend < 2) break;
         zPend = (zPend - 2) / 2;
      };
      zPend = 0;
   }

   // 添加块结束标志
   mtfv[wr] = EOB; wr++; s->mtfFreq[EOB]++;

   // 设置MTF值的数量
   s->nMTF = wr;
}

#include <CUnit/Basic.h>

void test_generateMTFValues(void) {
    // 初始化EState结构体
    EState testState;
    memset(&testState, 0, sizeof(testState));
    EState* s = &testState;

    // 设定输入数据
    s->nInUse = 10;  // 假设有10个字符正在使用
    s->nblock = 20;  // 假设block的长度为20

    for (int i = 0; i < s->nInUse; i++) {
        s->unseqToSeq[i] = (UChar)i;
    }

    // 我们的block只包含unseqToSeq[0]的字符
    for (int i = 0; i < s->nblock; i++) {
        s->block[i] = s->unseqToSeq[0];
    }

    generateMTFValues(s);

    // 验证输出
    // 由于block中的每个字符都与yy[0]匹配，所以mtfv应该全部是0
    for (int i = 0; i < s->nblock; i++) {
        CU_ASSERT_EQUAL(s->mtfv[i], 0);
    }
}


int main() {
   CU_initialize_registry();

   CU_pSuite suite = CU_add_suite("Your Test Suite", NULL, NULL);

   CU_add_test(suite, "test_makeMaps_e", test_makeMaps_e);
   CU_add_test(suite, "Test generateMTFValues", test_generateMTFValues);
   CU_basic_set_mode(CU_BRM_VERBOSE);
   CU_basic_run_tests();

   CU_cleanup_registry();
   return 0;
}