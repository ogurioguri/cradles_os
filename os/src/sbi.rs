#![allow(dead_code)]
use crate::drivers::uart::UART;
use crate::drivers::clint::Clint;


pub fn old_shutdown(failure: bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}



pub fn _console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}




pub fn _set_timer(timer: usize) {
    Clint::set_timer(timer);
}

// pub fn old_console_getchar() -> usize {
//     #[allow(deprecated)]
//     sbi_rt::legacy::console_getchar()
// }

pub fn shutdown(failure: bool) -> ! {
    UART.exclusive_access().shutdown(failure)
}

pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    UART.exclusive_access().putchar(c);
}

pub fn console_getchar() -> usize {
    #[allow(deprecated)]
    UART.exclusive_access().getchar()
}

pub fn _console_getchar() -> usize {
    #[allow(deprecated)]
    sbi_rt::legacy::console_getchar()
}



pub fn set_timer(timer: usize) {
    use sbi_rt::set_timer;
    //类型推断
    set_timer(timer as _);
}