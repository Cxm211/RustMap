#include "../typedef.h"
#include <stdio.h>
// 检查给定文件名的文件是否存在
static
Bool fileExists ( char* name )
{
   // 以二进制只读模式尝试打开文件
   FILE *tmp = fopen(name, "rb");
   
   // 判断文件是否成功打开
   Bool exists = (tmp != NULL);
   
   // 如果文件打开成功，则关闭文件
   if (tmp != NULL) fclose(tmp);
   
   // 返回文件是否存在的结果
   return exists;
}

// CUnit测试
#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>

void test_fileExists(void) {
    // 假设当前目录下有一个名为"testfile.txt"的文件
    CU_ASSERT_TRUE(fileExists("testfile.txt"));
    
    // 假设当前目录下没有一个名为"nonexistent.txt"的文件
    CU_ASSERT_FALSE(fileExists("nonexistent.txt"));
}

// int main(void) {
//     CU_initialize_registry();
    
//     CU_pSuite suite1 = CU_add_suite("FileExistTests", 0, 0);
//     CU_add_test(suite1, "test_fileExists", test_fileExists);
    
//     CU_basic_set_mode(CU_BRM_VERBOSE);
//     CU_basic_run_tests();
//     CU_cleanup_registry();
    
//     return 0;
// }
