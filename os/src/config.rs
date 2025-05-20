//! Constants used in rCore

pub const MAX_APP_NUM: usize = 4;

pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;

pub const CLOCK_FREQ: usize = 12500000;

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SIZE_BITS: usize = 12;

/// the end of kernel memory
pub const MEMORY_END: usize = 0x80800000;