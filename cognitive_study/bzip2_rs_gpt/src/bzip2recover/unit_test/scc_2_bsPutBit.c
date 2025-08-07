#include "../typedef.h"
#include "../bzip2recover/scc_1_root_1_writeError.c"
static void bsPutBit ( BitStream* bs, Int32 bit ) {
   // 如果缓存已满（8位）
   if (bs->buffLive == 8) {
      // 写入缓存内容到文件
      Int32 retVal = putc ( (UChar) bs->buffer, bs->handle );
      // 检查写入是否成功
      if (retVal == (-1)) writeError();
      bytesOut++;
      bs->buffLive = 1;
      bs->buffer = bit & 0x1;
   } else {
      // 否则，将位添加到缓存
      bs->buffer = ( (bs->buffer << 1) | (bit & 0x1) );
      bs->buffLive++;
   };
}
static void bsPutUChar ( BitStream* bs, UChar c ) {
   Int32 i;
   for (i = 7; i >= 0; i--)
      // 为每个位调用 bsPutBit 函数
      bsPutBit ( bs, (((UInt32) c) >> i) & 0x1 );
}
static void bsPutUInt32 ( BitStream* bs, UInt32 c ) {
   Int32 i;
   for (i = 31; i >= 0; i--)
      // 为每个位调用 bsPutBit 函数
      bsPutBit ( bs, (c >> i) & 0x1 );
}
#include <CUnit/Basic.h>
#include <stdio.h>
void test_bsPutUChar(void) {
   BitStream bs;
   FILE* tmp = tmpfile();
   resetBitStream(&bs, tmp);
   UChar testValue = 0x5A;  // Arbitrary test value
   bsPutUChar(&bs, testValue);
   // Rewind and read the file
   rewind(tmp);
   UChar readValue = fgetc(tmp);
   assert(readValue == testValue);
   
   fclose(tmp);
}
void test_bsPutBit(void) {
   BitStream bs;
   FILE* tmp = tmpfile();
   resetBitStream(&bs, tmp);
   // Write 8 bits one by one
   bsPutBit(&bs, 0);
   bsPutBit(&bs, 1);
   bsPutBit(&bs, 0);
   bsPutBit(&bs, 1);
   bsPutBit(&bs, 1);
   bsPutBit(&bs, 0);
   bsPutBit(&bs, 1);
   bsPutBit(&bs, 0);
   // Rewind and read the file
   rewind(tmp);
   UChar readValue = fgetc(tmp);
   assert(readValue == 0x5A);  // Corresponds to the bits written
   fclose(tmp);
}
void resetBitStream(BitStream* bs, FILE* handle) {
   bs->buffer = 0;
   bs->buffLive = 0;
   bs->handle = handle;
   bytesOut = 0;
}
void test_bsPutUInt32(void) {
   BitStream bs;
   FILE* tmp = tmpfile();
   resetBitStream(&bs, tmp);
   UInt32 testValue = 0xDEADBEEF;  // Arbitrary test value
   bsPutUInt32(&bs, testValue);
   // Rewind and read the file
   rewind(tmp);
   UInt32 readValue;
   fread(&readValue, sizeof(UInt32), 1, tmp);
   assert(readValue == testValue);
   fclose(tmp);
}
int main(void) {
   CU_pSuite pSuite = NULL;
   // 初始化CUnit库
   if (CUE_SUCCESS != CU_initialize_registry())
      return CU_get_error();
   // 添加测试套件到注册中心
   pSuite = CU_add_suite("BitStream_test_suite", NULL, NULL);
   if (NULL == pSuite) {
      CU_cleanup_registry();
      return CU_get_error();
   }
   // 将测试添加到套件中
   CU_ADD_TEST(pSuite, test_bsPutUChar);
   CU_ADD_TEST(pSuite, test_bsPutBit);
   CU_ADD_TEST(pSuite, test_bsPutUInt32);
   // 运行所有测试
   CU_basic_set_mode(CU_BRM_VERBOSE);
   CU_basic_run_tests();
   CU_cleanup_registry();
   return CU_get_error();
}
