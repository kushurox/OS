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

load_kernel:
    mov ah, 0x02 ; read sectors
    mov al, 0xFF ; sector count
    mov cl, 0x02 ; sector start, 0x01 is boot sector
    mov ch, 0x00 ; cylinder start
    mov dh, 0x00 ; head start
    mov dl, 0x0 ; drive number
    mov bx, 0x7e00 ; buffer address
    int 0x13



jmp SwitchToLongMode


%include "src/paging.asm"
lgdt [GDT_ADDR]
jmp 0x08:LongMode
%include "src/gdt.asm"
%include "src/print.asm"

ALIGN 4
IDT:
    .Length       dw 0
    .Base         dd 0

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