#include "../typedef.h"
static void mallocFail ( Int32 n )
{
   // 输出内存分配失败的错误消息
   fprintf(stderr, "%s: malloc failed on request for %d bytes.\n", progName, n);
   fprintf(stderr, "%s: warning: output file(s) may be incomplete.\n", progName);
   // 退出程序
   exit(1);
}
static BitStream* bsOpenReadStream ( FILE* stream )
{
   BitStream *bs = malloc(sizeof(BitStream));
   // 如果内存分配失败，调用mallocFail函数
   if (bs == NULL) mallocFail(sizeof(BitStream));
   // 初始化BitStream的字段
   bs->handle = stream;
   bs->buffer = 0;
   bs->buffLive = 0;
   bs->mode = 'r';
   return bs;
}
static BitStream* bsOpenWriteStream ( FILE* stream )
{
   BitStream *bs = malloc(sizeof(BitStream));
   // 如果内存分配失败，调用mallocFail函数
   if (bs == NULL) mallocFail(sizeof(BitStream));
   // 初始化BitStream的字段
   bs->handle = stream;
   bs->buffer = 0;
   bs->buffLive = 0;
   bs->mode = 'w';
   return bs;
}
#include <CUnit/Basic.h>
void test_bsOpenReadStream(void) {
    FILE* stream = fopen("test_read.txt", "r");
    BitStream* bs = bsOpenReadStream(stream);
    CU_ASSERT_PTR_NOT_NULL(bs);
    CU_ASSERT_EQUAL(bs->buffLive, 0);
    CU_ASSERT_EQUAL(bs->mode, 'r');
    fclose(stream);
    free(bs);
}
void test_bsOpenWriteStream(void) {
    FILE* stream = fopen("test_write.txt", "w");
    BitStream* bs = bsOpenWriteStream(stream);
    CU_ASSERT_PTR_NOT_NULL(bs);
    CU_ASSERT_EQUAL(bs->buffLive, 0);
    CU_ASSERT_EQUAL(bs->mode, 'w');
    fclose(stream);
    free(bs);
}
int main() {
    CU_pSuite pSuite = NULL;
    if (CUE_SUCCESS != CU_initialize_registry()) {
        return CU_get_error();
    }
    pSuite = CU_add_suite("BitStreamTestSuite", NULL, NULL);
    if (NULL == pSuite) {
        CU_cleanup_registry();
        return CU_get_error();
    }
    if (NULL == CU_add_test(pSuite, "test_bsOpenReadStream", test_bsOpenReadStream) ||
        NULL == CU_add_test(pSuite, "test_bsOpenWriteStream", test_bsOpenWriteStream)) {
        CU_cleanup_registry();
        return CU_get_error();
    }
    CU_basic_set_mode(CU_BRM_VERBOSE);
    CU_basic_run_tests();
    CU_cleanup_registry();
    return CU_get_error();
}
