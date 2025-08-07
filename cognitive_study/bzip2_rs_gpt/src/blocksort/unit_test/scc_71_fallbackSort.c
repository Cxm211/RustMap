static
void fallbackSort ( UInt32* fmap,
                    UInt32* eclass,
                    UInt32* bhtab,
                    Int32 nblock,
                    Int32 verb )
{
   Int32 ftab[257];
   Int32 ftabCopy[256];
   Int32 H, i, j, k, l, r, cc, cc1;
   Int32 nNotDone;
   Int32 nBhtab;
   UChar* eclass8 = (UChar*)eclass;





   if (verb >= 4)
      fprintf(
     stderr
     ,"        bucket sorting ...\n");
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
   for (i = 0; i < 256; i++) bhtab[(ftab[i]) >> 5] |= ((UInt32)1 << ((ftab[i]) & 31));
   for (i = 0; i < 32; i++) {
      bhtab[(nblock + 2*i) >> 5] |= ((UInt32)1 << ((nblock + 2*i) & 31));
      bhtab[(nblock + 2*i + 1) >> 5] &= ~((UInt32)1 << ((nblock + 2*i + 1) & 31));
   }


   H = 1;
   while (1) {

      if (verb >= 4)
         fprintf(
        stderr
        ,"        depth %6d has ",H);

      j = 0;
      for (i = 0; i < nblock; i++) {
         if ((bhtab[(i) >> 5] & ((UInt32)1 << ((i) & 31)))) j = i;
         k = fmap[i] - H; if (k < 0) k += nblock;
         eclass[k] = j;
      }

      nNotDone = 0;
      r = -1;
      while (1) {


         k = r + 1;
         while ((bhtab[(k) >> 5] & ((UInt32)1 << ((k) & 31))) && ((k) & 0x01f)) k++;
         if ((bhtab[(k) >> 5] & ((UInt32)1 << ((k) & 31)))) {
            while (bhtab[(k) >> 5] == 0xffffffff) k += 32;
            while ((bhtab[(k) >> 5] & ((UInt32)1 << ((k) & 31)))) k++;
         }
         l = k - 1;
         if (l >= nblock) break;
         while (!(bhtab[(k) >> 5] & ((UInt32)1 << ((k) & 31))) && ((k) & 0x01f)) k++;
         if (!(bhtab[(k) >> 5] & ((UInt32)1 << ((k) & 31)))) {
            while (bhtab[(k) >> 5] == 0x00000000) k += 32;
            while (!(bhtab[(k) >> 5] & ((UInt32)1 << ((k) & 31)))) k++;
         }
         r = k - 1;
         if (r >= nblock) break;


         if (r > l) {
            nNotDone += (r - l + 1);
            fallbackQSort3 ( fmap, eclass, l, r );
   

            cc = -1;
            for (i = l; i <= r; i++) {
               cc1 = eclass[fmap[i]];
               if (cc != cc1) { bhtab[(i) >> 5] |= ((UInt32)1 << ((i) & 31)); cc = cc1; };
            }
         }
      }

      if (verb >= 4)
         fprintf(
        stderr
        ,"%6d unresolved strings\n",nNotDone);

      H *= 2;
      if (H > nblock || nNotDone == 0) break;
   }






   if (verb >= 4)
      fprintf(
     stderr
     ,"        reconstructing block ...\n");
   j = 0;
   for (i = 0; i < nblock; i++) {
      while (ftabCopy[j] == 0) j++;
      ftabCopy[j]--;
      eclass8[fmap[i]] = (UChar)j;
   }
   { if (!(j < 256)) BZ2_bz__AssertH__fail ( 1005 ); };
}