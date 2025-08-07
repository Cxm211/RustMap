#include "../typedef.h"
#include "./scc_90_root_52_setExit.c"
void cleanUpAndFail ( Int32 ec )
{
   IntNative retVal;
   struct stat statBuf;

   // 如果源文件是打开状态，操作模式不是3，并且在中断时需要删除输出文件
   if ( srcMode == 3 && opMode != 3 && deleteOutputOnInterrupt ) {
      // 检查输入文件是否存在
      retVal = stat ( inName, &statBuf );

      // 如果输入文件存在
      if (retVal == 0) {
         // 如果开启了详细输出模式，打印删除输出文件的消息
         if (noisy)
            fprintf(stderr, "%s: Deleting output file %s, if it exists.\n", progName, outName);
         
         // 如果输出文件句柄已打开，关闭它
         if (outputHandleJustInCase != ((void *)0))
            fclose(outputHandleJustInCase);
         
         // 尝试删除输出文件
         retVal = remove(outName);
         
         // 如果删除失败，打印警告信息
         if (retVal != 0)
            fprintf(stderr, "%s: WARNING: deletion of output file (apparently) failed.\n", progName);
      } else {
         // 如果输入文件不存在，打印一系列的警告信息，建议用户对输出文件进行完整性测试
         fprintf(stderr, "%s: WARNING: deletion of output file suppressed\n", progName);
         fprintf(stderr, "%s:    since input file no longer exists.  Output file\n", progName);
         fprintf(stderr, "%s:    `%s' may be incomplete.\n", progName, outName);
         fprintf(stderr, "%s:    I suggest doing an integrity test (bzip2 -tv) of it.\n", progName);
      }
   }

   // 如果开启了详细输出模式，并且有些文件尚未处理完，打印警告信息
   if (noisy && numFileNames > 0 && numFilesProcessed < numFileNames) {
      fprintf(stderr, "%s: WARNING: some files have not been processed:\n", progName);
      fprintf(stderr, "%s:    %d specified on command line, %d not processed yet.\n\n", progName, numFileNames, numFileNames - numFilesProcessed);
   }

   // 设置退出值并退出程序
   setExit(ec);
   exit(exitValue);
}


#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>


    
    
    
