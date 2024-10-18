// 这个函数myfeof用于检查一个文件流f是否已经读到了文件的结尾。它尝试从文件中读取一个字符，如果读取到的字符是EOF（值为-1），则返回真（表示已经到了文件的结尾）；否则将字符放回文件流，并返回假
#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>
#include "../typedef.h"
// 检查文件流f是否已经读到了文件的结尾
static Bool myfeof(FILE* f) {
    // 尝试从文件中读取一个字符
    Int32 c = fgetc(f);
    // 如果读取到的字符是EOF（值为-1）
    if (c == -1) return ((Bool)1);
    // 将字符放回文件流
    ungetc(c, f);
    // 返回假，表示还没有到文件的结尾
    return ((Bool)0);
}

// 测试myfeof函数
void test_myfeof() {
    // 创建一个临时文件
    FILE* f = tmpfile();
    // 写入一些数据
    fputs("test", f);
    // 回到文件的开头
    rewind(f);
    // 确保文件没有到结尾
    CU_ASSERT_EQUAL(myfeof(f), ((Bool)0));
    // 读取文件的所有内容
    while (fgetc(f) != EOF);
    // 确保文件已经到结尾
    CU_ASSERT_EQUAL(myfeof(f), ((Bool)1));
    // 关闭文件
    fclose(f);
}

// int main() {
//     CU_initialize_registry();

//     CU_pSuite suite = CU_add_suite("myfeof Tests", NULL, NULL);
//     CU_add_test(suite, "test_myfeof", test_myfeof);

//     CU_basic_set_mode(CU_BRM_VERBOSE);
//     CU_basic_run_tests();
//     CU_cleanup_registry();

//     return 0;
// }
