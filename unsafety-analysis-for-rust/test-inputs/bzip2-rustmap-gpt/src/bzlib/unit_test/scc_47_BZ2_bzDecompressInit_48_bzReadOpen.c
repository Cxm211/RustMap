#include "../typedef.h"
#include "./scc_44_root_30_default_bzfree.c"
/**
 * 初始化bzip2解压流。
 *
 * @param strm       - 指向bz_stream结构的指针，该结构保存压缩或解压缩操作的状态。
 * @param verbosity  - 控制库输出多少辅助信息，范围为0到4。
 * @param small      - 控制解压缩操作是否使用较小的内存模型。
 *
 * @return 如果成功则返回0，如果失败则返回错误代码。
 */


int BZ2_bzDecompressInit
                     ( bz_stream* strm,
                       int verbosity,
                       int small )
{
   DState* s;

   if (!bz_config_ok()) return (-9);

   if (strm == 
              ((void *)0)
                  ) return (-2);
   if (small != 0 && small != 1) return (-2);
   if (verbosity < 0 || verbosity > 4) return (-2);

   if (strm->bzalloc == 
                       ((void *)0)
                           ) strm->bzalloc = default_bzalloc;
   if (strm->bzfree == 
                      ((void *)0)
                          ) strm->bzfree = default_bzfree;

   s = (strm->bzalloc)(strm->opaque,(sizeof(DState)),1);
   if (s == 
           ((void *)0)
               ) return (-3);
   s->strm = strm;
   strm->state = s;
   s->state = 10;
   s->bsLive = 0;
   s->bsBuff = 0;
   s->calculatedCombinedCRC = 0;
   strm->total_in_lo32 = 0;
   strm->total_in_hi32 = 0;
   strm->total_out_lo32 = 0;
   strm->total_out_hi32 = 0;
   s->smallDecompress = (Bool)small;
   s->ll4 = 
                             ((void *)0)
                                 ;
   s->ll16 = 
                             ((void *)0)
                                 ;
   s->tt = 
                             ((void *)0)
                                 ;
   s->currBlockNo = 0;
   s->verbosity = verbosity;

   return 0;
}





// #include <CUnit/CUnit.h>
// #include <CUnit/Basic.h>

// void test_BZ2_bzDecompressInit_valid() {
//     bz_stream strm;
//     memset(&strm, 0, sizeof(bz_stream));

//     int result = BZ2_bzDecompressInit(&strm, 2, 1);
//     CU_ASSERT_EQUAL(result, 0);
//     CU_ASSERT_PTR_NOT_NULL(strm.state);
// }

/*

函数解释：
BZ2_bzReadOpen初始化一个用于读取bzip2压缩文件的结构。它接受以下参数：

bzerror：指向一个整数的指针，用于存储可能发生的错误代码。
f：指向一个已打开的文件的FILE指针。
verbosity：控制库输出的详细程度，范围从0到4。
small：控制是否使用内存少的解压模式。
unused：指向未使用的数据的指针。
nUnused：未使用数据的长度。
该函数首先对传入的参数进行一些基本的检查，例如检查f是否为NULL，verbosity是否在有效范围内，以及其他几个参数。如果检查不通过，它会设置错误代码并返回NULL。

接着，该函数会为bzFile结构分配内存，并初始化其字段。然后，它将nUnused字节的数据从unused复制到bzFile结构的缓冲区。

最后，它使用BZ2_bzDecompressInit初始化解压流，并返回初始化的bzFile结构。


*/

// 打开bzip2压缩文件并准备进行解压操作
BZFILE* BZ2_bzReadOpen(int* bzerror, FILE* f, int verbosity, int small, void* unused, int nUnused) {
    bzFile* bzf = NULL;
    int ret;

    // 初始化错误代码为0
    if (bzerror != NULL) *bzerror = 0;
    if (bzf != NULL) bzf->lastErr = 0;

    // 参数有效性检查
    if (f == NULL ||
        (small != 0 && small != 1) ||
        (verbosity < 0 || verbosity > 4) ||
        (unused == NULL && nUnused != 0) ||
        (unused != NULL && (nUnused < 0 || nUnused > 5000))) {
        if (bzerror != NULL) *bzerror = (-2);
        if (bzf != NULL) bzf->lastErr = (-2);
        return NULL;
    }

    // 检查文件错误
    if (ferror(f)) {
        if (bzerror != NULL) *bzerror = (-6);
        if (bzf != NULL) bzf->lastErr = (-6);
        return NULL;
    }

    // 为bzFile结构分配内存
    bzf = malloc(sizeof(bzFile));
    if (bzf == NULL) {
        if (bzerror != NULL) *bzerror = (-3);
        if (bzf != NULL) bzf->lastErr = (-3);
        return NULL;
    }

    // 初始化bzFile结构
    bzf->initialisedOk = false;
    bzf->handle = f;
    bzf->bufN = 0;
    bzf->writing = false;
    bzf->strm.bzalloc = NULL;
    bzf->strm.bzfree = NULL;
    bzf->strm.opaque = NULL;

    // 复制未使用的数据到缓冲区
    while (nUnused > 0) {
        bzf->buf[bzf->bufN] = *((UChar*)(unused));
        bzf->bufN++;
        unused = ((void*)(1 + ((UChar*)(unused))));
        nUnused--;
    }

    // 初始化解压流
    ret = BZ2_bzDecompressInit(&(bzf->strm), verbosity, small);
    if (ret != 0) {
        if (bzerror != NULL) *bzerror = ret;
        if (bzf != NULL) bzf->lastErr = ret;
        free(bzf);
        return NULL;
    }

    bzf->strm.avail_in = bzf->bufN;
    bzf->strm.next_in = bzf->buf;

    bzf->initialisedOk = true;
    return bzf;
}

// void test_BZ2_bzReadOpen_valid() {
//     int error;
//     FILE* file = fopen("test.bzip2", "rb"); // 请确保有一个叫做test.bzip2的压缩文件
//     BZFILE* bzf = BZ2_bzReadOpen(&error, file, 2, 1, NULL, 0);

//     CU_ASSERT_PTR_NOT_NULL(bzf);
//     CU_ASSERT_EQUAL(error, 0);
    
//     // 在此处添加额外的测试，如关闭文件，释放资源等
//     fclose(file);
// }

// int main() {
//     CU_initialize_registry();

//     CU_pSuite suite = CU_add_suite("Test Two", NULL, NULL);
//     CU_add_test(suite, "test_BZ2_bzReadOpen_valid", test_BZ2_bzReadOpen_valid);
//     CU_add_test(suite, "test_BZ2_bzDecompressInit_valid", test_BZ2_bzDecompressInit_valid);


//     CU_basic_set_mode(CU_BRM_VERBOSE);
//     CU_basic_run_tests();

//     CU_cleanup_registry();
//     return 0;
// }
