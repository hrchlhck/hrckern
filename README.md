# hrckern

Kernel simples feito em Rust. Baseado no livro [Writing an OS in Rust](https://os.phil-opp.com/).

# Dependências
- `qemu v10.1.0`
- `rust v1.92.0-nightly`

# Importante
- O código é desenvolvido sobre a arquitetura x86

# TODO
- [x] Implementar *bootloader* com suporte a multiboot e long mode (Implementação do próprio livro)
- [ ] TTY simples
  - [x] Colocar `char` na tela
  - [x] Escrever `&str`
  - [x] Quebra de linha
  - [x] Alterar cor do terminal
  - [ ] Implementação `format!` e `println!` 
  - [ ] Conversão de `char` para inteiro
  - [ ] Conversão de `char` para float
  - [x] Inserir `char` em uma posição específica
  - [ ] Rolagem da tela
- [ ] Interrupções
  - [ ] IRQ
    - [ ] Teclado
    - [ ] Mouse

# Executando o kernel
Para executar o kernel precisamos do `make` e do Docker. Nele conseguimos construir um ambiente que possui as ferramentas necessárias para a montagem, compilação e criação do `.iso` do kernel.

##### Para criar a imagem
```bash
make build-env
```

##### Para executar
```bash
make
```

Exemplo:

![assets/exemplo-kernel.png](assets/exemplo-kernel.png)