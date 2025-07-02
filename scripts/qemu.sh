#!/bin/bash

# =======================================================
# Script QEMU com Suporte Avançado a Interrupções
# Raspberry Pi 3B+ - Rust MINIX Interrupt System Test
# =======================================================

# Configurações
TARGET="aarch64-unknown-none"
KERNEL_NAME="rust_minix"
OUTPUT_IMG="kernel8.img"
QEMU_CMD="qemu-system-aarch64"

echo "=== Rust MINIX Interrupt System Test ==="
echo "🎯 Target: Raspberry Pi 3B+ (BCM2837)"
echo "⚡ Testing: Timer interrupts, Exception handling, IRQ/FIQ"
echo ""

# Compilar o projeto
echo "🛠️  Building kernel with interrupt support..."
cargo build --release

# Verificar compilação
if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"

# Copiar kernel (sem objcopy para simplificar)
echo "📦 Preparing kernel image..."
cp "target/$TARGET/release/$KERNEL_NAME" "$OUTPUT_IMG"

# Verificar se a imagem foi gerada
if [ ! -f "$OUTPUT_IMG" ]; then
    echo "❌ Failed to create $OUTPUT_IMG!"
    exit 1
fi

echo "✅ Kernel image ready: $OUTPUT_IMG"
echo ""
echo "🚀 Starting QEMU with interrupt debugging..."
echo ""
echo "📋 Expected Output:"
echo "   - System initialization"
echo "   - Exception vector table setup"
echo "   - BCM2837 interrupt controller init"
echo "   - Timer setup (10ms intervals)"
echo "   - IRQ exceptions every 10ms"
echo "   - Timer tick counter incrementing"
echo "   - System uptime reports"
echo ""
echo "💡 Controls:"
echo "   - Ctrl+A then X: Exit QEMU"
echo "   - Ctrl+A then C: QEMU monitor"
echo "   - Let it run ~10 seconds to see interrupts"
echo ""

echo "🖥️  Starting QEMU with visual display..."

# Executar no QEMU com debugging de interrupções e display visual
$QEMU_CMD \
    -M raspi3b \
    -kernel "$OUTPUT_IMG" \
    -serial stdio \
    -display gtk \
    -d int \
    -D qemu_debug.log \
    -smp 4 \
    -m 1024 \
    -rtc base=localtime \
    -no-reboot \
    -machine kernel-irqchip=on \
    -device usb-kbd \
    -device usb-mouse