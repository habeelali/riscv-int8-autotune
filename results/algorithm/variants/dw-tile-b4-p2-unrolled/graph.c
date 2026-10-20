#include <stdint.h>
#include "constants.h"
#include "vectors.h"
typedef void (*step_fn)(const int8_t*,int8_t*);
static int8_t arena[2][36864], padded[38416];
extern void *memcpy(void*,const void*,unsigned);
extern void muriscv_nn_softmax_s8(const int8_t*,int32_t,int32_t,int32_t,int32_t,int32_t,int8_t*);
static void putc_uart(char c) { *(volatile unsigned char*)0x10000000=c; }
static void puts_uart(const char*s) {while(*s)putc_uart(*s++);}
static void number(uint32_t v) {char b[12];int n=0;do{b[n++]='0'+v%10;v/=10;}while(v);while(n)putc_uart(b[--n]);}
static void signed_number(int v) {if(v<0){putc_uart('-');v=-v;}number(v);}
extern void layer_0(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_0(const int8_t*i,int8_t*o){
for(unsigned y=0;y<97;y++)for(unsigned x=0;x<97;x++)for(unsigned c=0;c<1;c++){
unsigned a=y-0,b=x-0;
padded[(y*97+x)*1+c]=(a<96&&b<96)?i[(a*96+b)*1+c]:-1;}i=padded;
layer_0(i,weights_0,bias_0,mult_0,shift_0,o);}
extern void layer_1(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_1(const int8_t*i,int8_t*o){
for(unsigned y=0;y<50;y++)for(unsigned x=0;x<50;x++)for(unsigned c=0;c<8;c++){
unsigned a=y-1,b=x-1;
padded[(y*50+x)*8+c]=(a<48&&b<48)?i[(a*48+b)*8+c]:-128;}i=padded;
layer_1(i,weights_1,bias_1,mult_1,shift_1,o);}
extern void layer_2(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_2(const int8_t*i,int8_t*o){
layer_2(i,weights_2,bias_2,mult_2,shift_2,o);}
extern void layer_3(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_3(const int8_t*i,int8_t*o){
for(unsigned y=0;y<49;y++)for(unsigned x=0;x<49;x++)for(unsigned c=0;c<16;c++){
unsigned a=y-0,b=x-0;
padded[(y*49+x)*16+c]=(a<48&&b<48)?i[(a*48+b)*16+c]:-128;}i=padded;
layer_3(i,weights_3,bias_3,mult_3,shift_3,o);}
extern void layer_4(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_4(const int8_t*i,int8_t*o){
layer_4(i,weights_4,bias_4,mult_4,shift_4,o);}
extern void layer_5(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_5(const int8_t*i,int8_t*o){
for(unsigned y=0;y<26;y++)for(unsigned x=0;x<26;x++)for(unsigned c=0;c<32;c++){
unsigned a=y-1,b=x-1;
padded[(y*26+x)*32+c]=(a<24&&b<24)?i[(a*24+b)*32+c]:-128;}i=padded;
layer_5(i,weights_5,bias_5,mult_5,shift_5,o);}
extern void layer_6(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_6(const int8_t*i,int8_t*o){
layer_6(i,weights_6,bias_6,mult_6,shift_6,o);}
extern void layer_7(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_7(const int8_t*i,int8_t*o){
for(unsigned y=0;y<25;y++)for(unsigned x=0;x<25;x++)for(unsigned c=0;c<32;c++){
unsigned a=y-0,b=x-0;
padded[(y*25+x)*32+c]=(a<24&&b<24)?i[(a*24+b)*32+c]:-128;}i=padded;
layer_7(i,weights_7,bias_7,mult_7,shift_7,o);}
extern void layer_8(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_8(const int8_t*i,int8_t*o){
layer_8(i,weights_8,bias_8,mult_8,shift_8,o);}
extern void layer_9(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_9(const int8_t*i,int8_t*o){
for(unsigned y=0;y<14;y++)for(unsigned x=0;x<14;x++)for(unsigned c=0;c<64;c++){
unsigned a=y-1,b=x-1;
padded[(y*14+x)*64+c]=(a<12&&b<12)?i[(a*12+b)*64+c]:-128;}i=padded;
layer_9(i,weights_9,bias_9,mult_9,shift_9,o);}
extern void layer_10(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_10(const int8_t*i,int8_t*o){
layer_10(i,weights_10,bias_10,mult_10,shift_10,o);}
extern void layer_11(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_11(const int8_t*i,int8_t*o){
for(unsigned y=0;y<13;y++)for(unsigned x=0;x<13;x++)for(unsigned c=0;c<64;c++){
unsigned a=y-0,b=x-0;
padded[(y*13+x)*64+c]=(a<12&&b<12)?i[(a*12+b)*64+c]:-128;}i=padded;
layer_11(i,weights_11,bias_11,mult_11,shift_11,o);}
extern void layer_12(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_12(const int8_t*i,int8_t*o){
layer_12(i,weights_12,bias_12,mult_12,shift_12,o);}
extern void layer_13(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_13(const int8_t*i,int8_t*o){
for(unsigned y=0;y<8;y++)for(unsigned x=0;x<8;x++)for(unsigned c=0;c<128;c++){
unsigned a=y-1,b=x-1;
padded[(y*8+x)*128+c]=(a<6&&b<6)?i[(a*6+b)*128+c]:-128;}i=padded;
layer_13(i,weights_13,bias_13,mult_13,shift_13,o);}
extern void layer_14(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_14(const int8_t*i,int8_t*o){
layer_14(i,weights_14,bias_14,mult_14,shift_14,o);}
extern void layer_15(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_15(const int8_t*i,int8_t*o){
for(unsigned y=0;y<8;y++)for(unsigned x=0;x<8;x++)for(unsigned c=0;c<128;c++){
unsigned a=y-1,b=x-1;
padded[(y*8+x)*128+c]=(a<6&&b<6)?i[(a*6+b)*128+c]:-128;}i=padded;
layer_15(i,weights_15,bias_15,mult_15,shift_15,o);}
extern void layer_16(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_16(const int8_t*i,int8_t*o){
layer_16(i,weights_16,bias_16,mult_16,shift_16,o);}
extern void layer_17(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_17(const int8_t*i,int8_t*o){
for(unsigned y=0;y<8;y++)for(unsigned x=0;x<8;x++)for(unsigned c=0;c<128;c++){
unsigned a=y-1,b=x-1;
padded[(y*8+x)*128+c]=(a<6&&b<6)?i[(a*6+b)*128+c]:-128;}i=padded;
layer_17(i,weights_17,bias_17,mult_17,shift_17,o);}
extern void layer_18(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_18(const int8_t*i,int8_t*o){
layer_18(i,weights_18,bias_18,mult_18,shift_18,o);}
extern void layer_19(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_19(const int8_t*i,int8_t*o){
for(unsigned y=0;y<8;y++)for(unsigned x=0;x<8;x++)for(unsigned c=0;c<128;c++){
unsigned a=y-1,b=x-1;
padded[(y*8+x)*128+c]=(a<6&&b<6)?i[(a*6+b)*128+c]:-128;}i=padded;
layer_19(i,weights_19,bias_19,mult_19,shift_19,o);}
extern void layer_20(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_20(const int8_t*i,int8_t*o){
layer_20(i,weights_20,bias_20,mult_20,shift_20,o);}
extern void layer_21(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_21(const int8_t*i,int8_t*o){
for(unsigned y=0;y<8;y++)for(unsigned x=0;x<8;x++)for(unsigned c=0;c<128;c++){
unsigned a=y-1,b=x-1;
padded[(y*8+x)*128+c]=(a<6&&b<6)?i[(a*6+b)*128+c]:-128;}i=padded;
layer_21(i,weights_21,bias_21,mult_21,shift_21,o);}
extern void layer_22(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_22(const int8_t*i,int8_t*o){
layer_22(i,weights_22,bias_22,mult_22,shift_22,o);}
extern void layer_23(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_23(const int8_t*i,int8_t*o){
for(unsigned y=0;y<7;y++)for(unsigned x=0;x<7;x++)for(unsigned c=0;c<128;c++){
unsigned a=y-0,b=x-0;
padded[(y*7+x)*128+c]=(a<6&&b<6)?i[(a*6+b)*128+c]:-128;}i=padded;
layer_23(i,weights_23,bias_23,mult_23,shift_23,o);}
extern void layer_24(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_24(const int8_t*i,int8_t*o){
layer_24(i,weights_24,bias_24,mult_24,shift_24,o);}
extern void layer_25(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_25(const int8_t*i,int8_t*o){
for(unsigned y=0;y<5;y++)for(unsigned x=0;x<5;x++)for(unsigned c=0;c<256;c++){
unsigned a=y-1,b=x-1;
padded[(y*5+x)*256+c]=(a<3&&b<3)?i[(a*3+b)*256+c]:-128;}i=padded;
layer_25(i,weights_25,bias_25,mult_25,shift_25,o);}
extern void layer_26(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_26(const int8_t*i,int8_t*o){
layer_26(i,weights_26,bias_26,mult_26,shift_26,o);}
__attribute__((noinline)) static void step_27(const int8_t*i,int8_t*o){
for(unsigned c=0;c<256;c++){int sum=0;for(unsigned p=0;p<9;p++)sum+=i[p*256+c];o[c]=(sum+(sum>0?4:-4))/9;}}
extern void layer_28(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
__attribute__((noinline)) static void step_28(const int8_t*i,int8_t*o){
layer_28(i,weights_28,bias_28,mult_28,shift_28,o);}
__attribute__((noinline)) static void step_29(const int8_t*i,int8_t*o){memcpy(o,i,2);}
__attribute__((noinline)) static void step_30(const int8_t*i,int8_t*o){muriscv_nn_softmax_s8(i,1,2,1720564096,20,-1984,o);}
static step_fn steps[]={step_0,step_1,step_2,step_3,step_4,step_5,step_6,step_7,step_8,step_9,step_10,step_11,step_12,step_13,step_14,step_15,step_16,step_17,step_18,step_19,step_20,step_21,step_22,step_23,step_24,step_25,step_26,step_27,step_28,step_29,step_30};
static const unsigned lengths[]={18432,18432,36864,9216,18432,18432,18432,4608,9216,9216,9216,2304,4608,4608,4608,4608,4608,4608,4608,4608,4608,4608,4608,1152,2304,2304,2304,256,2,2,2};

__attribute__((noinline)) static void empty_step(const int8_t*i,int8_t*o){asm volatile("" ::: "memory");}
__attribute__((noinline)) static void empty_model(const int8_t*i){asm volatile("" ::: "memory");}
__attribute__((noinline)) static void model(const int8_t*i){
for(unsigned j=0;j<31;j++){int8_t*o=arena[j&1];steps[j](i,o);i=o;}}
__attribute__((noinline)) static uint32_t measure_model(void(*f)(const int8_t*),const int8_t*i){
uint32_t a,b;asm volatile("rdinstret %0":"=r"(a)::"memory");f(i);asm volatile("rdinstret %0":"=r"(b)::"memory");return b-a;}
__attribute__((noinline)) static uint32_t measure_step(step_fn f,const int8_t*i,int8_t*o){
uint32_t a,b;asm volatile("rdinstret %0":"=r"(a)::"memory");f(i,o);asm volatile("rdinstret %0":"=r"(b)::"memory");return b-a;}
extern unsigned char __stack_bottom[],__stack_top[];
int main(void){
for(unsigned sample=0;sample<4;sample++){
uint32_t sp;asm volatile("mv %0,sp":"=r"(sp));
for(unsigned char*p=__stack_bottom;p<(unsigned char*)(sp-256);p++)*p=0xa5;
uint32_t overhead=measure_model(empty_model,images[sample]);
uint32_t raw=measure_model(model,images[sample]);
unsigned char*p=__stack_bottom;while(p<(unsigned char*)(sp-256)&&*p==0xa5)p++;
uint32_t stack=(uint32_t)(__stack_top-p);
int8_t timed_a=arena[0][0],timed_b=arena[0][1];
uint32_t mismatches=(timed_a!=oracles[sample][30][0])+(timed_b!=oracles[sample][30][1]);const int8_t*i=images[sample];
for(unsigned j=0;j<31;j++){int8_t*o=arena[j&1];
uint32_t empty=measure_step(empty_step,i,o),count=measure_step(steps[j],i,o),wrong=0;
for(unsigned n=0;n<lengths[j];n++)wrong+=o[n]!=oracles[sample][j][n];
mismatches+=wrong;
puts_uart("LAYER ");number(sample);putc_uart(' ');number(j);putc_uart(' ');number(count-empty);putc_uart(' ');number(wrong);putc_uart(10);i=o;}
puts_uart("MODEL ");number(sample);putc_uart(' ');number(raw);putc_uart(' ');number(overhead);putc_uart(' ');number(mismatches);putc_uart(' ');number(stack);
putc_uart(' ');signed_number(i[0]);putc_uart(' ');signed_number(i[1]);putc_uart(10);
}return 0;}
