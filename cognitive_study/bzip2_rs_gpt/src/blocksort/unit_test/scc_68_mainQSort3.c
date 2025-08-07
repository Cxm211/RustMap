#include "../typedef.h"
#include <stdio.h>
#include <stdlib.h>
#include <CUnit/Basic.h>
#include "../blocksort/scc_29_mainSimpleSort✅.c"
#include "../blocksort/scc_30_root_20_mmed3.c"
#include "../bzlib/scc_62_BZ2_bz__AssertH__fail.c"




static
void mainQSort3 ( UInt32* ptr,
                  UChar* block,
                  UInt16* quadrant,
                  Int32 nblock,
                  Int32 loSt,
                  Int32 hiSt,
                  Int32 dSt,
                  Int32* budget )
{
   Int32 unLo, unHi, ltLo, gtHi, n, m, med;
   Int32 sp, lo, hi, d;

   Int32 stackLo[100];
   Int32 stackHi[100];
   Int32 stackD [100];

   Int32 nextLo[3];
   Int32 nextHi[3];
   Int32 nextD [3];

   sp = 0;
   { stackLo[sp] = loSt; stackHi[sp] = hiSt; stackD [sp] = dSt; sp++; };
     // 当栈不为空时继续处理
   while (sp > 0) {
    // 断言栈大小
      { if (!(sp < 100 - 2)) BZ2_bz__AssertH__fail ( 1001 ); };
        // 从栈中弹出一个元素
      { sp--; lo = stackLo[sp]; hi = stackHi[sp]; d = stackD [sp]; };
       // 对于小数组或深度过大的情况，使用简单排序
      if (hi - lo < 20 ||
          d > (2 + 12)) {
         mainSimpleSort ( ptr, block, quadrant, nblock, lo, hi, d, budget );
         if (*budget < 0) return;
         continue;
      }

    // 选择中值进行三向切分的快速排序  
      med = (Int32)
            mmed3 ( block[ptr[ lo ]+d],
                    block[ptr[ hi ]+d],
                    block[ptr[ (lo+hi)>>1 ]+d] );
    // 初始化边界
      unLo = ltLo = lo;
      unHi = gtHi = hi;
        // 开始三向切分
      while (((Bool)1)) {
         while (((Bool)1)) {
            if (unLo > unHi) break;
            n = ((Int32)block[ptr[unLo]+d]) - med;
            if (n == 0) {
               { Int32 zztmp = ptr[unLo]; ptr[unLo] = ptr[ltLo]; ptr[ltLo] = zztmp; };
               ltLo++; unLo++; continue;
            };
            if (n > 0) break;
            unLo++;
         }
         while (((Bool)1)) {
            if (unLo > unHi) break;
            n = ((Int32)block[ptr[unHi]+d]) - med;
            if (n == 0) {
               { Int32 zztmp = ptr[unHi]; ptr[unHi] = ptr[gtHi]; ptr[gtHi] = zztmp; };
               gtHi--; unHi--; continue;
            };
            if (n < 0) break;
            unHi--;
         }
         if (unLo > unHi) break;
         { Int32 zztmp = ptr[unLo]; ptr[unLo] = ptr[unHi]; ptr[unHi] = zztmp; }; unLo++; unHi--;
      }

      ;

      if (gtHi < ltLo) {
         { stackLo[sp] = lo; stackHi[sp] = hi; stackD [sp] = d+1; sp++; };
         continue;
      }

      n = ((ltLo-lo) < (unLo-ltLo)) ? (ltLo-lo) : (unLo-ltLo); { Int32 yyp1 = (lo); Int32 yyp2 = (unLo-n); Int32 yyn = (n); while (yyn > 0) { { Int32 zztmp = ptr[yyp1]; ptr[yyp1] = ptr[yyp2]; ptr[yyp2] = zztmp; }; yyp1++; yyp2++; yyn--; } };
      m = ((hi-gtHi) < (gtHi-unHi)) ? (hi-gtHi) : (gtHi-unHi); { Int32 yyp1 = (unLo); Int32 yyp2 = (hi-m+1); Int32 yyn = (m); while (yyn > 0) { { Int32 zztmp = ptr[yyp1]; ptr[yyp1] = ptr[yyp2]; ptr[yyp2] = zztmp; }; yyp1++; yyp2++; yyn--; } };

      n = lo + unLo - ltLo - 1;
      m = hi - (gtHi - unHi) + 1;

      nextLo[0] = lo; nextHi[0] = n; nextD[0] = d;
      nextLo[1] = m; nextHi[1] = hi; nextD[1] = d;
      nextLo[2] = n+1; nextHi[2] = m-1; nextD[2] = d+1;

      if ((nextHi[0]-nextLo[0]) < (nextHi[1]-nextLo[1])) { Int32 tz; tz = nextLo[0]; nextLo[0] = nextLo[1]; nextLo[1] = tz; tz = nextHi[0]; nextHi[0] = nextHi[1]; nextHi[1] = tz; tz = nextD [0]; nextD [0] = nextD [1]; nextD [1] = tz; };
      if ((nextHi[1]-nextLo[1]) < (nextHi[2]-nextLo[2])) { Int32 tz; tz = nextLo[1]; nextLo[1] = nextLo[2]; nextLo[2] = tz; tz = nextHi[1]; nextHi[1] = nextHi[2]; nextHi[2] = tz; tz = nextD [1]; nextD [1] = nextD [2]; nextD [2] = tz; };
      if ((nextHi[0]-nextLo[0]) < (nextHi[1]-nextLo[1])) { Int32 tz; tz = nextLo[0]; nextLo[0] = nextLo[1]; nextLo[1] = tz; tz = nextHi[0]; nextHi[0] = nextHi[1]; nextHi[1] = tz; tz = nextD [0]; nextD [0] = nextD [1]; nextD [1] = tz; };

      ;
      ;
         // 将新的范围压入栈
      { stackLo[sp] = nextLo[0]; stackHi[sp] = nextHi[0]; stackD [sp] = nextD[0]; sp++; };
      { stackLo[sp] = nextLo[1]; stackHi[sp] = nextHi[1]; stackD [sp] = nextD[1]; sp++; };
      { stackLo[sp] = nextLo[2]; stackHi[sp] = nextHi[2]; stackD [sp] = nextD[2]; sp++; };
   }
}

void test_mainQSort3_fromFile(int counter) {
    char filename[50];
    sprintf(filename, "params_mainQSort3_%d.dat", counter);

    FILE *file = fopen(filename, "r");
    if (!file) {
        printf("Error opening file for reading.\n");
        return;
    }

    // Dynamic array allocation
    UInt32* ptr;
    UChar* block;
    UInt16* quadrant;
    Int32 nblock;

    fscanf(file, "nblock:%d", &nblock);

    ptr = (UInt32*) malloc(sizeof(UInt32) * nblock);
    block = (UChar*) malloc(sizeof(UChar) * nblock);
    quadrant = (UInt16*) malloc(sizeof(UInt16) * nblock);

    // Read arrays from file
    for (int i = 0; i < nblock; i++) {
        fscanf(file, "%u,", &ptr[i]);
    }
    for (int i = 0; i < nblock; i++) {
        fscanf(file, "%u,", &block[i]);
    }
    for (int i = 0; i < nblock; i++) {
        fscanf(file, "%u,", &quadrant[i]);
    }

    // Read individual parameters
    Int32 loSt, hiSt, dSt, budget;
    fscanf(file, "loSt:%d", &loSt);
    fscanf(file, "hiSt:%d", &hiSt);
    fscanf(file, "dSt:%d", &dSt);
    fscanf(file, "budget:%d", &budget);

    fclose(file);

    // Call the function
    mainQSort3(ptr, block, quadrant, nblock, loSt, hiSt, dSt, &budget);

    // Assertion with placeholder expected values
    // CU_ASSERT_EQUAL(variable, expected_value);
    // Replace with actual assertions and expected values

    // Free allocated memory
    free(ptr);
    free(block);
    free(quadrant);
}





