
#include "../typedef.h"
// 目的是比较block数组中两个位置（i1和i2）的子序列。这个比较是在BWT（Burrows-Wheeler transform）数据压缩算法中使用的，主要用于排序前缀。
// 函数的具体行为是逐个比较两个子序列的字符，直到找到一个不相等的字符或达到预算限制为止。
static
__inline__
Bool mainGtU ( UInt32 i1,  // 第一个索引
               UInt32 i2,  // 第二个索引
               UChar* block,  // 包含字符的块
               UInt16* quadrant, // 四分区数组
               UInt32 nblock,  // 块的大小
               Int32* budget ) // 预算（可能与循环迭代次数有关
{
   Int32 k;
   UChar c1, c2;
   UInt16 s1, s2;
    for (int j = 0; j < 12; j++) {
      c1 = block[i1]; c2 = block[i2];
      // 如果在某个位置发现字符不同，则根据这些字符的大小返回布尔值。
      if (c1 != c2) return (c1 > c2);
      i1++; i2++;
   }
   k = nblock + 8;

   do {
      // 同样的逻辑，但在这里，我们还要比较quadrant中的值。
      for (int j = 0; j < 8; j++) {
         c1 = block[i1]; c2 = block[i2];
         if (c1 != c2) return (c1 > c2);
         s1 = quadrant[i1]; s2 = quadrant[i2];
         if (s1 != s2) return (s1 > s2);
         i1++; i2++;
     
    // 更新索引，如果超出块的大小，则回到块的开头。 
      if (i1 >= nblock) i1 -= nblock;
      if (i2 >= nblock) i2 -= nblock;
      }

       k -= 8;       // 更新k的值
      (*budget)--;  // 更新预算值。这可能用于外部函数限制这个函数的迭代次数。
   }
      while (k >= 0);// 当k非负时，继续循环
   // 如果函数到达这里，说明两个字符串是相同的。
   // 返回False表示它们是相等的
   return ((Bool)0);
}

#include <CUnit/Basic.h>

// 声明函数
// static __inline__ Bool mainGtU ( UInt32, UInt32, UChar*, UInt16*, UInt32, Int32* );

#include <assert.h>
// ... 其他必要的头文件 ...
#include <CUnit/CUnit.h>

void test_mainGtU_from_file() {
    int counter = 1;
    char filename[100];
    snprintf(filename, sizeof(filename), "params_mainGtU_%d.dat", counter);

    FILE* file = fopen(filename, "r");
    if (!file) {
        CU_FAIL("Error opening file for reading.");
        return;
    }

    UInt32 i1, i2, nblock;
    Int32 budget;

    if (fscanf(file, "i1: %u\n", &i1) != 1) {
        CU_FAIL("Failed to read i1.");
        fclose(file);
        return;
    }
    printf("Read i1: %u\n", i1);

    if (fscanf(file, "i2: %u\n", &i2) != 1) {
        CU_FAIL("Failed to read i2.");
        fclose(file);
        return;
    }
    printf("Read i2: %u\n", i2);

    // 忽略直到左方括号读取block数组
    while(fgetc(file) != '[');
    UChar block[100000];  // assuming maximum size
    UInt32 idx = 0;
    while (1) {
        unsigned int val;
        if (idx) fgetc(file); // 跳过逗号
        if (fscanf(file, "%02X", &val) != 1) break;
        block[idx++] = val;
    }

    // 忽略直到下一个左方括号读取quadrant数组
    while(fgetc(file) != '[');
    UInt16 quadrant[100000];  // assuming maximum size
    idx = 0;
    while (1) {
        unsigned int val;
        if (idx) fgetc(file); // 跳过逗号
        if (fscanf(file, "%hu", &val) != 1) break;
        quadrant[idx++] = val;
    }

    if (fscanf(file, "nblock: %u\n", &nblock) != 1) {
        CU_FAIL("Failed to read nblock.");
        fclose(file);
        return;
    }
    printf("Read nblock: %u\n", nblock);

    if (fscanf(file, "budget: %d\n", &budget) != 1) {
        CU_FAIL("Failed to read budget.");
        fclose(file);
        return;
    }
    printf("Read budget: %d\n", budget);

    fclose(file);

    // 打印函数调用前的值
    printf("Before calling mainGtU:\n");
    printf("i1: %u, i2: %u, budget: %d\n", i1, i2, budget);

    Bool result = mainGtU(i1, i2, block, quadrant, nblock, &budget);

    // 打印函数调用后的值
    printf("After calling mainGtU:\n");
    printf("i1: %u, i2: %u, budget: %d\n", i1, i2, budget);

    // 使用CUnit断言进行检查
    CU_ASSERT_EQUAL(i1, "期待值");  // 请替换"期待值"
    CU_ASSERT_EQUAL(i2, "期待值");  // 请替换"期待值"
    CU_ASSERT_EQUAL(budget, "期待值");  // 请替换"期待值"
    // 根据需要添加其他断言
}




// int main() {
//     CU_initialize_registry();
//     CU_add_suite("mainGtU_test", NULL, NULL);
//     CU_add_test(CU_get_suite("mainGtU_test"), "test_mainGtU", test_mainGtU_from_file);
//     CU_basic_run_tests();
//     CU_cleanup_registry();
//     return CU_get_error();
// }

// void test_mainGtU(void) {
//     UInt32 nblock = 98170;
//     UChar block[nblock + 20];  
//     UInt16 quadrant[nblock + 20];
//     Int32 budget = 883445;
    
//     // 使用循环来初始化block和quadrant数组
//     for (UInt32 i = 0; i < nblock + 20; i++) {
//         block[i] = 0;
//         quadrant[i] = 0;
//     }

//     // 基于你给的例子：
//     assert(mainGtU(90047, 91194, block, quadrant, nblock, &budget) == FALSE);
//        if(budget != 871172) {
//         printf("Expected budget: 871172, but got: %d\n", budget);
//     }
//     assert(budget == 871172);  // 根据函数的逻辑，budget值应该递减1

//     // 根据新提供的情况添加的测试用例：
//     Int32 budget2 = 883290;
//     assert(mainGtU(53306, 38287, block, quadrant, nblock, &budget2) == FALSE);
//     if(budget2 != 871017) {
//         printf("Expected budget: 883290, but got: %d\n", budget2);
//     }
//     assert(budget2 == 871017);  // 根据给定的输出，budget值应该保持不变
    
//     // 在此处，你可以添加更多的断言来测试其他情况
// }


// // int main() {
// //     CU_initialize_registry();
// //     CU_add_suite("mainGtU_test", NULL, NULL);
// //     CU_add_test(CU_get_suite("mainGtU_test"), "test_mainGtU", test_mainGtU);
// //     CU_basic_run_tests();
// //     CU_cleanup_registry();
// //     return CU_get_error();
// // }
