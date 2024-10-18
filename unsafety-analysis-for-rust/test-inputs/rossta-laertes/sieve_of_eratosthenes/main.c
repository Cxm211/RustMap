#include <stdlib.h>
#include <math.h>
#include <malloc.h>

char *
eratosthenes(int n, int *c)
{
    char *sieve;
    int i, j, m;

    if (n < 2)
        return NULL;

    *c = n - 1; /* primes count */
    m = (int)sqrt((double)n);

    /* calloc initializes to zero */
    sieve = calloc(n + 1, sizeof(char));
    sieve[0] = 1;
    sieve[1] = 1;
    for (i = 2; i <= m; i++)
        if (!sieve[i])
            for (j = i * i; j <= n; j += i)
                if (!sieve[j])
                {
                    sieve[j] = 1;
                    --(*c);
                }
    return sieve;
}
void sieve(int *, int);

int main(void)
{
    int *array, n = 10;
    array = (int *)malloc((n + 1) * sizeof(int));
    sieve(array, n);
    return 0;
}

void sieve(int *a, int n)
{
    int i = 0, j = 0;

    for (i = 2; i <= n; i++)
    {
        a[i] = 1;
    }

    for (i = 2; i <= n; i++)
    {
        printf("\ni:%d", i);
        if (a[i] == 1)
        {
            for (j = i; (i * j) <= n; j++)
            {
                printf("\nj:%d", j);
                printf("\nBefore a[%d*%d]: %d", i, j, a[i * j]);
                a[(i * j)] = 0;
                printf("\nAfter a[%d*%d]: %d", i, j, a[i * j]);
            }
        }
    }

    printf("\nPrimes numbers from 1 to %d are : ", n);
    for (i = 2; i <= n; i++)
    {
        if (a[i] == 1)
            printf("%d, ", i);
    }
    printf("\n\n");
}
