SECTIONS
{
  . = 0x80000;
  .text : {
    *(.text.boot)
    *(.text.*)
  }
  .rodata : {
    *(.rodata)
    *(.rodata.*)
  }
  .data : { *(.data) }
  .bss ALIGN(0x8) : {
    __bss_begin = .;
    *(.bss*)
    __bss_end = .;
  } 
}
