use aarch64_cpu::asm;

#[inline(always)]
pub fn spin_for_cycle(cycle: usize) {
    for _ in 0..cycle {
        asm::nop();
    }
}
