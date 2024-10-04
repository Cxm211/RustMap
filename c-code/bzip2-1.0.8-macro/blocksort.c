
/*-------------------------------------------------------------*/
/*--- Block sorting machinery                               ---*/
/*---                                           blocksort.c ---*/
/*-------------------------------------------------------------*/

/* ------------------------------------------------------------------
   This file is part of bzip2/libbzip2, a program and library for
   lossless, block-sorting data compression.

   bzip2/libbzip2 version 1.0.8 of 13 July 2019
   Copyright (C) 1996-2019 Julian Seward <jseward@acm.org>

   Please read the WARNING, DISCLAIMER and PATENTS sections in the 
   README file.

   This program is released under the terms of the license contained
   in the file LICENSE.
   ------------------------------------------------------------------ */

#include "bzlib_private.h"

/*---------------------------------------------*/
/*--- Fallback O(N log(N)^2) sorting        ---*/
/*--- algorithm, for repetitive blocks      ---*/
/*---------------------------------------------*/

/*---------------------------------------------*/
static 
__inline__
void fallbackSimpleSort ( UInt32* fmap, 
                          UInt32* eclass, 
                          Int32   lo, 
                          Int32   hi )
{
   Int32 i, j, tmp;
   UInt32 ec_tmp;

   if (lo == hi) return;

   if (hi - lo > 3) {
      for ( i = hi-4; i >= lo; i-- ) {
         tmp = fmap[i];
         ec_tmp = eclass[tmp];
         for ( j = i+4; j <= hi && ec_tmp > eclass[fmap[j]]; j += 4 )
            fmap[j-4] = fmap[j];
         fmap[j-4] = tmp;
      }
   }

   for ( i = hi-1; i >= lo; i-- ) {
      tmp = fmap[i];
      ec_tmp = eclass[tmp];
      for ( j = i+1; j <= hi && ec_tmp > eclass[fmap[j]]; j++ )
         fmap[j-1] = fmap[j];
      fmap[j-1] = tmp;
   }
}

/*---------------------------------------------*/
/* Replacing macros with inline functions */

/* Inline function to swap two integers */
static __inline__ void fswap(UInt32* zz1, UInt32* zz2) {
    UInt32 zztmp = *zz1;
    *zz1 = *zz2;
    *zz2 = zztmp;
}

/* Inline function to swap ranges of elements in fmap */
static __inline__ void fvswap(Int32 zzp1, Int32 zzp2, Int32 zzn, UInt32* fmap) {
    Int32 yyp1 = zzp1;
    Int32 yyp2 = zzp2;
    Int32 yyn  = zzn;
    while (yyn > 0) {
        fswap(&fmap[yyp1], &fmap[yyp2]);
        yyp1++;
        yyp2++;
        yyn--;
    }
}

/* Inline function to return the minimum of two values */
static __inline__ Int32 fmin(Int32 a, Int32 b) {
    return (a < b) ? a : b;
}

/* Inline function to push lo and hi values onto the stack */
static __inline__ void fpush(Int32* stackLo, Int32* stackHi, Int32* sp, Int32 lz, Int32 hz) {
    stackLo[*sp] = lz;
    stackHi[*sp] = hz;
    (*sp)++;
}

/* Inline function to pop lo and hi values from the stack */
static __inline__ void fpop(Int32* stackLo, Int32* stackHi, Int32* sp, Int32* lz, Int32* hz) {
    (*sp)--;
    *lz = stackLo[*sp];
    *hz = stackHi[*sp];
}

/* Constants */
const Int32 FALLBACK_QSORT_SMALL_THRESH = 10;
const Int32 FALLBACK_QSORT_STACK_SIZE = 100;

static
void fallbackQSort3 ( UInt32* fmap, 
                      UInt32* eclass,
                      Int32   loSt, 
                      Int32   hiSt )
{
   Int32 unLo, unHi, ltLo, gtHi, n, m;
   Int32 sp, lo, hi;
   UInt32 med, r, r3;
   Int32 stackLo[FALLBACK_QSORT_STACK_SIZE];
   Int32 stackHi[FALLBACK_QSORT_STACK_SIZE];

   r = 0;

   sp = 0;
   fpush(stackLo, stackHi, &sp, loSt, hiSt);

   while (sp > 0) {

      AssertH(sp < FALLBACK_QSORT_STACK_SIZE - 1, 1004);

      fpop(stackLo, stackHi, &sp, &lo, &hi);
      if (hi - lo < FALLBACK_QSORT_SMALL_THRESH) {
         fallbackSimpleSort(fmap, eclass, lo, hi);
         continue;
      }

      /* Random partitioning.  Median of 3 sometimes fails to
         avoid bad cases.  Median of 9 seems to help but 
         looks rather expensive.  This too seems to work but
         is cheaper.  Guidance for the magic constants 
         7621 and 32768 is taken from Sedgewick's algorithms
         book, chapter 35.
      */
      r = ((r * 7621) + 1) % 32768;
      r3 = r % 3;
      if (r3 == 0) med = eclass[fmap[lo]];
      else if (r3 == 1) med = eclass[fmap[(lo+hi)>>1]];
      else med = eclass[fmap[hi]];

      unLo = ltLo = lo;
      unHi = gtHi = hi;

      while (1) {
         while (1) {
            if (unLo > unHi) break;
            n = (Int32)eclass[fmap[unLo]] - (Int32)med;
            if (n == 0) {
               fswap(&fmap[unLo], &fmap[ltLo]);
               ltLo++; unLo++;
               continue;
            }
            if (n > 0) break;
            unLo++;
         }
         while (1) {
            if (unLo > unHi) break;
            n = (Int32)eclass[fmap[unHi]] - (Int32)med;
            if (n == 0) {
               fswap(&fmap[unHi], &fmap[gtHi]);
               gtHi--; unHi--;
               continue;
            }
            if (n < 0) break;
            unHi--;
         }
         if (unLo > unHi) break;
         fswap(&fmap[unLo], &fmap[unHi]); unLo++; unHi--;
      }

      AssertD(unHi == unLo-1, "fallbackQSort3(2)");

      if (gtHi < ltLo) continue;

      n = fmin(ltLo-lo, unLo-ltLo);
      fvswap(lo, unLo-n, n, fmap);
      m = fmin(hi-gtHi, gtHi-unHi);
      fvswap(unLo, hi-m+1, m, fmap);

      n = lo + unLo - ltLo - 1;
      m = hi - (gtHi - unHi) + 1;

      if (n - lo > hi - m) {
         fpush(stackLo, stackHi, &sp, lo, n);
         fpush(stackLo, stackHi, &sp, m, hi);
      } else {
         fpush(stackLo, stackHi, &sp, m, hi);
         fpush(stackLo, stackHi, &sp, lo, n);
      }
   }
}

/*---------------------------------------------*/
/* Pre:
      nblock > 0
      eclass exists for [0 .. nblock-1]
      ((UChar*)eclass) [0 .. nblock-1] holds block
      ptr exists for [0 .. nblock-1]

   Post:
      ((UChar*)eclass) [0 .. nblock-1] holds block
      All other areas of eclass destroyed
      fmap [0 .. nblock-1] holds sorted order
      bhtab [ 0 .. 2+(nblock/32) ] destroyed
*/
static __inline__ void SET_BH(UInt32* bhtab, Int32 zz) {
    bhtab[zz >> 5] |= ((UInt32)1 << (zz & 31));
}

static __inline__ void CLEAR_BH(UInt32* bhtab, Int32 zz) {
    bhtab[zz >> 5] &= ~((UInt32)1 << (zz & 31));
}

static __inline__ Int32 ISSET_BH(UInt32* bhtab, Int32 zz) {
    return (bhtab[zz >> 5] & ((UInt32)1 << (zz & 31)));
}

static __inline__ UInt32 WORD_BH(UInt32* bhtab, Int32 zz) {
    return bhtab[zz >> 5];
}

static __inline__ Int32 UNALIGNED_BH(Int32 zz) {
    return zz & 0x01f;
}

static
void fallbackSort ( UInt32* fmap, 
                    UInt32* eclass, 
                    UInt32* bhtab,
                    Int32   nblock,
                    Int32   verb )
{
   Int32 ftab[257];
   Int32 ftabCopy[256];
   Int32 H, i, j, k, l, r, cc, cc1;
   Int32 nNotDone;
   Int32 nBhtab;
   UChar* eclass8 = (UChar*)eclass;

   /*--
      Initial 1-char radix sort to generate
      initial fmap and initial BH bits.
   --*/
   if (verb >= 4)
      VPrintf0 ( "        bucket sorting ...\n" );
   for (i = 0; i < 257; i++) ftab[i] = 0;
   for (i = 0; i < nblock; i++) ftab[eclass8[i]]++;
   for (i = 0; i < 256; i++) ftabCopy[i] = ftab[i];
   for (i = 1; i < 257; i++) ftab[i] += ftab[i-1];

   for (i = 0; i < nblock; i++) {
      j = eclass8[i];
      k = ftab[j] - 1;
      ftab[j] = k;
      fmap[k] = i;
   }

   nBhtab = 2 + (nblock / 32);
   for (i = 0; i < nBhtab; i++) bhtab[i] = 0;
   for (i = 0; i < 256; i++) SET_BH(bhtab, ftab[i]);

   /*--
      Inductively refine the buckets. Kind-of an
      "exponential radix sort" (!), inspired by the
      Manber-Myers suffix array construction algorithm.
   --*/

   /*-- set sentinel bits for block-end detection --*/
   for (i = 0; i < 32; i++) { 
      SET_BH(bhtab, nblock + 2*i);
      CLEAR_BH(bhtab, nblock + 2*i + 1);
   }

   /*-- the log(N) loop --*/
   H = 1;
   while (1) {

      if (verb >= 4) 
         VPrintf1 ( "        depth %6d has ", H );

      j = 0;
      for (i = 0; i < nblock; i++) {
         if (ISSET_BH(bhtab, i)) j = i;
         k = fmap[i] - H; if (k < 0) k += nblock;
         eclass[k] = j;
      }

      nNotDone = 0;
      r = -1;
      while (1) {

         /*-- find the next non-singleton bucket --*/
         k = r + 1;
         while (ISSET_BH(bhtab, k) && UNALIGNED_BH(k)) k++;
         if (ISSET_BH(bhtab, k)) {
            while (WORD_BH(bhtab, k) == 0xffffffff) k += 32;
            while (ISSET_BH(bhtab, k)) k++;
         }
         l = k - 1;
         if (l >= nblock) break;
         while (!ISSET_BH(bhtab, k) && UNALIGNED_BH(k)) k++;
         if (!ISSET_BH(bhtab, k)) {
            while (WORD_BH(bhtab, k) == 0x00000000) k += 32;
            while (!ISSET_BH(bhtab, k)) k++;
         }
         r = k - 1;
         if (r >= nblock) break;

         /*-- now [l, r] bracket current bucket --*/
         if (r > l) {
            nNotDone += (r - l + 1);
            fallbackQSort3(fmap, eclass, l, r);

            /*-- scan bucket and generate header bits-- */
            cc = -1;
            for (i = l; i <= r; i++) {
               cc1 = eclass[fmap[i]];
               if (cc != cc1) { SET_BH(bhtab, i); cc = cc1; };
            }
         }
      }

      if (verb >= 4) 
         VPrintf1("%6d unresolved strings\n", nNotDone);

      H *= 2;
      if (H > nblock || nNotDone == 0) break;
   }

   /*-- 
      Reconstruct the original block in
      eclass8 [0 .. nblock-1], since the
      previous phase destroyed it.
   --*/
   if (verb >= 4)
      VPrintf0("        reconstructing block ...\n");
   j = 0;
   for (i = 0; i < nblock; i++) {
      while (ftabCopy[j] == 0) j++;
      ftabCopy[j]--;
      eclass8[fmap[i]] = (UChar)j;
   }
   AssertH(j < 256, 1005);
}

/*---------------------------------------------*/
/*--- The main, O(N^2 log(N)) sorting       ---*/
/*--- algorithm.  Faster for "normal"       ---*/
/*--- non-repetitive blocks.                ---*/
/*---------------------------------------------*/

/*---------------------------------------------*/
static
__inline__
Bool mainGtU ( UInt32  i1, 
               UInt32  i2,
               UChar*  block, 
               UInt16* quadrant,
               UInt32  nblock,
               Int32*  budget )
{
   Int32  k;
   UChar  c1, c2;
   UInt16 s1, s2;

   AssertD ( i1 != i2, "mainGtU" );
   /* 1 */
   c1 = block[i1]; c2 = block[i2];
   if (c1 != c2) return (c1 > c2);
   i1++; i2++;
   /* 2 */
   c1 = block[i1]; c2 = block[i2];
   if (c1 != c2) return (c1 > c2);
   i1++; i2++;
   /* 3 */
   c1 = block[i1]; c2 = block[i2];
   if (c1 != c2) return (c1 > c2);
   i1++; i2++;
   /* 4 */
   c1 = block[i1]; c2 = block[i2];
   if (c1 != c2) return (c1 > c2);
   i1++; i2++;
   /* 5 */
   c1 = block[i1]; c2 = block[i2];
   if (c1 != c2) return (c1 > c2);
   i1++; i2++;
   /* 6 */
   c1 = block[i1]; c2 = block[i2];
   if (c1 != c2) return (c1 > c2);
   i1++; i2++;
   /* 7 */
   c1 = block[i1]; c2 = block[i2];
   if (c1 != c2) return (c1 > c2);
   i1++; i2++;
   /* 8 */
   c1 = block[i1]; c2 = block[i2];
   if (c1 != c2) return (c1 > c2);
   i1++; i2++;
   /* 9 */
   c1 = block[i1]; c2 = block[i2];
   if (c1 != c2) return (c1 > c2);
   i1++; i2++;
   /* 10 */
   c1 = block[i1]; c2 = block[i2];
   if (c1 != c2) return (c1 > c2);
   i1++; i2++;
   /* 11 */
   c1 = block[i1]; c2 = block[i2];
   if (c1 != c2) return (c1 > c2);
   i1++; i2++;
   /* 12 */
   c1 = block[i1]; c2 = block[i2];
   if (c1 != c2) return (c1 > c2);
   i1++; i2++;

   k = nblock + 8;

   do {
      /* 1 */
      c1 = block[i1]; c2 = block[i2];
      if (c1 != c2) return (c1 > c2);
      s1 = quadrant[i1]; s2 = quadrant[i2];
      if (s1 != s2) return (s1 > s2);
      i1++; i2++;
      /* 2 */
      c1 = block[i1]; c2 = block[i2];
      if (c1 != c2) return (c1 > c2);
      s1 = quadrant[i1]; s2 = quadrant[i2];
      if (s1 != s2) return (s1 > s2);
      i1++; i2++;
      /* 3 */
      c1 = block[i1]; c2 = block[i2];
      if (c1 != c2) return (c1 > c2);
      s1 = quadrant[i1]; s2 = quadrant[i2];
      if (s1 != s2) return (s1 > s2);
      i1++; i2++;
      /* 4 */
      c1 = block[i1]; c2 = block[i2];
      if (c1 != c2) return (c1 > c2);
      s1 = quadrant[i1]; s2 = quadrant[i2];
      if (s1 != s2) return (s1 > s2);
      i1++; i2++;
      /* 5 */
      c1 = block[i1]; c2 = block[i2];
      if (c1 != c2) return (c1 > c2);
      s1 = quadrant[i1]; s2 = quadrant[i2];
      if (s1 != s2) return (s1 > s2);
      i1++; i2++;
      /* 6 */
      c1 = block[i1]; c2 = block[i2];
      if (c1 != c2) return (c1 > c2);
      s1 = quadrant[i1]; s2 = quadrant[i2];
      if (s1 != s2) return (s1 > s2);
      i1++; i2++;
      /* 7 */
      c1 = block[i1]; c2 = block[i2];
      if (c1 != c2) return (c1 > c2);
      s1 = quadrant[i1]; s2 = quadrant[i2];
      if (s1 != s2) return (s1 > s2);
      i1++; i2++;
      /* 8 */
      c1 = block[i1]; c2 = block[i2];
      if (c1 != c2) return (c1 > c2);
      s1 = quadrant[i1]; s2 = quadrant[i2];
      if (s1 != s2) return (s1 > s2);
      i1++; i2++;

      if (i1 >= nblock) i1 -= nblock;
      if (i2 >= nblock) i2 -= nblock;

      k -= 8;
      (*budget)--;
   }
      while (k >= 0);

   return False;
}


/*---------------------------------------------*/
/*--
   Knuth's increments seem to work better
   than Incerpi-Sedgewick here.  Possibly
   because the number of elems to sort is
   usually small, typically <= 20.
--*/
static
Int32 incs[14] = { 1, 4, 13, 40, 121, 364, 1093, 3280,
                   9841, 29524, 88573, 265720,
                   797161, 2391484 };

static
void mainSimpleSort ( UInt32* ptr,
                      UChar*  block,
                      UInt16* quadrant,
                      Int32   nblock,
                      Int32   lo, 
                      Int32   hi, 
                      Int32   d,
                      Int32*  budget )
{
   Int32 i, j, h, bigN, hp;
   UInt32 v;

   bigN = hi - lo + 1;
   if (bigN < 2) return;

   hp = 0;
   while (incs[hp] < bigN) hp++;
   hp--;

   for (; hp >= 0; hp--) {
      h = incs[hp];

      i = lo + h;
      while (True) {

         /*-- copy 1 --*/
         if (i > hi) break;
         v = ptr[i];
         j = i;
         while ( mainGtU ( 
                    ptr[j-h]+d, v+d, block, quadrant, nblock, budget 
                 ) ) {
            ptr[j] = ptr[j-h];
            j = j - h;
            if (j <= (lo + h - 1)) break;
         }
         ptr[j] = v;
         i++;

         /*-- copy 2 --*/
         if (i > hi) break;
         v = ptr[i];
         j = i;
         while ( mainGtU ( 
                    ptr[j-h]+d, v+d, block, quadrant, nblock, budget 
                 ) ) {
            ptr[j] = ptr[j-h];
            j = j - h;
            if (j <= (lo + h - 1)) break;
         }
         ptr[j] = v;
         i++;

         /*-- copy 3 --*/
         if (i > hi) break;
         v = ptr[i];
         j = i;
         while ( mainGtU ( 
                    ptr[j-h]+d, v+d, block, quadrant, nblock, budget 
                 ) ) {
            ptr[j] = ptr[j-h];
            j = j - h;
            if (j <= (lo + h - 1)) break;
         }
         ptr[j] = v;
         i++;

         if (*budget < 0) return;
      }
   }
}


/*---------------------------------------------*/
/*--
   The following is an implementation of
   an elegant 3-way quicksort for strings,
   described in a paper "Fast Algorithms for
   Sorting and Searching Strings", by Robert
   Sedgewick and Jon L. Bentley.
--*/

static __inline__ void mswap(UInt32* zz1, UInt32* zz2) {
    UInt32 zztmp = *zz1;
    *zz1 = *zz2;
    *zz2 = zztmp;
}

static __inline__ void mvswap(Int32 zzp1, Int32 zzp2, Int32 zzn, UInt32* ptr) {
    Int32 yyp1 = zzp1;
    Int32 yyp2 = zzp2;
    Int32 yyn  = zzn;
    while (yyn > 0) {
        mswap(&ptr[yyp1], &ptr[yyp2]);
        yyp1++;
        yyp2++;
        yyn--;
    }
}

static __inline__ UChar mmed3(UChar a, UChar b, UChar c) {
    UChar t;
    if (a > b) { t = a; a = b; b = t; }
    if (b > c) { 
        b = c;
        if (a > b) b = a;
    }
    return b;
}

static __inline__ Int32 mmin(Int32 a, Int32 b) {
    return (a < b) ? a : b;
}

static __inline__ void mpush(Int32 lz, Int32 hz, Int32 dz, Int32* stackLo, Int32* stackHi, Int32* stackD, Int32* sp) {
    stackLo[*sp] = lz;
    stackHi[*sp] = hz;
    stackD[*sp] = dz;
    (*sp)++;
}


static __inline__ void mpop(Int32* lz, Int32* hz, Int32* dz, Int32* stackLo, Int32* stackHi, Int32* stackD, Int32* sp) {
    (*sp)--;
    *lz = stackLo[*sp];
    *hz = stackHi[*sp];
    *dz = stackD[*sp];
}

static __inline__ Int32 mnextsize(Int32 az, Int32* nextHi, Int32* nextLo) {
    return nextHi[az] - nextLo[az];
}

static __inline__ void mnextswap(Int32 az, Int32 bz, Int32* nextLo, Int32* nextHi, Int32* nextD) {
    Int32 tz;
    tz = nextLo[az];
    nextLo[az] = nextLo[bz];
    nextLo[bz] = tz;

    tz = nextHi[az];
    nextHi[az] = nextHi[bz];
    nextHi[bz] = tz;

    tz = nextD[az];
    nextD[az] = nextD[bz];
    nextD[bz] = tz;
}

const Int32 MAIN_QSORT_SMALL_THRESH = 20;
const Int32 MAIN_QSORT_DEPTH_THRESH = (BZ_N_RADIX + BZ_N_QSORT);
const Int32 MAIN_QSORT_STACK_SIZE = 100;

static
void mainQSort3 ( UInt32* ptr,
                  UChar*  block,
                  UInt16* quadrant,
                  Int32   nblock,
                  Int32   loSt, 
                  Int32   hiSt, 
                  Int32   dSt,
                  Int32*  budget )
{
   Int32 unLo, unHi, ltLo, gtHi, n, m, med;
   Int32 sp, lo, hi, d;

   Int32 stackLo[MAIN_QSORT_STACK_SIZE];
   Int32 stackHi[MAIN_QSORT_STACK_SIZE];
   Int32 stackD [MAIN_QSORT_STACK_SIZE];

   Int32 nextLo[3];
   Int32 nextHi[3];
   Int32 nextD [3];

   sp = 0;
   mpush(loSt, hiSt, dSt, stackLo, stackHi, stackD, &sp);

   while (sp > 0) {

      AssertH(sp < MAIN_QSORT_STACK_SIZE - 2, 1001);

      mpop(&lo, &hi, &d, stackLo, stackHi, stackD, &sp);
      if (hi - lo < MAIN_QSORT_SMALL_THRESH || d > MAIN_QSORT_DEPTH_THRESH) {
         mainSimpleSort(ptr, block, quadrant, nblock, lo, hi, d, budget);
         if (*budget < 0) return;
         continue;
      }

      med = (Int32) mmed3(block[ptr[lo] + d], block[ptr[hi] + d], block[ptr[(lo + hi) >> 1] + d]);

      unLo = ltLo = lo;
      unHi = gtHi = hi;

      while (True) {
         while (True) {
            if (unLo > unHi) break;
            n = ((Int32)block[ptr[unLo] + d]) - med;
            if (n == 0) {
               mswap(&ptr[unLo], &ptr[ltLo]);
               ltLo++; unLo++; continue;
            }
            if (n > 0) break;
            unLo++;
         }
         while (True) {
            if (unLo > unHi) break;
            n = ((Int32)block[ptr[unHi] + d]) - med;
            if (n == 0) {
               mswap(&ptr[unHi], &ptr[gtHi]);
               gtHi--; unHi--; continue;
            }
            if (n < 0) break;
            unHi--;
         }
         if (unLo > unHi) break;
         mswap(&ptr[unLo], &ptr[unHi]); unLo++; unHi--;
      }

      AssertD(unHi == unLo - 1, "mainQSort3(2)");

      if (gtHi < ltLo) {
         mpush(lo, hi, d + 1, stackLo, stackHi, stackD, &sp);
         continue;
      }

      n = mmin(ltLo - lo, unLo - ltLo); mvswap(lo, unLo - n, n, ptr);
      m = mmin(hi - gtHi, gtHi - unHi); mvswap(unLo, hi - m + 1, m, ptr);

      n = lo + unLo - ltLo - 1;
      m = hi - (gtHi - unHi) + 1;

      nextLo[0] = lo;  nextHi[0] = n;   nextD[0] = d;
      nextLo[1] = m;   nextHi[1] = hi;  nextD[1] = d;
      nextLo[2] = n + 1; nextHi[2] = m - 1; nextD[2] = d + 1;

      if (mnextsize(0, nextHi, nextLo) < mnextsize(1, nextHi, nextLo)) mnextswap(0, 1, nextLo, nextHi, nextD);
      if (mnextsize(1, nextHi, nextLo) < mnextsize(2, nextHi, nextLo)) mnextswap(1, 2, nextLo, nextHi, nextD);
      if (mnextsize(0, nextHi, nextLo) < mnextsize(1, nextHi, nextLo)) mnextswap(0, 1, nextLo, nextHi, nextD);

      AssertD(mnextsize(0, nextHi, nextLo) >= mnextsize(1, nextHi, nextLo), "mainQSort3(8)");
      AssertD(mnextsize(1, nextHi, nextLo) >= mnextsize(2, nextHi, nextLo), "mainQSort3(9)");

      mpush(nextLo[0], nextHi[0], nextD[0], stackLo, stackHi, stackD, &sp);
      mpush(nextLo[1], nextHi[1], nextD[1], stackLo, stackHi, stackD, &sp);
      mpush(nextLo[2], nextHi[2], nextD[2], stackLo, stackHi, stackD, &sp);
   }
}


/*---------------------------------------------*/
/* Pre:
      nblock > N_OVERSHOOT
      block32 exists for [0 .. nblock-1 +N_OVERSHOOT]
      ((UChar*)block32) [0 .. nblock-1] holds block
      ptr exists for [0 .. nblock-1]

   Post:
      ((UChar*)block32) [0 .. nblock-1] holds block
      All other areas of block32 destroyed
      ftab [0 .. 65536 ] destroyed
      ptr [0 .. nblock-1] holds sorted order
      if (*budget < 0), sorting was abandoned
*/

static __inline__ Int32 BIGFREQ(Int32 b, UInt32* ftab) {
    return ftab[(b + 1) << 8] - ftab[b << 8];
}

const Int32 SETMASK = (1 << 21);

const Int32 CLEARMASK = ~(SETMASK);

static
void mainSort ( UInt32* ptr, 
                UChar*  block,
                UInt16* quadrant, 
                UInt32* ftab,
                Int32   nblock,
                Int32   verb,
                Int32*  budget )
{
   Int32  i, j, k, ss, sb;
   Int32  runningOrder[256];
   Bool   bigDone[256];
   Int32  copyStart[256];
   Int32  copyEnd  [256];
   UChar  c1;
   Int32  numQSorted;
   UInt16 s;
   if (verb >= 4) VPrintf0 ( "        main sort initialise ...\n" );

   /*-- set up the 2-byte frequency table --*/
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
   for (i = 0; i < BZ_N_OVERSHOOT; i++) {
      block   [nblock+i] = block[i];
      quadrant[nblock+i] = 0;
   }

   if (verb >= 4) VPrintf0 ( "        bucket sorting ...\n" );

   /*-- Complete the initial radix sort --*/
   for (i = 1; i <= 65536; i++) ftab[i] += ftab[i-1];

   s = block[0] << 8;
   i = nblock-1;
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
      bigDone     [i] = False;
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
            while (BIGFREQ(runningOrder[j-h], ftab) > BIGFREQ(vv, ftab)) {
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

      /*--
         Process big buckets, starting with the least full.
         Basically this is a 3-step process in which we call
         mainQSort3 to sort the small buckets [ss, j], but
         also make a big effort to avoid the calls if we can.
      --*/
      ss = runningOrder[i];

      /*--
         Step 1:
         Complete the big bucket [ss] by quicksorting
         any unsorted small buckets [ss, j], for j != ss.  
         Hopefully previous pointer-scanning phases have already
         completed many of the small buckets [ss, j], so
         we don't have to sort them at all.
      --*/
      for (j = 0; j <= 255; j++) {
         if (j != ss) {
            sb = (ss << 8) + j;
            if ( ! (ftab[sb] & SETMASK) ) {
               Int32 lo = ftab[sb]   & CLEARMASK;
               Int32 hi = (ftab[sb+1] & CLEARMASK) - 1;
               if (hi > lo) {
                  if (verb >= 4)
                     VPrintf4 ( "        qsort [0x%x, 0x%x]   "
                                "done %d   this %d\n",
                                ss, j, numQSorted, hi - lo + 1 );
                  mainQSort3 ( 
                     ptr, block, quadrant, nblock, 
                     lo, hi, BZ_N_RADIX, budget 
                  );   
                  numQSorted += (hi - lo + 1);
                  if (*budget < 0) return;
               }
            }
            ftab[sb] |= SETMASK;
         }
      }

      AssertH ( !bigDone[ss], 1006 );

      /*--
         Step 2:
         Now scan this big bucket [ss] so as to synthesise the
         sorted order for small buckets [t, ss] for all t,
         including, magically, the bucket [ss,ss] too.
         This will avoid doing Real Work in subsequent Step 1's.
      --*/
      {
         for (j = 0; j <= 255; j++) {
            copyStart[j] =  ftab[(j << 8) + ss]     & CLEARMASK;
            copyEnd  [j] = (ftab[(j << 8) + ss + 1] & CLEARMASK) - 1;
         }
         for (j = ftab[ss << 8] & CLEARMASK; j < copyStart[ss]; j++) {
            k = ptr[j]-1; if (k < 0) k += nblock;
            c1 = block[k];
            if (!bigDone[c1])
               ptr[ copyStart[c1]++ ] = k;
         }
         for (j = (ftab[(ss+1) << 8] & CLEARMASK) - 1; j > copyEnd[ss]; j--) {
            k = ptr[j]-1; if (k < 0) k += nblock;
            c1 = block[k];
            if (!bigDone[c1]) 
               ptr[ copyEnd[c1]-- ] = k;
         }
      }

      AssertH ( (copyStart[ss]-1 == copyEnd[ss])
                || 
                /* Extremely rare case missing in bzip2-1.0.0 and 1.0.1.
                   Necessity for this case is demonstrated by compressing 
                   a sequence of approximately 48.5 million of character 
                   251; 1.0.0/1.0.1 will then die here. */
                (copyStart[ss] == 0 && copyEnd[ss] == nblock-1),
                1007 )

      for (j = 0; j <= 255; j++) ftab[(j << 8) + ss] |= SETMASK;

      /*--
         Step 3:
         The [ss] big bucket is now done.  Record this fact,
         and update the quadrant descriptors.  Remember to
         update quadrants in the overshoot area too, if
         necessary.  The "if (i < 255)" test merely skips
         this updating for the last bucket processed, since
         updating for the last bucket is pointless.

         The quadrant array provides a way to incrementally
         cache sort orderings, as they appear, so as to 
         make subsequent comparisons in fullGtU() complete
         faster.  For repetitive blocks this makes a big
         difference (but not big enough to be able to avoid
         the fallback sorting mechanism, exponential radix sort).

         The precise meaning is: at all times:

            for 0 <= i < nblock and 0 <= j <= nblock

            if block[i] != block[j], 

               then the relative values of quadrant[i] and 
                    quadrant[j] are meaningless.

               else {
                  if quadrant[i] < quadrant[j]
                     then the string starting at i lexicographically
                     precedes the string starting at j

                  else if quadrant[i] > quadrant[j]
                     then the string starting at j lexicographically
                     precedes the string starting at i

                  else
                     the relative ordering of the strings starting
                     at i and j has not yet been determined.
               }
      --*/
      bigDone[ss] = True;

      if (i < 255) {
         Int32 bbStart  = ftab[ss << 8] & CLEARMASK;
         Int32 bbSize   = (ftab[(ss+1) << 8] & CLEARMASK) - bbStart;
         Int32 shifts   = 0;

         while ((bbSize >> shifts) > 65534) shifts++;

         for (j = bbSize-1; j >= 0; j--) {
            Int32 a2update     = ptr[bbStart + j];
            UInt16 qVal        = (UInt16)(j >> shifts);
            quadrant[a2update] = qVal;
            if (a2update < BZ_N_OVERSHOOT)
               quadrant[a2update + nblock] = qVal;
         }
         AssertH ( ((bbSize-1) >> shifts) <= 65535, 1002 );
      }

   }

   if (verb >= 4)
      VPrintf3 ( "        %d pointers, %d sorted, %d scanned\n",
                 nblock, numQSorted, nblock - numQSorted );
}


/*---------------------------------------------*/
/* Pre:
      nblock > 0
      arr2 exists for [0 .. nblock-1 +N_OVERSHOOT]
      ((UChar*)arr2)  [0 .. nblock-1] holds block
      arr1 exists for [0 .. nblock-1]

   Post:
      ((UChar*)arr2) [0 .. nblock-1] holds block
      All other areas of block destroyed
      ftab [ 0 .. 65536 ] destroyed
      arr1 [0 .. nblock-1] holds sorted order
*/
void BZ2_blockSort ( EState* s )
{
   UInt32* ptr    = s->ptr; 
   UChar*  block  = s->block;
   UInt32* ftab   = s->ftab;
   Int32   nblock = s->nblock;
   Int32   verb   = s->verbosity;
   Int32   wfact  = s->workFactor;
   UInt16* quadrant;
   Int32   budget;
   Int32   budgetInit;
   Int32   i;

   if (nblock < 10000) {
      fallbackSort ( s->arr1, s->arr2, ftab, nblock, verb );
   } else {
      /* Calculate the location for quadrant, remembering to get
         the alignment right.  Assumes that &(block[0]) is at least
         2-byte aligned -- this should be ok since block is really
         the first section of arr2.
      */
      i = nblock+BZ_N_OVERSHOOT;
      if (i & 1) i++;
      quadrant = (UInt16*)(&(block[i]));

      /* (wfact-1) / 3 puts the default-factor-30
         transition point at very roughly the same place as 
         with v0.1 and v0.9.0.  
         Not that it particularly matters any more, since the
         resulting compressed stream is now the same regardless
         of whether or not we use the main sort or fallback sort.
      */
      if (wfact < 1  ) wfact = 1;
      if (wfact > 100) wfact = 100;
      budgetInit = nblock * ((wfact-1) / 3);
      budget = budgetInit;

      mainSort ( ptr, block, quadrant, ftab, nblock, verb, &budget );
      if (verb >= 3) 
         VPrintf3 ( "      %d work, %d block, ratio %5.2f\n",
                    budgetInit - budget,
                    nblock, 
                    (float)(budgetInit - budget) /
                    (float)(nblock==0 ? 1 : nblock) ); 
      if (budget < 0) {
         if (verb >= 2) 
            VPrintf0 ( "    too repetitive; using fallback"
                       " sorting algorithm\n" );
         fallbackSort ( s->arr1, s->arr2, ftab, nblock, verb );
      }
   }

   s->origPtr = -1;
   for (i = 0; i < s->nblock; i++)
      if (ptr[i] == 0)
         { s->origPtr = i; break; };

   AssertH( s->origPtr != -1, 1003 );
}


/*-------------------------------------------------------------*/
/*--- end                                       blocksort.c ---*/
/*-------------------------------------------------------------*/
