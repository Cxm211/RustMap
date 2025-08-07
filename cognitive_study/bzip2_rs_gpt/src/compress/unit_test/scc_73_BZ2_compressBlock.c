#include "./typedef.h"

void BZ2_compressBlock ( EState* s, Bool is_last_block )
{
   // 如果当前的数据块大小大于0
   if (s->nblock > 0) {

      // 计算当前数据块的CRC
      { s->blockCRC = ~(s->blockCRC); };

      // 更新累积的CRC值
      s->combinedCRC = (s->combinedCRC << 1) | (s->combinedCRC >> 31);
      s->combinedCRC ^= s->blockCRC;

      // 如果不是第一个数据块，则重置numZ为0
      if (s->blockNo > 1) s->numZ = 0;

      // 输出诊断信息
      if (s->verbosity >= 2)
         fprintf(
        stderr
        ,"    block %d: crc = 0x%08x, " "combined CRC = 0x%08x, size = %d\n",s->blockNo,s->blockCRC,s->combinedCRC,s->nblock);

      // 对数据块进行排序
      BZ2_blockSort ( s );
   }

   // 设置zbits指针
   s->zbits = (UChar*) (&((UChar*)s->arr2)[s->nblock]);

   // 如果是第一个块，则输出bzip2的文件标头
   if (s->blockNo == 1) {
      BZ2_bsInitWrite ( s );
      bsPutUChar ( s, 0x42 );
      bsPutUChar ( s, 0x5a );
      bsPutUChar ( s, 0x68 );
      bsPutUChar ( s, (UChar)(0x30 + s->blockSize100k) );
   }

   // 如果当前的数据块大小大于0
   if (s->nblock > 0) {

      // 输出固定的标记
      bsPutUChar ( s, 0x31 ); bsPutUChar ( s, 0x41 );
      bsPutUChar ( s, 0x59 ); bsPutUChar ( s, 0x26 );
      bsPutUChar ( s, 0x53 ); bsPutUChar ( s, 0x59 );

      // 输出块的CRC和原始指针
      bsPutUInt32 ( s, s->blockCRC );
      bsW(s,1,0);
      bsW ( s, 24, s->origPtr );

      // 生成和发送MTF值
      generateMTFValues ( s );
      sendMTFValues ( s );
   }

   // 如果是最后一个数据块
   if (is_last_block) {

      // 输出结束标记和累积的CRC
      bsPutUChar ( s, 0x17 ); bsPutUChar ( s, 0x72 );
      bsPutUChar ( s, 0x45 ); bsPutUChar ( s, 0x38 );
      bsPutUChar ( s, 0x50 ); bsPutUChar ( s, 0x90 );
      bsPutUInt32 ( s, s->combinedCRC );

      // 输出诊断信息
      if (s->verbosity >= 2)
         fprintf(
        stderr
        ,"    final combined CRC = 0x%08x\n   ",s->combinedCRC);

      // 完成写操作
      bsFinishWrite ( s );
   }
}
