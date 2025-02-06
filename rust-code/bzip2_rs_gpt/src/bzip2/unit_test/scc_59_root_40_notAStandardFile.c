#include "../typedef.h"

static
Bool notAStandardFile ( Char* name )
{
   IntNative i;
   struct stat statBuf;

   // 尝试获取文件的状态信息。
   i = lstat ( name, &statBuf );
   // 如果lstat返回错误，表示它不是标准文件。
   if (i != 0) return ((Bool)1);
   // 如果文件的模式表示它是一个标准的文件，返回false。
   if ( (statBuf.st_mode & 0170000) == (0100000)) return ((Bool)0);
   // 在其他情况下返回true。
   return ((Bool)1);
}

#include <CUnit/Basic.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>











