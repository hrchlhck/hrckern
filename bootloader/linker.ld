/* https://wiki.osdev.org/Bare_Bones#Verifying_Multiboot */

ENTRY(start)
INPUT
(
	boot.o 
	longmode.o 
	libhrckern.a
)

OUTPUT(kernel.bin)

SECTIONS {
    . = 2M;

    .text BLOCK(4K) : ALIGN(4K)
    {
        KEEP(*(.multiboot_header))
        *(.text)
    }

    /* Read-only data. */
	.rodata BLOCK(4K) : ALIGN(4K)
	{
		*(.rodata)
	}

	/* Read-write data (initialized) */
	.data BLOCK(4K) : ALIGN(4K)
	{
		*(.data)
	}

	/* Read-write data (uninitialized) and stack */
	.bss BLOCK(4K) : ALIGN(4K)
	{
		*(COMMON)
		*(.bss)
	}
}