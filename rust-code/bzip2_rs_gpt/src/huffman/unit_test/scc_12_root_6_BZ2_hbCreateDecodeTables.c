

typedef unsigned char UChar;
typedef int Int32;
#include <stdio.h>
#include <CUnit/Basic.h>

void BZ2_hbCreateDecodeTables ( Int32 *limit,
                                Int32 *base,
                                Int32 *perm,
                                UChar *length,
                                Int32 minLen,
                                Int32 maxLen,
                                Int32 alphaSize )
{
   Int32 pp, i, j, vec;

   pp = 0;
   for (i = minLen; i <= maxLen; i++)
      for (j = 0; j < alphaSize; j++)
         if (length[j] == i) { perm[pp] = j; pp++; };

   for (i = 0; i < 23; i++) base[i] = 0;  // 2. 初始化base和limit数组：这两个数组都被初始化为0。
   for (i = 0; i < alphaSize; i++) base[length[i]+1]++;

   for (i = 1; i < 23; i++) base[i] += base[i-1];

   for (i = 0; i < 23; i++) limit[i] = 0; // 2. 初始化base和limit数组：这两个数组都被初始化为0。
   vec = 0;

   for (i = minLen; i <= maxLen; i++) {
      vec += (base[i+1] - base[i]); // 更新vec的值为base[i+1] - base[i]的和。
      limit[i] = vec-1; 
      vec <<= 1; // vec值翻倍。
   }
   // 5. 更新base数组：更新每个base[i]的值。
   for (i = minLen + 1; i <= maxLen; i++)
      base[i] = ((limit[i-1] + 1) << 1) - base[i];
}
void test_BZ2_hbCreateDecodeTables(void) {
    // Set up test input
    Int32 limit[23] = {0};
    Int32 base[23] = {0};
    Int32 perm[23] = {0};
    UChar length[23] = {2, 3, 3, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8};
    Int32 minLen = 2;
    Int32 maxLen = 8;
    Int32 alphaSize = 23;

    BZ2_hbCreateDecodeTables(limit, base, perm, length, minLen, maxLen, alphaSize);

    // Check the function's results against expected outputs
    Int32 expected_perm[23] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22};
    Int32 expected_base[23] = {0, 0, 0, 1, 5, 19, 51, 119, 259, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23};
    Int32 expected_limit[23] = {0, 0, 0, 3, 13, 31, 67, 139, 281, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

    for (int i = 0; i < 23; i++) {
        CU_ASSERT_EQUAL(perm[i], expected_perm[i]);
        CU_ASSERT_EQUAL(base[i], expected_base[i]);
        CU_ASSERT_EQUAL(limit[i], expected_limit[i]);
    }
}

int main() {
    CU_pSuite pSuite = NULL;

    /* Initialize the CUnit test registry */
    if (CU_initialize_registry() != CUE_SUCCESS) {
        return CU_get_error();
    }

    /* Add a suite to the registry */
    pSuite = CU_add_suite("BZ2_hbCreateDecodeTables_test", NULL, NULL);
    if (pSuite == NULL) {
        CU_cleanup_registry();
        return CU_get_error();
    }

    /* Add the test to the suite */
    if (CU_add_test(pSuite, "test_BZ2_hbCreateDecodeTables", test_BZ2_hbCreateDecodeTables) == NULL) {
        CU_cleanup_registry();
        return CU_get_error();
    }

    /* Run all tests using the CUnit Basic interface */
    CU_basic_set_mode(CU_BRM_VERBOSE);
    CU_basic_run_tests();
    CU_cleanup_registry();

    return CU_get_error();
}

