#include <stdint.h>
extern void hw_backend_error(int);
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_0(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={97,97,1,1}, fd={3,3,1,1}, od={48,48,8,1};
dw_conv_params_t p={1,-128,8,{2,2},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_depthwise_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_1(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={48,48,8,1}, fd={3,3,8,1}, od={48,48,8,1};
dw_conv_params_t p={128,-128,1,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_depthwise_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_2(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={48,48,8,1}, fd={1,1,8,1}, od={48,48,16,1};
conv_params_t p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_3(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={49,49,16,1}, fd={3,3,16,1}, od={24,24,16,1};
dw_conv_params_t p={128,-128,1,{2,2},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_depthwise_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_4(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={24,24,16,1}, fd={1,1,16,1}, od={24,24,32,1};
conv_params_t p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_5(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={24,24,32,1}, fd={3,3,32,1}, od={24,24,32,1};
dw_conv_params_t p={128,-128,1,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_depthwise_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_6(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={24,24,32,1}, fd={1,1,32,1}, od={24,24,32,1};
conv_params_t p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_7(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={25,25,32,1}, fd={3,3,32,1}, od={12,12,32,1};
dw_conv_params_t p={128,-128,1,{2,2},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_depthwise_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_8(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={12,12,32,1}, fd={1,1,32,1}, od={12,12,64,1};
conv_params_t p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_9(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={12,12,64,1}, fd={3,3,64,1}, od={12,12,64,1};
dw_conv_params_t p={128,-128,1,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_depthwise_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_10(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={12,12,64,1}, fd={1,1,64,1}, od={12,12,64,1};
conv_params_t p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_11(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={13,13,64,1}, fd={3,3,64,1}, od={6,6,64,1};
dw_conv_params_t p={128,-128,1,{2,2},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_depthwise_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_12(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={6,6,64,1}, fd={1,1,64,1}, od={6,6,128,1};
conv_params_t p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_13(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={6,6,128,1}, fd={3,3,128,1}, od={6,6,128,1};
dw_conv_params_t p={128,-128,1,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_depthwise_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_14(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={6,6,128,1}, fd={1,1,128,1}, od={6,6,128,1};
conv_params_t p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_15(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={6,6,128,1}, fd={3,3,128,1}, od={6,6,128,1};
dw_conv_params_t p={128,-128,1,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_depthwise_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_16(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={6,6,128,1}, fd={1,1,128,1}, od={6,6,128,1};
conv_params_t p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_17(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={6,6,128,1}, fd={3,3,128,1}, od={6,6,128,1};
dw_conv_params_t p={128,-128,1,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_depthwise_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_18(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={6,6,128,1}, fd={1,1,128,1}, od={6,6,128,1};
conv_params_t p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_19(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={6,6,128,1}, fd={3,3,128,1}, od={6,6,128,1};
dw_conv_params_t p={128,-128,1,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_depthwise_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_20(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={6,6,128,1}, fd={1,1,128,1}, od={6,6,128,1};
conv_params_t p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_21(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={6,6,128,1}, fd={3,3,128,1}, od={6,6,128,1};
dw_conv_params_t p={128,-128,1,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_depthwise_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_22(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={6,6,128,1}, fd={1,1,128,1}, od={6,6,128,1};
conv_params_t p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_23(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={7,7,128,1}, fd={3,3,128,1}, od={3,3,128,1};
dw_conv_params_t p={128,-128,1,{2,2},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_depthwise_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_24(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={3,3,128,1}, fd={1,1,128,1}, od={3,3,256,1};
conv_params_t p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_25(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={3,3,256,1}, fd={3,3,256,1}, od={3,3,256,1};
dw_conv_params_t p={128,-128,1,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_depthwise_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_26(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={3,3,256,1}, fd={1,1,256,1}, od={3,3,256,1};
conv_params_t p={128,-128,{1,1},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void p_esp_nn_layer_28(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={1,1,256,1}, fd={1,1,256,1}, od={1,1,2,1};
conv_params_t p={128,-1,{1,1},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_esp_nn_layer_0(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={32,32,3,1}, fd={3,3,3,1}, od={32,32,16,1};
conv_params_t p={128,-128,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_esp_nn_layer_1(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={32,32,16,1}, fd={3,3,16,1}, od={32,32,16,1};
conv_params_t p={128,-128,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_esp_nn_layer_2(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={32,32,16,1}, fd={3,3,16,1}, od={32,32,16,1};
conv_params_t p={128,4,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_esp_nn_layer_4(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={33,33,16,1}, fd={3,3,16,1}, od={16,16,32,1};
conv_params_t p={128,-128,{2,2},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_esp_nn_layer_5(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={16,16,32,1}, fd={3,3,32,1}, od={16,16,32,1};
conv_params_t p={128,4,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_esp_nn_layer_6(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={32,32,16,1}, fd={1,1,16,1}, od={16,16,32,1};
conv_params_t p={128,-17,{2,2},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_esp_nn_layer_8(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={17,17,32,1}, fd={3,3,32,1}, od={8,8,64,1};
conv_params_t p={128,-128,{2,2},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_esp_nn_layer_9(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={8,8,64,1}, fd={3,3,64,1}, od={8,8,64,1};
conv_params_t p={128,-2,{1,1},{1,1},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_esp_nn_layer_10(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
data_dims_t id={16,16,32,1}, fd={1,1,32,1}, od={8,8,64,1};
conv_params_t p={128,38,{2,2},{0,0},{1,1},{-128,127}};
quant_data_t q={(int32_t*)s,(int32_t*)m};
esp_nn_conv_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}
#include <esp_nn_ansi_headers.h>
void r_esp_nn_layer_14(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {
esp_nn_fully_connected_per_ch_s8_ansi(i,128,64,w,0,b,o,10,24,s,m,-128,127);}
