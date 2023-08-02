; loading 1 MB kernel from disk

read_disk_param:
    mov ah, 0x08 ; get disk parameters
    mov dl, 0x80 ; drive number
    int 0x13
    ; last logical head index is 15, just for debugging
    ; last logical sector index: 63
    ; last logical cylinder index: 0

load_kernel:
    mov ah, 0x02 ; read disk
    mov al, 10 ; number of sectors to read
    mov dh, 0x00 ; head number
    mov dl, 0x80 ; drive number
    mov ch, 0x00 ; cylinder number
    mov cl, 0x02 ; sector number
    mov bx, 0x7e00 ; load at 0x7e00
    int 0x13
    ; 512 * 10 = 5120 bytes
    ; loads upto 0x7e00 + 5120 = 0x9200
load_done:

