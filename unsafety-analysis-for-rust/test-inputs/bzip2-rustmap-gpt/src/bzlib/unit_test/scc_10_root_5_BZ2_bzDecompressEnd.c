// use crate::global_vars::bzip2_c1::{DState, bz_stream};
#include "../typedef.h"
int BZ2_bzDecompressEnd ( bz_stream *strm )
{
   DState* s;
   if (strm == 
              ((void *)0)
                  ) return (-2);
   s = strm->state;
   if (s == 
           ((void *)0)
               ) return (-2);
   if (s->strm != strm) return (-2);

   if (s->tt != 
                 ((void *)0)
                     ) (strm->bzfree)(strm->opaque,(s->tt));
   if (s->ll16 != 
                 ((void *)0)
                     ) (strm->bzfree)(strm->opaque,(s->ll16));
   if (s->ll4 != 
                 ((void *)0)
                     ) (strm->bzfree)(strm->opaque,(s->ll4));

   (strm->bzfree)(strm->opaque,(strm->state));
   strm->state = 
                ((void *)0)
                    ;

   return 0;
}