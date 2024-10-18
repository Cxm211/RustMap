#include "../typedef.h"

#include <CUnit/Basic.h>

// 定义用于存储文件时间信息的结构体
struct utimbuf {
    __time_t actime;   // 文件访问时间
    __time_t modtime;  // 文件修改时间
};

// 保存输入文件的元数据信息
static void saveInputFileMetaInfo(Char *srcName) {
   IntNative retVal;
   // 获取文件的状态信息并保存到fileMetaInfo中
   retVal = stat(srcName, &fileMetaInfo);
   // 如果出现错误，调用ioError()处理
   if (retVal != 0) ioError();
}

// 将保存的时间信息应用于输出文件
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

// 将保存的文件属性应用于输出文件
static void applySavedFileAttrToOutputFile(IntNative fd) {
   IntNative retVal;

   // 更新文件的权限
   retVal = fchmod(fd, fileMetaInfo.st_mode);
   // 如果出现错误，调用ioError()处理
   if (retVal != 0) ioError();

   // 更新文件的所有者和组信息，忽略返回值
   (void) fchown(fd, fileMetaInfo.st_uid, fileMetaInfo.st_gid);
}

// 以下是CUnit测试案例

// void test_saveAndApplyFileMetaInfo() {
//     // 假设有一个输入文件 "input.txt" 和一个输出文件 "output.txt"
//     // 在实际测试之前，请确保这两个文件存在
//     saveInputFileMetaInfo("input.txt");
//     applySavedTimeInfoToOutputFile("output.txt");
//     int fd = open("output.txt", O_RDWR);
//     applySavedFileAttrToOutputFile(fd);
//     close(fd);

//     // 在这里可以添加断言来验证 "output.txt" 的属性是否与 "input.txt" 匹配
//     // CU_ASSERT_EQUAL( ... );
// }



// int main() {
//     CU_pSuite pSuite = NULL;

//     if (CUE_SUCCESS != CU_initialize_registry())
//         return CU_get_error();

//     pSuite = CU_add_suite("FileMetaInfo_TestSuite", NULL, NULL);
//     if (!pSuite) {
//         CU_cleanup_registry();
//         return CU_get_error();
//     }

//     if (!CU_add_test(pSuite, "test_saveAndApplyFileMetaInfo", test_saveAndApplyFileMetaInfo)) {
//         CU_cleanup_registry();
//         return CU_get_error();
//     }

//     CU_basic_set_mode(CU_BRM_VERBOSE);
//     CU_basic_run_tests();
//     CU_cleanup_registry();

//     return CU_get_error();
// }