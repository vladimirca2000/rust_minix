#!/bin/bash

# Configurações
TARGET="aarch64-unknown-none"
KERNEL_NAME="rust_minix"
OUTPUT_IMG="kernel8.img"
QEMU_CMD="qemu-system-aarch64"
OBJCOPY_TOOL="aarch64-linux-gnu-objcopy"

# Compilar o projeto
echo "🛠️  Compilando o kernel..."
cargo build --release

# Verificar compilação
if [ $? -ne 0 ]; then
    echo "❌ Falha na compilação!"
    exit 1
fi

# Gerar imagem binária
echo "🔧 Gerando imagem binária..."
if ! command -v $OBJCOPY_TOOL &> /dev/null; then
    echo "❌ $OBJCOPY_TOOL não encontrado!"
    echo "   Instale com: sudo apt-get install binutils-aarch64-linux-gnu"
    exit 1
fi

$OBJCOPY_TOOL -O binary \
    "target/$TARGET/release/$KERNEL_NAME" \
    "$OUTPUT_IMG"

# Verificar se a imagem foi gerada
if [ ! -f "$OUTPUT_IMG" ]; then
    echo "❌ Falha ao gerar $OUTPUT_IMG!"
    exit 1
fi

# Executar no QEMU
echo "🚀 Iniciando QEMU..."
$QEMU_CMD \
    -M raspi3b \
    -kernel "$OUTPUT_IMG" \
    -serial stdio \
    -display sdl