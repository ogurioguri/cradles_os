//! UART 驱动实现
#![allow(unused)]
use crate::sync::UPSafeCell;


// UART 寄存器地址常量（假设基地址为 0x10000000，QEMU virt 机器的 UART0 地址）
const UART_BASE: usize = 0x10000000;

// 寄存器偏移量
const RHR: usize = 0; // 接收保持寄存器
const THR: usize = 0; // 发送保持寄存器
const IER: usize = 1; // 中断使能寄存器
const FCR: usize = 2; // FIFO 控制寄存器
const LCR: usize = 3; // 线路控制寄存器

const LSR: usize = 5;

// 控制位
const LCR_BAUD_LATCH: u8 = 0x80; // 设置波特率模式
const LCR_EIGHT_BITS: u8 = 0x03; // 8位数据长度
const FCR_FIFO_ENABLE: u8 = 0x01; // 启用 FIFO
const FCR_FIFO_CLEAR: u8 = 0x06;  // 清空 FIFO
const IER_TX_ENABLE: u8 = 0x02;   // 启用发送中断
const IER_RX_ENABLE: u8 = 0x01;   // 启用接收中断

/// 向特定寄存器写入值
fn write_reg(reg: usize, val: u8) {
    let addr = UART_BASE + reg;
    unsafe {
        (addr as *mut u8).write_volatile(val);
    }
}

/// 从特定寄存器读取值
fn read_reg(reg: usize) -> u8 {
    let addr = UART_BASE + reg;
    unsafe {
        (addr as *const u8).read_volatile()
    }
}

/// 初始化 UART
pub fn uart_init() {
    // 禁用中断
    write_reg(IER, 0x00);

    // 设置波特率模式
    write_reg(LCR, LCR_BAUD_LATCH);

    // 设置波特率为 38.4K
    write_reg(0, 0x03);
    write_reg(1, 0x00);

    // 退出波特率设置模式，设置 8 位字长，无奇偶校验
    write_reg(LCR, LCR_EIGHT_BITS);

    // 重置并启用 FIFO
    write_reg(FCR, FCR_FIFO_ENABLE | FCR_FIFO_CLEAR);

    // 启用发送和接收中断
    write_reg(IER, IER_TX_ENABLE | IER_RX_ENABLE);
}

/// 通过 UART 输出一个字符
pub fn putchar(c: usize) {
    // 等待 UART 准备好发送
    loop {
        if (read_reg(5) & 0x20) != 0 {
            break;
        }
    }
    write_reg(THR, c as u8);
}

/// 从 UART 接收一个字符
pub fn getchar() -> usize {
    // 检查是否有数据可读
    if (read_reg(5) & 0x01) != 0 {
        read_reg(RHR) as usize
    } else {
       0 // 没有数据时返回特殊值
    }
}
/// 关机函数
pub fn shutdown(failure: bool) -> ! {
    // QEMU test finisher 设备地址
    const TEST_FINISHER_ADDR: usize = 0x100000;
    const TEST_FINISHER_PASS: u32 = 0x5555;
    const TEST_FINISHER_FAIL: u32 = 0x3333;

    // 等待 UART 发送完成
    loop {
        if (read_reg(LSR) & 0x20) != 0 {  // LSR_TX_IDLE
            break;
        }
    }

    // 使用 QEMU 的 test finisher 设备触发关机
    let finisher_addr = TEST_FINISHER_ADDR as *mut u32;
    let finisher_val = if failure { TEST_FINISHER_FAIL } else { TEST_FINISHER_PASS };

    unsafe {
        finisher_addr.write_volatile(finisher_val);
    }

    // 如果关机失败，进入低功耗等待状态
    loop {
        unsafe { core::arch::asm!("wfi"); }
    }
}
/// UART 驱动结构体
pub struct Uart{
    initialized: bool,
}

impl Uart {
    /// 创建一个新的 Uart 实例
    pub fn new() -> Self {
        // 初始化 UART
        Self {
            initialized: false,
        }
    }

    /// 初始化 UART
    pub fn init(&mut self) {
        if !self.initialized {
            uart_init();
            self.initialized = true;
        }
    }


    /// 通过 UART 输出一个字符
    pub fn putchar(&self, c: usize) {
        putchar(c);
    }

    /// 从 UART 接收一个字符
    pub fn getchar(&self) -> usize {
        getchar()
    }

    /// 关机函数
    pub fn shutdown(&self, failure: bool) -> ! {
        shutdown(failure)
    }
}

use lazy_static::lazy_static;


lazy_static! {
    /// 全局唯一的 UART 实例
    pub static ref UART: UPSafeCell<Uart> = unsafe {
        UPSafeCell::new(Uart::new())
    };
}

/// 初始化函数，应在主函数中调用
pub fn init() {
    UART.exclusive_access().init();
}