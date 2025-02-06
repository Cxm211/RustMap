#include "../typedef.h"
#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>
#include <ctype.h>
#include "./scc_91_cleanUpAndFail.c"
#include "./scc_80_root_43_showFileNames.c"

static
void outOfMemory ( void )
{
   // 向标准错误输出流打印错误信息，其中包含程序名
   fprintf(stderr, "\n%s: couldn't allocate enough memory\n", progName);
   // 显示相关文件名
   showFileNames();
   // 清理并以错误代码1退出
   cleanUpAndFail(1);
}


static
void *myMalloc ( Int32 n )
{
   void* p;

   // 尝试分配n个字节的内存
   p = malloc ( (size_t)n );
   
   // 如果分配失败，调用outOfMemory函数
   if (p == ((void *)0)) outOfMemory ();
   
   return p;
}


void test_outOfMemory(void) {
    // 假设我们可以模拟内存分配失败
    // outOfMemory();  // 调用这个函数会导致程序退出，所以在实际测试中应该避免直接调用
    CU_FAIL("This test is just a placeholder and shouldn't be executed directly.");
}
void test_myMalloc(void) {
    // 测试正常情况下的内存分配
    void* p = myMalloc(10);
    CU_ASSERT_PTR_NOT_NULL(p); // 断言p不为NULL
    free(p);

    // 虽然在真实环境中很难模拟malloc失败的情况，但我们可以通过其他方式来测试outOfMemory被调用的逻辑。
    // 例如，可以暂时修改myMalloc中的逻辑来模拟malloc失败，或者尝试分配极大的内存来触发失败。
}

    

