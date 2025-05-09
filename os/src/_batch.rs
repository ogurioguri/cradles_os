// use crate::sbi::shutdown;
// use crate::sync::UPSafeCell;
// use core::arch::asm;
// use lazy_static::*;
// use crate::println;
// use crate::trap::TrapContext;
// 
// const MAX_APP_NUM: usize = 16;
// const APP_BASE_ADDRESS: usize = 0x80400000;
// const APP_SIZE_LIMIT: usize = 0x20000;
// const USER_STACK_SIZE: usize = 4096 * 2;
// const KERNEL_STACK_SIZE: usize = 4096 * 2;
// struct AppManager {
//     num_app: usize,
//     current_app: usize,
//     app_start: [usize; MAX_APP_NUM + 1],
// }
// 
// impl AppManager {
//     pub fn print_app_info(&self) {
//         println!("[kernel] num_app = {}", self.num_app);
//         for i in 0..self.num_app {
//             println!(
//                 "[kernel] app {} start addr = {:#x}",
//                 i,
//                 self.app_start[i + 1]
//             );
//         }
//     }
// 
//     pub fn get_current_app(&self) -> usize {
//         self.current_app
//     }
//     
//     
//     pub fn move_to_next_app(&mut self) {
//         self.current_app += 1;
//         if self.current_app >= self.num_app {
//             println!("[kernel] All applications completed!");
//             shutdown(false);
//         }
//     }
// 
//     fn load_app(&self, app_id: usize) {
//         if app_id >= self.num_app {
//             println!("All applications completed!");
//             shutdown(false);
//         }
//         println!("[kernel] Loading app_{}", app_id);
//         unsafe {
//             // clear app area
//             core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
//             let app_src = core::slice::from_raw_parts(
//                 self.app_start[app_id] as *const u8,
//                 self.app_start[app_id + 1] - self.app_start[app_id],
//             );
//             let app_dst =
//                 core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
//             app_dst.copy_from_slice(app_src);
//             asm!("fence.i");
//         }
//     }
// }
// 
// //make the appmanager can be used in global
// lazy_static! {
//     static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
//         UPSafeCell::new({
//             unsafe extern "C" {
//                 safe fn _num_app();
//             }
//             let num_app_ptr = _num_app as usize as *const usize;
//             let num_app = num_app_ptr.read_volatile();
//             let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
//             let app_start_raw: &[usize] =
//                 core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
//             app_start[..=num_app].copy_from_slice(app_start_raw);
//             AppManager {
//                 num_app,
//                 current_app: 0,
//                 app_start,
//             }
//         })
//     };
// }
// 
// 
// 
// #[repr(align(4096))]
// struct KernelStack {
//     data: [u8; KERNEL_STACK_SIZE],
// }
// 
// #[repr(align(4096))]
// struct UserStack {
//     data: [u8; USER_STACK_SIZE],
// }
// 
// static KERNEL_STACK: KernelStack = KernelStack { data: [0; KERNEL_STACK_SIZE] };
// static USER_STACK: UserStack = UserStack { data: [0; USER_STACK_SIZE] };
// 
// 
// impl KernelStack {
//     fn get_sp(&self) -> usize {
//         self.data.as_ptr() as usize + KERNEL_STACK_SIZE
//     }
//     pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
//         let cx_ptr = (self.get_sp() - size_of::<TrapContext>()) as *mut TrapContext;
//         unsafe {
//             *cx_ptr = cx;
//         }
//         unsafe { cx_ptr.as_mut().unwrap() }
//     }
// }
// 
// impl UserStack {
//     fn get_sp(&self) -> usize {
//         self.data.as_ptr() as usize + USER_STACK_SIZE
//     }
// }
// 
// /// make the kernel stack and user stack can be used in global
// pub fn init () {
//     APP_MANAGER.exclusive_access().print_app_info();
// }
// 
// ///load the next application and run it
// // pub fn run_next_app() -> ! {
// //     let mut app_manager = APP_MANAGER.exclusive_access();
// //     let current_app = app_manager.get_current_app();
// //     app_manager.load_app(current_app);
// //     app_manager.move_to_next_app();
// //     drop(app_manager);
// //     // before this we have to drop local variables related to resources manually
// //     // and release the resources
// //     unsafe extern "C" {
// //         fn __restore(cx_addr: usize);
// //     }
// //     // restore the context in the kernel stack
// //     unsafe {
// //         __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
// //             APP_BASE_ADDRESS,
// //             USER_STACK.get_sp(),
// //         )) as *const _ as usize);
// //         panic!("Should not reach here!");
// //     }
// // }