#include "../typedef.h"

static
void compressedStreamEOF ( void )
{
  // 如果noisy为true，那么输出详细的错误信息
  if (noisy) {
    // 输出错误信息到stderr
    fprintf ( 
             stderr, // 输出到stderr
       "\n%s: Compressed file ends unexpectedly;\n\t"
       "perhaps it is corrupted?  *Possible* reason follows.\n",
       progName ); // progName是程序的名称

    // 输出更详细的错误原因
    perror ( progName );
    
    // 显示相关的文件名信息
    showFileNames();

    // 提供可能的建议
    cadvise();
  }

  // 执行清理工作并终止程序，返回错误码2
  cleanUpAndFail( 2 );
}

void mock_cleanUpAndFail(int exitCode) {
    longjmp(test_env, exitCode);
}

void test_compressedStreamEOF(void) {
    // 重定向stderr到临时文件，以捕获输出
    FILE *tmp = freopen("temp.txt", "w", stderr);
    CU_ASSERT_PTR_NOT_NULL_FATAL(tmp);

    // 使用setjmp创建一个返回点，这样当mock_cleanUpAndFail被调用时，我们可以返回到这里
    int exit_status = setjmp(test_env);
    if (exit_status == 0) {
        // 设定 noisy 为 true，以确保输出
        noisy = TRUE;
        
        // 调用测试函数
        compressedStreamEOF();
        
        // 如果函数没有调用mock_cleanUpAndFail，那么以下的断言会被触发
        CU_FAIL("Function should have called mock_cleanUpAndFail");
    } else {
        // 检查退出状态
        CU_ASSERT_EQUAL(exit_status, 2);
    }

    // 关闭文件
    fclose(tmp);

    // 重新打开文件，准备读取内容
    tmp = fopen("temp.txt", "r");
    CU_ASSERT_PTR_NOT_NULL_FATAL(tmp);

    // 读取文件内容
    char buffer[2048];
    fread(buffer, sizeof(char), 2047, tmp);
    buffer[2047] = '\0';

    // 检查输出内容
    CU_ASSERT_PTR_NOT_NULL(strstr(buffer, "Compressed file ends unexpectedly"));
    CU_ASSERT_PTR_NOT_NULL(strstr(buffer, "perhaps it is corrupted?"));

    // 清理：关闭临时文件并删除
    fclose(tmp);
    remove("temp.txt");
}

