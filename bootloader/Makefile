all: assemble iso clean

assemble:
	nasm -f elf64 boot.asm
	ld -n -T linker.ld 

iso:
	mkdir -p isofiles/boot/grub
	cp kernel.bin isofiles/boot
	cp grub.cfg isofiles/boot/grub
	grub-mkrescue -o hrckern.iso isofiles

clean:
	rm *.o *.a
	rm -rf isofiles