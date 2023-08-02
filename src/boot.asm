section .bootsector

[bits 16]
main:
    xor ax, ax
    mov ss, ax
    mov sp, main
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    cld

%include "src/load_kernel.asm"



jmp SwitchToLongMode


%include "src/paging.asm"
lgdt [GDT_ADDR]
jmp 0x08:LongMode
%include "src/gdt.asm"

ALIGN 4
IDT:
    .Length       dw 0
    .Base         dd 0

%include "src/ata.asm"


[bits 64]
LongMode:
    mov ax, 0x0
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    extern kmain
    call kmain
