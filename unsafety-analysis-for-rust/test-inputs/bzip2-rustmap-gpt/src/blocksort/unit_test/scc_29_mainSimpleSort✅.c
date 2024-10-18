// use crate::blocksort::scc_30_root_20_mmed3::*;
// use crate::global_vars::bzip2_c6::{incs-blocksort.i};
#include <stdio.h>
#include "../typedef.h"
#include "./scc_28_root_19_mainGtU✅.c"
// 预定义的希尔排序增量序列
static Int32 incs[14] = { 1, 4, 13, 40, 121, 364, 1093, 3280,
                   9841, 29524, 88573, 265720,
                   797161, 2391484 };

static void mainSimpleSort ( UInt32* ptr,       // 待排序数组
                             UChar* block,      // 包含字符的块
                             UInt16* quadrant,  // 四分区数组
                             Int32 nblock,      // 块的大小
                             Int32 lo,          // 排序开始的索引
                             Int32 hi,          // 排序结束的索引
                             Int32 d,           // 索引的偏移量
                             Int32* budget )    // 预算（可能与循环迭代次数有关）
{
   Int32 i, j, h, bigN, hp;
   UInt32 v;

   // 计算待排序的元素数量
   bigN = hi - lo + 1;
   // 如果元素数量小于2，则不需要排序
   if (bigN < 2) return;

   // 根据待排序元素的数量，选择合适的增量
   hp = 0;
   while (incs[hp] < bigN) hp++;
   hp--;

   // 对每一个增量进行希尔排序
   for (; hp >= 0; hp--) {
      h = incs[hp];

      i = lo + h;
      while (true) {

         // 如果超出上限则跳出循环
         if (i > hi) break;
         v = ptr[i];
         j = i;

         // 进行插入排序的一部分
         while ( mainGtU(ptr[j-h]+d, v+d, block, quadrant, nblock, budget) ) {
            ptr[j] = ptr[j-h];
            j = j - h;
            if (j <= (lo + h - 1)) break;
         }
         ptr[j] = v;
         i++;

         // 以下两个部分与上面的代码块类似，是为了提高效率
         if (i > hi) break;
         v = ptr[i];
         j = i;
         while ( mainGtU(ptr[j-h]+d, v+d, block, quadrant, nblock, budget) ) {
            ptr[j] = ptr[j-h];
            j = j - h;
            if (j <= (lo + h - 1)) break;
         }
         ptr[j] = v;
         i++;

         if (i > hi) break;
         v = ptr[i];
         j = i;
         while ( mainGtU(ptr[j-h]+d, v+d, block, quadrant, nblock, budget) ) {
            ptr[j] = ptr[j-h];
            j = j - h;
            if (j <= (lo + h - 1)) break;
         }
         ptr[j] = v;
         i++;

         // 如果预算小于0，则返回
         if (*budget < 0) return;
      }
   }
}

int count_items(FILE *file) {
    int count = 0;

    while (true) {
        char ch;
        if (fscanf(file, "%*[^,],") != 1) {
            break;
        }
        count++;
    }
    fseek(file, -1, SEEK_CUR);  // Step back to adjust for the final comma.
    return count;
}


int read_array_values(FILE *file, UInt32* arr) {
    int count = 0;
    fscanf(file, "["); 
    while (true) {
        if (fscanf(file, "%u,", &arr[count]) != 1) {
            // 尝试读取不带逗号的值，这可能是数组的最后一个值
            if (fscanf(file, "%u", &arr[count]) != 1) {
                break;
            }
        }
        count++;
    }
    fscanf(file, "]");
    return count;
}

int read_array_hex_values(FILE *file, UInt32* arr) {
    int count = 0;
    fscanf(file, "["); 
    while (true) {
        if (fscanf(file, "%x,", &arr[count]) != 1) {
            // 尝试读取不带逗号的值，这可能是数组的最后一个值
            if (fscanf(file, "%x", &arr[count]) != 1) {
                break;
            }
        }
        count++;
    }
    fscanf(file, "]");
    return count;
}

void test_mainSimpleSort() {
    FILE* file = fopen("/root/rustmap/bzip2/params_mainSimpleSort_1.dat", "r");
    if (!file) {
        perror("打开params_mainSimpleSort_1.dat失败");
        return;
    }

    UInt32 ptr[100000], block[100000], quadrant[100000];

    fscanf(file, "ptr: ");
    read_array_values(file, ptr);
    fscanf(file, " block: ");
    read_array_hex_values(file, block);
    fscanf(file, " quadrant: ");
    read_array_values(file, quadrant);

    // 修复了读取标量值的问题
    UInt32 nblock, lo, hi, d, budget;
    fscanf(file, " nblock: %u lo: %u hi: %u d: %u budget: %u", &nblock, &lo, &hi, &d, &budget);

    printf("Read nblock: %u\n", nblock);
    printf("Read ptr[%u]=%u\n", lo, ptr[lo]);
    printf("Read ptr[%u]=%u\n", hi, ptr[hi]);
    printf("Read d: %u\n", d);
    printf("Read budget: %u\n", budget);


    // 在所有打印结束之后，调用mainSimpleSort函数
    mainSimpleSort(ptr, block, quadrant, nblock, lo, hi, d, &budget);

    // 打印调用mainSimpleSort之后的ptr[lo]和ptr[hi]
    printf("After mainSimpleSort:\n");
    printf("ptr[%u]=%u\n", lo, ptr[lo]);
    printf("ptr[%u]=%u\n", hi, ptr[hi]);


    fclose(file);

}


// 🌟 Before mainSimpleSort:
// 🌼 59890, ptr[lo]: 19782
// 🌻 59891, ptr[hi]: 19913


// 🌙 After mainSimpleSort:
// 🌷 59890, ptr[lo]: 19913
// 🌹 59891, ptr[hi]: 19782
// 🍀 budget: 883530


// int main() {
//     CU_initialize_registry();
//     CU_add_suite("mainSimpleSort", NULL, NULL);
//     CU_add_test(CU_get_suite("mainSimpleSort"), "test_mainSimpleSort", test_mainSimpleSort);
//     CU_basic_run_tests();
//     CU_cleanup_registry();
//     return CU_get_error();
// }