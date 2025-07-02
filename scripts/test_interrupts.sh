#!/bin/bash

# Script para testar o sistema de interrupções no QEMU
# Raspberry Pi 3B+ emulation

echo "Testing Rust MINIX Interrupt System on Raspberry Pi 3B+"
echo "================================================="

# Build the kernel
echo "Building kernel..."
cargo build

if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

# Copy kernel to proper location
cp target/aarch64-unknown-none/debug/rust_minix kernel8.img

echo "Starting QEMU with Raspberry Pi 3B+ emulation..."
echo "Expected output:"
echo "- Exception vector table installation"
echo "- Interrupt controller initialization"  
echo "- Timer setup with 10ms intervals"
echo "- Timer ticks every 100 interrupts (~1 second)"
echo "- System uptime and tick counter updates"
echo ""
echo "Press Ctrl+A then X to exit QEMU"
echo ""

# Run QEMU with Raspberry Pi 3B+ machine type
qemu-system-aarch64 \
    -M raspi3b \
    -kernel kernel8.img \
    -serial stdio \
    -display none \
    -d guest_errors
