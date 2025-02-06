
#include "../typedef.h"
static
Bool hasSuffix ( Char* s, const Char* suffix )
{
   Int32 ns = strlen(s);         // 获取s的长度
   Int32 nx = strlen(suffix);    // 获取suffix的长度
   if (ns < nx) return ((Bool)0);  // 如果s的长度小于suffix的长度，则返回false
   // 比较s的最后几个字符和suffix是否相同，相同则返回true，否则返回false
   if (strcmp(s + ns - nx, suffix) == 0) return ((Bool)1);
   return ((Bool)0);
}

static
Bool mapSuffix ( Char* name,
                 const Char* oldSuffix,
                 const Char* newSuffix )
{
   // 如果name不以oldSuffix为后缀，则返回false
   if (!hasSuffix(name,oldSuffix)) return ((Bool)0);
   // 将name的后缀oldSuffix去除
   name[strlen(name)-strlen(oldSuffix)] = 0;
   // 追加新后缀newSuffix
   strcat ( name, newSuffix );
   return ((Bool)1);  // 返回true表示后缀已成功更改
}
#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>

void test_hasSuffix(void) {
    CU_ASSERT_TRUE(hasSuffix("filename.txt", ".txt"));
    CU_ASSERT_FALSE(hasSuffix("filename.txt", ".png"));
}

void test_mapSuffix(void) {
    Char name1[] = "image.jpg";
    Char name2[] = "document.txt";
    
    CU_ASSERT_TRUE(mapSuffix(name1, ".jpg", ".png"));
    CU_ASSERT_STRING_EQUAL(name1, "image.png");
    
    CU_ASSERT_FALSE(mapSuffix(name2, ".jpg", ".png"));
    CU_ASSERT_STRING_EQUAL(name2, "document.txt");
}

    
    
    
