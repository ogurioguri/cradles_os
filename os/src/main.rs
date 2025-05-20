//! The main module and entrypoint
//!
//! Various facilities of the kernels are implemented as submodules. The most
//! important ones are:
//!
//! - [`trap`]: Handles all cases of switching from userspace to the kernel
//! - [`syscall`]: System call handling and implementation
//!
//! The operating system also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality. (See its source code for
//! details.)
//!
//! We then call [`_batch::run_next_app()`] and for the first time go to
//! userspace.


#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]

use core::arch::global_asm;
use log::*;

#[macro_use]
mod console;
mod lang_items;

/// 批处理系统模块
///
/// 负责应用程序的加载、初始化和执行控制。
/// 实现了简单的批处理系统功能，按顺序加载并运行用户
pub mod _batch;
mod sbi;

mod sync;
/// 陷入处理模块
///
/// 处理所有从用户空间切换到内核空间的情况，包括：
/// - 系统调用
/// - 异常
/// - 中断
pub mod trap;
/// 系统调用模块
///
/// 实现了基本的系统调用处理和功能，包括：
/// - write：用于输出
/// - exit：用于退出程序
pub mod syscall;

mod timer;

mod config;

mod loader;
pub mod task;
mod mm;

extern crate alloc;
extern crate bitflags;


global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));




/// clear BSS segment
fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

/// the rust entry-point of os
#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    // logging::init();
    info!("[kernel] Hello, world!");
    trap::init();
    loader::load_apps();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}