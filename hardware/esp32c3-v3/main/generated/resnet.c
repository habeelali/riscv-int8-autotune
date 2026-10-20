#include <stdint.h>
#include <string.h>
#include "constants.h"
#include "vectors.h"
#include "r_tuned.h"
extern int8_t activations[73728],padded[38416];
extern void record_check(unsigned,unsigned,unsigned,unsigned,const int8_t*,const int8_t*,unsigned);
extern void muriscv_nn_softmax_s8(const int8_t*,int32_t,int32_t,int32_t,int32_t,int32_t,int8_t*);
extern int muriscv_nn_elementwise_add_s8(const int8_t*,const int8_t*,int,int,int,int,int,int,int,int8_t*,int,int,int,int,int,int);
extern void r_esp_nn_layer_0(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_scalar_layer_0(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_fixed_layer_0(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_rust_layer_0(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_esp_nn_layer_1(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_scalar_layer_1(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_fixed_layer_1(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_rust_layer_1(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_esp_nn_layer_2(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_scalar_layer_2(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_fixed_layer_2(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_rust_layer_2(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_esp_nn_layer_4(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_scalar_layer_4(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_fixed_layer_4(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_rust_layer_4(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_esp_nn_layer_5(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_scalar_layer_5(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_fixed_layer_5(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_rust_layer_5(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_esp_nn_layer_6(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_scalar_layer_6(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_fixed_layer_6(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_rust_layer_6(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_esp_nn_layer_8(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_scalar_layer_8(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_fixed_layer_8(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_rust_layer_8(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_esp_nn_layer_9(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_scalar_layer_9(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_fixed_layer_9(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_rust_layer_9(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_esp_nn_layer_10(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_scalar_layer_10(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_fixed_layer_10(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_rust_layer_10(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_esp_nn_layer_14(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_scalar_layer_14(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_mu_fixed_layer_14(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
extern void r_rust_layer_14(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
typedef void (*r_fn)(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
static const r_fn r_kernels[4][16]={{r_esp_nn_layer_0,r_esp_nn_layer_1,r_esp_nn_layer_2,0,r_esp_nn_layer_4,r_esp_nn_layer_5,r_esp_nn_layer_6,0,r_esp_nn_layer_8,r_esp_nn_layer_9,r_esp_nn_layer_10,0,0,0,r_esp_nn_layer_14,0},{r_mu_scalar_layer_0,r_mu_scalar_layer_1,r_mu_scalar_layer_2,0,r_mu_scalar_layer_4,r_mu_scalar_layer_5,r_mu_scalar_layer_6,0,r_mu_scalar_layer_8,r_mu_scalar_layer_9,r_mu_scalar_layer_10,0,0,0,r_mu_scalar_layer_14,0},{r_mu_fixed_layer_0,r_mu_fixed_layer_1,r_mu_fixed_layer_2,0,r_mu_fixed_layer_4,r_mu_fixed_layer_5,r_mu_fixed_layer_6,0,r_mu_fixed_layer_8,r_mu_fixed_layer_9,r_mu_fixed_layer_10,0,0,0,r_mu_fixed_layer_14,0},{r_rust_layer_0,r_rust_layer_1,r_rust_layer_2,0,r_rust_layer_4,r_rust_layer_5,r_rust_layer_6,0,r_rust_layer_8,r_rust_layer_9,r_rust_layer_10,0,0,0,r_rust_layer_14,0}};
void resnet_execute(unsigned backend,unsigned sample,unsigned check){
{
const int8_t*i=r_images[sample];int8_t*o=activations+0;
r_kernels[backend][0](i,(backend==3?t_weights_0:r_weights_0),(backend==3?t_bias_0:r_bias_0),(backend==3?t_mult_0:r_mult_0),(backend==3?t_shift_0:r_shift_0),o);
if(check)record_check(1,backend,sample,0,o,r_oracles[sample][0],16384);
}
{
const int8_t*i=activations+0;int8_t*o=activations+16384;
r_kernels[backend][1](i,(backend==3?t_weights_1:r_weights_1),(backend==3?t_bias_1:r_bias_1),(backend==3?t_mult_1:r_mult_1),(backend==3?t_shift_1:r_shift_1),o);
if(check)record_check(1,backend,sample,1,o,r_oracles[sample][1],16384);
}
{
const int8_t*i=activations+16384;int8_t*o=activations+32768;
r_kernels[backend][2](i,(backend==3?t_weights_2:r_weights_2),(backend==3?t_bias_2:r_bias_2),(backend==3?t_mult_2:r_mult_2),(backend==3?t_shift_2:r_shift_2),o);
if(check)record_check(1,backend,sample,2,o,r_oracles[sample][2],16384);
}
{
const int8_t*i=activations+0;int8_t*o=activations+16384;
muriscv_nn_elementwise_add_s8(i,activations+32768,128,1623821475,-2,-4,1073741824,0,20,o,-128,1098017566,-17,-128,127,16384);
if(check)record_check(1,backend,sample,3,o,r_oracles[sample][3],16384);
}
{
const int8_t*i=activations+16384;int8_t*o=activations+0;
for(unsigned y=0;y<33;y++)for(unsigned x=0;x<33;x++)for(unsigned c=0;c<16;c++){unsigned a=y-0,b=x-0;padded[(y*33+x)*16+c]=(a<32&&b<32)?i[(a*32+b)*16+c]:-128;}
i=padded;
r_kernels[backend][4](i,(backend==3?t_weights_4:r_weights_4),(backend==3?t_bias_4:r_bias_4),(backend==3?t_mult_4:r_mult_4),(backend==3?t_shift_4:r_shift_4),o);
if(check)record_check(1,backend,sample,4,o,r_oracles[sample][4],8192);
}
{
const int8_t*i=activations+0;int8_t*o=activations+8192;
r_kernels[backend][5](i,(backend==3?t_weights_5:r_weights_5),(backend==3?t_bias_5:r_bias_5),(backend==3?t_mult_5:r_mult_5),(backend==3?t_shift_5:r_shift_5),o);
if(check)record_check(1,backend,sample,5,o,r_oracles[sample][5],8192);
}
{
const int8_t*i=activations+16384;int8_t*o=activations+0;
r_kernels[backend][6](i,(backend==3?t_weights_6:r_weights_6),(backend==3?t_bias_6:r_bias_6),(backend==3?t_mult_6:r_mult_6),(backend==3?t_shift_6:r_shift_6),o);
if(check)record_check(1,backend,sample,6,o,r_oracles[sample][6],8192);
}
{
const int8_t*i=activations+0;int8_t*o=activations+16384;
muriscv_nn_elementwise_add_s8(i,activations+8192,17,1699529983,-2,-4,1073741824,0,20,o,-128,1140768826,-17,-128,127,8192);
if(check)record_check(1,backend,sample,7,o,r_oracles[sample][7],8192);
}
{
const int8_t*i=activations+16384;int8_t*o=activations+0;
for(unsigned y=0;y<17;y++)for(unsigned x=0;x<17;x++)for(unsigned c=0;c<32;c++){unsigned a=y-0,b=x-0;padded[(y*17+x)*32+c]=(a<16&&b<16)?i[(a*16+b)*32+c]:-128;}
i=padded;
r_kernels[backend][8](i,(backend==3?t_weights_8:r_weights_8),(backend==3?t_bias_8:r_bias_8),(backend==3?t_mult_8:r_mult_8),(backend==3?t_shift_8:r_shift_8),o);
if(check)record_check(1,backend,sample,8,o,r_oracles[sample][8],4096);
}
{
const int8_t*i=activations+0;int8_t*o=activations+4096;
r_kernels[backend][9](i,(backend==3?t_weights_9:r_weights_9),(backend==3?t_bias_9:r_bias_9),(backend==3?t_mult_9:r_mult_9),(backend==3?t_shift_9:r_shift_9),o);
if(check)record_check(1,backend,sample,9,o,r_oracles[sample][9],4096);
}
{
const int8_t*i=activations+16384;int8_t*o=activations+0;
r_kernels[backend][10](i,(backend==3?t_weights_10:r_weights_10),(backend==3?t_bias_10:r_bias_10),(backend==3?t_mult_10:r_mult_10),(backend==3?t_shift_10:r_shift_10),o);
if(check)record_check(1,backend,sample,10,o,r_oracles[sample][10],4096);
}
{
const int8_t*i=activations+0;int8_t*o=activations+8192;
muriscv_nn_elementwise_add_s8(i,activations+4096,-38,1657902019,-2,2,1073741824,0,20,o,-128,1835721671,-18,-128,127,4096);
if(check)record_check(1,backend,sample,11,o,r_oracles[sample][11],4096);
}
{
const int8_t*i=activations+8192;int8_t*o=activations+0;
for(unsigned c=0;c<64;c++){int sum=0;for(unsigned p=0;p<64;p++)sum+=i[p*64+c];o[c]=(sum+(sum>0?32:-32))/64;}
if(check)record_check(1,backend,sample,12,o,r_oracles[sample][12],64);
}
{
const int8_t*i=activations+0;int8_t*o=activations+64;
memcpy(o,i,64);
if(check)record_check(1,backend,sample,13,o,r_oracles[sample][13],64);
}
{
const int8_t*i=activations+64;int8_t*o=activations+0;
r_kernels[backend][14](i,(backend==3?t_weights_14:r_weights_14),(backend==3?t_bias_14:r_bias_14),(backend==3?t_mult_14:r_mult_14),(backend==3?t_shift_14:r_shift_14),o);
if(check)record_check(1,backend,sample,14,o,r_oracles[sample][14],10);
}
{
const int8_t*i=activations+0;int8_t*o=activations+16;
muriscv_nn_softmax_s8(i,1,10,1476210432,24,-124,o);
if(check)record_check(1,backend,sample,15,o,r_oracles[sample][15],10);
}
}
const int8_t*resnet_output(void){return activations+16;}

const int8_t*resnet_expected(unsigned s){return r_oracles[s][15];}
