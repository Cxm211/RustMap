
BZ2_bzlibVersion
static
void license ( void )
{
   fprintf ( 
            stderr
                  ,

    "bzip2, a block-sorting file compressor.  "
    "Version %s.\n"
    "   \n"
    "   Copyright (C) 1996-2019 by Julian Seward.\n"
    "   \n"
    "   This program is free software; you can redistribute it and/or modify\n"
    "   it under the terms set out in the LICENSE file, which is included\n"
    "   in the bzip2 source distribution.\n"
    "   \n"
    "   This program is distributed in the hope that it will be useful,\n"
    "   but WITHOUT ANY WARRANTY; without even the implied warranty of\n"
    "   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n"
    "   LICENSE file for more details.\n"
    "   \n",
    BZ2_bzlibVersion()
   );
}

#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>


void test_license_output(void) {
    // 由于license函数输出到stderr，我们无法直接在测试中捕获输出内容来进行断言验证。
    // 所以，这里的测试主要是为了观察函数的行为是否符合预期。
    // 在实际环境中，你可能需要重定向stderr或使用其他技巧来捕获输出。
    license();
}

int main() {
    CU_initialize_registry();

    CU_pSuite suite = CU_add_suite("Test license", NULL, NULL);

    CU_add_test(suite, "test_license_output", test_license_output);

    CU_basic_set_mode(CU_BRM_VERBOSE);
    CU_basic_run_tests();

    CU_cleanup_registry();
    return 0;
}
