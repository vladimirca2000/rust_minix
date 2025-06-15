#!/bin/bash

# Configurações
TARGET="aarch64-unknown-none"
KERNEL_NAME="rust_minix"
OUTPUT_IMG="kernel8.img"
SD_PARTITION="/dev/sdX1"  # ATUALIZE PARA SUA PARTIÇÃO SD!
MOUNT_POINT="/mnt/rpi_sd"

# Compilar o projeto
echo "Compilando o kernel..."
cargo build --release

# Verificar compilação
if [ $? -ne 0 ]; then
    echo "Falha na compilação!"
    exit 1
fi

# Gerar imagem binária
echo "Gerando imagem binária..."
aarch64-none-elf-objcopy -O binary \
    "target/$TARGET/release/$KERNEL_NAME" \
    "$OUTPUT_IMG"

# Montar o sistema de arquivos do SD card
echo "Montando SD card..."
sudo mkdir -p "$MOUNT_POINT"
sudo mount "$SD_PARTITION" "$MOUNT_POINT"

# Verificar montagem
if [ $? -ne 0 ]; then
    echo "Falha ao montar o SD card!"
    exit 1
fi

# Copiar arquivos para o SD card
echo "Copiando kernel para o SD card..."
sudo cp "$OUTPUT_IMG" "$MOUNT_POINT/"

# Criar config.txt se necessário
if [ ! -f "$MOUNT_POINT/config.txt" ]; then
    echo "Criando config.txt..."
    echo -e "kernel=kernel8.img\narm_64bit=1\nenable_uart=1" | sudo tee "$MOUNT_POINT/config.txt"
fi

# Desmontar o SD card
echo "Desmontando SD card..."
sudo umount "$MOUNT_POINT"

echo "SD card pronto! Remova com segurança e use no Raspberry Pi 3B+."