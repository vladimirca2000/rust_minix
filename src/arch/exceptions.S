.section .text

// Exception vector table for AArch64
// Each entry is 0x80 bytes (128 bytes) apart
.align 11
.global exception_vector_table
exception_vector_table:

// Current EL with SP0
.align 7
curr_el_sp0_sync:
    b default_exception_handler

.align 7
curr_el_sp0_irq:
    b default_exception_handler

.align 7
curr_el_sp0_fiq:
    b default_exception_handler

.align 7
curr_el_sp0_serror:
    b default_exception_handler

// Current EL with SPx
.align 7
curr_el_spx_sync:
    b sync_exception_handler

.align 7
curr_el_spx_irq:
    b irq_exception_handler

.align 7
curr_el_spx_fiq:
    b fiq_exception_handler

.align 7
curr_el_spx_serror:
    b serror_exception_handler

// Lower EL using AArch64
.align 7
lower_el_aarch64_sync:
    b sync_exception_handler_el0

.align 7
lower_el_aarch64_irq:
    b irq_exception_handler_el0

.align 7
lower_el_aarch64_fiq:
    b fiq_exception_handler_el0

.align 7
lower_el_aarch64_serror:
    b serror_exception_handler_el0

// Lower EL using AArch32
.align 7
lower_el_aarch32_sync:
    b default_exception_handler

.align 7
lower_el_aarch32_irq:
    b default_exception_handler

.align 7
lower_el_aarch32_fiq:
    b default_exception_handler

.align 7
lower_el_aarch32_serror:
    b default_exception_handler

// Exception context save/restore macros
.macro save_context
    // Allocate space for ExceptionContext on stack
    sub sp, sp, #272    // 31*8 + 8 + 8 + 8 = 272 bytes
    
    // Save general purpose registers x0-x30
    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x19, [sp, #144]
    stp x20, x21, [sp, #160]
    stp x22, x23, [sp, #176]
    stp x24, x25, [sp, #192]
    stp x26, x27, [sp, #208]
    stp x28, x29, [sp, #224]
    str x30, [sp, #240]
    
    // Save sp_el0
    mrs x0, sp_el0
    str x0, [sp, #248]
    
    // Save elr_el1 and spsr_el1
    mrs x0, elr_el1
    str x0, [sp, #256]
    mrs x0, spsr_el1
    str x0, [sp, #264]
.endm

.macro restore_context
    // Restore elr_el1 and spsr_el1
    ldr x0, [sp, #256]
    msr elr_el1, x0
    ldr x0, [sp, #264]
    msr spsr_el1, x0
    
    // Restore sp_el0
    ldr x0, [sp, #248]
    msr sp_el0, x0
    
    // Restore general purpose registers
    ldp x28, x29, [sp, #224]
    ldr x30, [sp, #240]
    ldp x26, x27, [sp, #208]
    ldp x24, x25, [sp, #192]
    ldp x22, x23, [sp, #176]
    ldp x20, x21, [sp, #160]
    ldp x18, x19, [sp, #144]
    ldp x16, x17, [sp, #128]
    ldp x14, x15, [sp, #112]
    ldp x12, x13, [sp, #96]
    ldp x10, x11, [sp, #80]
    ldp x8, x9, [sp, #64]
    ldp x6, x7, [sp, #48]
    ldp x4, x5, [sp, #32]
    ldp x2, x3, [sp, #16]
    ldp x0, x1, [sp, #0]
    
    // Deallocate stack space
    add sp, sp, #272
.endm

// Exception handlers
sync_exception_handler:
    save_context
    mov x0, sp              // Pass context as parameter
    bl sync_exception_el1
    restore_context
    eret

irq_exception_handler:
    save_context
    mov x0, sp              // Pass context as parameter
    bl irq_exception_el1
    restore_context
    eret

fiq_exception_handler:
    save_context
    mov x0, sp              // Pass context as parameter
    bl fiq_exception_el1
    restore_context
    eret

serror_exception_handler:
    save_context
    mov x0, sp              // Pass context as parameter
    bl serror_exception_el1
    restore_context
    eret

sync_exception_handler_el0:
    save_context
    mov x0, sp              // Pass context as parameter
    bl sync_exception_el0
    restore_context
    eret

irq_exception_handler_el0:
    save_context
    mov x0, sp              // Pass context as parameter
    bl irq_exception_el0
    restore_context
    eret

fiq_exception_handler_el0:
    save_context
    mov x0, sp              // Pass context as parameter
    bl fiq_exception_el0
    restore_context
    eret

serror_exception_handler_el0:
    save_context
    mov x0, sp              // Pass context as parameter
    bl serror_exception_el0
    restore_context
    eret

default_exception_handler:
    // For now, just hang
    wfe
    b default_exception_handler
