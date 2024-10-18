#ifndef TYPEDEF_H_INCLUDED
#define TYPEDEF_H_INCLUDED
#pragma once

// 以下是您的头文件内容
// ...

// 以下是您的头文件内容
// ...


#include <stdlib.h> // 为了使用 malloc, calloc
#include <string.h> // 为了使用 memset
#include <stdbool.h>
#include <stdio.h>    // 为fprintf和stderr
#include <assert.h>   // 为assert
#include <sys/stat.h>
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
typedef int IntNative;
typedef unsigned long long int MaybeUInt64;

// compress + uncompress
const Char* zSuffix[4]
   = { ".bz2", ".bz", ".tbz2", ".tbz" };
// uncompress
const Char* unzSuffix[4]
   = { "", "", ".tar", ".tar" };


Int32 longestFileName;
Char inName [1034];
Char outName[1034];
Char tmpName[1034];
FILE *outputHandleJustInCase;

// FILE* handle;:
// 这是一个文件指针，通常用于表示打开的文件的句柄。使用这个指针，你可以对文件进行读取或写入等操作。
// Int32 buffer;:
// 这是一个名为buffer的32位整数（很可能是int类型）。从其名称来看，它可能用于存储某种缓冲数据，具体是什么取决于BitStream的上下文。考虑到这是一个位流，buffer可能用于暂存即将写入或刚刚读取的位。
// Int32 buffLive;:
// 另一个32位整数，叫做buffLive。从名字上可以推断，它可能表示buffer中当前有效的位数。例如，如果你逐位地写入数据，当你写入3位后，buffLive可能就是3。
// Char mode;:
// 这是一个字符型数据，表示“模式”。不知道具体的上下文，但常见的模式包括读取（'R'）、写入（'W'）等。这个字段可能用于确定BitStream的当前状态或操作模式。
// 总体来说，BitStream结构体似乎是为了处理位级别的文件操作而设计的。它使用一个文件句柄进行文件操作，有一个缓冲区来存储位，并且还跟踪了缓冲区中的有效位数和当前的操作模式。
typedef
   struct {
      FILE* handle;
      Int32 buffer;
      Int32 buffLive;
      Char mode;
   }
   BitStream;
Int32 BZ2_rNums[512];
UInt32 BZ2_crc32Table[256];
Int32 longestFileName;

Char inFileName[2000];
Char outFileName[2000];
Char progName[2000];

MaybeUInt64 bytesOut = 0;
MaybeUInt64 bytesIn = 0;
Char progNameReally[1034];
FILE *outputHandleJustInCase;
Int32 workFactor;
Int32 srcMode;
struct stat statBuf;


Int32 verbosity;
Bool keepInputFiles, smallMode, deleteOutputOnInterrupt;
Bool forceOverwrite, testFailsExist, unzFailsExist, noisy;
Int32 numFileNames, numFilesProcessed, blockSize100k;
Int32 exitValue;
Int32 opMode;
Int32 srcMode;


typedef
   struct { UChar b[8]; }
   UInt64;

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

   
bz_stream bz_stream_default() {
    bz_stream strm = {
        .next_in = NULL,
        .avail_in = 0,
        .total_in_lo32 = 0,
        .total_in_hi32 = 0,
        .next_out = NULL,
        .avail_out = 0,
        .total_out_lo32 = 0,
        .total_out_hi32 = 0,
        .state = NULL,
        .bzalloc = NULL,
        .bzfree = NULL,
        .opaque = NULL
    };
    return strm;
}







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


typedef
   struct {

      bz_stream* strm;



      Int32 mode;
      Int32 state;


      UInt32 avail_in_expect;


      UInt32* arr1;
      UInt32* arr2;
      UInt32* ftab;
      Int32 origPtr;


      UInt32* ptr;
      UChar* block;
      UInt16* mtfv;
      UChar* zbits;


      Int32 workFactor;


      UInt32 state_in_ch;
      Int32 state_in_len;
      Int32 rNToGo; Int32 rTPos;


      Int32 nblock;
      Int32 nblockMAX;
      Int32 numZ;
      Int32 state_out_pos;


      Int32 nInUse;
      Bool inUse[256];
      UChar unseqToSeq[256];


      UInt32 bsBuff;
      Int32 bsLive;


      UInt32 blockCRC;
      UInt32 combinedCRC;


      Int32 verbosity;
      Int32 blockNo;
      Int32 blockSize100k;


      Int32 nMTF;
      Int32 mtfFreq [258];
      UChar selector [(2 + (900000 / 50))];
      UChar selectorMtf[(2 + (900000 / 50))];

      UChar len [6][258];
      Int32 code [6][258];
      Int32 rfreq [6][258];

      UInt32 len_pack[258][4];

   }
   EState;
EState EState_default() {
    EState s = {
        .strm = NULL,
        .mode = 0,
        .state = 0,
        .avail_in_expect = 0,
        .arr1 = NULL,
        .arr2 = NULL,
        .ftab = NULL,
        .origPtr = 0,
        .ptr = NULL,
        .block = NULL,
        .mtfv = NULL,
        .zbits = NULL,
        .workFactor = 0,
        .state_in_ch = 0,
        .state_in_len = 0,
        .rNToGo = 0,
        .rTPos = 0,
        .nblock = 0,
        .nblockMAX = 0,
        .numZ = 0,
        .state_out_pos = 0,
        .bsBuff = 0,
        .bsLive = 0,
        .blockCRC = 0,
        .combinedCRC = 0,
        .verbosity = 0,
        .blockNo = 0,
        .blockSize100k = 0,
        .nMTF = 0,
        .nInUse = 0,
        .inUse = {false},
        .unseqToSeq = {0},
        .mtfFreq = {0},
        .selector = {0},
        .selectorMtf = {0},
        .len = {{0}},
        .code = {{0}},
        .rfreq = {{0}},
        .len_pack = {{0}}
    };
    return s;
}

static
struct stat fileMetaInfo;
typedef void BZFILE; 
typedef
   struct {
      FILE* handle;
      Char buf[5000];
      Int32 bufN;
      Bool writing;
      bz_stream strm;
      Int32 lastErr;
      Bool initialisedOk;
   }
   bzFile;

   #endif // TYPEDEF_H_INCLUDED