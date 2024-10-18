#include "../typedef.h"
#include "./scc_44_root_30_default_bzfree.c"
// 打开并初始化一个用于写入的 BZFILE 结构
// 定义BZ2_bzWriteOpen函数
BZFILE* BZ2_bzWriteOpen
                    ( int* bzerror,          // 用于返回的错误码指针
                      FILE* f,               // 目标文件的文件句柄
                      int blockSize100k,     // 压缩块的大小（100K的倍数，例如9表示900K）
                      int verbosity,         // 输出冗余级别，控制压缩过程中的输出信息
                      int workFactor )       // 工作因子，影响压缩的性能与效果
{
   Int32 ret; // 存储BZ2_bzCompressInit函数的返回值
   // 定义bzf为bzFile结构体的指针，并初始化为NULL
   bzFile* bzf = ((void *)0);

   // 初始化错误码和bzf的lastErr为0
   { if (bzerror != ((void *)0)) *bzerror = 0; if (bzf != ((void *)0)) bzf->lastErr = 0; };

   // 检查输入参数的有效性
   if (f == ((void *)0) || // 文件句柄不能为空
       (blockSize100k < 1 || blockSize100k > 9) || // 压缩块大小范围1-9
       (workFactor < 0 || workFactor > 250) || // 工作因子范围0-250
       (verbosity < 0 || verbosity > 4))      // 输出冗余级别范围0-4
   {
      // 如果有任何参数不在有效范围内，设置错误码为-2并返回NULL
      { if (bzerror != ((void *)0)) *bzerror = (-2); if (bzf != ((void *)0)) bzf->lastErr = (-2); };
      return ((void *)0);
   }

   // 检查文件是否有错误
   if (ferror(f))
   {
      // 如果文件有错误，设置错误码为-6并返回NULL
      { if (bzerror != ((void *)0)) *bzerror = (-6); if (bzf != ((void *)0)) bzf->lastErr = (-6); };
      return ((void *)0);
   }

   // 为bzFile结构体分配内存
   bzf = malloc ( sizeof(bzFile) );
   if (bzf == ((void *)0))
   {
      // 如果分配失败，设置错误码为-3并返回NULL
      { if (bzerror != ((void *)0)) *bzerror = (-3); if (bzf != ((void *)0)) bzf->lastErr = (-3); };
      return ((void *)0);
   }

   // 初始化bzf结构体的部分字段
   { if (bzerror != ((void *)0)) *bzerror = 0; if (bzf != ((void *)0)) bzf->lastErr = 0; };
   bzf->initialisedOk = ((Bool)0); // 尚未完成初始化
   bzf->bufN = 0; // 缓冲区索引为0
   bzf->handle = f; // 文件句柄
   bzf->writing = ((Bool)1); // 设置为写模式
   bzf->strm.bzalloc = ((void *)0); // 分配函数指针为NULL
   bzf->strm.bzfree = ((void *)0); // 释放函数指针为NULL
   bzf->strm.opaque = ((void *)0); // 不透明指针为NULL

   // 如果工作因子为0，则设置为默认值30
   if (workFactor == 0) workFactor = 30;
   // 初始化压缩流
   ret = BZ2_bzCompressInit ( &(bzf->strm), blockSize100k, verbosity, workFactor );
   if (ret != 0)
   {
      // 如果初始化失败，释放bzf，并将错误码设置为BZ2_bzCompressInit的返回值
      { if (bzerror != ((void *)0)) *bzerror = ret; if (bzf != ((void *)0)) bzf->lastErr = ret; };
      free(bzf);
      return ((void *)0);
   }

   // 设置输入流的可用字节数为0
   bzf->strm.avail_in = 0;
   // 设置初始化完成标志
   bzf->initialisedOk = ((Bool)1);
   // 返回bzf指针
   return bzf;
}



// BZ2_bzCompressInit 函数用于初始化 bz_stream 结构，以进行数据压缩。
int BZ2_bzCompressInit(bz_stream* strm,      // bz_stream 结构指针
                       int blockSize100k,    // 压缩块的大小
                       int verbosity,        // 输出冗余级别
                       int workFactor)       // 工作因子
{
   Int32 n;
   EState* s;

   // 检查配置是否正确
   if (!bz_config_ok()) return (-9);

   // 输入参数验证
   if (strm == ((void *)0) || 
       blockSize100k < 1 || blockSize100k > 9 ||
       workFactor < 0 || workFactor > 250)
     return (-2);

   // 如果 workFactor 为 0，则默认设置为 30
   if (workFactor == 0) workFactor = 30;

   // 如果没有为 bzalloc 或 bzfree 分配内存，则使用默认的分配函数
   if (strm->bzalloc == ((void *)0)) strm->bzalloc = default_bzalloc;
   if (strm->bzfree == ((void *)0)) strm->bzfree = default_bzfree;

   // 使用 bzalloc 分配 EState 结构的内存
   s = (strm->bzalloc)(strm->opaque, sizeof(EState), 1);
   if (s == ((void *)0)) return (-3);
   s->strm = strm;

   // 初始化指针
   s->arr1 = ((void *)0);
   s->arr2 = ((void *)0);
   s->ftab = ((void *)0);

   // 根据 blockSize100k 计算 n 的值
   n = 100000 * blockSize100k;

   // 分配内存
   s->arr1 = (strm->bzalloc)(strm->opaque, n * sizeof(UInt32), 1);
   s->arr2 = (strm->bzalloc)(strm->opaque, (n + (2 + 12 + 18 + 2)) * sizeof(UInt32), 1);
   s->ftab = (strm->bzalloc)(strm->opaque, 65537 * sizeof(UInt32), 1);

   // 如果任一内存分配失败，释放已分配的内存并返回错误
   if (s->arr1 == ((void *)0) || 
       s->arr2 == ((void *)0) || 
       s->ftab == ((void *)0))
   {
      if (s->arr1 != ((void *)0)) (strm->bzfree)(strm->opaque, s->arr1);
      if (s->arr2 != ((void *)0)) (strm->bzfree)(strm->opaque, s->arr2);
      if (s->ftab != ((void *)0)) (strm->bzfree)(strm->opaque, s->ftab);
      if (s != ((void *)0)) (strm->bzfree)(strm->opaque, s);
      return (-3);
   }

   // 初始化 s 结构的其他成员
   s->blockNo = 0;
   s->state = 2;
   s->mode = 2;
   s->combinedCRC = 0;
   s->blockSize100k = blockSize100k;
   s->nblockMAX = 100000 * blockSize100k - 19;
   s->verbosity = verbosity;
   s->workFactor = workFactor;
   s->block = (UChar*)s->arr2;
   s->mtfv = (UInt16*)s->arr1;
   s->zbits = ((void *)0);
   s->ptr = (UInt32*)s->arr1;

   // 初始化 strm 结构的成员
   strm->state = s;
   strm->total_in_lo32 = 0;
   strm->total_in_hi32 = 0;
   strm->total_out_lo32 = 0;
   strm->total_out_hi32 = 0;

   // 调用初始化函数
   init_RL(s);
   prepare_new_block(s);

   // 函数成功，返回 0
   return 0;
}


void test_BZ2_bzWriteOpen(void) {
   int bzerror;
   FILE* test_file = fopen("test.txt", "wb");  // 打开测试文件用于写入
   BZFILE* bzf = BZ2_bzWriteOpen(&bzerror, test_file, 5, 1, 30);
   
   CU_ASSERT_PTR_NOT_NULL(bzf);     // 检查返回的 BZFILE 指针是否非空
   CU_ASSERT_EQUAL(bzerror, 0);     // 检查是否没有错误
   
   // 清理代码
   if (bzf != NULL) {
      BZ2_bzWriteClose(&bzerror, bzf, 0, NULL, NULL);
   }
   if (test_file != NULL) {
      fclose(test_file);
   }
}

void test_BZ2_bzCompressInit(void) {
   bz_stream strm;
   int ret = BZ2_bzCompressInit(&strm, 5, 1, 30);
   
   CU_ASSERT_EQUAL(ret, 0);         // 检查函数返回值是否为 0
   CU_ASSERT_PTR_NOT_NULL(strm.state);  // 检查 strm 的 state 指针是否已初始化
   
   // 清理代码
   if (ret == 0) {
      BZ2_bzCompressEnd(&strm); 
   }
}
