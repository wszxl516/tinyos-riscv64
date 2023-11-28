#include "common.h"
void *memset(void *dest, int val, usize len) {
  unsigned char *ptr = dest;
  while (len-- > 0)
    *ptr++ = val;
  return dest;
}
void *memcpy(void *dest, const void *src, usize len) {
  char *d = dest;
  const char *s = src;
  while (len--)
    *d++ = *s++;
  return dest;
}

void *memchr(void const *buf, int c, usize len) {
  usize i;
  unsigned char const *b = buf;
  unsigned char x = (c & 0xff);

  for (i = 0; i < len; i++) {
    if (b[i] == x) {
      return (void *)(b + i);
    }
  }

  return NULL;
}

int memcmp(const void *cs, const void *ct, usize count) {
  const unsigned char *su1, *su2;
  signed char res = 0;

  for (su1 = cs, su2 = ct; 0 < count; ++su1, ++su2, count--)
    if ((res = *su1 - *su2) != 0)
      break;
  return res;
}

int strcmp(char const *cs, char const *ct) {
  signed char __res;

  while (1) {
    if ((__res = *cs - *ct++) != 0 || !*cs++)
      break;
  }

  return __res;
}

char *strcpy(char *dest, char const *src) {
  char *tmp = dest;

  while ((*dest++ = *src++) != '\0')
    ;
  return tmp;
}

usize strlen(char const *s) {
  usize i;

  i = 0;
  while (s[i]) {
    i += 1;
  }

  return i;
}

char *strncat(char *dest, char const *src, usize count) {
  char *tmp = dest;

  if (count > 0) {
    while (*dest)
      dest++;
    while ((*dest++ = *src++)) {
      if (--count == 0) {
        *dest = '\0';
        break;
      }
    }
  }

  return tmp;
}

char *strncpy(char *dest, char const *src, usize count) {
  char *tmp = dest;

  usize i;
  for (i = 0; i++ < count && (*dest++ = *src++) != '\0';)
    ;
  for (; i < count; i++)
    *dest++ = '\0';

  return tmp;
}
