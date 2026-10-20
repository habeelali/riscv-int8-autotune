#include <stdint.h>
#include <esp_nn_ansi_headers.h>
void r_kernel_layer_0(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={32,32,3,1}, fd={3,3,3,1}, od={32,32,16,1};
conv_params_t p={128,-128,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_kernel_layer_1(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={32,32,16,1}, fd={3,3,16,1}, od={32,32,16,1};
conv_params_t p={128,-128,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_kernel_layer_2(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={32,32,16,1}, fd={3,3,16,1}, od={32,32,16,1};
conv_params_t p={128,4,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_kernel_layer_4(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={33,33,16,1}, fd={3,3,16,1}, od={16,16,32,1};
conv_params_t p={128,-128,{2,2},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_kernel_layer_5(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={16,16,32,1}, fd={3,3,32,1}, od={16,16,32,1};
conv_params_t p={128,4,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_kernel_layer_6(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={32,32,16,1}, fd={1,1,16,1}, od={16,16,32,1};
conv_params_t p={128,-17,{2,2},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_kernel_layer_8(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={17,17,32,1}, fd={3,3,32,1}, od={8,8,64,1};
conv_params_t p={128,-128,{2,2},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_kernel_layer_9(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={8,8,64,1}, fd={3,3,64,1}, od={8,8,64,1};
conv_params_t p={128,-2,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_kernel_layer_10(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={16,16,32,1}, fd={1,1,32,1}, od={8,8,64,1};
conv_params_t p={128,38,{2,2},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_kernel_layer_14(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
esp_nn_fully_connected_per_ch_s8_ansi(i,128,64,w,0,b,o,10,24,s,m,-128,127);}
