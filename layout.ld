MEMORY
{
  FLASH (RX) : ORIGIN = 0x08000000, LENGTH = 64K
  SRAM (WX)  : ORIGIN = 0x20000000, LENGTH = 20K
}

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    LONG(ORIGIN(SRAM) + LENGTH(SRAM)); /* Main Stack Pointer */
    KEEP(*(.rodata.VECTOR_TABLE));
  } > FLASH

  .text :
  {
    *(.text.*);
  } > FLASH

  .rodata : ALIGN(4)
  {
    *(.rodata.*);
  } > FLASH

  .bss : ALIGN(4)
  {
    BSS_START = .;
    *(.bss.*);
    BSS_END = ALIGN(4);
  } > SRAM

  .data : ALIGN(4)
  {
    DATA_START = .;
    *(.data.*);
    DATA_END = ALIGN(4);
  } > SRAM AT > FLASH
  DATA_CONST = LOADADDR(.data);

  /DISCARD/ :
  {
    *(.ARM.*)
  }
}
