#include "../typedef.h"
/**
 * 函数：BZ2_bzCompressEnd
 * 功能：结束BZIP2压缩过程，并释放相关资源
 * 参数：strm - 指向bz_stream的指针
 * 返回值：整型，成功返回0，失败返回错误码
 */
int BZ2_bzCompressEnd ( bz_stream *strm )
{
   EState* s;
   // 如果输入流为null，返回错误码 -2
   if (strm == ((void *)0)) return (-2);
   
   // 获取输入流的状态
   s = strm->state;
   
   // 如果状态为null，返回错误码 -2
   if (s == ((void *)0)) return (-2);
   
   // 如果状态的流不匹配输入流，返回错误码 -2
   if (s->strm != strm) return (-2);

   // 释放相关资源
   if (s->arr1 != ((void *)0)) (strm->bzfree)(strm->opaque,(s->arr1));
   if (s->arr2 != ((void *)0)) (strm->bzfree)(strm->opaque,(s->arr2));
   if (s->ftab != ((void *)0)) (strm->bzfree)(strm->opaque,(s->ftab));
   
   // 释放状态
   (strm->bzfree)(strm->opaque,(strm->state));

   // 将输入流的状态设置为null
   strm->state = ((void *)0);

   // 返回成功
   return 0;
}


#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>

int init_suite1(void) {
   // 进行任何需要的初始化操作
   return 0;
}

int clean_suite1(void) {
   // 进行任何需要的清理操作
   return 0;
}

void testBZ2_bzCompressEnd_null_stream(void) {
   bz_stream* stream = NULL;
   int result = BZ2_bzCompressEnd(stream);
   CU_ASSERT_EQUAL(result, -2);
}





