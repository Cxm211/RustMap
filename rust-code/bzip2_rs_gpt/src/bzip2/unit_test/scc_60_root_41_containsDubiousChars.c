#include "../typedef.h"
static
Bool containsDubiousChars ( Char* name )
{
   // 目前直接返回False
   return ((Bool)0);
}

#include <CUnit/Basic.h>
void test_containsDubiousChars(void) {
    // 因为函数总是返回False，所以任何输入都应该返回False
    CU_ASSERT_EQUAL(containsDubiousChars("someName"), 0);
    CU_ASSERT_EQUAL(containsDubiousChars("anotherName"), 0);
}





