//! App management syscalls
// use crate::batch::run_next_app;

use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};

use crate::timer::get_time_ms;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("exist_current_and_run_next() should never return!");
}

/// 功能：获取当前的时间，保存在 TimeVal 结构体 ts 中，_tz 在我们的实现中忽略
/// 返回值：返回是否执行成功，成功则返回 0
/// syscall ID：169
pub fn sys_get_time() -> isize{
    get_time_ms() as isize
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}