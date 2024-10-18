#include "../typedef.h"
#include "./scc_40_root_27_isempty_RL.c"
#include "./scc_38_copy_input_until_stop.c"
// 定义一个函数handle_compress，用于处理压缩任务
static
Bool handle_compress(bz_stream* strm) {
   Bool progress_in = ((Bool)0);    // 标记是否在输入上取得了进展
   Bool progress_out = ((Bool)0);   // 标记是否在输出上取得了进展
   EState* s = strm->state;         // 获取压缩的状态

   // 持续进行压缩，直到不再可能压缩为止
   while (((Bool)1)) {

      // 当状态为1时
      if (s->state == 1) {
         // 尝试复制输出，直到达到停止条件
         progress_out |= copy_output_until_stop(s);

         // 如果输出位置小于总数，跳出循环
         if (s->state_out_pos < s->numZ) break;

         // 在特定条件下，准备新的压缩块并更改状态
         if (s->mode == 4 &&
             s->avail_in_expect == 0 &&
             isempty_RL(s)) break;
         prepare_new_block(s);
         s->state = 2;

         if (s->mode == 3 &&
             s->avail_in_expect == 0 &&
             isempty_RL(s)) break;
      }

      // 当状态为2时
      if (s->state == 2) {
         // 尝试复制输入，直到达到停止条件
         progress_in |= copy_input_until_stop(s);

         // 在特定条件下，处理压缩块并更改状态
         if (s->mode != 2 && s->avail_in_expect == 0) {
            flush_RL(s);
            BZ2_compressBlock(s, (Bool)(s->mode == 4));
            s->state = 1;
         } else if (s->nblock >= s->nblockMAX) {
            BZ2_compressBlock(s, ((Bool)0));
            s->state = 1;
         } else if (s->strm->avail_in == 0) {
            break;
         }
      }
   }

   // 如果在输入或输出上取得了进展，返回true
   return progress_in || progress_out;
}


#include "CUnit/Basic.h"
#include "../typedef.h"

// 初始化函数
int init_suite(void) {
   return 0;
}

// 清除函数
int clean_suite(void) {
   return 0;
}

// 测试handle_compress函数
void test_handle_compress(void) {
   bz_stream strm = bz_stream_default();
   
   Bool result = handle_compress(&strm);
   CU_ASSERT_TRUE(result); // 根据你的期望调整这里的断言
}

// int main() {
//    CU_pSuite pSuite = NULL;

//    // 初始化CUnit库
//    if (CUE_SUCCESS != CU_initialize_registry())
//       return CU_get_error();

//    // 添加测试套件
//    pSuite = CU_add_suite("handle_compress_test_suite", init_suite, clean_suite);
//    if (NULL == pSuite) {
//       CU_cleanup_registry();
//       return CU_get_error();
//    }

//    // 添加测试案例
//    if ((NULL == CU_add_test(pSuite, "test_handle_compress", test_handle_compress))) {
//       CU_cleanup_registry();
//       return CU_get_error();
//    }

//    // 运行所有测试套件
//    CU_basic_set_mode(CU_BRM_VERBOSE);
//    CU_basic_run_tests();
//    CU_cleanup_registry();
//    return CU_get_error();
// }
