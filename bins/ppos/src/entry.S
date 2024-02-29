.section ".text.boot"

.global _start

_start:
    // Get cpuid
    mrs x0, MPIDR_EL1
    and x0, x0, #3 // Get the first two bit in Aff0 fields
    cbz x0, 1f     // only cpuid=0 could go to label 1

// cpuid = 0
1:
    // Initialize stack pointer: set to stack_top
    ldr x0, =__phys_binary_load_addr // load __phys_binary_load_addr
    mov sp, x0 // move value to sp

    // Clear bss
    ldr     x0, =__bss_begin // load __bss_begin
    ldr     x1, =__bss_end // load __bss_end
    sub     x1, x1, x0 // set x1 = bss size
2:  
    str     xzr, [x0], #8   //  *x0 = zero, x0 = x0 + 8
    sub     x1, x1, #8  //  x1 = x1 - 8
    b.gt    2b  // Loop back to label 2 until bss is cleared

3:  bl      kernel_start  // Jump to Kernel Entry Point (kernel_start function)
    b       4f    // Stay in busy loop if returned

// Busy Loop
4:
    wfe
    b 4b
