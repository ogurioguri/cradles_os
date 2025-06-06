//! Task management implementation
//!
//! Everything about task management, like starting and switching tasks is
//! implemented here.
//!
//! A single global instance of [`TaskManager`] called `TASK_MANAGER` controls
//! all the tasks in the operating system.
//!
//! Be careful when you see `__switch` ASM function in `switch.S`. Control flow around this function
//! might not be what you expect.

mod context;
mod switch;

#[allow(clippy::module_inception)]
mod task;
mod pid;
mod manager;
mod processor;

use crate::loader::get_app_data_by_name;
use crate::sbi::shutdown;
use alloc::sync::Arc;
use lazy_static::*;
pub use manager::{TaskManager, fetch_task};
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

pub use context::TaskContext;
pub use manager::add_task;
pub use pid::{KernelStack, PidAllocator, PidHandle, pid_alloc};
pub use processor::{
    Processor, current_task, current_trap_cx, current_user_token, run_tasks, schedule,
    take_current_task,
};

/// The task manager, where all the tasks are managed.
///
/// Functions implemented on `TaskManager` deals with all task state transitions
/// and task context switching. For convenience, you can find wrappers around it
/// in the module level.
///
/// Most of `TaskManager` are hidden behind the field `inner`, to defer
/// borrowing checks to runtime. You can see examples on how to use `inner` in
/// existing functions on `TaskManager`.
// pub struct TaskManager {
//     /// total number of tasks
//     num_app: usize,
//     /// use inner value to get mutable access
//     inner: UPSafeCell<TaskManagerInner>,
// }
// 
// struct TaskManagerInner {
//     /// task list
//     tasks: Vec<TaskControlBlock>,
//     /// id of current `Running` task
//     current_task: usize,
// }
// 
// lazy_static! {
//     /// a `TaskManager` global instance through lazy_static!
//     pub static ref TASK_MANAGER: TaskManager = {
//         println!("init TASK_MANAGER");
//         let num_app = get_num_app();
//         println!("num_app = {}", num_app);
//         let mut tasks: Vec<TaskControlBlock> = Vec::new();
//         for i in 0..num_app {
//             println!("now_init_app = {}", i);
//             tasks.push(TaskControlBlock::new(get_app_data(i), i));
//         }
//         TaskManager {
//             num_app,
//             inner: unsafe {
//                 UPSafeCell::new(TaskManagerInner {
//                     tasks,
//                     current_task: 0,
//                 })
//             },
//         }
//     };
// }
// 
// 
// 
// 
// 
// impl TaskManager {
//     /// Run the first task in task list.
//     ///
//     /// Generally, the first task in task list is an idle task (we call it zero process later).
//     /// But in ch3, we load apps statically, so the first task is a real app.
//     fn run_first_task(&self) -> ! {
//         let mut inner = self.inner.exclusive_access();
//         let task0 = &mut inner.tasks[0];
//         task0.task_status = TaskStatus::Running;
//         let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
//         drop(inner);
//         let mut _unused = TaskContext::zero_init();
//         // before this, we should drop local variables that must be dropped manually
//         unsafe {
//             __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
//         }
//         panic!("unreachable in run_first_task!");
//     }
// 
//     ///Change the current 'Running' task's program break
//     pub fn change_current_program_brk(&self, size: i32) -> Option<usize> {
//         let mut inner = self.inner.exclusive_access();
//         let cur = inner.current_task;
//         inner.tasks[cur].change_program_brk(size)
//     }
// 
//     /// Change the status of current `Running` task into `Ready`.
//     fn mark_current_suspended(&self) {
//         let mut inner = self.inner.exclusive_access();
//         let current = inner.current_task;
//         inner.tasks[current].task_status = TaskStatus::Ready;
//     }
// 
//     /// Change the status of current `Running` task into `Exited`.
//     fn mark_current_exited(&self) {
//         let mut inner = self.inner.exclusive_access();
//         let current = inner.current_task;
//         inner.tasks[current].task_status = TaskStatus::Exited;
//     }
// 
//     /// Find next task to run and return task id.
//     ///
//     /// In this case, we only return the first `Ready` task in task list.
//     fn find_next_task(&self) -> Option<usize> {
//         let inner = self.inner.exclusive_access();
//         let current = inner.current_task;
//         (current + 1..current + self.num_app + 1)
//             .map(|id| id % self.num_app)
//             .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
//     }
// 
//     /// Switch current `Running` task to the task we have found,
//     /// or there is no `Ready` task and we can exit with all applications completed
//     fn run_next_task(&self) {
//         if let Some(next) = self.find_next_task() {
//             let mut inner = self.inner.exclusive_access();
//             let current = inner.current_task;
//             inner.tasks[next].task_status = TaskStatus::Running;
//             inner.current_task = next;
//             let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
//             let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
//             if !next_task_cx_ptr.is_null() && !current_task_cx_ptr.is_null() {
//                 drop(inner);
//                 // before this, we should drop local variables that must be dropped manually
//                 unsafe {
//                     // println!("Switching from task {} to task {}", current, next);
//                     // println!("current task cx ptr: {:#x}", current_task_cx_ptr as usize);
//                     // println!("next task cx ptr: {:#x}", next_task_cx_ptr as usize);
//                     __switch(current_task_cx_ptr, next_task_cx_ptr);
//                     // println!("finish task {}", current);
//                 }
//                 // go back to user mode
//             }else {
//                 drop(inner);
//                 shutdown(false);
//             }
//         } else {
//             println!("All applications completed!");
//             shutdown(false);
//         }
// 
//     }
//     /// Get the current 'Running' task's token.
//     fn get_current_token(&self) -> usize {
//         let inner = self.inner.exclusive_access();
//         inner.tasks[inner.current_task].get_user_token()
//     }
// 
//     /// Get the current 'Running' task's trap contexts.
//     fn get_current_trap_cx(&self) -> &'static mut TrapContext {
//         let inner = self.inner.exclusive_access();
//         inner.tasks[inner.current_task].get_trap_cx()
//     }
// }
// 
// /// run first task
// pub fn run_first_task() {
//     TASK_MANAGER.run_first_task();
// }
// 
// /// rust next task
// fn run_next_task() {
//     TASK_MANAGER.run_next_task();
// }
// 
// /// suspend current task
// fn mark_current_suspended() {
//     TASK_MANAGER.mark_current_suspended();
// }
// 
// /// exit current task
// fn mark_current_exited() {
//     TASK_MANAGER.mark_current_exited();
// }
// 
// /// suspend current task, then run next task
// pub fn suspend_current_and_run_next() {
//     mark_current_suspended();
//     run_next_task();
// }
// 
// /// exit current task,  then run next task
// pub fn exit_current_and_run_next() {
//     mark_current_exited();
//     run_next_task();
// }
// 
// /// get current task's token
// pub fn current_user_token() -> usize {
//     TASK_MANAGER.get_current_token()
// }
// 
// 
// /// get current task's trap context
// pub fn current_trap_cx() -> &'static mut TrapContext {
//     TASK_MANAGER.get_current_trap_cx()
// }
// 
// /// Change the current 'Running' task's program break
// pub fn change_program_brk(size: i32) -> Option<usize> {
//     TASK_MANAGER.change_current_program_brk(size)
// }

pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = take_current_task().unwrap();

    // ---- access current TCB exclusively
    let mut task_inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    // ---- release current PCB

    // push back to ready queue.
    add_task(task);
    // jump to scheduling cycle
    schedule(task_cx_ptr);
}

/// pid of usertests app in make run TEST=1
pub const IDLE_PID: usize = 0;

/// Exit the current 'Running' task and run the next task in task list.
pub fn exit_current_and_run_next(exit_code: i32) {
    // take from Processor
    let task = take_current_task().unwrap();

    let pid = task.getpid();
    if pid == IDLE_PID {
        println!(
            "[kernel] Idle process exit with exit_code {} ...",
            exit_code
        );
        if exit_code != 0 {
            //crate::sbi::shutdown(255); //255 == -1 for err hint
            shutdown(true)
        } else {
            //crate::sbi::shutdown(0); //0 for success hint
            shutdown(false)
        }
    }

    // **** access current TCB exclusively
    let mut inner = task.inner_exclusive_access();
    // Change status to Zombie
    inner.task_status = TaskStatus::Zombie;
    // Record exit code
    inner.exit_code = exit_code;
    // do not move to its parent but under initproc

    // ++++++ access initproc TCB exclusively
    {
        let mut initproc_inner = INITPROC.inner_exclusive_access();
        for child in inner.children.iter() {
            child.inner_exclusive_access().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
    }
    // ++++++ release parent PCB

    inner.children.clear();
    // deallocate user space
    inner.memory_set.recycle_data_pages();
    drop(inner);
    // **** release current PCB
    // drop task manually to maintain rc correctly
    drop(task);
    // we do not have to save task context
    let mut _unused = TaskContext::zero_init();
    schedule(&mut _unused as *mut _);
}

lazy_static! {
    ///Globle process that init user shell
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(TaskControlBlock::new(
        get_app_data_by_name("initproc").unwrap()
    ));
}
///Add init process to the manager
pub fn add_initproc() {
    add_task(INITPROC.clone());
}