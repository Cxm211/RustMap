// use crate::global_vars::bzip2_c1::{BZ2_crc32Table, BZ2_rNums, DState};


// unRLE_obuf_to_output_FAST ( DState* s )

#include <stdlib.h> // 为了使用 malloc, calloc
#include <string.h> // 为了使用 memset
#include <stdbool.h>
#include <stdio.h>    // 为fprintf和stderr
#include <assert.h>   // 为assert

#ifndef TRUE
#define TRUE 1
#endif

#ifndef FALSE
#define FALSE 0
#endif


typedef char Char;
typedef unsigned char Bool;
typedef unsigned char UChar;
typedef int Int32;
typedef unsigned int UInt32;
typedef short Int16;
typedef unsigned short UInt16;
Int32 BZ2_rNums[512];
UInt32 BZ2_crc32Table[256];

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

typedef
   struct {

      bz_stream* strm;


      Int32 state;


      UChar state_out_ch;
      Int32 state_out_len;
      Bool blockRandomised;
      Int32 rNToGo; Int32 rTPos;


      UInt32 bsBuff;
      Int32 bsLive;


      Int32 blockSize100k;
      Bool smallDecompress;
      Int32 currBlockNo;
      Int32 verbosity;


      Int32 origPtr;
      UInt32 tPos;
      Int32 k0;
      Int32 unzftab[256];
      Int32 nblock_used;
      Int32 cftab[257];
      Int32 cftabCopy[257];


      UInt32 *tt;


      UInt16 *ll16;
      UChar *ll4;


      UInt32 storedBlockCRC;
      UInt32 storedCombinedCRC;
      UInt32 calculatedBlockCRC;
      UInt32 calculatedCombinedCRC;


      Int32 nInUse;
      Bool inUse[256];
      Bool inUse16[16];
      UChar seqToUnseq[256];


      UChar mtfa [4096];
      Int32 mtfbase[256 / 16];
      UChar selector [(2 + (900000 / 50))];
      UChar selectorMtf[(2 + (900000 / 50))];
      UChar len [6][258];

      Int32 limit [6][258];
      Int32 base [6][258];
      Int32 perm [6][258];
      Int32 minLens[6];


      Int32 save_i;
      Int32 save_j;
      Int32 save_t;
      Int32 save_alphaSize;
      Int32 save_nGroups;
      Int32 save_nSelectors;
      Int32 save_EOB;
      Int32 save_groupNo;
      Int32 save_groupPos;
      Int32 save_nextSym;
      Int32 save_nblockMAX;
      Int32 save_nblock;
      Int32 save_es;
      Int32 save_N;
      Int32 save_curr;
      Int32 save_zt;
      Int32 save_zn;
      Int32 save_zvec;
      Int32 save_zj;
      Int32 save_gSel;
      Int32 save_gMinlen;
      Int32* save_gLimit;
      Int32* save_gBase;
      Int32* save_gPerm;

   }
   DState;

Int32 BZ2_indexIntoF ( Int32 indx, Int32 *cftab )
{
   Int32 nb, na, mid;
   nb = 0;
   na = 256;
   do {
      mid = (nb + na) >> 1;
      if (indx >= cftab[mid]) nb = mid; else na = mid;
   }
   while (na - nb != 1);
   return nb;
}



//    函数名称中的"RLE"代表Run-Length Encoding，这是一个简单的无损数据压缩方法。

// 简而言之，该函数的目的是为了将RLE编码的数据解码并输出


// 初始化 DState 结构体的函数
DState* initializeDState() {
    DState* ds = (DState*)malloc(sizeof(DState));
    if (ds == NULL) {
        return NULL; // 分配内存失败
    }

    memset(ds, 0, sizeof(DState)); // 将所有成员设置为0

    ds->strm = (bz_stream*)malloc(sizeof(bz_stream));
    if (ds->strm == NULL) {
        free(ds);
        return NULL; // 为 bz_stream 分配内存失败
    }

    memset(ds->strm, 0, sizeof(bz_stream)); // 将 bz_stream 的所有成员设置为0

    // 如有需要，设置默认值

    return ds;
}

// 上面的函数 `initializeDState` 会创建一个 `DState` 的实例并进行初始化。
// 所有成员都会被设置为 0（对于指针则为 NULL）。如果你想使用特定的默认值而不是零，可以明确地设置它们。

// 你可能还需要编写一个函数，用于在完成结构体后释放内存：


void freeDState(DState* ds) {
    if (ds) {
        if (ds->strm) {
            free(ds->strm);
        }
        free(ds);
    }
}

static
Bool unRLE_obuf_to_output_FAST ( DState* s )
{
   UChar k1;

   if (s->blockRandomised) {   // 如果数据块是随机化的
 
      while (((Bool)1)) {

         while (((Bool)1)) {
              // 如果输出缓冲区没有可用的空间，则返回false
            if (s->strm->avail_out == 0) return ((Bool)0);
            // 如果state_out_len为0，则退出内部循环
            if (s->state_out_len == 0) break;
            // 将state_out_ch的值写入到输出流中，并进行CRC校验
            *( (UChar*)(s->strm->next_out) ) = s->state_out_ch;
            { s->calculatedBlockCRC = (s->calculatedBlockCRC << 8) ^ BZ2_crc32Table[(s->calculatedBlockCRC >> 24) ^ ((UChar)s->state_out_ch)]; };
            // 更新相关计数和指针
            s->state_out_len--;
            s->strm->next_out++;
            s->strm->avail_out--;
            s->strm->total_out_lo32++;
            if (s->strm->total_out_lo32 == 0) s->strm->total_out_hi32++;
         }
         // 检查nblock_used是否超过保存的nblock+1，如果是，则返回false
         if (s->nblock_used == s->save_nblock+1) return ((Bool)0);

        // 如果nblock_used大于保存的nblock+1，返回true
         if (s->nblock_used > s->save_nblock+1)
            return ((Bool)1);

        // 初始化状态输出长度为1
        s->state_out_len = 1;

        // 设置输出字符为k0的当前值
        s->state_out_ch = s->k0;

        // 检查当前位置是否超过阈值，如果超过则返回真
        if (s->tPos >= (UInt32)100000 * (UInt32)s->blockSize100k) return ((Bool)1);

        // 使用tPos作为索引从tt数组中获取下一个值
        s->tPos = s->tt[s->tPos];

        // 从tPos中提取最低有效字节
        k1 = (UChar)(s->tPos & 0xff);

        // 将tPos的值右移8位
        s->tPos >>= 8;

        // 检查并处理与rNToGo和rTPos相关的随机数生成
        if (s->rNToGo == 0) {
            s->rNToGo = BZ2_rNums[s->rTPos];
            s->rTPos++;
            if (s->rTPos == 512) s->rTPos = 0;
        }
        s->rNToGo--;

        // 根据rNToGo的值切换k1的值
        k1 ^= ((s->rNToGo == 1) ? 1 : 0);

        // 增加已使用的块数
        s->nblock_used++;

        // 检查我们是否达到某个块限制，如果达到则继续循环
        if (s->nblock_used == s->save_nblock+1) continue;

        // 如果k1与k0不同，更新k0并继续下一次迭代
        if (k1 != s->k0) {
            s->k0 = k1;
            continue;
        };

        // 接下来的几个代码块是重复的，处理state_out_len值为2、3和更高。
        // 对于每个代码块:

        // 1. 设置state_out_len
        // 2. 检查tPos阈值并在超过时返回
        // 3. 从tt数组中提取值
        // 4. 使用rNToGo和rTPos处理随机数生成
        // 5. 根据rNToGo切换k1的值
        // 6. 增加nblock_used
        // 7. 检查块使用限制
        // 8. 如果k1与k0不同则更新k0并继续下一次迭代
        // 这个循环继续，其中如何设置和使用state_out_len和k0有所不同。
         s->state_out_len = 2;
         if (s->tPos >= (UInt32)100000 * (UInt32)s->blockSize100k) return ((Bool)1); s->tPos = s->tt[s->tPos]; k1 = (UChar)(s->tPos & 0xff); s->tPos >>= 8;; if (s->rNToGo == 0) { s->rNToGo = BZ2_rNums[s->rTPos]; s->rTPos++; if (s->rTPos == 512) s->rTPos = 0; } s->rNToGo--;;
         k1 ^= ((s->rNToGo == 1) ? 1 : 0); s->nblock_used++;
         if (s->nblock_used == s->save_nblock+1) continue;
         if (k1 != s->k0) { s->k0 = k1; continue; };

         s->state_out_len = 3;
         if (s->tPos >= (UInt32)100000 * (UInt32)s->blockSize100k) return ((Bool)1); s->tPos = s->tt[s->tPos]; k1 = (UChar)(s->tPos & 0xff); s->tPos >>= 8;; if (s->rNToGo == 0) { s->rNToGo = BZ2_rNums[s->rTPos]; s->rTPos++; if (s->rTPos == 512) s->rTPos = 0; } s->rNToGo--;;
         k1 ^= ((s->rNToGo == 1) ? 1 : 0); s->nblock_used++;
         if (s->nblock_used == s->save_nblock+1) continue;
         if (k1 != s->k0) { s->k0 = k1; continue; };

         if (s->tPos >= (UInt32)100000 * (UInt32)s->blockSize100k) return ((Bool)1); s->tPos = s->tt[s->tPos]; k1 = (UChar)(s->tPos & 0xff); s->tPos >>= 8;; if (s->rNToGo == 0) { s->rNToGo = BZ2_rNums[s->rTPos]; s->rTPos++; if (s->rTPos == 512) s->rTPos = 0; } s->rNToGo--;;
         k1 ^= ((s->rNToGo == 1) ? 1 : 0); s->nblock_used++;
         s->state_out_len = ((Int32)k1) + 4;
         if (s->tPos >= (UInt32)100000 * (UInt32)s->blockSize100k) return ((Bool)1); s->tPos = s->tt[s->tPos]; s->k0 = (UChar)(s->tPos & 0xff); s->tPos >>= 8;; if (s->rNToGo == 0) { s->rNToGo = BZ2_rNums[s->rTPos]; s->rTPos++; if (s->rTPos == 512) s->rTPos = 0; } s->rNToGo--;;
         s->k0 ^= ((s->rNToGo == 1) ? 1 : 0); s->nblock_used++;
      }

   } else {
       // 如果数据块不是随机化的
      // 这部分代码与上面的逻辑类似，但是省略了与随机化相关的处理
   // 复制某些压缩状态变量为局部变量，以优化性能和简化访问
    UInt32 c_calculatedBlockCRC = s->calculatedBlockCRC;
    UChar c_state_out_ch = s->state_out_ch;
    Int32 c_state_out_len = s->state_out_len;
    Int32 c_nblock_used = s->nblock_used;
    Int32 c_k0 = s->k0;
    UInt32* c_tt = s->tt;
    UInt32 c_tPos = s->tPos;
    char* cs_next_out = s->strm->next_out;
    unsigned int cs_avail_out = s->strm->avail_out;
    Int32 ro_blockSize100k = s->blockSize100k;

    // 初始化可用输出空间
    UInt32 avail_out_INIT = cs_avail_out;

    // 计算nblock的保存值
    Int32 s_save_nblockPP = s->save_nblock+1;

    unsigned int total_out_lo32_old;

    while (((Bool)1)) {

        // 当c_state_out_len大于0时，我们有未写入的数据
        if (c_state_out_len > 0) {
            while (((Bool)1)) {
                // 如果没有输出空间，返回
                if (cs_avail_out == 0) goto return_notr;
                if (c_state_out_len == 1) break;
                
                // 写入输出字符并更新CRC校验值
                *( (UChar*)(cs_next_out) ) = c_state_out_ch;
                { c_calculatedBlockCRC = (c_calculatedBlockCRC << 8) ^ BZ2_crc32Table[(c_calculatedBlockCRC >> 24) ^ ((UChar)c_state_out_ch)]; };
                c_state_out_len--;
                cs_next_out++;
                cs_avail_out--;
            }

            s_state_out_len_eq_one:
            {
                // 如果没有输出空间，设置c_state_out_len为1并返回
                if (cs_avail_out == 0) {
                    c_state_out_len = 1; goto return_notr;
                };
                
                // 写入输出字符并更新CRC校验值
                *( (UChar*)(cs_next_out) ) = c_state_out_ch;
                { c_calculatedBlockCRC = (c_calculatedBlockCRC << 8) ^ BZ2_crc32Table[(c_calculatedBlockCRC >> 24) ^ ((UChar)c_state_out_ch)]; };
                cs_next_out++;
                cs_avail_out--;
            }
        }

        // 如果使用的块数量大于保存的块数量，返回
        if (c_nblock_used > s_save_nblockPP)
            return ((Bool)1);

        // 如果使用的块数量等于保存的块数量，设置c_state_out_len为0并返回
        if (c_nblock_used == s_save_nblockPP) {
            c_state_out_len = 0; goto return_notr;
        };


         c_state_out_ch = c_k0;
         if (c_tPos >= (UInt32)100000 * (UInt32)ro_blockSize100k) return ((Bool)1); c_tPos = c_tt[c_tPos]; k1 = (UChar)(c_tPos & 0xff); c_tPos >>= 8;; c_nblock_used++;
         if (k1 != c_k0) {
            c_k0 = k1; goto s_state_out_len_eq_one;
         };
         if (c_nblock_used == s_save_nblockPP)
            goto s_state_out_len_eq_one;

         c_state_out_len = 2;
         if (c_tPos >= (UInt32)100000 * (UInt32)ro_blockSize100k) return ((Bool)1); c_tPos = c_tt[c_tPos]; k1 = (UChar)(c_tPos & 0xff); c_tPos >>= 8;; c_nblock_used++;
         if (c_nblock_used == s_save_nblockPP) continue;
         if (k1 != c_k0) { c_k0 = k1; continue; };

         c_state_out_len = 3;
         if (c_tPos >= (UInt32)100000 * (UInt32)ro_blockSize100k) return ((Bool)1); c_tPos = c_tt[c_tPos]; k1 = (UChar)(c_tPos & 0xff); c_tPos >>= 8;; c_nblock_used++;
         if (c_nblock_used == s_save_nblockPP) continue;
         if (k1 != c_k0) { c_k0 = k1; continue; };

         if (c_tPos >= (UInt32)100000 * (UInt32)ro_blockSize100k) return ((Bool)1); c_tPos = c_tt[c_tPos]; k1 = (UChar)(c_tPos & 0xff); c_tPos >>= 8;; c_nblock_used++;
         c_state_out_len = ((Int32)k1) + 4;
         if (c_tPos >= (UInt32)100000 * (UInt32)ro_blockSize100k) return ((Bool)1); c_tPos = c_tt[c_tPos]; c_k0 = (UChar)(c_tPos & 0xff); c_tPos >>= 8;; c_nblock_used++;
      }

      return_notr:
      total_out_lo32_old = s->strm->total_out_lo32;
      s->strm->total_out_lo32 += (avail_out_INIT - cs_avail_out);
      if (s->strm->total_out_lo32 < total_out_lo32_old)
         s->strm->total_out_hi32++;


      s->calculatedBlockCRC = c_calculatedBlockCRC;
      s->state_out_ch = c_state_out_ch;
      s->state_out_len = c_state_out_len;
      s->nblock_used = c_nblock_used;
      s->k0 = c_k0;
      s->tt = c_tt;
      s->tPos = c_tPos;
      s->strm->next_out = cs_next_out;
      s->strm->avail_out = cs_avail_out;

   }
   return ((Bool)0);
}


void test_unRLE_obuf_to_output_FAST(void) {
    DState* ds = initializeDState();
    // ... 设定s的各种属性，可能需要制定输入和输出数据等
    Bool result = unRLE_obuf_to_output_FAST(ds);
    CU_ASSERT_TRUE(result);  // 或者 CU_ASSERT_FALSE(result); 取决于你的预期输出
    freeDState(ds);
}

int main() {
    CU_initialize_registry();
    CU_pSuite suite = CU_add_suite("TestSuite", NULL, NULL);
    CU_add_test(suite, "test_unRLE_obuf_to_output_FAST", test_unRLE_obuf_to_output_FAST);
    CU_basic_set_mode(CU_BRM_VERBOSE);
    CU_basic_run_tests();
    CU_cleanup_registry();
    return 0;
}