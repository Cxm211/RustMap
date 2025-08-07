#include "../typedef.h"

#include <CUnit/Basic.h>

struct utimbuf {
    __time_t actime;   // 文件访问时间
    __time_t modtime;  // 文件修改时间
};

static void saveInputFileMetaInfo(Char *srcName) {
   IntNative retVal;
   // 获取文件的状态信息并保存到fileMetaInfo中
   retVal = stat(srcName, &fileMetaInfo);
   // 如果出现错误，调用ioError()处理
   if (retVal != 0) ioError();
}

static void applySavedTimeInfoToOutputFile(Char *dstName) {
   IntNative retVal;
   struct utimbuf uTimBuf;

   uTimBuf.actime = fileMetaInfo.st_atim.tv_sec;
   uTimBuf.modtime = fileMetaInfo.st_mtim.tv_sec;

   // 更新目标文件的时间信息
   retVal = utime(dstName, &uTimBuf);
   // 如果出现错误，调用ioError()处理
   if (retVal != 0) ioError();
}

static void applySavedFileAttrToOutputFile(IntNative fd) {
   IntNative retVal;

   // 更新文件的权限
   retVal = fchmod(fd, fileMetaInfo.st_mode);
   // 如果出现错误，调用ioError()处理
   if (retVal != 0) ioError();

   // 更新文件的所有者和组信息，忽略返回值
   (void) fchown(fd, fileMetaInfo.st_uid, fileMetaInfo.st_gid);
}











