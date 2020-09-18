MEMORY
{
  /* Leave 8k for the default bootloader*/
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 64K
  RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 8K
}
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

