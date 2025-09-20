; https://os.phil-opp.com/multiboot-kernel/

section .multiboot_header

header_start:
    dd 0xe85250d6                
    dd 0                         
    dd header_end - header_start 
    
    ; Checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size

header_end: