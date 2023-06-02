OUTPUT_FORMAT("elf32-littlearm", "elf32-littlearm", "elf32-littlearm")
OUTPUT_ARCH(arm)

SECTIONS
{
  .text       : { 
    KEEP(*(.text.rustapi_*)) /* 保留以 rustapi_ 开头的符号，也就是类似导出函数 */
    *(.text*)
  }
  .rodata     : { *(.rodata) }
  .data       : { *(.data) }
  .bss        : { *(.bss*) }
}
