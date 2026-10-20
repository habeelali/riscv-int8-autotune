#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <inttypes.h>
#include <ctype.h>
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "esp_cpu.h"
#include "esp_timer.h"
#include "esp_system.h"
#include "esp_random.h"
#include "esp_attr.h"
#include "esp_rom_sys.h"
#include "esp_private/esp_clk.h"
#include "driver/usb_serial_jtag.h"
#include "driver/uart.h"

#define BUILD_ID "@BUILD_ID@"
int8_t activations[73728],padded[38416];
extern void person_execute(unsigned,unsigned,unsigned);
extern void resnet_execute(unsigned,unsigned,unsigned);
extern const int8_t*person_output(void);
extern const int8_t*resnet_output(void);
extern const int8_t*person_expected(unsigned);
extern const int8_t*resnet_expected(unsigned);
RTC_NOINIT_ATTR static uint32_t boot_counter;
static char challenge[33];
static unsigned mismatches=0,checks=0,run_count=0,cal_count=0,next_seq=1;
static uint32_t seen[2][4][4];
static inline uint32_t ticks(void){asm volatile("":::"memory");uint32_t t=esp_cpu_get_cycle_count();asm volatile("":::"memory");return t;}
static uint32_t fingerprint(const int8_t*p,unsigned n){uint32_t h=2166136261u;for(unsigned j=0;j<n;j++)h=(h^(uint8_t)p[j])*16777619u;return h;}
void hw_backend_error(int s){printf("V5_ERROR {\"fatal_backend_status\":%d}\n",s);fflush(stdout);abort();}
void hw_rust_panic(void){puts("V5_ERROR {\"rust_panic\":true}");fflush(stdout);abort();}
static void execute(unsigned model,unsigned backend,unsigned sample,unsigned check){if(model==0)person_execute(backend,sample,check);else resnet_execute(backend,sample,check);}
static unsigned output_check(unsigned model,unsigned sample){const int8_t*a=model?resnet_output():person_output();const int8_t*b=model?resnet_expected(sample):person_expected(sample);unsigned n=model?10:2,w=0;for(unsigned j=0;j<n;j++)w+=a[j]!=b[j];return w;}
void record_check(unsigned model,unsigned backend,unsigned sample,unsigned layer,const int8_t*out,const int8_t*expected,unsigned n){
unsigned w=0;for(unsigned j=0;j<n;j++)w+=out[j]!=expected[j];mismatches+=w;checks++;
printf("V5_CHECK {\"challenge\":\"%s\",\"model\":%u,\"backend\":%u,\"sample\":%u,\"layer\":%u,\"bytes\":%u,\"mismatches\":%u,\"output_hash\":%"PRIu32",\"expected_hash\":%"PRIu32"}\n",challenge,model,backend,sample,layer,n,w,fingerprint(out,n),fingerprint(expected,n));
fflush(stdout);vTaskDelay(1);
}
static bool token_ok(const char*s){return strlen(s)==32&&strcmp(s,challenge)==0;}
static void reject(const char*s){printf("V5_ERROR {\"protocol\":\"%s\"}\n",s);fflush(stdout);}
// Native USB is the user's connection; UART0 provides the identical protocol for QEMU/adapters.
static int read_line(char*out,unsigned cap){unsigned n=0;for(;;){char c;int r=usb_serial_jtag_read_bytes(&c,1,0);if(!r)r=uart_read_bytes(UART_NUM_0,&c,1,1);if(r>0){if(c=='\r')continue;if(c=='\n'){out[n]=0;return n;}if(n+1>=cap){reject("line_too_long");n=0;}else out[n++]=c;}else vTaskDelay(1);}}
static void worker(void*unused){
usb_serial_jtag_driver_config_t usj={.tx_buffer_size=2048,.rx_buffer_size=2048};ESP_ERROR_CHECK(usb_serial_jtag_driver_install(&usj));
ESP_ERROR_CHECK(uart_driver_install(UART_NUM_0,2048,0,0,NULL,0));
if(esp_reset_reason()==ESP_RST_POWERON){boot_counter=0;}boot_counter++;
uint32_t nonce_hi=esp_random(),nonce_lo=esp_random();
printf("V5_READY {\"schema\":5,\"build_id\":\"%s\",\"boot_counter\":%"PRIu32",\"nonce_hi\":%"PRIu32",\"nonce_lo\":%"PRIu32",\"cpu_hz\":%d,\"tick_hz\":%u,\"reset_reason\":%d}\n",BUILD_ID,boot_counter,nonce_hi,nonce_lo,esp_clk_cpu_freq(),(unsigned)configTICK_RATE_HZ,esp_reset_reason());fflush(stdout);
bool active=false,validated=false,done=false;char line[256];
for(;;){read_line(line,sizeof(line));char command[16],token[40];unsigned seq=0,model=0,backend=0,sample=0,phase=0,pause=0,delay=0;
if(sscanf(line,"%15s %39s",command,token)!=2){reject("bad_command");continue;}
if(strcmp(command,"HELLO")==0&&!active&&!done){
printf("V5_READY {\"schema\":5,\"build_id\":\"%s\",\"boot_counter\":%"PRIu32",\"nonce_hi\":%"PRIu32",\"nonce_lo\":%"PRIu32",\"cpu_hz\":%d,\"tick_hz\":%u,\"reset_reason\":%d}\n",BUILD_ID,boot_counter,nonce_hi,nonce_lo,esp_clk_cpu_freq(),(unsigned)configTICK_RATE_HZ,esp_reset_reason());fflush(stdout);continue;}
if(strcmp(command,"BEGIN")==0){
 if(active||done||strlen(token)!=32){reject("begin_requires_fresh_boot");continue;}bool ok=true;for(unsigned j=0;j<32;j++)ok&=isxdigit((unsigned char)token[j])!=0;if(!ok){reject("bad_token");continue;}
 memcpy(challenge,token,33);active=true;
 printf("V5_SESSION {\"challenge\":\"%s\",\"build_id\":\"%s\",\"boot_counter\":%"PRIu32",\"nonce_hi\":%"PRIu32",\"nonce_lo\":%"PRIu32",\"cpu_hz\":%d,\"models\":[\"person\",\"cifar10_resnet\"],\"backends\":[\"esp_nn\",\"mu_scalar\",\"mu_fixed\",\"rust\"],\"samples\":4,\"rounds\":5,\"expected_checks\":752,\"expected_runs\":160,\"tensor_ram_bytes\":112144}\n",challenge,BUILD_ID,boot_counter,nonce_hi,nonce_lo,esp_clk_cpu_freq());fflush(stdout);continue;}
if(!active||!token_ok(token)||done){reject("session_token_or_state");continue;}
if(strcmp(command,"CHECK")==0){
 if(validated){reject("already_checked");continue;}
 for(unsigned m=0;m<2;m++)for(unsigned b=0;b<4;b++)for(unsigned s=0;s<4;s++)execute(m,b,s,1);
 validated=true;printf("V5_CHECK_DONE {\"challenge\":\"%s\",\"checks\":%u,\"mismatches\":%u}\n",challenge,checks,mismatches);fflush(stdout);continue;}
if(!validated){reject("check_first");continue;}
if(strcmp(command,"CAL")==0){
 if(sscanf(line,"%15s %39s %u %u",command,token,&seq,&delay)!=4||seq!=next_seq||delay<200||delay>8000||cal_count>=3){reject("bad_calibration");continue;}
 int64_t a=esp_timer_get_time();uint32_t c=ticks();esp_rom_delay_us(delay);uint32_t cycles=ticks()-c;int64_t z=esp_timer_get_time();
 printf("V5_CAL {\"challenge\":\"%s\",\"seq\":%u,\"requested_us\":%u,\"cycles_raw\":%"PRIu32",\"device_start_us\":%"PRId64",\"device_end_us\":%"PRId64"}\n",challenge,seq,delay,cycles,a,z);next_seq++;cal_count++;fflush(stdout);continue;}
if(strcmp(command,"RUN")==0){
 if(sscanf(line,"%15s %39s %u %u %u %u %u %u",command,token,&seq,&model,&backend,&sample,&phase,&pause)!=8||seq!=next_seq||cal_count!=3||model>1||backend>3||sample>3||phase>9000||pause<1||pause>3||seen[model][backend][sample]>=5){reject("bad_run");continue;}
 execute(model,backend,sample,0);mismatches+=output_check(model,sample);vTaskDelay(pause);esp_rom_delay_us(phase);
 int64_t start=esp_timer_get_time();uint32_t c=ticks();execute(model,backend,sample,0);uint32_t cycles=ticks()-c;int64_t end=esp_timer_get_time();
 unsigned wrong=output_check(model,sample);mismatches+=wrong;unsigned round=seen[model][backend][sample]++;
 const int8_t*out=model?resnet_output():person_output();unsigned n=model?10:2;
 printf("V5_RUN {\"challenge\":\"%s\",\"seq\":%u,\"model\":%u,\"backend\":%u,\"sample\":%u,\"round\":%u,\"requested_phase_us\":%u,\"pause_ticks\":%u,\"observed_tick_phase_us\":%u,\"cycles_raw\":%"PRIu32",\"device_start_us\":%"PRId64",\"device_end_us\":%"PRId64",\"mismatches\":%u,\"output_hash\":%"PRIu32",\"stack_min_free_bytes\":%u,\"free_heap\":%u}\n",challenge,seq,model,backend,sample,round,phase,pause,(unsigned)(start%10000),cycles,start,end,wrong,fingerprint(out,n),(unsigned)uxTaskGetStackHighWaterMark(NULL),(unsigned)esp_get_free_heap_size());
 next_seq++;run_count++;fflush(stdout);continue;}
if(strcmp(command,"END")==0){
 if(checks!=752||cal_count!=3||run_count!=160){reject("incomplete_session");continue;}
 printf("V5_DONE {\"challenge\":\"%s\",\"build_id\":\"%s\",\"correct\":%s,\"mismatches\":%u,\"checks\":%u,\"runs\":%u,\"calibrations\":%u,\"boot_counter\":%"PRIu32",\"stack_min_free_bytes\":%u}\n",challenge,BUILD_ID,mismatches?"false":"true",mismatches,checks,run_count,cal_count,boot_counter,(unsigned)uxTaskGetStackHighWaterMark(NULL));fflush(stdout);done=true;continue;}
reject("unknown_command");
}}
void app_main(void){if(xTaskCreate(worker,"two_models",16384,NULL,5,NULL)!=pdPASS)abort();}
