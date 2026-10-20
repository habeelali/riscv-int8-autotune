OUTPUT_ARCH(riscv)
ENTRY(_start)
MEMORY { RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 4M }
SECTIONS {
 .text : { KEEP(*(.text.init)) *(.text .text.*) } > RAM
 .rodata : { *(.rodata .rodata.* .srodata .srodata.*) } > RAM
 .data : { *(.sdata .sdata.* .data .data.*) } > RAM
 .bss ALIGN(16) (NOLOAD) : { . = ALIGN(16); __bss_start = .; *(.sbss .sbss.* .bss .bss.*) *(COMMON) . = ALIGN(16); __bss_end = .; } > RAM
 .stack (NOLOAD) : { __stack_bottom = .; . += 65536; __stack_top = .; } > RAM
 /DISCARD/ : { *(.eh_frame*) *(.comment*) }
}
