.section .text.init
.global _start
_start:
.option push
.option norelax
la sp, __stack_top
.option pop
la t0, __bss_start
la t1, __bss_end
1: bgeu t0, t1, 2f
sw zero, 0(t0)
addi t0, t0, 4
j 1b
2: call main
li t0, 0x100000
li t1, 0x5555
sw t1, 0(t0)
3: j 3b
