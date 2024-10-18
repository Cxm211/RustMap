// use crate::global_vars::bzip2_c5::{longestFileName};

#include "../typedef.h"
#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>
// 该函数的目的是对齐字符串s的长度到longestFileName，通过向stderr打印空格实现对齐。
static
void pad ( Char *s )
{
   Int32 i;
   // 如果s的长度已经大于或等于longestFileName，则直接返回，不做任何操作。
   if ( (Int32)strlen(s) >= longestFileName ) return;
   
   // 计算并打印需要的空格数量，使得s的长度对齐到longestFileName。
   for (i = 1; i <= longestFileName - (Int32)strlen(s); i++)
      fprintf(stderr, " ");
}



void test_pad() {
    // 假设longestFileName为10用于测试。在真实环境中，您应该使用实际的longestFileName值。
    longestFileName = 10; 

    // 备份原始的stderr，以便之后恢复。
    FILE* original_stderr = stderr;
    
    // 打开一个临时文件用于写入，并重定向stderr到该文件。
    stderr = fopen("tmp.txt", "w");
    if (stderr == NULL) {
        perror("无法打开临时文件");
        exit(EXIT_FAILURE);
    }

    // 调用pad函数来输出空格到重定向的stderr。
    pad("test");
    
    // 关闭重定向的stderr。
    fclose(stderr);
    
    // 恢复原始的stderr。
    stderr = original_stderr;
    
    // 打开临时文件以读取内容。
    FILE* tmp = fopen("tmp.txt", "r");
    if (tmp == NULL) {
        perror("无法读取临时文件");
        exit(EXIT_FAILURE);
    }
    
    Char buffer[11]; // 为读取的内容预留空间。
    
    // 读取文件内容。
    size_t numRead = fread(buffer, sizeof(Char), 10, tmp);
    
    // 根据实际读取的内容设置空终止符。
    buffer[numRead] = '\0';
    
    // 断言：检查读取的内容是否是6个空格。
    CU_ASSERT_STRING_EQUAL(buffer, "      ");
    
    // 关闭临时文件。
    fclose(tmp);
    
    // 删除临时文件。
    remove("tmp.txt");
}


// int main() {
//     CU_pSuite pSuite = NULL;

//     if (CUE_SUCCESS != CU_initialize_registry())
//         return CU_get_error();

//     pSuite = CU_add_suite("Suite_1", NULL, NULL);
//     if (NULL == pSuite) {
//         CU_cleanup_registry();
//         return CU_get_error();
//     }

//     if (NULL == CU_add_test(pSuite, "test of pad", test_pad)) {
//         CU_cleanup_registry();
//         return CU_get_error();
//     }

//     CU_basic_run_tests();
//     CU_cleanup_registry();
//     return CU_get_error();
// }