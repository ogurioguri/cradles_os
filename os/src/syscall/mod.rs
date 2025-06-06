//! 系统调用模块
//!
//! 本模块负责处理所有从用户空间应用程序发起的系统调用请求。
//! 模块结构：
//! - `fs`：文件系统相关系统调用，如 `sys_write`
//! - `process`：进程管理相关系统调用，如 `sys_exit`、`sys_yield` 等
//!
//! 系统调用是用户程序与操作系统内核交互的唯一接口，
//! 提供了一系列受控的内核服务访问，包括输入/输出操作、
//! 进程控制和时间查询等功能。

mod fs;
mod process;

use fs::*;
use process::*;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

const SYSCALL_YIELD: usize = 124;
const SYSCALL_SBRK: usize = 214;
const SYSCALL_GET_TIME: usize = 169;

const SYSCALL_GETPID: usize = 172;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAITPID: usize = 260;
const SYSCALL_READ: usize = 63;

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_READ => sys_read(args[0], args[1] as *const u8, args[2]),
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(),
        SYSCALL_GETPID => sys_getpid(),
        SYSCALL_FORK => sys_fork(),
        SYSCALL_EXEC => sys_exec(args[0] as *const u8),
        SYSCALL_WAITPID => sys_waitpid(args[0] as isize, args[1] as *mut i32),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}