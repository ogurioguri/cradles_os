//! Constants used in rCore

// pub const MAX_APP_NUM: usize = 4;

pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;

pub const CLOCK_FREQ: usize = 12500000;

pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SIZE_BITS: usize = 12;

/// the end of kernel memory
pub const MEMORY_END: usize = 0x80800000;

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

/// Return (bottom, top) of a kernel stack in kernel space.
pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE); //guard page
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}

pub use crate::board::{MMIO};