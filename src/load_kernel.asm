; loading 1 MB kernel from disk

read_disk_param:
    mov ah, 0x08 ; get disk parameters
    mov dl, 0x80 ; drive number
    int 0x13
    ; last logical head number is 15, just for debugging

load_kernel:
    mov ah, 0x02 ; read disk
    mov al, 128 ; number of sectors to read
    mov dh, 0x00 ; head number
    mov dl, 0x80 ; drive number
    mov ch, 0x00 ; cylinder number
    mov cl, 0x02 ; sector number
    mov bx, 0x7e00 ; load at 0x7e00
    int 0x13
    ; this loads 128 sectors (64 KB) at 0x7e00
    ; 512 * 128 = 65536 = 64 KB
    ; loads upto 0x7e00 + 0x10000 = 0x17e00
load_done:

