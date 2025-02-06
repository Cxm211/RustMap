
#include "../typedef.h"
static
int bz_config_ok ( void )
{
   if (sizeof(int) != 4) return 0;   // 检查int类型大小是否为4字节
   if (sizeof(short) != 2) return 0; // 检查short类型大小是否为2字节
   if (sizeof(char) != 1) return 0;  // 检查char类型大小是否为1字节
   return 1;                         // 所有检查都通过，返回1
}

static
void* default_bzalloc ( void* opaque, Int32 items, Int32 size )
{
   void* v = malloc ( items * size ); // 分配内存
   return v;
}

static
void default_bzfree ( void* opaque, void* addr )
{
   if (addr != ((void *)0)) free ( addr ); // 如果地址非空，释放内存
}

#include <CUnit/Basic.h>
#include <stdio.h>

void test_bz_config_ok(void) {
   // 通常情况下，在大多数平台上，int是4字节，short是2字节，char是1字节。
   CU_ASSERT(bz_config_ok() == 1);
}

void test_default_bzalloc_and_bzfree(void) {
   void* mem = default_bzalloc(NULL, 5, sizeof(int));
   CU_ASSERT_PTR_NOT_NULL(mem); // 检查分配是否成功
   default_bzfree(NULL, mem);
}






