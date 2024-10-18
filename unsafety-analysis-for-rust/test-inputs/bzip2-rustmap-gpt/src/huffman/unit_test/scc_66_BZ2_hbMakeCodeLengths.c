#include "../typedef.h"
#include "../bzlib/scc_62_BZ2_bz__AssertH__fail.c"
// 创建Huffman编码长度
void BZ2_hbMakeCodeLengths ( UChar *len, Int32 *freq, Int32 alphaSize, Int32 maxLen ) {

   // 定义一些变量
   Int32 nNodes, nHeap, n1, n2, i, j, k;
   Bool tooLong;

   Int32 heap [ 258 + 2 ];
   Int32 weight [ 258 * 2 ];
   Int32 parent [ 258 * 2 ];

   // 初始化权重
   for (i = 0; i < alphaSize; i++)
      weight[i+1] = (freq[i] == 0 ? 1 : freq[i]) << 8;

   while (1) {

      nNodes = alphaSize;
      nHeap = 0;

      heap[0] = 0;
      weight[0] = 0;
      parent[0] = -2;

      // 构建初步的堆
      for (i = 1; i <= alphaSize; i++) {
         parent[i] = -1;
         nHeap++;
         heap[nHeap] = i;
         // 堆排序调整
         Int32 zz = nHeap, tmp = heap[zz];
         while (weight[tmp] < weight[heap[zz >> 1]]) {
            heap[zz] = heap[zz >> 1];
            zz >>= 1;
         }
         heap[zz] = tmp;
      }

      // 堆验证
      if (!(nHeap < (258 +2))) BZ2_bz__AssertH__fail ( 2001 );

      // 创建Huffman树
      while (nHeap > 1) {
         n1 = heap[1]; 
         // 堆排序调整
         heap[1] = heap[nHeap];
         nHeap--;
         Int32 zz = 1, yy, tmp = heap[zz];
         while (1) {
            yy = zz << 1;
            if (yy > nHeap) break;
            if (yy < nHeap && weight[heap[yy+1]] < weight[heap[yy]]) yy++;
            if (weight[tmp] < weight[heap[yy]]) break;
            heap[zz] = heap[yy];
            zz = yy;
         }
         heap[zz] = tmp;

         // 重复上述操作，得到第二个最小权重的节点
         n2 = heap[1];
         heap[1] = heap[nHeap];
         nHeap--;
         zz = 1; tmp = heap[zz];
         while (1) {
            yy = zz << 1;
            if (yy > nHeap) break;
            if (yy < nHeap && weight[heap[yy+1]] < weight[heap[yy]]) yy++;
            if (weight[tmp] < weight[heap[yy]]) break;
            heap[zz] = heap[yy];
            zz = yy;
         }
         heap[zz] = tmp;

         // 创建新的父节点
         nNodes++;
         parent[n1] = parent[n2] = nNodes;
         weight[nNodes] = ((weight[n1] & 0xffffff00) + (weight[n2] & 0xffffff00)) | (1 + (weight[n1] & 0x000000ff > weight[n2] & 0x000000ff ? weight[n1] & 0x000000ff : weight[n2] & 0x000000ff));
         parent[nNodes] = -1;
         nHeap++;
         heap[nHeap] = nNodes;
         zz = nHeap; tmp = heap[zz];
         while (weight[tmp] < weight[heap[zz >> 1]]) {
            heap[zz] = heap[zz >> 1];
            zz >>= 1;
         }
         heap[zz] = tmp;
      }

      if (!(nNodes < (258 * 2))) BZ2_bz__AssertH__fail ( 2002 );

      tooLong = 0;
      for (i = 1; i <= alphaSize; i++) {
         j = 0;
         k = i;
         // 计算每个符号的编码长度
         while (parent[k] >= 0) {
            k = parent[k];
            j++;
         }
         len[i-1] = j;
         if (j > maxLen) tooLong = 1;
      }

      // 如果没有超过最大编码长度，则结束循环
      if (! tooLong) break;
      
      // 否则，更新权重并重新开始
      for (i = 1; i <= alphaSize; i++) {
         j = weight[i] >> 8;
         j = 1 + (j / 2);
         weight[i] = j << 8;
      }
   }
}


#include <CUnit/Basic.h>

void test_BZ2_hbMakeCodeLengths(void) {
    UChar len[5]; // 假设alphaSize是5
    Int32 freq[5] = {5, 15, 10, 0, 1};
    Int32 alphaSize = 5;
    Int32 maxLen = 10;

    BZ2_hbMakeCodeLengths(len, freq, alphaSize, maxLen);

    CU_ASSERT(len[0] <= maxLen);  // 我们只测试长度是否小于或等于maxLen
    CU_ASSERT(len[1] <= maxLen);
    CU_ASSERT(len[2] <= maxLen);
    CU_ASSERT(len[3] <= maxLen);
    CU_ASSERT(len[4] <= maxLen);
}

int main() {
    CU_pSuite pSuite = NULL;

    /* 初始化CUnit库 */
    if (CUE_SUCCESS != CU_initialize_registry())
        return CU_get_error();

    /* 添加测试套件 */
    pSuite = CU_add_suite("Suite_1", NULL, NULL);
    if (NULL == pSuite) {
        CU_cleanup_registry();
        return CU_get_error();
    }

    /* 添加测试函数 */
    if (NULL == CU_add_test(pSuite, "test_BZ2_hbMakeCodeLengths", test_BZ2_hbMakeCodeLengths)) {
        CU_cleanup_registry();
        return CU_get_error();
    }

    /* 运行测试并清理 */
    CU_basic_set_mode(CU_BRM_VERBOSE);
    CU_basic_run_tests();
    CU_cleanup_registry();
    return CU_get_error();
}
