#ifndef __STRING_H__
#define __STRING_H__
#include "common.h"
#include "stdtypes.h"

int strncmp(char const *, char const *, usize);
char *strcpy(char *, char const *);
usize strlen(char const *);
char *strncat(char *, char const *, usize);
char *strncpy(char *, char const *, usize);

void *memchr(void const *, int, usize);
int memcmp(void const *, const void *, usize);
void *memcpy(void *, void const *, usize);
void *memset(void *, int, usize);
#endif //__STRING_H__