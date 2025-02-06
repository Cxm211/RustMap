#include <stdio.h>
#include <string.h>
#include <CUnit/Basic.h>
typedef char Char;
Char inFileName[2000];
Char outFileName[2000];
Char progName[2000];
static void writeError ( void )
{
   fprintf ( 
            stderr
                  ,
             "%s: I/O error reading `%s', possible reason follows.\n",
            progName, inFileName );
   perror ( progName );
   fprintf ( 
            stderr
                  , "%s: warning: output file(s) may be incomplete.\n",
             progName );
   exit ( 1 );
}
void test_writeError(void) {
    // 重定向stderr到一个文件以捕获输出
    freopen("stderr_output.txt", "w", stderr);
    strcpy(progName, "testProgram");
    strcpy(inFileName, "testFile.txt");
    
    writeError();
    // 重置stderr
    freopen("/dev/tty", "w", stderr);
    // 从文件中读取输出并验证
    FILE *fp = fopen("stderr_output.txt", "r");
    char line[4000];
    fgets(line, sizeof(line), fp);
    CU_ASSERT_STRING_EQUAL(line, "testProgram: I/O error reading `testFile.txt', possible reason follows.\n");
    fgets(line, sizeof(line), fp);
    // 由于perror产生的消息是基于系统的，所以我们只检查progName是否存在
    CU_ASSERT_PTR_NOT_NULL(strstr(line, "testProgram"));
    fgets(line, sizeof(line), fp);
    CU_ASSERT_STRING_EQUAL(line, "testProgram: warning: output file(s) may be incomplete.\n");
    fclose(fp);
}
