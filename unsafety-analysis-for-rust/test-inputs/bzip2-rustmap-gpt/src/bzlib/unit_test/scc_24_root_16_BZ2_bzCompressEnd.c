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

// 初始化函数
int init_suite1(void) {
   // 进行任何需要的初始化操作
   return 0;
}

// 清理函数
int clean_suite1(void) {
   // 进行任何需要的清理操作
   return 0;
}

// 测试函数
void testBZ2_bzCompressEnd_null_stream(void) {
   bz_stream* stream = NULL;
   int result = BZ2_bzCompressEnd(stream);
   CU_ASSERT_EQUAL(result, -2);
}

// int main() {
//    // 初始化CUnit测试注册系统
//    CU_pSuite pSuite = NULL;

//    if (CUE_SUCCESS != CU_initialize_registry())
//       return CU_get_error();

//    // 添加测试套件
//    pSuite = CU_add_suite("Suite_1", init_suite1, clean_suite1);
//    if (NULL == pSuite) {
//       CU_cleanup_registry();
//       return CU_get_error();
//    }

//    // 添加测试到测试套件
//    if ((NULL == CU_add_test(pSuite, "test of BZ2_bzCompressEnd with null stream", testBZ2_bzCompressEnd_null_stream))) {
//       CU_cleanup_registry();
//       return CU_get_error();
//    }

//    // 运行所有测试
//    CU_basic_set_mode(CU_BRM_VERBOSE);
//    CU_basic_run_tests();
//    CU_cleanup_registry();
//    return CU_get_error();
// }
