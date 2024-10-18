// use crate::bzip2::scc_90_root_52_setExit::*;
// use crate::bzip2::scc_91_cleanUpAndFail::*;
// use crate::global_vars::bzip2_c5::{exitValue};


// copyFileName ( Char*, Char* )

// copyFileName ( Char* to, Char* from )

/*/
数copyFileName的目的是从一个字符串from复制文件名到另一个字符串to。
但在复制之前，它会检查from的长度是否超过了FILE_NAME_LEN-10。如果超过了，它会向标准错误输出一个警告消息，并退出程序。如果没有超过，它会复制文件名，并确保to字符串以空字符结束。
*/
#include "../typedef.h"
#include "./scc_90_root_52_setExit.c"

static
void copyFileName ( Char* to, Char* from )
{
   if ( strlen(from) > 1034 -10 ) {
      fprintf (
         
        stderr
              ,
         "bzip2: file name\n`%s'\n"
         "is suspiciously (more than %d chars) long.\n"
         "Try using a reasonable file name instead.  Sorry! :-)\n",
         from, 1034 -10
      );
      setExit(1);
      exit(exitValue);
   }

  strncpy(to,from,1034 -10);
  to[1034 -10]='\0';
}



#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>

// 假设您已经包含了必要的头文件和定义

void test_copyFileName_valid(void) {
    Char dest[1035];
    Char source[] = "test.txt";

    copyFileName(dest, source);
    CU_ASSERT_STRING_EQUAL(dest, source);
}

void test_copyFileName_tooLong(void) {
    Char dest[1035];
    Char source[1035];

    memset(source, 'a', 1024);
    source[1024] = '\0';

    // 这个函数应该会退出程序，所以正常情况下这个测试会失败。
    // 如果您要测试这种情况，可能需要使用某种机制来捕捉这种退出。
    copyFileName(dest, source);
}

// int main() {
//     CU_initialize_registry();

//     CU_pSuite suite = CU_add_suite("Test copyFileName", NULL, NULL);

//     CU_add_test(suite, "test_copyFileName_valid", test_copyFileName_valid);
//     CU_add_test(suite, "test_copyFileName_tooLong", test_copyFileName_tooLong);

//     CU_basic_set_mode(CU_BRM_VERBOSE);
//     CU_basic_run_tests();

//     CU_cleanup_registry();
//     return 0;
// }
