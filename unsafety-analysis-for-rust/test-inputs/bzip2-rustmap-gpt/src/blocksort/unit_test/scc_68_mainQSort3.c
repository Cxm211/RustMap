#include "../typedef.h"
#include <stdio.h>
#include <stdlib.h>
#include <CUnit/Basic.h>
#include "../blocksort/scc_29_mainSimpleSort✅.c"
#include "../blocksort/scc_30_root_20_mmed3.c"
#include "../bzlib/scc_62_BZ2_bz__AssertH__fail.c"
// ptr: 这是一个需要排序的索引数组。数组的元素代表的是数据块的位置。
// block: 这是实际的数据块，它包含要排序的数据。
// quadrant: 这可能是一个辅助数组，用于优化排序。
// nblock: 数据块的大小。
// lo 和 hi: 这是ptr数组中我们要排序的部分的开始和结束索引。
// d: 这可能是一个偏移，用于在数据块中比较数据。
// budget: 这是一个指针，指向一个整数，可能用于限制或跟踪排序操作的次数。


// 改变了什么:
// 从你给出的打印输出中，我们可以观察到：

// ptr的特定位置（如ptr[lo]和ptr[hi]）在函数调用后可能会改变，这意味着ptr数组在这些位置上的元素已经被排序。
// budget有时会改变，这可能意味着在排序过程中，budget被用作计数或跟踪。
// 以给出的打印输出为例：

// 第一个mainSimpleSort调用，ptr[lo]从177730变为179245，而budget从1799620降低到1799616。
// 第二和第三个mainSimpleSort调用中，ptr[lo]和ptr[hi]并没有改变，而budget也没有改变。
// 这意味着在第一个mainSimpleSort调用中，排序确实发生了，并且某些操作消耗了budget。但在接下来的两次调用中，没有发生排序，这可能是因为数据已经是排序状态或者其他条件阻止了排序。
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

// Test function for mainQSort3
// 2. CUnit test function
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

// int main() {
//     // Initialize CUnit test registry
//     if (CUE_SUCCESS != CU_initialize_registry())
//         return CU_get_error();

//     // Add a suite to the registry
//     CU_pSuite suite = CU_add_suite("MainQSort3_Test", NULL, NULL);
//     if (NULL == suite) {
//         CU_cleanup_registry();
//         return CU_get_error();
//     }

//     // Add the test to the suite
//     if (NULL == CU_add_test(suite, "test of mainQSort3()", test_mainQSort3)) {
//         CU_cleanup_registry();
//         return CU_get_error();
//     }

//     // Run all tests in the suite
//     CU_basic_set_mode(CU_BRM_VERBOSE);
//     CU_basic_run_tests();

//     CU_cleanup_registry();
//     return CU_get_error();
// }
