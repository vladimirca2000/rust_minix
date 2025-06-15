#!/bin/bash

# Configuração
TARGET="aarch64-unknown-none"
KERNEL_NAME="rust_minix"
OUTPUT_IMG="kernel8.img"
QEMU_CMD="qemu-system-aarch64"

# Compilar o projeto
echo "Compilando o kernel..."
cargo build --release

# Verificar se a compilação foi bem sucedida
if [ $? -ne 0 ]; then
    echo "Falha na compilação!"
    exit 1
fi

# Gerar imagem binária
echo "Gerando imagem binária..."
aarch64-none-elf-objcopy -O binary \
    "target/$TARGET/release/$KERNEL_NAME" \
    "$OUTPUT_IMG"

# Executar no QEMU
echo "Iniciando QEMU..."
$QEMU_CMD \
    -M raspi3b \
    -kernel "$OUTPUT_IMG" \
    -serial stdio \
    -display none \
    -d in_asm,trace \
    -no-reboot