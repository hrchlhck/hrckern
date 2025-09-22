all: assemble iso clean

assemble:
	nasm -f elf64 multiboot_header.asm
	nasm -f elf64 boot.asm
	nasm -f elf64 longmode.asm
	ld -n -o kernel.bin -T linker.ld multiboot_header.o boot.o longmode.o libhrckern.a

iso:
	mkdir -p isofiles/boot/grub
	cp kernel.bin isofiles/boot
	cp grub.cfg isofiles/boot/grub
	grub-mkrescue -o hrckern.iso isofiles

clean:
	rm *.o *.bin
	rm -rf isofiles