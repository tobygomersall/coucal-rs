#!/bin/bash

set -euxo pipefail

crate=picorv32

# remove existing blobs because otherwise this will append object files to the old blobs
rm -f bin/*.a

riscv64-unknown-elf-gcc -c -mabi=ilp32 -march=rv32im asm.S -o bin/$crate.o
ar crs bin/riscv32imc-unknown-none-elf.a bin/$crate.o

rm bin/$crate.o
