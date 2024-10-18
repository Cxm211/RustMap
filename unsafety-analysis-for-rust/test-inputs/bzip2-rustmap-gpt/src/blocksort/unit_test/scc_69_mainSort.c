#include "../typedef.h"
#include "./scc_68_mainQSort3.c"
// 这个函数是一个压缩算法中的一部分，特别是bzip2算法。主要用于进行Burrows-Wheeler transform (BWT)的前期排序，这是bzip2压缩算法中的一个关键步骤。
// 主排序函数，用于BWT转换中的前期排序
static
void mainSort ( UInt32* ptr,          // 指针数组
                UChar* block,         // 块数据
                UInt16* quadrant,     // 存储字符信息的区域
                UInt32* ftab,         // 频率表
                Int32 nblock,         // 块大小
                Int32 verb,           // 详细度级别
                Int32* budget )       // 用于限制处理量
{
   Int32 i, j, k, ss, sb;
   Int32 runningOrder[256];
   Bool bigDone[256];
   Int32 copyStart[256];
   Int32 copyEnd [256];
   UChar c1;
   Int32 numQSorted;
   UInt16 s;
   if (verb >= 4) fprintf(
                 stderr
                 ,"        main sort initialise ...\n");

      /*-- set up the 2-byte frequency table --*/
    /* 初始化 2-byte 频率表 */
   for (i = 65536; i >= 0; i--) ftab[i] = 0;

   j = block[0] << 8;
   i = nblock-1;
   for (; i >= 3; i -= 4) {
      quadrant[i] = 0;
      j = (j >> 8) | ( ((UInt16)block[i]) << 8);
      ftab[j]++;
      quadrant[i-1] = 0;
      j = (j >> 8) | ( ((UInt16)block[i-1]) << 8);
      ftab[j]++;
      quadrant[i-2] = 0;
      j = (j >> 8) | ( ((UInt16)block[i-2]) << 8);
      ftab[j]++;
      quadrant[i-3] = 0;
      j = (j >> 8) | ( ((UInt16)block[i-3]) << 8);
      ftab[j]++;
   }
   for (; i >= 0; i--) {
      quadrant[i] = 0;
      j = (j >> 8) | ( ((UInt16)block[i]) << 8);
      ftab[j]++;
   }

   /*-- (emphasises close relationship of block & quadrant) --*/
   for (i = 0; i < (2 + 12 + 18 + 2); i++) {
      block [nblock+i] = block[i];
      quadrant[nblock+i] = 0;
   }

   if (verb >= 4) fprintf(
                 stderr
                 ,"        bucket sorting ...\n");

   /*-- Complete the initial radix sort --*/
   for (i = 1; i <= 65536; i++) ftab[i] += ftab[i-1];

   s = block[0] << 8;
   i = nblock-1;
      /* 为 2-byte 频率表填充值 */
   for (; i >= 3; i -= 4) {
      s = (s >> 8) | (block[i] << 8);
      j = ftab[s] -1;
      ftab[s] = j;
      ptr[j] = i;
      s = (s >> 8) | (block[i-1] << 8);
      j = ftab[s] -1;
      ftab[s] = j;
      ptr[j] = i-1;
      s = (s >> 8) | (block[i-2] << 8);
      j = ftab[s] -1;
      ftab[s] = j;
      ptr[j] = i-2;
      s = (s >> 8) | (block[i-3] << 8);
      j = ftab[s] -1;
      ftab[s] = j;
      ptr[j] = i-3;
   }
   for (; i >= 0; i--) {
      s = (s >> 8) | (block[i] << 8);
      j = ftab[s] -1;
      ftab[s] = j;
      ptr[j] = i;
   }



   /*--
      Now ftab contains the first loc of every small bucket.
      Calculate the running order, from smallest to largest
      big bucket.
   --*/
   for (i = 0; i <= 255; i++) {
      bigDone [i] = ((Bool)0);
      runningOrder[i] = i;
   }

   {
      Int32 vv;
      Int32 h = 1;
      do h = 3 * h + 1; while (h <= 256);
      do {
         h = h / 3;
         for (i = h; i <= 255; i++) {
            vv = runningOrder[i];
            j = i;
            while ( (ftab[((runningOrder[j-h])+1) << 8] - ftab[(runningOrder[j-h]) << 8]) > (ftab[((vv)+1) << 8] - ftab[(vv) << 8]) ) {
               runningOrder[j] = runningOrder[j-h];
               j = j - h;
               if (j <= (h - 1)) goto zero;
            }
            zero:
            runningOrder[j] = vv;
         }
      } while (h != 1);
   }




      /*--
      The main sorting loop.
   --*/
   numQSorted = 0;

   for (i = 0; i <= 255; i++) {
         /*
            处理开始时最不满的大桶。
            基本上，这是一个三步的过程，其中我们调用
            mainQSort3来排序小桶[ss, j]，
            但我们也会尽量避免调用，如果可以的话。
         */
      ss = runningOrder[i];
        /*--
         Step 1:
        通过快速排序任何未排序的小桶 [ss, j]，对于 j != ss
        来完成大桶 [ss]。
        希望先前的指针扫描阶段已经
        完成了许多小桶 [ss, j]，所以
        我们根本不需要对它们进行排序。
      --*/
      for (j = 0; j <= 255; j++) {
         if (j != ss) {
            sb = (ss << 8) + j;
            if ( ! (ftab[sb] & (1 << 21)) ) {
               Int32 lo = ftab[sb] & (~((1 << 21)));
               Int32 hi = (ftab[sb+1] & (~((1 << 21)))) - 1;
               if (hi > lo) {
                  if (verb >= 4)
                     fprintf(
                    stderr
                    ,"        qsort [0x%x, 0x%x]   " "done %d   this %d\n",ss,j,numQSorted,hi - lo + 1)

                                                                ;
                  mainQSort3 (
                     ptr, block, quadrant, nblock,
                     lo, hi, 2, budget
                  );
                  numQSorted += (hi - lo + 1);
                  if (*budget < 0) return;
               }
            }
            ftab[sb] |= (1 << 21);
         }
      }

      { if (!(!bigDone[ss])) BZ2_bz__AssertH__fail ( 1006 ); };
      /*--
         Step 2:
            现在扫描这个大桶 [ss]，以合成
            小桶 [t, ss] 的排序顺序，对于所有的 t，
            包括，像魔术一样，也包括桶 [ss,ss]。
            这将避免在后续的步骤1中做真正的工作。
      --*/
      {
         for (j = 0; j <= 255; j++) {
            copyStart[j] = ftab[(j << 8) + ss] & (~((1 << 21)));
            copyEnd [j] = (ftab[(j << 8) + ss + 1] & (~((1 << 21)))) - 1;
         }
         for (j = ftab[ss << 8] & (~((1 << 21))); j < copyStart[ss]; j++) {
            k = ptr[j]-1; if (k < 0) k += nblock;
            c1 = block[k];
            if (!bigDone[c1])
               ptr[ copyStart[c1]++ ] = k;
         }
         for (j = (ftab[(ss+1) << 8] & (~((1 << 21)))) - 1; j > copyEnd[ss]; j--) {
            k = ptr[j]-1; if (k < 0) k += nblock;
            c1 = block[k];
            if (!bigDone[c1])
               ptr[ copyEnd[c1]-- ] = k;
         }
      }

      { if (!((copyStart[ss]-1 == copyEnd[ss]) || (copyStart[ss] == 0 && copyEnd[ss] == nblock-1))) BZ2_bz__AssertH__fail ( 1007 ); }
                  /* 在bzip2-1.0.0和1.0.1中遗漏的极为罕见的情况。
               通过压缩大约48.5百万个字符251的序列，
               可以证明这种情况的必要性；然后1.0.0/1.0.1将在此处崩溃。*/
      for (j = 0; j <= 255; j++) ftab[(j << 8) + ss] |= (1 << 21);
      bigDone[ss] = ((Bool)1);
     /*--
         Step 3:
            [ss] 大桶现在已完成。记录这个事实，
            并更新象限描述符。记得还要
            更新超出区域的象限，如果
            必要的话。"if (i < 255)" 测试仅仅跳过
            最后一个桶的这个更新，因为
            对最后一个桶进行更新是没有意义的。

            象限数组提供了一种方式来增量
            缓存排序顺序，因为它们出现的时候，以便于
            使 fullGtU() 中的后续比较更快地完成。
            对于重复的块，这有很大的
            区别（但不足以避免
            回退排序机制，指数基数排序）。

            确切的含义是：在任何时候：

                对于 0 <= i < nblock 和 0 <= j <= nblock

                如果 block[i] != block[j]，

                    则quadrant[i] 和 
                        quadrant[j] 的相对值是无意义的。

                    否则 {
                        如果 quadrant[i] < quadrant[j]
                        那么从 i 开始的字符串在字典上
                        早于从 j 开始的字符串

                        否则如果 quadrant[i] > quadrant[j]
                        那么从 j 开始的字符串在字典上
                        早于从 i 开始的字符串

                        否则
                        从 i 和 j 开始的字符串的相对顺序
                        还没有被确定。
                    }
      --*/
      if (i < 255) {
         Int32 bbStart = ftab[ss << 8] & (~((1 << 21)));
         Int32 bbSize = (ftab[(ss+1) << 8] & (~((1 << 21)))) - bbStart;
         Int32 shifts = 0;

         while ((bbSize >> shifts) > 65534) shifts++;

         for (j = bbSize-1; j >= 0; j--) {
            Int32 a2update = ptr[bbStart + j];
            UInt16 qVal = (UInt16)(j >> shifts);
            quadrant[a2update] = qVal;
            if (a2update < (2 + 12 + 18 + 2))
               quadrant[a2update + nblock] = qVal;
         }
         { if (!(((bbSize-1) >> shifts) <= 65535)) BZ2_bz__AssertH__fail ( 1002 ); };
      }

   }

   if (verb >= 4)
      fprintf(
     stderr
     ,"        %d pointers, %d sorted, %d scanned\n",nblock,numQSorted,nblock - numQSorted)
                                                          ;
}

void testMainSort() {
    Int32 nblock = 199981;
    UInt32* ptr = (UInt32*) malloc(nblock * sizeof(UInt32));
    UChar* block = (UChar*) malloc((nblock + (2 + 12 + 18 + 2)) * sizeof(UChar));
    UInt16* quadrant = (UInt16*) malloc((nblock + (2 + 12 + 18 + 2)) * sizeof(UInt16));
    UInt32* ftab = (UInt32*) malloc(65537 * sizeof(UInt32));
    Int32 verb = 0;
    Int32 budget_initial = 1799829;
    Int32* budget = &budget_initial;

    // 初始化数据
    for (Int32 i = 0; i < nblock; i++) {
        block[i] = rand() % 256; // 假设每个字符是随机的
    }

    mainSort(ptr, block, quadrant, ftab, nblock, verb, budget);

    // 简单的检查，看budget是否被减少了
    printf("Initial Budget: %d\n", budget_initial);
    printf("Remaining Budget: %d\n", *budget);

    // 清理
    free(ptr);
    free(block);
    free(quadrant);
    free(ftab);
}

// int main() {
//     testMainSort();
//     return 0;
// }