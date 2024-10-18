
#include "../typedef.h"
#include "./scc_24_root_16_BZ2_bzCompressEnd.c"
#include "./scc_40_root_27_isempty_RL.c"
#include "./scc_74_handle_compress.c"
#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>
// BZ2_bzCompress函数压缩输入数据。
int BZ2_bzCompress(bz_stream *strm, int action) {
   Bool progress;            // 标记函数是否成功执行
   EState* s;

   // 检查strm是否为null
   if (strm == NULL) return (-2);

   s = strm->state;

   // 检查s或strm的状态是否为null
   if (s == NULL || s->strm != strm) return (-2);

   preswitch:
   switch (s->mode) {
      case 1:   // 模式1: 错误
         return (-1);

      case 2:   // 模式2: 压缩数据
         if (action == 0) {
            progress = handle_compress(strm);
            return progress ? 1 : (-2);
         } else if (action == 1) {
            s->avail_in_expect = strm->avail_in;
            s->mode = 3;
            goto preswitch;  // 重新进入switch-case
         } else if (action == 2) {
            s->avail_in_expect = strm->avail_in;
            s->mode = 4;
            goto preswitch;
         } else {
            return (-2);
         }

      case 3:  // 模式3: 继续压缩数据
         if (action != 1 || s->avail_in_expect != s->strm->avail_in)
            return (-1);
         progress = handle_compress(strm);
         if (s->avail_in_expect > 0 || !isempty_RL(s) ||
             s->state_out_pos < s->numZ) return 2;
         s->mode = 2;
         return 1;

      case 4:  // 模式4: 完成压缩
         if (action != 2 || s->avail_in_expect != s->strm->avail_in)
            return (-1);
         progress = handle_compress(strm);
         if (!progress) return (-1);
         if (s->avail_in_expect > 0 || !isempty_RL(s) ||
             s->state_out_pos < s->numZ) return 3;
         s->mode = 1;
         return 4;
   }
   return 0;
}

// 关闭BZIP2文件，但仅提供32位的输入/输出字节计数器
void BZ2_bzWriteClose(int* bzerror, BZFILE* b, int abandon, unsigned int* nbytes_in, unsigned int* nbytes_out) {
    // 使用64位版本的接口，但为64位的计数器传递null指针
    BZ2_bzWriteClose64(bzerror, b, abandon, nbytes_in, NULL, nbytes_out, NULL);
}

// 关闭BZIP2文件，并提供64位的输入/输出字节计数器
void BZ2_bzWriteClose64(
   int* bzerror, BZFILE* b, int abandon,
   unsigned int* nbytes_in_lo32, unsigned int* nbytes_in_hi32,
   unsigned int* nbytes_out_lo32, unsigned int* nbytes_out_hi32) {
   
   Int32 n, n2, ret;
   bzFile* bzf = (bzFile*)b;

   // 检查bzf是否为null
   if (bzf == NULL) {
      if (bzerror) *bzerror = 0;
      return;
   }

   // 检查是否处于写模式
   if (!bzf->writing) {
      if (bzerror) *bzerror = (-1);
      return;
   }

   // 检查文件操作是否有误
   if (ferror(bzf->handle)) {
      if (bzerror) *bzerror = (-6);
      return;
   }

   // 初始化字节计数器为0
   if (nbytes_in_lo32) *nbytes_in_lo32 = 0;
   if (nbytes_in_hi32) *nbytes_in_hi32 = 0;
   if (nbytes_out_lo32) *nbytes_out_lo32 = 0;
   if (nbytes_out_hi32) *nbytes_out_hi32 = 0;

   // 如果没有放弃且上次没有错误，则继续压缩
   if (!abandon && bzf->lastErr == 0) {
      while (1) {
         bzf->strm.avail_out = 5000;
         bzf->strm.next_out = bzf->buf;
         ret = BZ2_bzCompress(&(bzf->strm), 2);
         if (ret != 3 && ret != 4) {
            if (bzerror) *bzerror = ret;
            return;
         }

         if (bzf->strm.avail_out < 5000) {
            n = 5000 - bzf->strm.avail_out;
            n2 = fwrite((void*)(bzf->buf), sizeof(UChar), n, bzf->handle);
            if (n != n2 || ferror(bzf->handle)) {
               if (bzerror) *bzerror = (-6);
               return;
            }
         }

         if (ret == 4) break;
      }
   }

   // 刷新文件缓存
   if (!abandon && !ferror(bzf->handle)) {
      fflush(bzf->handle);
      if (ferror(bzf->handle)) {
         if (bzerror) *bzerror = (-6);
         return;
      }
   }

   // 更新字节计数器的值
   if (nbytes_in_lo32) *nbytes_in_lo32 = bzf->strm.total_in_lo32;
   if (nbytes_in_hi32) *nbytes_in_hi32 = bzf->strm.total_in_hi32;
   if (nbytes_out_lo32) *nbytes_out_lo32 = bzf->strm.total_out_lo32;
   if (nbytes_out_hi32) *nbytes_out_hi32 = bzf->strm.total_out_hi32;

   // 清除错误并结束压缩
   if (bzerror) *bzerror = 0;
   BZ2_bzCompressEnd(&(bzf->strm));
   free(bzf);
}
