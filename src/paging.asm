; references used so far: 
; https://wiki.osdev.org/Entering_Long_Mode_Directly
; https://wiki.osdev.org/Setting_Up_Long_Mode
; https://wiki.osdev.org/Setting_Up_Paging


; es:di must point to a page aligned memory location (16KB aligned)

%define PAGE_PRESENT (1 << 0)
%define PAGE_WRITE (1 << 1)
; from this we can infer that the last 2 bits are to tell if page is present and if it is writable
 

SwitchToLongMode:
    mov ecx, 0x1000 ; counter register, rep keeps running until ecx becomes 0, hence it runs 4096 times here
    xor eax, eax ; stosd copy's the content of eax to the memory pointed by edi, hence it fills the memory with 0
    cld ; clear direction flag, so that the memory is filled from low to high
    rep stosd ; fill the memory with 0


mov di, 0x100000 ; setting di to 1MB, this is where the PML4 will be stored
push di


; setting up the PML4
lea eax, [di + 0x1000] ; setting eax to address of PDPT
; from the line above we can infer that PDPT is 4kb away from first entry of PML4
or eax, PAGE_PRESENT | PAGE_WRITE ; setting the present and write bits
mov DWORD [di], eax ; setting the first entry of the PML4 to the address of PDPT

; setting up the PDPT
lea eax, [di + 0x2000] ; setting eax to address of PD
; from the line above we can infer that PD is 8kb away from first entry of PDPT
or eax, PAGE_PRESENT | PAGE_WRITE ; setting the present and write bits
mov DWORD [di + 0x1000], eax ; setting the first entry of the PDPT to the address of PD

; setting up the PD
lea eax, [di + 0x3000] ; setting eax to address of PT
; from the line above we can infer that PT is 12kb away from first entry of PD
or eax, PAGE_PRESENT | PAGE_WRITE ; setting the present and write bits
mov DWORD [di + 0x2000], eax ; setting the first entry of the PD to the address of PT


; setting up the PT
lea di, [di + 0x3000] ; setting di to address of PT (itself)
mov eax, PAGE_PRESENT | PAGE_WRITE ; setting the present and write bits
mov ecx, 512 ; counter register, loop keeps running until ecx becomes 0, hence it runs 512 times here

; the loop below fills the PT with 512 entries, each entry is 8 bytes long
PTLoop:
    mov DWORD [di], eax
    add di, 8 ; incrementing the address by 8 bytes
    add eax, 0x1000 ; incrementing the address by 4kb
    loop PTLoop ; loop until ecx becomes 0


; disabling IRQs
mov al, 0xFF
out 0x21, al
out 0xA1, al

nop
nop ; do delay before IRQs are disabled

lidt [IDT] ; load the IDT but it's empty to avoid any interrupts

; set PAE and PG flags in CR4
mov eax, cr4
or eax, (1 << 5) ; PAE
or eax, (1 << 7) ; PG
mov cr4, eax

pop di ; pop the address of PML4 into di

mov edx, edi
mov cr3, edx ; load the PML4 address into CR3



; using a model specific register to set the long mode bit

mov ecx, 0xC0000080 ; model specific register
rdmsr ; read the model specific register

or eax, 0x00000100 ; set the long mode bit
wrmsr ; write the model specific register

; enabling paging and protected mode
mov ebx, cr0
or ebx, (1<<0) | (1<<31) ; set the PG and PE bits
mov cr0, ebx
