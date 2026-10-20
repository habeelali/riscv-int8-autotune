#include <stdint.h>
#include <stddef.h>
#include "data.h"
extern void kernel(const int8_t*, const int8_t*, const int32_t*, const int32_t*, const int32_t*, int8_t*);
static int8_t output[OUT_LEN];
static void putc_uart(char c) { *(volatile unsigned char*)0x10000000=c; }
static void puts_uart(const char *s) { while(*s)putc_uart(*s++); }
static void number(uint32_t v) { char b[12]; int n=0; do{b[n++]='0'+v%10; v/=10;}while(v); while(n)putc_uart(b[--n]); }
__attribute__((noinline)) static void empty(const int8_t*a,const int8_t*b,const int32_t*c,const int32_t*d,const int32_t*e,int8_t*f) { asm volatile("" ::: "memory"); }
__attribute__((noinline)) static uint32_t measure(void (*f)(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*)) {
 uint32_t a,b; asm volatile("rdinstret %0":"=r"(a)::"memory");
 f(input,weights,bias,mult,shift,output);
 asm volatile("rdinstret %0":"=r"(b)::"memory"); return b-a;
}
__attribute__((noinline)) static uint32_t calibration(unsigned n) {
 uint32_t a,b; asm volatile("rdinstret %0":"=r"(a)::"memory");
 asm volatile("1: addi %0,%0,-1; bnez %0,1b":"+r"(n)::"memory");
 asm volatile("rdinstret %0":"=r"(b)::"memory"); return b-a;
}
extern unsigned char __stack_bottom[],__stack_top[];
int main(void) {
 uint32_t sp; asm volatile("mv %0,sp":"=r"(sp));
 for(unsigned char *p=__stack_bottom;p<(unsigned char*)(sp-256);p++)*p=0xa5;
 uint32_t overhead=measure(empty), raw=measure(kernel);
 unsigned char *p=__stack_bottom; while(p<(unsigned char*)(sp-256)&&*p==0xa5)p++;
 uint32_t stack=(uint32_t)(__stack_top-p);
 uint32_t mismatch=0,hash=2166136261u;
 for(int i=0;i<OUT_LEN;i++){mismatch+=output[i]!=expected[i];hash=(hash^(uint8_t)output[i])*16777619u;}
 puts_uart("RESULT ");number(raw);putc_uart(' ');number(overhead);putc_uart(' ');number(mismatch);putc_uart(' ');number(hash);putc_uart(' ');number(stack);putc_uart(' ');number(calibration(1000));putc_uart(' ');number(calibration(2000));putc_uart('\n');
 return 0;
}
