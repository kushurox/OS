
src:=src
target:=os


all: $(target) $(target)/boot.bin

$(target):
	mkdir -p $(target)

$(target)/boot.o: $(src)/boot.asm $(src)/gdt.asm $(src)/paging.asm
	nasm -f elf64 $< -o $@

$(target)/kernel: $(src)/*.rs
	cargo rustc --release -- --emit=obj -o $(target)/kernel

$(target)/boot.bin: linker.ld $(target)/boot.o $(target)/kernel
	ld.lld -T linker.ld $(target)/boot.o $(target)/kernel*.o -o $(target)/boot.bin --oformat=binary

run: all
	qemu-system-x86_64 -d in_asm -fda $(target)/boot.bin --no-reboot --no-shutdown


.PHONY: all clean run

clean:
	rm -rf $(target)
