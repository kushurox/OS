GDT:
    ; null descriptor
    dq 0

.code_seg:

    ; limit
    dw 0x0000

    ; base
    db 0x00
    dw 0x00

    ; access byte
    db 0b10011010

    ; flags + limit
    db 0b10100000

    ; base
    db 0x00

.data_seg:
    ; limit
    dw 0x0000

    ; base
    db 0x00
    dw 0x0000

    ; access byte
    db 0b10010010

    ; flags + limit
    db 0b10100000

    ; base
    db 0x00

ALIGN 4
dw 0 ; padding so that address of GDT is divisible by 4 due to performance reasons

GDT_ADDR:
    dw $ - GDT - 1
    dd GDT
