#include <string.h>
#include <stdio.h>

int palindrome(const char *s)
{
  int i, l;
  l = strlen(s);
  for (i = 0; i < l / 2; i++)
  {
    if (s[i] != s[l - i - 1])
      return 0;
  }
  return 1;
}

int palindrome_r(const char *s, int b, int e)
{
  if ((e - 1) <= b)
    return 1;
  if (s[b] != s[e - 1])
    return 0;
  return palindrome_r(s, b + 1, e - 1);
}

int main()
{
  const char *t = "ingirumimusnocteetconsumimurigni";
  const char *template = "sequence \"%s\" is%s palindrome\n";
  int l = strlen(t);

  printf(template,
         t, palindrome(t) ? "" : "n't");
  printf(template,
         t, palindrome_r(t, 0, l) ? "" : "n't");
  return 0;
}