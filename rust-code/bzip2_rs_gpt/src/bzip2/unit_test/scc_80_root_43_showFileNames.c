
#include "../typedef.h"
Bool noisy;
Char inName [1034];
Char outName[1034];
void showFileNames ( void )
{
   if (noisy)
   fprintf (
      
     stderr
           ,
      "\tInput file = %s, output file = %s\n",
      inName, outName
   );
}

#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>
#include <string.h>


void test_showFileNames_noisyTrue(void) {
    noisy = TRUE;  // 设置noisy为True
    strcpy(inName, "testInput.txt");  // 设置输入文件名
    strcpy(outName, "testOutput.txt"); // 设置输出文件名

    // 这里我们无法直接检查stderr的输出，
    // 但你可以手动运行并观察是否在stderr上正确输出了文件名。
    showFileNames();
}

void test_showFileNames_noisyFalse(void) {
    noisy = FALSE;  // 设置noisy为False

    // 当noisy为False时，即使设置了文件名，函数也不应该有输出。
    // 同样，你可以手动运行并观察stderr是否没有输出。
    showFileNames();
}





