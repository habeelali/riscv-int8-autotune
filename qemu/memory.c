/* SPDX-License-Identifier: MIT
 * Common freestanding memory routines for every implementation in the harness.
 * Scalar aligned 32-bit copies/fills, 16-byte loop body, byte tails. */
#include <stdint.h>
#include <stddef.h>
typedef uint32_t alias_u32 __attribute__((__may_alias__));
void *memcpy(void *dest, const void *src, size_t n) {
    unsigned char *d=dest; const unsigned char *s=src;
    if ((((uintptr_t)d ^ (uintptr_t)s) & 3u)==0) {
        while(n && ((uintptr_t)d & 3u)){*d++=*s++;n--;}
        while(n>=16){
            ((alias_u32*)d)[0]=((const alias_u32*)s)[0];
            ((alias_u32*)d)[1]=((const alias_u32*)s)[1];
            ((alias_u32*)d)[2]=((const alias_u32*)s)[2];
            ((alias_u32*)d)[3]=((const alias_u32*)s)[3];
            d+=16;s+=16;n-=16;
        }
        while(n>=4){*(alias_u32*)d=*(const alias_u32*)s;d+=4;s+=4;n-=4;}
    }
    while(n--){*d++=*s++;}
    return dest;
}
void *memset(void *dest,int value,size_t n) {
    unsigned char *d=dest; const unsigned char v=(unsigned char)value;
    while(n && ((uintptr_t)d & 3u)){*d++=v;n--;}
    const uint32_t word=(uint32_t)v*0x01010101u;
    while(n>=16){
        ((alias_u32*)d)[0]=word;((alias_u32*)d)[1]=word;
        ((alias_u32*)d)[2]=word;((alias_u32*)d)[3]=word;
        d+=16;n-=16;
    }
    while(n>=4){*(alias_u32*)d=word;d+=4;n-=4;}
    while(n--){*d++=v;}
    return dest;
}
