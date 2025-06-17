use crate::sync::UPSafeCell;
use lazy_static::lazy_static;

const CLINT_BASE: usize = 0x2000000;
const CLINT_MTIME: usize = CLINT_BASE + 0xBFF8;
const CLINT_MTIMECMP: usize = CLINT_BASE + 0x4000;

/// Core Local Interruptor (CLINT) 结构体
pub struct Clint {}

impl Clint {
    /// 创建一个新的 Clint 实例
    pub fn new() -> Self {
        Self {}
    }

    /// 设置定时器
    pub fn set_timer(timer: usize) {
        let hart_id: usize;
        unsafe {
            core::arch::asm!("csrr {}, mhartid", out(reg) hart_id);
        }

        let mtimecmp_addr = CLINT_MTIMECMP + 8 * hart_id;
        unsafe {
            (mtimecmp_addr as *mut u64).write_volatile(timer as u64);
        }
    }
}

lazy_static! {
    /// Global instance of the CLINT (Core Local Interruptor)
    pub static ref CLINT: UPSafeCell<Clint> = unsafe {
        UPSafeCell::new(Clint::new())
    };
}