// use crate::global_vars::bzip2_c5::{stat};

#include "../typedef.h"
#include <sys/stat.h>

// 统计一个文件或目录的硬链接数量
// 参数: name - 需要统计的文件或目录的名字
// 返回值: 硬链接数量（减去文件本身的一个链接）
static Int32 countHardLinks(Char* name)
{
    IntNative i;
    struct stat statBuf; // 使用<sys/stat.h>头文件定义的stat结构体

    i = lstat(name, &statBuf); // 获取文件或目录的状态信息
    if (i != 0) return 0;      // 如果lstat调用失败, 返回0
    return (statBuf.st_nlink - 1); // 返回硬链接数量减去1（文件本身的一个链接）
}


#include <CUnit/Basic.h>
#include <fcntl.h>
#include <unistd.h>

void test_countHardLinks(void) {
    // 创建一个临时文件并测试其硬链接数
    int fd = open("temp_file", O_CREAT | O_RDWR, 0777);
    CU_ASSERT(fd != -1);
    close(fd);

    // 确保新创建的文件只有一个硬链接
    CU_ASSERT(countHardLinks("temp_file") == 0);

    // 创建硬链接
    int result = link("temp_file", "temp_file_link");
    CU_ASSERT(result == 0);

    // 检查硬链接数量
    CU_ASSERT(countHardLinks("temp_file") == 1);

    // 清理
    unlink("temp_file_link");
    unlink("temp_file");
}

int main() {
    CU_pSuite pSuite = NULL;

    if (CUE_SUCCESS != CU_initialize_registry())
        return CU_get_error();

    pSuite = CU_add_suite("CountHardLinks Test Suite", NULL, NULL);
    if (NULL == pSuite) {
        CU_cleanup_registry();
        return CU_get_error();
    }

    if (NULL == CU_add_test(pSuite, "test_countHardLinks", test_countHardLinks)) {
        CU_cleanup_registry();
        return CU_get_error();
    }

    CU_basic_set_mode(CU_BRM_VERBOSE);
    CU_basic_run_tests();
    CU_cleanup_registry();
    return CU_get_error();
}
