#include "../typedef.h"
#include "../typedef.h"
static void bsClose ( BitStream* bs )
{
   Int32 retVal;
   // 如果是写模式
   if ( bs->mode == 'w' ) {
      // 保证缓冲区的所有位都被写入
      while ( bs->buffLive < 8 ) {
         bs->buffLive++;
         bs->buffer <<= 1;
      };
      // 写入文件
      retVal = putc ( (UChar) (bs->buffer), bs->handle );
      if (retVal == (-1)) writeError();
      bytesOut++;
      // 刷新文件缓冲区
      retVal = fflush ( bs->handle );
      if (retVal == (-1)) writeError();
   }
   // 关闭文件句柄
   retVal = fclose ( bs->handle );
   if (retVal == (-1)) {
      if (bs->mode == 'w') writeError(); else readError();
   }
   // 释放内存
   free ( bs );
}
static Int32 bsGetBit ( BitStream* bs )
{
   // 如果缓冲区中还有位
   if (bs->buffLive > 0) {
      bs->buffLive --;
      return ( ((bs->buffer) >> (bs->buffLive)) & 0x1 );
   } else {
      // 从文件中读取一个字符
      Int32 retVal = getc ( bs->handle );
      if ( retVal == (-1) ) {
         if ((*__errno_location ()) != 0) readError();
         return 2;
      }
      bs->buffLive = 7;
      bs->buffer = retVal;
      // 返回最高位
      return ( ((bs->buffer) >> 7) & 0x1 );
   }
}
void test_bsClose_bsGetBit() {
    // 初始化一个BitStream
    BitStream bs;
    bs.mode = 'w';
    bs.handle = tmpfile();
    bs.buffer = 0b10101010;
    bs.buffLive = 8;
    // 使用bsClose关闭BitStream
    bsClose(&bs);
    // 重新打开BitStream并尝试使用bsGetBit获取位
    bs.mode = 'r';
    Int32 bit = bsGetBit(&bs);
    CU_ASSERT_EQUAL(bit, 1);
    // ... (可以继续测试其他位)
    // 再次关闭BitStream
    bsClose(&bs);
}
