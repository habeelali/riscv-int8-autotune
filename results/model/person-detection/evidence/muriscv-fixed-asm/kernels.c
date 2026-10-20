#include "muriscv_nn_functions.h"
// Scratch: 48 bytes (local stack, included in watermark).
void layer_0(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[12];
muriscv_nn_context ctx={ scratch, 48 };
muriscv_nn_dims id={1,97,97,1}, fd={8,3,3,1}, bd={1,1,1,8}, od={1,48,48,8};
muriscv_nn_conv_params p={1,-128,{2,2},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_convolve_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_1(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,48,48,8}, fd={8,3,3,8}, bd={1,1,1,8}, od={1,48,48,8};
muriscv_nn_dw_conv_params p={128,-128,1,{1,1},{1,1},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_depthwise_conv_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_2(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,48,48,8}, fd={16,1,1,8}, bd={1,1,1,16}, od={1,48,48,16};
muriscv_nn_conv_params p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_convolve_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_3(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,49,49,16}, fd={16,3,3,16}, bd={1,1,1,16}, od={1,24,24,16};
muriscv_nn_dw_conv_params p={128,-128,1,{2,2},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_depthwise_conv_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_4(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,24,24,16}, fd={32,1,1,16}, bd={1,1,1,32}, od={1,24,24,32};
muriscv_nn_conv_params p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_convolve_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_5(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,24,24,32}, fd={32,3,3,32}, bd={1,1,1,32}, od={1,24,24,32};
muriscv_nn_dw_conv_params p={128,-128,1,{1,1},{1,1},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_depthwise_conv_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_6(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,24,24,32}, fd={32,1,1,32}, bd={1,1,1,32}, od={1,24,24,32};
muriscv_nn_conv_params p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_convolve_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_7(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,25,25,32}, fd={32,3,3,32}, bd={1,1,1,32}, od={1,12,12,32};
muriscv_nn_dw_conv_params p={128,-128,1,{2,2},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_depthwise_conv_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_8(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,12,12,32}, fd={64,1,1,32}, bd={1,1,1,64}, od={1,12,12,64};
muriscv_nn_conv_params p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_convolve_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_9(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,12,12,64}, fd={64,3,3,64}, bd={1,1,1,64}, od={1,12,12,64};
muriscv_nn_dw_conv_params p={128,-128,1,{1,1},{1,1},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_depthwise_conv_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_10(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,12,12,64}, fd={64,1,1,64}, bd={1,1,1,64}, od={1,12,12,64};
muriscv_nn_conv_params p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_convolve_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_11(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,13,13,64}, fd={64,3,3,64}, bd={1,1,1,64}, od={1,6,6,64};
muriscv_nn_dw_conv_params p={128,-128,1,{2,2},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_depthwise_conv_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_12(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,6,6,64}, fd={128,1,1,64}, bd={1,1,1,128}, od={1,6,6,128};
muriscv_nn_conv_params p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_convolve_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_13(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,6,6,128}, fd={128,3,3,128}, bd={1,1,1,128}, od={1,6,6,128};
muriscv_nn_dw_conv_params p={128,-128,1,{1,1},{1,1},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_depthwise_conv_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_14(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,6,6,128}, fd={128,1,1,128}, bd={1,1,1,128}, od={1,6,6,128};
muriscv_nn_conv_params p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_convolve_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_15(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,6,6,128}, fd={128,3,3,128}, bd={1,1,1,128}, od={1,6,6,128};
muriscv_nn_dw_conv_params p={128,-128,1,{1,1},{1,1},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_depthwise_conv_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_16(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,6,6,128}, fd={128,1,1,128}, bd={1,1,1,128}, od={1,6,6,128};
muriscv_nn_conv_params p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_convolve_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_17(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,6,6,128}, fd={128,3,3,128}, bd={1,1,1,128}, od={1,6,6,128};
muriscv_nn_dw_conv_params p={128,-128,1,{1,1},{1,1},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_depthwise_conv_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_18(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,6,6,128}, fd={128,1,1,128}, bd={1,1,1,128}, od={1,6,6,128};
muriscv_nn_conv_params p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_convolve_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_19(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,6,6,128}, fd={128,3,3,128}, bd={1,1,1,128}, od={1,6,6,128};
muriscv_nn_dw_conv_params p={128,-128,1,{1,1},{1,1},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_depthwise_conv_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_20(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,6,6,128}, fd={128,1,1,128}, bd={1,1,1,128}, od={1,6,6,128};
muriscv_nn_conv_params p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_convolve_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_21(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,6,6,128}, fd={128,3,3,128}, bd={1,1,1,128}, od={1,6,6,128};
muriscv_nn_dw_conv_params p={128,-128,1,{1,1},{1,1},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_depthwise_conv_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_22(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,6,6,128}, fd={128,1,1,128}, bd={1,1,1,128}, od={1,6,6,128};
muriscv_nn_conv_params p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_convolve_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_23(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,7,7,128}, fd={128,3,3,128}, bd={1,1,1,128}, od={1,3,3,128};
muriscv_nn_dw_conv_params p={128,-128,1,{2,2},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_depthwise_conv_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_24(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,3,3,128}, fd={256,1,1,128}, bd={1,1,1,256}, od={1,3,3,256};
muriscv_nn_conv_params p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_convolve_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_25(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,3,3,256}, fd={256,3,3,256}, bd={1,1,1,256}, od={1,3,3,256};
muriscv_nn_dw_conv_params p={128,-128,1,{1,1},{1,1},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_depthwise_conv_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_26(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,3,3,256}, fd={256,1,1,256}, bd={1,1,1,256}, od={1,3,3,256};
muriscv_nn_conv_params p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_convolve_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
#include "muriscv_nn_functions.h"
// Scratch: 0 bytes (local stack, included in watermark).
void layer_28(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
uint32_t scratch[1];
muriscv_nn_context ctx={ 0, 0 };
muriscv_nn_dims id={1,1,1,256}, fd={2,1,1,256}, bd={1,1,1,2}, od={1,1,1,2};
muriscv_nn_conv_params p={128,-1,{1,1},{0,0},{1,1},{-128,127}};
muriscv_nn_per_channel_quant_params q={(int32_t*)m,(int32_t*)s};
int status=muriscv_nn_convolve_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }
}
