#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>
#include "../typedef.h"
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




