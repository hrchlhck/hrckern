FROM --platform=linux/amd64 ubuntu:24.04  as BUILDER

RUN apt update \
    && apt install -y \
    make nasm gcc \
    xorriso grub2-common \
    grub-pc-bin \
    && apt clean

WORKDIR /bootloader
