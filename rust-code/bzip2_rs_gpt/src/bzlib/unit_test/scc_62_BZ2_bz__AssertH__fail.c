#include "./scc_61_root_42_BZ2_bzlibVersion.c"

#include <stdio.h>
#include <stdlib.h>
static void BZ2_bz__AssertH__fail ( int errcode )
{
   fprintf(
          stderr
                ,
      "\n\nbzip2/libbzip2: internal error number %d.\n"
      "This is a bug in bzip2/libbzip2, %s.\n"
      "Please report it to: bzip2-devel@sourceware.org.  If this happened\n"
      "when you were using some program which uses libbzip2 as a\n"
      "component, you should also report this bug to the author(s)\n"
      "of that program.  Please make an effort to report this bug;\n"
      "timely and accurate bug reports eventually lead to higher\n"
      "quality software.  Thanks.\n\n",
      errcode,
      BZ2_bzlibVersion()
   );

   if (errcode == 1007) {
   fprintf(
          stderr
                ,
      "\n*** A special note about internal error number 1007 ***\n"
      "\n"
      "Experience suggests that a common cause of i.e. 1007\n"
      "is unreliable memory or other hardware.  The 1007 assertion\n"
      "just happens to cross-check the results of huge numbers of\n"
      "memory reads/writes, and so acts (unintendedly) as a stress\n"
      "test of your memory system.\n"
      "\n"
      "I suggest the following: try compressing the file again,\n"
      "possibly monitoring progress in detail with the -vv flag.\n"
      "\n"
      "* If the error cannot be reproduced, and/or happens at different\n"
      "  points in compression, you may have a flaky memory system.\n"
      "  Try a memory-test program.  I have used Memtest86\n"
      "  (www.memtest86.com).  At the time of writing it is free (GPLd).\n"
      "  Memtest86 tests memory much more thorougly than your BIOSs\n"
      "  power-on test, and may find failures that the BIOS doesn't.\n"
      "\n"
      "* If the error can be repeatably reproduced, this is a bug in\n"
      "  bzip2, and I would very much like to hear about it.  Please\n"
      "  let me know, and, ideally, save a copy of the file causing the\n"
      "  problem -- without which I will be unable to investigate it.\n"
      "\n"
   );
   }

   exit(3);
}


#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>
#include <stdio.h>
#include <stdlib.h>
#include <setjmp.h>

jmp_buf test_env;


static void test_BZ2_bz__AssertH__fail(void) {
    // 1. 重定向stderr到临时文件
    FILE *tmp = freopen("temp.txt", "w", stderr);
    CU_ASSERT_PTR_NOT_NULL_FATAL(tmp);

    // 2. 使用setjmp创建一个返回点，为了在exit被调用时能返回到这里
    int exit_status = setjmp(test_env);
    if (exit_status == 0) {
        // 3. 如果是初次进入setjmp，调用目标函数
        BZ2_bz__AssertH__fail(1007);
        // 如果函数没有调用exit, 那么以下的断言会被触发
        CU_FAIL("Function should have called exit");
    } else {
        // 4. 如果从longjmp返回（即exit被调用），检查退出状态是否符合预期
        CU_ASSERT_EQUAL(exit_status, 3);
    }

    // 5. 关闭文件以确保所有内容都被写入
    fclose(tmp);

    // 6. 重新打开文件，准备读取内容
    tmp = fopen("temp.txt", "r");
    CU_ASSERT_PTR_NOT_NULL_FATAL(tmp);

    // 7. 读取临时文件的内容到buffer
    char buffer[2048]; // 假设错误消息不会超过这个大小
    fread(buffer, sizeof(char), 2047, tmp);
    buffer[2047] = '\0'; // 确保字符串以'\0'结束

    // 8. 根据具体需求检查输出内容，确认错误消息是否正确写入了文件
    CU_ASSERT_PTR_NOT_NULL(strstr(buffer, "bzip2/libbzip2: internal error number 1007."));
    CU_ASSERT_PTR_NOT_NULL(strstr(buffer, "A special note about internal error number 1007"));

    // 9. 清理操作：关闭临时文件并删除它
    fclose(tmp);
    remove("temp.txt");
}


