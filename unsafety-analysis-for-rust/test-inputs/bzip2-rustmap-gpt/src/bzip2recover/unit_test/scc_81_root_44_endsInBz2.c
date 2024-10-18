// endsInBz2 ( Char* name )
#include "../typedef.h"
/* 检查字符串（可能是文件名）是否以“.bz2”结尾 */
static Bool endsInBz2 ( Char* name )
{
   /* 获取name的长度 */
   Int32 n = strlen ( name );
   /* 如果name的长度小于等于4，则直接返回假（False） */
   if (n <= 4) return ((Bool)0);
   /* 检查name是否以“.bz2”结尾，如果是则返回真（True），否则返回假（False） */
   return
      (name[n-4] == '.' &&
       name[n-3] == 'b' &&
       name[n-2] == 'z' &&
       name[n-1] == '2');
}
#include <CUnit/Basic.h>
#include <CUnit/CUnit.h>
#include <stdio.h>
#include <string.h>
/* 假设Bool、Char、Int32等类型定义已经在其它地方定义 */
/* endsInBz2函数声明 */
static Bool endsInBz2 ( Char* name );
/* 测试函数 */
void test_endsInBz2(void)
{
   CU_ASSERT(endsInBz2("file.bz2"));           // 期望为真
   CU_ASSERT_FALSE(endsInBz2("file.bz"));      // 期望为假
   CU_ASSERT_FALSE(endsInBz2("bz2"));          // 期望为假
   CU_ASSERT(endsInBz2(".bz2"));               // 期望为真
   CU_ASSERT_FALSE(endsInBz2("example.txt"));  // 期望为假
}
int main()
{
   CU_initialize_registry();
   CU_pSuite suite = CU_add_suite("Test_endsInBz2", NULL, NULL);
   CU_add_test(suite, "test_endsInBz2", test_endsInBz2);
   
   CU_basic_set_mode(CU_BRM_VERBOSE);
   CU_basic_run_tests();
   CU_cleanup_registry();
   return 0;
}
