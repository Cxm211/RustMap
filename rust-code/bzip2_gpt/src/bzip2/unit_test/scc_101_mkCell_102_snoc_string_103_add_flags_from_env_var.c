#include "../typedef.h"
#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>
#include "./scc_99_outOfMemory_scc_100_myMalloc.c"
typedef
   struct zzzz {
      Char        *name;  // 节点名称
      struct zzzz *link;  // 指向下一个节点的指针
   }
   Cell;



static
Cell *mkCell ( void )
{
   Cell *c;
   c = (Cell*) myMalloc ( sizeof ( Cell ) );
   c->name = ((void *)0);  // 初始化名称为NULL
   c->link = ((void *)0);  // 初始化指向下一个节点的指针为NULL
   return c;
}



static
Cell *snocString ( Cell *root, Char *name )
{
   if (root == ((void *)0)) {
      Cell *tmp = mkCell();
      tmp->name = (Char*) myMalloc ( 5 + strlen(name) );
      strcpy ( tmp->name, name );
      return tmp;
   } else {
      Cell *tmp = root;
      while (tmp->link != ((void *)0)) tmp = tmp->link;
      tmp->link = snocString ( tmp->link, name );
      return root;
   }
}
void test_snocString(void) {
    Cell* root = ((void *)0);  // 初始为空的链表
    Char str1[] = "Hello";
    Char str2[] = "World";

    root = snocString(root, str1);
    CU_ASSERT_STRING_EQUAL(root->name, "Hello");  // 检查第一个节点的名称

    root = snocString(root, str2);
    CU_ASSERT_STRING_EQUAL(root->link->name, "World");  // 检查第二个节点的名称
}





static
void addFlagsFromEnvVar ( Cell** argList, Char* varName )
{
   Int32 i, j, k;
   Char *envbase, *p;

   envbase = getenv(varName);
   if (envbase != 
                 ((void *)0)
                     ) {
      p = envbase;
      i = 0;
      while (((Bool)1)) {
         if (p[i] == 0) break;
         p += i;
         i = 0;
         while (
               ((*__ctype_b_loc ())[(int) ((
               (Int32)(p[0])
               ))] & (unsigned short int) _ISspace)
                                     ) p++;
         while (p[i] != 0 && !
                             ((*__ctype_b_loc ())[(int) ((
                             (Int32)(p[i])
                             ))] & (unsigned short int) _ISspace)
                                                   ) i++;
         if (i > 0) {
            k = i; if (k > 1034 -10) k = 1034 -10;
            for (j = 0; j < k; j++) tmpName[j] = p[j];
            tmpName[k] = 0;
            *argList=snocString((*argList), (tmpName));
         }
      }
   }
}



Char* getenv(const Char* varName) {
    if (strcmp(varName, "MY_TEST_ENV") == 0) {
        return "flag1 flag2 flag3";  // 这是模拟的环境变量值
    }
    return ((void *)0);
}

void test_addFlagsFromEnvVar(void) {
    Cell* root = ((void *)0);  // 初始为空的链表

    addFlagsFromEnvVar(&root, "MY_TEST_ENV");  // 添加从模拟环境变量获取的标志

    CU_ASSERT_STRING_EQUAL(root->name, "flag1");  // 检查第一个节点的名称
    CU_ASSERT_STRING_EQUAL(root->link->name, "flag2");  // 检查第二个节点的名称
    CU_ASSERT_STRING_EQUAL(root->link->link->name, "flag3");  // 检查第三个节点的名称
}




