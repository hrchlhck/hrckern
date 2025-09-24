.PHONY: build

all: build run

build-env:
	docker build -t builder .

build:
	cargo build 
	cp target/hrckern/debug/libhrckern.a bootloader
	docker run -v ./bootloader:/bootloader --rm --workdir=/bootloader -it builder make 

run:
	qemu-system-x86_64 -vga std -s -boot d -cdrom bootloader/hrckern.iso

clean:
	cargo clean
	rm bootloader/hrckern.iso
