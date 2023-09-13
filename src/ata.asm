; reference: https://wiki.osdev.org/ATA_PIO_Mode

[bits 64]

%define READ_PORT 0x1f0
%define ATA_DRIVE_HEAD_PORT 0x1f6
%define SECTOR_COUNT_PORT 0x1f2
%define SECTOR_NUM_PORT 0x1f3
%define CYLINDER_LOW_PORT 0x1f4
%define CYLINDER_HIGH_PORT 0x1f5
%define COMMAND_PORT 0x1f7
%define STATUS_REGISTER 0x1f7
%define SLAVE_BIT 0


global read_chs

; CHS in rsi | 3 bytes unused | 2 bytes: cylinder | 1 byte: drive_head | 1 byte: sector count | 1 byte: sector number
; dest in rdi (helps with insw)

read_chs:
    pushall
    xor rax, rax
    mov dx, ATA_DRIVE_HEAD_PORT
    mov rax, rsi
    shr rax, 16
    out dx, al

; loop fifteen times to wait 400 ns?
.init_loop:
    mov cx, 0xf
    mov dx, STATUS_REGISTER
    xor rax, rax

.wait:
    in al, dx
    cmp cx, 0
    je .check_bsy
    loop .wait

.check_bsy:
    and al, 0b10000000
    cmp al, 0
    je drive_ready

drive_error:
    mov rax, 0x1
    ret

drive_ready:
    mov rax, rsi
    mov dx, SECTOR_COUNT_PORT
    and rax, 0xFF00
    shr rax, 8
    out dx, al

    mov rax, rsi
    mov dx, SECTOR_NUM_PORT
    and rax, 0xFF
    out dx, al

    mov rax, rsi
    mov dx, CYLINDER_LOW_PORT
    shr rax, 24
    and rax, 0xFF
    out dx, al

    shr rax, 8
    mov dx, CYLINDER_HIGH_PORT
    out dx, al

    mov dx, COMMAND_PORT
    mov al, 0x20
    out dx, al

    xor rax, rax

.check_data_transfer:
    in al, dx
    shr al, 3                   ; checking if DRQ bit is set
    and al, 1
    cmp al, 0
    je .check_data_transfer

.read_data:
    mov rax, 512/2              ; 256 words
    mov rbx, rsi
    shr rbx, 8
    and rbx, 0xFF
    mul bx
    mov rcx, rax
    mov dx, READ_PORT       ; read to [rdi]
    rep insw

    mov rax, 0
    popall
    ret