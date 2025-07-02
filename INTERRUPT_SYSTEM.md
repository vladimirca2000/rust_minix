# Sistema de Interrupções e Exceções - Rust MINIX

## ✅ Implementações Concluídas

### 🎯 Sistema de Interrupções e Exceções
- **Tabela de Vetores de Exceção**: Implementada em assembly (`exceptions.S`) com suporte completo para:
  - Exceções síncronas (EL0 e EL1)
  - Interrupções IRQ (EL0 e EL1)  
  - Interrupções FIQ (EL0 e EL1)
  - Exceções SError (EL0 e EL1)

- **Context Switching**: Sistema completo de salvamento/restauração de contexto:
  - Todos os registradores de propósito geral (x0-x30)
  - Stack pointer (sp_el0)
  - Exception Link Register (elr_el1)
  - Saved Program Status Register (spsr_el1)

### 🔧 Driver GIC (Generic Interrupt Controller)
- **BCM2837 Support**: Driver específico para o chip do Raspberry Pi 3B+
- **Interrupt Management**: 
  - Habilitação/desabilitação de IRQs individuais
  - Suporte para interrupts locais (64-71) e globais (0-63)
  - Detecção automática de interrupts pendentes
- **Handler Registration**: Sistema de registro de handlers por tipo de interrupt

### ⏰ Sistema de Timer
- **Local ARM Timer**: Configurado para ticks de 10ms
- **Interrupt-driven**: Utiliza interrupções para contagem precisa
- **Sleep Functions**: 
  - `sleep_ticks()`: Sleep baseado em ticks
  - `sleep_ms()`: Sleep em milissegundos
- **Tick Counter**: Contador global de ticks do sistema

### 🛡️ Exception Handlers
- **Synchronous Exceptions**: 
  - Análise do ESR_EL1 para identificar tipo de exceção
  - Suporte para system calls (SVC)
  - Debugging detalhado com informações de contexto
- **Asynchronous Interrupts**:
  - IRQ: Delega para o sistema GIC
  - FIQ: Handler básico implementado
  - SError: Handler com informações de debug

## 🏗️ Arquitetura Implementada

### Vector Table Layout
```
0x000: Current EL SP0 - Sync/IRQ/FIQ/SError
0x200: Current EL SPx - Sync/IRQ/FIQ/SError  
0x400: Lower EL AArch64 - Sync/IRQ/FIQ/SError
0x600: Lower EL AArch32 - Sync/IRQ/FIQ/SError
```

### Interrupt Sources Supported
- **Timer Interrupts** (IRQ 64): Local ARM timer
- **UART Interrupts** (IRQ 57): Serial communication
- **Mailbox Interrupts** (IRQ 65): GPU communication

### Context Structure
```rust
struct ExceptionContext {
    gpr: [u64; 31],      // x0-x30
    sp_el0: u64,         // EL0 stack pointer
    elr_el1: u64,        // Return address
    spsr_el1: u64,       // Processor state
}
```

## 🚀 Como Usar

### Compilação
```bash
cargo build
```

### Teste no QEMU
```bash
./scripts/qemu.sh
```

### Deploy no Raspberry Pi 3B+
```bash
./scripts/rpi3.sh
```

## 📋 Funcionalidades Demonstradas

1. **Sistema de Boot**: Inicialização completa com setup de exceções
2. **Timer Interrupts**: Ticks a cada 10ms com contagem automática  
3. **Context Switching**: Preservação completa do estado do processador
4. **Exception Handling**: Tratamento robusto de exceções síncronas
5. **Sleep Functions**: Delays precisos baseados em interrupts

## 🎮 Exemplo de Uso

```rust
// Inicializar sistema de exceções
arch::exceptions::init();

// Habilitar interrupções
arch::exceptions::enable_interrupts();

// Usar timer para sleep
drivers::timer::sleep_ms(2000); // Sleep por 2 segundos

// Verificar status de interrupções
let enabled = arch::exceptions::interrupts_enabled();
```

## 🔍 Output Esperado

```
=== Rust MINIX (ARM64) ===
Setting up exception handling...
Exception vector table installed
Initializing BCM2837 interrupt controller...
Interrupt controller initialized
Setting up local timer with interval 10000 us
Local timer configured and enabled
Timer interrupt handler registered
Enabling interrupts...
IRQ interrupts enabled
Testing interrupt system...
Timer tick: 100
Timer tick: 200
...
```

## 🔧 Hardware Específico (Pi 3B+)

- **Processor**: Cortex-A53 (ARMv8-A)
- **Interrupt Controller**: BCM2837 + Local ARM interrupts
- **Timer**: ARM Generic Timer + Local timer
- **Memory**: 1GB RAM starting at 0x80000

## 📚 Próximos Passos

1. **MMU Implementation**: Gerenciamento de memória virtual
2. **Process Scheduler**: Scheduler preemptivo baseado em timer
3. **System Calls**: Interface completa kernel/userspace
4. **IPC**: Comunicação inter-processos estilo MINIX
5. **Device Drivers**: GPIO, SD card, USB

## 🏆 Características Técnicas

- **Zero-cost abstractions**: Handlers eficientes em assembly
- **Type-safe**: Interface Rust segura sobre hardware
- **Interrupt latency**: Mínima latência com assembly otimizado
- **Scalable**: Arquitetura preparada para múltiplos cores
- **Debuggable**: Logging detalhado de exceções e estados

Este sistema de interrupções forma a base fundamental para implementação de um scheduler preemptivo e gerenciamento avançado de processos no kernel MINIX.
