
#include "../typedef.h"
#include "../bzlib/scc_37_root_25_add_pair_to_block.c"

 // 准备新的数据块
static void prepare_new_block (EState* s)
{
   Int32 i;
   s->nblock = 0;               // 初始化块的大小
   s->numZ = 0;                 // 初始化numZ为0
   s->state_out_pos = 0;        // 初始化输出位置
   s->blockCRC = 0xffffffffL;  // 初始化CRC值
   for (i = 0; i < 256; i++) 
      s->inUse[i] = ((Bool)0);  // 将inUse数组全部设为false
   s->blockNo++;               // 块编号递增
}

static void init_RL (EState* s)
{
   s->state_in_ch = 256;      // 设置当前输入字符为256
   s->state_in_len = 0;       // 初始化输入长度为0
}

static Bool isempty_RL (EState* s)
{
   if (s->state_in_ch < 256 && s->state_in_len > 0)
      return ((Bool)0);
   else
      return ((Bool)1);
}

static void flush_RL (EState* s)
{
   if (s->state_in_ch < 256) 
      add_pair_to_block(s);  // 如果当前输入字符有效，则添加到块中
   init_RL(s);               // 重新初始化RLE状态
}


void test_prepare_new_block() {
    EState s;
    s.strm = NULL;  // 其他字段在prepare_new_block中设置，所以这里只需初始化strm

    prepare_new_block(&s);
    CU_ASSERT_EQUAL(s.nblock, 0);
    CU_ASSERT_EQUAL(s.numZ, 0);
    CU_ASSERT_EQUAL(s.state_out_pos, 0);
    CU_ASSERT_EQUAL(s.blockCRC, 0xffffffffL);
    for(int i = 0; i < 256; i++) {
        CU_ASSERT_EQUAL(s.inUse[i], 0);
    }
    CU_ASSERT_EQUAL(s.blockNo, 1); // 我们应该确保blockNo增加
}


void test_init_RL() {
    EState s;
    s.strm = NULL;  // 其他字段在init_RL中设置，所以这里只需初始化strm

    init_RL(&s);
    CU_ASSERT_EQUAL(s.state_in_ch, 256);
    CU_ASSERT_EQUAL(s.state_in_len, 0);
}

void test_isempty_RL() {
    EState s;
    s.strm = NULL;
    s.state_in_ch = 255;
    s.state_in_len = 1;

    Bool result = isempty_RL(&s);
    CU_ASSERT_FALSE(result);

    s.state_in_ch = 256;
    s.state_in_len = 0;

    result = isempty_RL(&s);
    CU_ASSERT_TRUE(result);
}





