#include "../typedef.h"
typedef
   struct {
      char *next_in;
      unsigned int avail_in;
      unsigned int total_in_lo32;
      unsigned int total_in_hi32;

      char *next_out;
      unsigned int avail_out;
      unsigned int total_out_lo32;
      unsigned int total_out_hi32;

      void *state;

      void *(*bzalloc)(void *,int,int);
      void (*bzfree)(void *,void *);
      void *opaque;
   }
   bz_stream;
int BZ2_bzDecompress ( bz_stream *strm )
{
   Bool corrupt; // 变量用于标识数据是否损坏
   DState* s; 
   
   // 对输入流进行null检查
   if (strm == ((void *)0)) return (-2);
   
   s = strm->state;
   // 对状态进行null检查
   if (s == ((void *)0)) return (-2);
   // 检查流状态
   if (s->strm != strm) return (-2);

   // 循环直到数据完全解压或遇到错误
   while (((Bool)1)) {
      if (s->state == 1) return (-1); // 如果状态为1，则返回错误
      if (s->state == 2) {
         // 根据是否是smallDecompress来调用相应的解压函数
         if (s->smallDecompress)
            corrupt = unRLE_obuf_to_output_SMALL ( s );
         else
            corrupt = unRLE_obuf_to_output_FAST ( s );
         // 如果数据损坏，返回错误
         if (corrupt) return (-4);
         
         // 检查数据块的CRC校验
         if (s->nblock_used == s->save_nblock+1 && s->state_out_len == 0) {
            s->calculatedBlockCRC = ~(s->calculatedBlockCRC);
            // 根据verbosity级别打印调试信息
            if (s->verbosity >= 3)
               fprintf(stderr," {0x%08x, 0x%08x}",s->storedBlockCRC,s->calculatedBlockCRC);
            if (s->verbosity >= 2) fprintf(stderr,"]");
            
            // 检查计算的CRC与存储的CRC是否匹配
            if (s->calculatedBlockCRC != s->storedBlockCRC)
               return (-4);
            s->calculatedCombinedCRC
               = (s->calculatedCombinedCRC << 1) |
                    (s->calculatedCombinedCRC >> 31);
            s->calculatedCombinedCRC ^= s->calculatedBlockCRC;
            s->state = 14;
         } else {
            return 0;
         }
      }
      if (s->state >= 10) {
         // 调用BZ2_decompress进行解压
         Int32 r = BZ2_decompress ( s );
         if (r == 4) {
            if (s->verbosity >= 3)
               fprintf(stderr,"\n    combined CRCs: stored = 0x%08x, computed = 0x%08x",s->storedCombinedCRC,s->calculatedCombinedCRC);
            // 检查组合的CRC是否匹配
            if (s->calculatedCombinedCRC != s->storedCombinedCRC)
               return (-4);
            return r;
         }
         if (s->state != 2) return r;
      }
   }

   BZ2_bz__AssertH__fail ( 6001 ); // 断言失败

   return 0;
}

int BZ2_bzRead
           ( int* bzerror,
             BZFILE* b,
             void* buf,
             int len )
{
   Int32 n, ret;
   bzFile* bzf = (bzFile*)b;

   { if (bzerror != 
  ((void *)0)
  ) *bzerror = 0; if (bzf != 
  ((void *)0)
  ) bzf->lastErr = 0; };

   if (bzf == 
             ((void *)0) 
                  || buf == 
                            ((void *)0) 
                                 || len < 0)
      { { if (bzerror != 
       ((void *)0)
       ) *bzerror = (-2); if (bzf != 
       ((void *)0)
       ) bzf->lastErr = (-2); }; return 0; };

   if (bzf->writing)
      { { if (bzerror != 
       ((void *)0)
       ) *bzerror = (-1); if (bzf != 
       ((void *)0)
       ) bzf->lastErr = (-1); }; return 0; };

   if (len == 0)
      { { if (bzerror != 
       ((void *)0)
       ) *bzerror = 0; if (bzf != 
       ((void *)0)
       ) bzf->lastErr = 0; }; return 0; };

   bzf->strm.avail_out = len;
   bzf->strm.next_out = buf;

   while (((Bool)1)) {

      if (ferror(bzf->handle))
         { { if (bzerror != 
          ((void *)0)
          ) *bzerror = (-6); if (bzf != 
          ((void *)0)
          ) bzf->lastErr = (-6); }; return 0; };

      if (bzf->strm.avail_in == 0 && !myfeof(bzf->handle)) {
         n = fread ( bzf->buf, sizeof(UChar),
                     5000, bzf->handle );
         if (ferror(bzf->handle))
            { { if (bzerror != 
             ((void *)0)
             ) *bzerror = (-6); if (bzf != 
             ((void *)0)
             ) bzf->lastErr = (-6); }; return 0; };
         bzf->bufN = n;
         bzf->strm.avail_in = bzf->bufN;
         bzf->strm.next_in = bzf->buf;
      }

      ret = BZ2_bzDecompress ( &(bzf->strm) );

      if (ret != 0 && ret != 4)
         { { if (bzerror != 
          ((void *)0)
          ) *bzerror = ret; if (bzf != 
          ((void *)0)
          ) bzf->lastErr = ret; }; return 0; };

      if (ret == 0 && myfeof(bzf->handle) &&
          bzf->strm.avail_in == 0 && bzf->strm.avail_out > 0)
         { { if (bzerror != 
          ((void *)0)
          ) *bzerror = (-7); if (bzf != 
          ((void *)0)
          ) bzf->lastErr = (-7); }; return 0; };

      if (ret == 4)
         { { if (bzerror != 
          ((void *)0)
          ) *bzerror = 4; if (bzf != 
          ((void *)0)
          ) bzf->lastErr = 4; };
           return len - bzf->strm.avail_out; };
      if (bzf->strm.avail_out == 0)
         { { if (bzerror != 
          ((void *)0)
          ) *bzerror = 0; if (bzf != 
          ((void *)0)
          ) bzf->lastErr = 0; }; return len; };

   }

   return 0;
}




#include <CUnit/Basic.h>

void test_BZ2_bzDecompress() {
    bz_stream strm;
    // 初始化strm
    // 调用BZ2_bzDecompress，并传入初始化的strm
    int result = BZ2_bzDecompress(&strm);
    // 基于预期结果进行断言
    CU_ASSERT_EQUAL(result, /*预期结果*/);
}

int main() {
    CU_initialize_registry();
    CU_pSuite suite = CU_add_suite("bzDecompress_test_suite", 0, 0);
    
    CU_add_test(suite, "test_BZ2_bzDecompress", test_BZ2_bzDecompress);
    
    CU_basic_set_mode(CU_BRM_VERBOSE);
    CU_basic_run_tests();
    CU_cleanup_registry();

    return 0;
}
