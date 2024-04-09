#include <stdint.h>
// #include <stddef.h>

#define nop asm volatile("nop")

#ifdef _cplusplus
#define NULL 0
#else
#define NULL (void*)0
#endif

#ifndef __SIZE_TYPE__
#define __SIZE_TYPE__ long unsigned int
#endif
// #if !(defined (__GNUG__) && defined (size_t))
typedef __SIZE_TYPE__ size_t;


int utils_string_compare(const char* i, const char* j);
unsigned long utils_atoi(const char *s, int char_size);
void utils_align(void *size, unsigned int s);
uint32_t utils_align_up(uint32_t size, int alignment);
size_t utils_strlen(const char *s);
char *utils_strcpy(char* dst, const char *src);
char *utils_strdup(const char *src);
