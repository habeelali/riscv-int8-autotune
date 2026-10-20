/* SPDX-License-Identifier: GPL-2.0-or-later
 * Independent measurement cross-check using QEMU's executed-instruction callbacks.
 * Single vCPU only. Translation callbacks register execution callbacks; they do
 * not themselves contribute to the dynamic instruction count.
 */
#include "qemu-plugin.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
QEMU_PLUGIN_EXPORT int qemu_plugin_version = QEMU_PLUGIN_VERSION;
static uint64_t start_pc, end_pc, count;
static bool active;
static void execute(unsigned int cpu, void *userdata) {
    uint64_t pc=(uintptr_t)userdata;
    if(pc==start_pc){active=true;count=0;}
    if(pc==end_pc && active){fprintf(stderr,"PLUGIN_REGION %" PRIu64 "\n",count);active=false;}
    if(active)count++;
}
static void translate(qemu_plugin_id_t id, struct qemu_plugin_tb *tb) {
    for(size_t i=0;i<qemu_plugin_tb_n_insns(tb);i++) {
        struct qemu_plugin_insn *insn=qemu_plugin_tb_get_insn(tb,i);
        qemu_plugin_register_vcpu_insn_exec_cb(insn,execute,QEMU_PLUGIN_CB_NO_REGS,(void*)(uintptr_t)qemu_plugin_insn_vaddr(insn));
    }
}
QEMU_PLUGIN_EXPORT int qemu_plugin_install(qemu_plugin_id_t id,const qemu_info_t *info,int argc,char **argv) {
    if(argc!=2)return -1;
    start_pc=strtoull(argv[0],0,0);end_pc=strtoull(argv[1],0,0);
    qemu_plugin_register_vcpu_tb_trans_cb(id,translate);
    return 0;
}
