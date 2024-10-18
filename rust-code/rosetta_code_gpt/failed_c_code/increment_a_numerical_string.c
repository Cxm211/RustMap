#include <stdio.h>
#include <string.h>
#include <stdlib.h>

// 函数声明
char * incr(char *s);
void string_test(const char *s);

int main()
{
    // 测试几种不同的情况
    string_test("+0");
    string_test("-1");
    string_test("-41");
    string_test("+41");
    string_test("999");
    string_test("+999");
    string_test("109999999999999999999999999999999999999999");
    string_test("-100000000000000000000000000000000000000000000");

    return 0;
}

// 主要的增加函数
char * incr(char *s)
{
    // 声明一些局部变量
    int i, begin, tail, len;
    int neg = (*s == '-');  // 判断是否是负数
    char tgt = neg ? '0' : '9';  // 目标字符，用于比较

    // 特殊情况："-1"
    if (!strcmp(s, "-1")) {
        s[0] = '0', s[1] = '\0';
        return s;
    }

    // 计算字符串长度
    len = strlen(s);
    // 计算开始的位置，看是不是有正负号
    begin = (*s == '-' || *s == '+') ? 1 : 0;

    // 找出需要变更的数字有多少个
    for (tail = len - 1; tail >= begin && s[tail] == tgt; tail--);

    if (tail < begin && !neg) {
        // 特殊情况：所有数字都是9，字符串长度会增加
        if (!begin) s = realloc(s, len + 2);
        s[0] = '1';
        for (i = 1; i <= len - begin; i++) s[i] = '0';
        s[len + 1] = '\0';
    } else if (tail == begin && neg && s[1] == '1') {
        // 特殊情况：字符串是-1000...，这样字符串长度会减小
        for (i = 1; i < len - begin; i++) s[i] = '9';
        s[len - 1] = '\0';
    } else {
        // 正常情况：将尾部的数字全部变成0或者9，并让前一个数字加1或减1
        for (i = len - 1; i > tail; i--)
            s[i] = neg ? '9' : '0';
        s[tail] += neg ? -1 : 1;
    }

    return s;
}

// 用于测试的函数
void string_test(const char *s)
{
    char *ret = malloc(strlen(s) + 1);  // +1是为了null-terminator
    strcpy(ret, s);

    printf("text: %s\n", ret);
    ret = incr(ret);
    printf("  ->: %s\n", ret);
    free(ret);
}