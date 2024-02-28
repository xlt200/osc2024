pub const STACK_SIZE: usize = 4 * 1024 * 1024;
pub const STACK_LOW_POS: usize = 0x0;
#[no_mangle]
pub static STACK_HIGH_POS: usize = 0x80000;
