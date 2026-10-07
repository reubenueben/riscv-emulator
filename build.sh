#!/bin/sh
# usage: ./build.sh NAME        (produces NAME.bin)
name="${1%.s}"
riscv64-unknown-elf-gcc -march=rv64g -Wl,-Ttext=0x0 -nostdlib -o "$name.elf" "$name.s" &&
  riscv64-unknown-elf-objcopy -O binary "$name.elf" "$name.bin" && rm "$name.elf"
