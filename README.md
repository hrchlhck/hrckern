# hrckern

Simple kernel written in Rust. Based on ["Writing an OS in Rust"](https://os.phil-opp.com/).

# Dependencies
- `qemu v10.1.0`
- `rust v1.92.0-nightly`

# Must know
- The code is designed for x86 processors

# TODO
- [x] Implement a multiboot compliant *bootloader* within long mode (book's implementation)
- [x] Simple TTY
  - [x] Put `char` at
  - [x] Write `&str`
  - [x] Line break
  - [x] Change color
  - [x] `format!` and `println!` macros
  - [x] Put `char` at specific position
  - [x] Scroll
- [ ] Interrupts
  - [ ] IDT
  - [ ] IRQ
    - [ ] Keyboard
    - [ ] Mouse

# Running
To run the kernel with QEMU we need `make` and Docker. I used Docker mainly to wrap the needed tools for building, compiling, linking and the kernel's `.iso`. There is no need to install any of those tools on your host OS.

##### Create image
```bash
make build-env
```

##### Executing
```bash
make
```

Example:

![assets/exemplo-kernel.png](assets/exemplo-kernel.png)
