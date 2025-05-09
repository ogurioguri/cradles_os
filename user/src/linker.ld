OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0x80400000;

SECTIONS
{
    . = BASE_ADDRESS;
    .text : {
        *(.text.entry)
        *(.text .text.*)
        end_text = .;
    }
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        end_rodata = .;
    }
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
        end_data = .;
    }
    .bss : {
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    }
    /DISCARD/ : {
        *(.eh_frame)
        *(.debug*)

    }
}