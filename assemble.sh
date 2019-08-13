#!/bin/bash

set -euxo pipefail

crate=picorv32

# remove existing blobs because otherwise this will append object files to the old blobs
rm -f bin/*.a

for arch in i ic; do
	for cpu_features in RV32RT_BARE RV32RT_INTERRUPTS_QREGS; do
    riscv64-unknown-elf-gcc -c -mabi=ilp32 -march=rv32$arch -D$cpu_features asm.S -o bin/$crate.o
    ar crs bin/riscv32${arch}-unknown-none-elf_${cpu_features}.a bin/$crate.o
  done
done

rm bin/$crate.o
