// use crate::global_vars::bzip2_c2::{inFileName, progName};
// readError ( void )
typedef char Char;
typedef unsigned long long int MaybeUInt64;
Char inFileName[2000];
Char progName[2000];
MaybeUInt64 bytesOut = 0;
MaybeUInt64 bytesIn = 0;
static void readError ( void )
{
   fprintf ( 
            stderr
                  ,
             "%s: I/O error reading `%s', possible reason follows.\n",
            progName, inFileName );
   perror ( progName );
   fprintf ( 
            stderr
                  , "%s: warning: output file(s) may be incomplete.\n",
             progName );
   exit ( 1 );
}
