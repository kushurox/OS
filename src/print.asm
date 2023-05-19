print:
    ; uses si for buffer location
    mov ah, 0x0e

loop:
    mov al, [si]
    cmp al, 0
    je done
    int 0x10
    inc si
    jmp loop

done:
    ret