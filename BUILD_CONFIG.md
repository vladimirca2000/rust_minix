# Configuração de Build - Rust MINIX ARM64

Este documento descreve a configuração completa de build para o Rust MINIX voltado para ARM64/Raspberry Pi 3B+.

## Arquivos de Configuração

### 1. Cargo.toml
```toml
[package]
name = "rust_minix"
version = "0.1.0"
edition = "2021"
build = "build.rs"
authors = ["Rust MINIX Team"]
description = "A MINIX-like operating system kernel written in Rust for ARM64/Raspberry Pi 3B+"

[[bin]]
name = "rust_minix"
path = "src/main.rs"
test = false  # Disable tests for the kernel binary

[dependencies]
spin = "0.9.8"

[features]
default = []
alloc = []  # Habilita suporte a alocação dinâmica

[profile.dev]
panic = "abort"
overflow-checks = true
lto = false
opt-level = 1

[profile.release]
panic = "abort"
lto = true
codegen-units = 1
opt-level = "s"  # Optimize for size
overflow-checks = false
debug = false
strip = true
```

### 2. .cargo/config.toml
```toml
# Target específico: Raspberry Pi 3B+ (Cortex-A53)
[build]
target = "aarch64-unknown-none"

[target.aarch64-unknown-none]
rustflags = [
    "-C", "link-arg=-Tlinker.ld",        # Linker script customizado
    "-C", "target-cpu=cortex-a53",       # CPU específico
    "-C", "target-feature=+neon",        # Features ARM64
    "-C", "link-arg=--build-id=none",    # Otimizações
    "-C", "link-arg=--no-undefined-version",
    "-C", "link-arg=--no-rosegment",
    "-C", "link-arg=--gc-sections",      # Remove seções não utilizadas
    "-C", "link-arg=-z",
    "-C", "link-arg=max-page-size=4096", # Page size ARM64
    "-C", "link-arg=--no-dynamic-linker", # Kernel estático
    "-C", "symbol-mangling-version=v0",  # Símbolos limpos
    "-C", "no-redzone",                  # Importante para kernels
]
```

### 3. build.rs
- Verifica existência do linker script
- Gera informações de build (versão, target, CPU)
- Configura recompilação automática quando arquivos mudam
- Gera mapa de memória (kernel.map)

### 4. linker.ld
```ld
MEMORY {
    RAM (rwx) : ORIGIN = 0x80000, LENGTH = 128M
}

SECTIONS {
    .text   : Boot code + código executável
    .rodata : Dados somente leitura
    .data   : Dados inicializados
    .bss    : Dados não inicializados (zerados)
    Stack   : 64KB stack do kernel
    Heap    : Início do heap para alocação dinâmica
}
```

## Características da Configuração

### Target específico
- **Arquitetura**: ARM64 (aarch64-unknown-none)
- **CPU**: Cortex-A53 (Raspberry Pi 3B+)
- **Features**: NEON enabled
- **Modelo de memória**: Bare metal, sem OS

### Otimizações
- **Release**: Otimizado para tamanho (`opt-level = "s"`)
- **LTO**: Link Time Optimization habilitado
- **Strip**: Símbolos de debug removidos no release
- **GC Sections**: Remove código não utilizado

### Memória
- **Base RAM**: 0x80000 (512KB offset do Raspberry Pi)
- **Tamanho**: 128MB disponível
- **Stack**: 64KB reservado
- **Page size**: 4KB (padrão ARM64)

### Segurança
- **No redzone**: Essencial para kernels
- **Panic = abort**: Sem unwinding
- **Static linking**: Sem dependências dinâmicas

## Arquivos Gerados

### Durante o Build
- `kernel.map`: Mapa detalhado de memória e símbolos
- `kernel8.img`: Imagem final do kernel para Raspberry Pi
- `build_info.rs`: Informações de build incluídas no código

### Informações de Build
O kernel mostra na inicialização:
```
Version: 0.1.0
Target: aarch64 (cortex-a53)  
Build: Compiled with Rust
```

## Comandos de Build

```bash
# Build debug
cargo build

# Build release (otimizado)
cargo build --release

# Executar no QEMU
pwsh scripts/qemu.ps1

# Limpeza
cargo clean
```

## Verificação da Configuração

### Testes Realizados
✅ Build debug successful
✅ Build release successful  
✅ QEMU execution working
✅ Memory map generation
✅ No unstable features
✅ No test crate errors
✅ Cortex-A53 specific flags
✅ Linker script validation

### Saída do Sistema
```
=== Rust MINIX (ARM64) ===
Version: 0.1.0
Target: aarch64 (cortex-a53)
Build: Compiled with Rust
BSS cleared: 72 bytes
Stack start: 0x89000
```

A configuração está otimizada para o desenvolvimento de um kernel ARM64 bare-metal com foco na plataforma Raspberry Pi 3B+.
