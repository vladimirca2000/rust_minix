#!/usr/bin/env pwsh

Write-Host "🔧 Testing Multi-Core Kernel..." -ForegroundColor Cyan

# Build the kernel
Write-Host "Building kernel..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}

# Copy to kernel image
Copy-Item target\aarch64-unknown-none\release\rust_minix kernel8.img -Force

# Start QEMU with multi-core support
Write-Host "🚀 Starting QEMU with 4 cores..." -ForegroundColor Green
Write-Host "Expected: Multi-core initialization messages" -ForegroundColor Yellow

$qemuProcess = Start-Process -FilePath "qemu-system-aarch64" -ArgumentList @(
    "-M", "raspi3b"
    "-kernel", "kernel8.img"
    "-serial", "stdio"
    "-display", "none"
    "-smp", "4"
    "-m", "1024"
) -NoNewWindow -PassThru

# Wait a bit and then kill
Start-Sleep -Seconds 15
$qemuProcess.Kill()

Write-Host "Test completed" -ForegroundColor Green
