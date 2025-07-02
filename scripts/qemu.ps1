# =======================================================
# Script QEMU com Suporte Avançado a Interrupções
# Raspberry Pi 3B+ - Rust MINIX Interrupt System Test
# PowerShell Version for Windows
# =======================================================

$TARGET = "aarch64-unknown-none"
$KERNEL_NAME = "rust_minix"
$OUTPUT_IMG = "kernel8.img"
$QEMU_CMD = "qemu-system-aarch64"

Write-Host "=== Rust MINIX Interrupt System Test ===" -ForegroundColor Cyan
Write-Host "🎯 Target: Raspberry Pi 3B+ (BCM2837)" -ForegroundColor Yellow
Write-Host "⚡ Testing: Timer interrupts, Exception handling, IRQ/FIQ" -ForegroundColor Yellow
Write-Host ""

# Compilar o projeto
Write-Host "🛠️  Building kernel with interrupt support..." -ForegroundColor Green
cargo build --release

# Verificar compilação
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Build successful!" -ForegroundColor Green

# Copiar kernel
Write-Host "📦 Preparing kernel image..." -ForegroundColor Blue
Copy-Item "target\$TARGET\release\$KERNEL_NAME" "$OUTPUT_IMG" -Force

# Verificar se a imagem foi gerada
if (-not (Test-Path "$OUTPUT_IMG")) {
    Write-Host "❌ Failed to create $OUTPUT_IMG!" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Kernel image ready: $OUTPUT_IMG" -ForegroundColor Green
Write-Host ""
Write-Host "🚀 Starting QEMU with interrupt debugging..." -ForegroundColor Cyan
Write-Host ""
Write-Host "📋 Expected Output:" -ForegroundColor Yellow
Write-Host "   - System initialization"
Write-Host "   - Exception vector table setup"
Write-Host "   - BCM2837 interrupt controller init"
Write-Host "   - Timer setup (10ms intervals)"
Write-Host "   - IRQ exceptions every 10ms"
Write-Host "   - Timer tick counter incrementing"
Write-Host "   - System uptime reports"
Write-Host ""
Write-Host "💡 Controls:" -ForegroundColor Magenta
Write-Host "   - Ctrl+A then X: Exit QEMU"
Write-Host "   - Ctrl+A then C: QEMU monitor"
Write-Host "   - Let it run ~10 seconds to see interrupts"
Write-Host ""

# Executar no QEMU com debugging de interrupções e display visual
Write-Host "🖥️  Starting QEMU with visual display..." -ForegroundColor Cyan

& $QEMU_CMD `
    -M raspi3b `
    -kernel "$OUTPUT_IMG" `
    -serial stdio `
    -display gtk `
    -d int `
    -D qemu_debug.log `
    -smp 4 `
    -m 1024 `
    -rtc base=localtime `
    -no-reboot `
    -machine kernel-irqchip=on `
    -device usb-kbd `
    -device usb-mouse
