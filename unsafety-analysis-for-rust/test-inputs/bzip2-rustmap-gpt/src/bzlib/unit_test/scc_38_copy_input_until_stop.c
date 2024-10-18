
// 从输入流中复制数据到块中，直到满足停止条件
#include "../typedef.h"
#include "./scc_37_root_25_add_pair_to_block.c"
static
Bool copy_input_until_stop ( EState* s )
{
   Bool progress_in = ((Bool)0);

   if (s->mode == 2) {


      while (((Bool)1)) {

         if (s->nblock >= s->nblockMAX) break;

         if (s->strm->avail_in == 0) break;
         progress_in = ((Bool)1);
         { UInt32 zchh = (UInt32)((UInt32)(*((UChar*)(s->strm->next_in)))); if (zchh != s->state_in_ch && s->state_in_len == 1) { UChar ch = (UChar)(s->state_in_ch); { s->blockCRC = (s->blockCRC << 8) ^ BZ2_crc32Table[(s->blockCRC >> 24) ^ ((UChar)ch)]; }; s->inUse[s->state_in_ch] = ((Bool)1); s->block[s->nblock] = (UChar)ch; s->nblock++; s->state_in_ch = zchh; } else if (zchh != s->state_in_ch || s->state_in_len == 255) { if (s->state_in_ch < 256) add_pair_to_block ( s ); s->state_in_ch = zchh; s->state_in_len = 1; } else { s->state_in_len++; } };
         s->strm->next_in++;
         s->strm->avail_in--;
         s->strm->total_in_lo32++;
         if (s->strm->total_in_lo32 == 0) s->strm->total_in_hi32++;
      }

   } else {


      while (((Bool)1)) {

         if (s->nblock >= s->nblockMAX) break;

         if (s->strm->avail_in == 0) break;

         if (s->avail_in_expect == 0) break;
         progress_in = ((Bool)1);
         { UInt32 zchh = (UInt32)((UInt32)(*((UChar*)(s->strm->next_in)))); if (zchh != s->state_in_ch && s->state_in_len == 1) { UChar ch = (UChar)(s->state_in_ch); { s->blockCRC = (s->blockCRC << 8) ^ BZ2_crc32Table[(s->blockCRC >> 24) ^ ((UChar)ch)]; }; s->inUse[s->state_in_ch] = ((Bool)1); s->block[s->nblock] = (UChar)ch; s->nblock++; s->state_in_ch = zchh; } else if (zchh != s->state_in_ch || s->state_in_len == 255) { if (s->state_in_ch < 256) add_pair_to_block ( s ); s->state_in_ch = zchh; s->state_in_len = 1; } else { s->state_in_len++; } };
         s->strm->next_in++;
         s->strm->avail_in--;
         s->strm->total_in_lo32++;
         if (s->strm->total_in_lo32 == 0) s->strm->total_in_hi32++;
         s->avail_in_expect--;
      }
   }
   return progress_in;
}


// 从处理完的数据中复制到输出流，直到满足停止条件
static
Bool copy_output_until_stop ( EState* s )
{
   Bool progress_out = ((Bool)0);

   while (((Bool)1)) {


      if (s->strm->avail_out == 0) break;


      if (s->state_out_pos >= s->numZ) break;

      progress_out = ((Bool)1);
      *(s->strm->next_out) = s->zbits[s->state_out_pos];
      s->state_out_pos++;
      s->strm->avail_out--;
      s->strm->next_out++;
      s->strm->total_out_lo32++;
      if (s->strm->total_out_lo32 == 0) s->strm->total_out_hi32++;
   }

   return progress_out;
}

#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>

// CUnit测试用例
void test_copy_input_until_stop() {

    EState s = EState_default();
    Bool result = copy_input_until_stop(&s);
    // 使用CUnit断言进行验证
    CU_ASSERT_TRUE(result); // 假设我们期望返回true

    // ... [其他需要验证的断言]
    // 例如，你可能想要检查s->nblock的最终值，或者s->strm->avail_in的值。
}

void test_copy_output_until_stop() {
    EState s = EState_default();

    Bool result = copy_output_until_stop(&s);
    // 使用CUnit断言进行验证
    CU_ASSERT_TRUE(result); // 假设我们期望返回true

    // ... [其他需要验证的断言]
    // 例如，你可能想要检查输出流的大小或值。
}

// int main() {
//     CU_initialize_registry();

//     CU_pSuite suite = CU_add_suite("Compression Stream Tests", NULL, NULL);
//     CU_add_test(suite, "test_copy_input_until_stop", test_copy_input_until_stop);
//     CU_add_test(suite, "test_copy_output_until_stop", test_copy_output_until_stop);

//     CU_basic_set_mode(CU_BRM_VERBOSE);
//     CU_basic_run_tests();
//     CU_cleanup_registry();

//     return 0;
// }
