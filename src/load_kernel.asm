; loading 1 MB kernel from disk

read_disk_param:
    mov ah, 0x08 ; get disk parameters
    mov dl, 0x80 ; drive number
    int 0x13
    ; last logical head number is 15

load_kernel:
    mov ah, 0x02 ; read disk
    mov al, 128 ; number of sectors to read
    mov dh, 0x00 ; head number
    mov dl, 0x80 ; drive number
    mov ch, 0x00 ; cylinder number
    mov cl, 0x02 ; sector number
    mov bx, 0x7e00 ; load at 0x7e00
    int 0x13
    ; making the code unreachable for now.
    ; this will load upto 0x7c00 + 64KB = 0x17c00
    ; loading consecutive sectors, each head can load upto 128 sectors which is 64 KB
    ; we need to load 1 MB, so we need 16 heads but we already loaded 1 sector hence 15 heads
    ; 15 * 64 KB = 960 KB
    mov dh, 0x01 ; head number
    add bx, 0x2000 ; 64 KB


load_heads:
    mov ah, 0x02 ; read disk
    mov al, 128 ; number of sectors to read
    mov dl, 0x80 ; drive number
    mov cl, 0x01 ; sector number
    int 0x13
    inc dh
    add bx, 0x2000 ; 64 KB
    cmp dh, 0xf ; 15 heads
    jne load_heads
load_done:

