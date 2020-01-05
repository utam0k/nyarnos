OUTPUT_FORMAT(elf32-i386)
OUTPUT_ARCH(i386)
ENTRY(start)
IPLBASE = 0x7c00;

SECTIONS {
    . = IPLBASE;
    .text : {
        . = ALIGN(4);
        *(.text)
        *(.text.*)
    }
    .rodata : {
        . = ALIGN(4);
        *(.rodata)
        *(.rodata.*)
    }
    . = IPLBASE + 510;
    .signature : {
        SHORT(0xaa55);
    }
}
