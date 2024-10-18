// use crate::global_vars::bzip2_c1::{EState};
#include  "../typedef.h"


// 初始化写入位流的状态
void BZ2_bsInitWrite ( EState* s )
{
   // 设置当前有效的位数为0
   s->bsLive = 0;
   // 清空位缓冲
   s->bsBuff = 0;
}



#include <CUnit/Basic.h>

// 假设已经定义了EState结构体和BZ2_bsInitWrite函数...

void test_BZ2_bsInitWrite(void) {
   EState s;
   s.bsLive = 5;  // 为测试目的随意设置一个值
   s.bsBuff = 10; // 同上

   BZ2_bsInitWrite(&s);

   CU_ASSERT_EQUAL(s.bsLive, 0);
   CU_ASSERT_EQUAL(s.bsBuff, 0);
}

int main(void) {
   CU_pSuite pSuite = NULL;

   // 初始化CUnit库
   if (CUE_SUCCESS != CU_initialize_registry())
      return CU_get_error();

   // 添加测试套件到注册中心
   pSuite = CU_add_suite("BZ2_bsInitWrite_test_suite", NULL, NULL);
   if (NULL == pSuite) {
      CU_cleanup_registry();
      return CU_get_error();
   }

   // 将测试添加到套件中
   CU_ADD_TEST(pSuite, test_BZ2_bsInitWrite);

   // 运行所有测试
   CU_basic_set_mode(CU_BRM_VERBOSE);
   CU_basic_run_tests();

   CU_cleanup_registry();
   return CU_get_error();
}
