// use crate::global_vars::bzip2_c5::{noisy};


// cadvise ( void )


#include "typedef.h"


void cadvise ( void )
{
   if (noisy)
   fprintf (
      
     stderr
           ,
      "\nIt is possible that the compressed file(s) have become corrupted.\n"
        "You can use the -tvv option to test integrity of such files.\n\n"
        "You can use the `bzip2recover' program to attempt to recover\n"
        "data from undamaged sections of corrupted files.\n\n"
    );
}



void showFileNames ( void )
{
   if (noisy)
   fprintf (
      
     stderr
           ,
      "\tInput file = %s, output file = %s\n",
      inName, outName
   );
}


