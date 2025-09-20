; https://os.phil-opp.com/multiboot-kernel/

global start

section .text
bits 32

start:
    ; Mostra ROCHA-OS com fundo verde (2F)
    mov dword [0xB8000], 0x2F4F2F52 
    mov dword [0xB8004], 0x2F482F43
    mov dword [0xB8008], 0x2F2D2F41
    mov dword [0xB800C], 0x2F532F4F
    hlt
