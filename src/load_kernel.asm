; loading 1 MB kernel from disk
load_kernel:
    mov ah, 0x02 ; read disk
    mov al, 128 ; number of sectors to read
    mov dh, 0x00 ; head number
    mov dl, 0x80 ; drive number
    mov ch, 0x00 ; cylinder number
    mov cl, 0x02 ; sector number
    mov bx, 0x7e00 ; load at 0x7e00
    int 0x13
    ; this will load upto 0x7c00 + 64KB = 0x17c00
    ; loading consecutive sectors, each head can load upto 128 sectors which is 64 KB
    ; we need to load 1 MB, so we need 16 heads but we already loaded 1 sector hence 15 heads
    ; 15 * 64 KB = 960 KB
    mov dh, 0x01 ; head number
    add bx, 0x2000 ; 64 KB


load_heads:
    cmp dh, 0xFF
    mov ah, 0x02 ; read disk
    mov al, 128 ; number of sectors to read
    mov dl, 0x80 ; drive number
    mov cl, 0x01 ; sector number
    int 0x13
    inc dh
    add bx, 0x2000 ; 64 KB
    je load_done
load_done_c:

