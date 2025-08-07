
#include "./scc_60_root_41_containsDubiousChars.c"
#include "./scc_90_root_52_setExit.c"



static
void testf ( Char *name )
{
   FILE *inStr;
   Bool allOK;
   struct stat statBuf;

   deleteOutputOnInterrupt = ((Bool)0);

   if (name == 
              ((void *)0) 
                   && srcMode != 1)
      panic ( "testf: bad modes\n" );

   copyFileName ( outName, (Char*)"(none)" );
   switch (srcMode) {
      case 1: copyFileName ( inName, (Char*)"(stdin)" ); break;
      case 3: copyFileName ( inName, name ); break;
      case 2: copyFileName ( inName, name ); break;
   }

   if ( srcMode != 1 && containsDubiousChars ( inName ) ) {
      if (noisy)
      fprintf ( 
               stderr
                     , "%s: There are no files matching `%s'.\n",
                progName, inName );
      setExit(1);
      return;
   }
   if ( srcMode != 1 && !fileExists ( inName ) ) {
      fprintf ( 
               stderr
                     , "%s: Can't open input %s: %s.\n",
                progName, inName, strerror(
                                          (*__errno_location ())
                                               ) );
      setExit(1);
      return;
   }
   if ( srcMode != 1 ) {
      stat(inName, &statBuf);
      if ( 
          ((((
          statBuf.st_mode
          )) & 0170000) == (0040000)) 
                                      ) {
         fprintf( 
                 stderr
                       ,
                  "%s: Input file %s is a directory.\n",
                  progName,inName);
         setExit(1);
         return;
      }
   }

   switch ( srcMode ) {

      case 1:
         if ( isatty ( fileno ( 
                               stdin 
                                     ) ) ) {
            fprintf ( 
                     stderr
                           ,
                      "%s: I won't read compressed data from a terminal.\n",
                      progName );
            fprintf ( 
                     stderr
                           , "%s: For help, type: `%s --help'.\n",
                              progName, progName );
            setExit(1);
            return;
         };
         inStr = 
                stdin
                     ;
         break;

      case 2: case 3:
         inStr = fopen ( inName, "rb" );
         if ( inStr == 
                      ((void *)0) 
                           ) {
            fprintf ( 
                     stderr
                           , "%s: Can't open input file %s:%s.\n",
                      progName, inName, strerror(
                                                (*__errno_location ())
                                                     ) );
            setExit(1);
            return;
         };
         break;

      default:
         panic ( "testf: bad srcMode" );
         break;
   }

   if (verbosity >= 1) {
      fprintf ( 
               stderr
                     , "  %s: ", inName );
      pad ( inName );
      fflush ( 
              stderr 
                     );
   }


   outputHandleJustInCase = 
                           ((void *)0)
                               ;
   allOK = testStream ( inStr );

   if (allOK && verbosity >= 1) fprintf ( 
                                         stderr
                                               , "ok\n" );
   if (!allOK) testFailsExist = ((Bool)1);
}
