IntNative main ( IntNative argc, Char *argv[] )
{
   Int32 i, j;
   Char *tmp;
   Cell *argList;
   Cell *aa;
   Bool decode;
   if (sizeof(Int32) != 4 || sizeof(UInt32) != 4 ||
       sizeof(Int16) != 2 || sizeof(UInt16) != 2 ||
       sizeof(Char) != 1 || sizeof(UChar) != 1)
      configError();
   outputHandleJustInCase = 
                            ((void *)0)
                                ;
   smallMode = ((Bool)0);
   keepInputFiles = ((Bool)0);
   forceOverwrite = ((Bool)0);
   noisy = ((Bool)1);
   verbosity = 0;
   blockSize100k = 9;
   testFailsExist = ((Bool)0);
   unzFailsExist = ((Bool)0);
   numFileNames = 0;
   numFilesProcessed = 0;
   workFactor = 30;
   deleteOutputOnInterrupt = ((Bool)0);
   exitValue = 0;
   i = j = 0;
   signal (
          11
                 , mySIGSEGVorSIGBUScatcher);
   signal (
          7
                , mySIGSEGVorSIGBUScatcher);
   copyFileName ( inName, (Char*)"(none)" );
   copyFileName ( outName, (Char*)"(none)" );
   copyFileName ( progNameReally, argv[0] );
   progName = &progNameReally[0];
   for (tmp = &progNameReally[0]; *tmp != '\0'; tmp++)
      if (*tmp == '/') progName = tmp + 1;
   argList = 
            ((void *)0)
                ;
   addFlagsFromEnvVar ( &argList, (Char*)"BZIP2" );
   addFlagsFromEnvVar ( &argList, (Char*)"BZIP" );
   for (i = 1; i <= argc-1; i++)
      argList=snocString((argList), (argv[i]));
   longestFileName = 7;
   numFileNames = 0;
   decode = ((Bool)1);
   for (aa = argList; aa != 
                           ((void *)0)
                               ; aa = aa->link) {
      if ((strcmp(aa->name, ("--"))==0)) { decode = ((Bool)0); continue; }
      if (aa->name[0] == '-' && decode) continue;
      numFileNames++;
      if (longestFileName < (Int32)strlen(aa->name) )
         longestFileName = (Int32)strlen(aa->name);
   }
   if (numFileNames == 0)
      srcMode = 1; else srcMode = 3;
   opMode = 1;
   if ( (strstr ( progName, "unzip" ) != 0) ||
        (strstr ( progName, "UNZIP" ) != 0) )
      opMode = 2;
   if ( (strstr ( progName, "z2cat" ) != 0) ||
        (strstr ( progName, "Z2CAT" ) != 0) ||
        (strstr ( progName, "zcat" ) != 0) ||
        (strstr ( progName, "ZCAT" ) != 0) ) {
      opMode = 2;
      srcMode = (numFileNames == 0) ? 1 : 2;
   }
   for (aa = argList; aa != 
                           ((void *)0)
                               ; aa = aa->link) {
      if ((strcmp(aa->name, ("--"))==0)) break;
      if (aa->name[0] == '-' && aa->name[1] != '-') {
         for (j = 1; aa->name[j] != '\0'; j++) {
            switch (aa->name[j]) {
               case 'c': srcMode = 2; break;
               case 'd': opMode = 2; break;
               case 'z': opMode = 1; break;
               case 'f': forceOverwrite = ((Bool)1); break;
               case 't': opMode = 3; break;
               case 'k': keepInputFiles = ((Bool)1); break;
               case 's': smallMode = ((Bool)1); break;
               case 'q': noisy = ((Bool)0); break;
               case '1': blockSize100k = 1; break;
               case '2': blockSize100k = 2; break;
               case '3': blockSize100k = 3; break;
               case '4': blockSize100k = 4; break;
               case '5': blockSize100k = 5; break;
               case '6': blockSize100k = 6; break;
               case '7': blockSize100k = 7; break;
               case '8': blockSize100k = 8; break;
               case '9': blockSize100k = 9; break;
               case 'V':
               case 'L': license(); break;
               case 'v': verbosity++; break;
               case 'h': usage ( progName );
                         exit ( 0 );
                         break;
               default: fprintf ( 
                                  stderr
                                        , "%s: Bad flag `%s'\n",
                                   progName, aa->name );
                         usage ( progName );
                         exit ( 1 );
                         break;
            }
         }
      }
   }
   for (aa = argList; aa != 
                           ((void *)0)
                               ; aa = aa->link) {
      if ((strcmp(aa->name, ("--"))==0)) break;
      if ((strcmp(aa->name, ("--stdout"))==0)) srcMode = 2; else
      if ((strcmp(aa->name, ("--decompress"))==0)) opMode = 2; else
      if ((strcmp(aa->name, ("--compress"))==0)) opMode = 1; else
      if ((strcmp(aa->name, ("--force"))==0)) forceOverwrite = ((Bool)1); else
      if ((strcmp(aa->name, ("--test"))==0)) opMode = 3; else
      if ((strcmp(aa->name, ("--keep"))==0)) keepInputFiles = ((Bool)1); else
      if ((strcmp(aa->name, ("--small"))==0)) smallMode = ((Bool)1); else
      if ((strcmp(aa->name, ("--quiet"))==0)) noisy = ((Bool)0); else
      if ((strcmp(aa->name, ("--version"))==0)) license(); else
      if ((strcmp(aa->name, ("--license"))==0)) license(); else
      if ((strcmp(aa->name, ("--exponential"))==0)) workFactor = 1; else
      if ((strcmp(aa->name, ("--repetitive-best"))==0)) redundant(aa->name); else
      if ((strcmp(aa->name, ("--repetitive-fast"))==0)) redundant(aa->name); else
      if ((strcmp(aa->name, ("--fast"))==0)) blockSize100k = 1; else
      if ((strcmp(aa->name, ("--best"))==0)) blockSize100k = 9; else
      if ((strcmp(aa->name, ("--verbose"))==0)) verbosity++; else
      if ((strcmp(aa->name, ("--help"))==0)) { usage ( progName ); exit ( 0 ); }
         else
         if (strncmp ( aa->name, "--", 2) == 0) {
            fprintf ( 
                     stderr
                           , "%s: Bad flag `%s'\n", progName, aa->name );
            usage ( progName );
            exit ( 1 );
         }
   }
   if (verbosity > 4) verbosity = 4;
   if (opMode == 1 && smallMode && blockSize100k > 2)
      blockSize100k = 2;
   if (opMode == 3 && srcMode == 2) {
      fprintf ( 
               stderr
                     , "%s: -c and -t cannot be used together.\n",
                progName );
      exit ( 1 );
   }
   if (srcMode == 2 && numFileNames == 0)
      srcMode = 1;
   if (opMode != 1) blockSize100k = 0;
   if (srcMode == 3) {
      signal (
             2
                   , mySignalCatcher);
      signal (
             15
                    , mySignalCatcher);
      signal (
             1
                   , mySignalCatcher);
   }
   if (opMode == 1) {
     if (srcMode == 1) {
        compress ( 
                  ((void *)0) 
                       );
     } else {
        decode = ((Bool)1);
        for (aa = argList; aa != 
                                ((void *)0)
                                    ; aa = aa->link) {
           if ((strcmp(aa->name, ("--"))==0)) { decode = ((Bool)0); continue; }
           if (aa->name[0] == '-' && decode) continue;
           numFilesProcessed++;
           compress ( aa->name );
        }
     }
   }
}