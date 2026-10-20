#include <stdint.h>
int8_t activations[73728],padded[38416];
extern void resnet_execute(unsigned,unsigned,unsigned);
static void put(char c){*(volatile unsigned char*)0x10000000=c;}
static void num(uint32_t x){char s[12];unsigned n=0;do{s[n++]='0'+x%10;x/=10;}while(x);while(n)put(s[--n]);}
static void str(const char*s){while(*s)put(*s++);}
static unsigned wrong=0;
void record_check(unsigned model,unsigned b,unsigned sample,unsigned layer,const int8_t*out,const int8_t*expected,unsigned n){unsigned w=0;for(unsigned i=0;i<n;i++)w+=out[i]!=expected[i];wrong+=w;str("CHECK ");num(sample);put(' ');num(layer);put(' ');num(w);put(10);}
__attribute__((noinline)) static uint32_t measure(unsigned s){uint32_t a,b;asm volatile("rdinstret %0":"=r"(a)::"memory");resnet_execute(0,s,0);asm volatile("rdinstret %0":"=r"(b)::"memory");return b-a;}
int main(void){for(unsigned s=0;s<4;s++){resnet_execute(0,s,1);uint32_t n=measure(s);str("MODEL ");num(s);put(' ');num(n);put(' ');num(wrong);put(10);}return 0;}
