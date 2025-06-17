#![allow(dead_code)]
#[cfg(feature = "use_spin")]
use spin::Mutex;
#[cfg(feature = "use_spin")]
use core::ops::Deref;

extern crate alloc;

use core::alloc::{Layout};
use core::alloc::GlobalAlloc;
use core::cmp::{max, min};
use core::ptr::NonNull;
use crate::console::print;
use super::LinkedList;


pub struct Heap {
    // buddy system with max order of 32
    free_list: [LinkedList; 32],

    // statistics
    user: usize,
    allocated: usize,
    total: usize,
}

impl Heap {
    pub const fn new() -> Self {
        Heap {
            free_list: [LinkedList::new(); 32],
            user: 0,
            allocated: 0,
            total: 0,
        }
    }

    pub const fn empty() -> Self {
        Self::new()
    }

    /// Add a range of memory [start, end) to the heap
    pub unsafe fn init(&mut self, start: usize, size: usize){
        self.add(start, start + size);
    }



    pub unsafe fn add(&mut self, mut start: usize, mut end: usize) {
        // Ensure start and end are aligned to the size of usize
        start = (start + size_of::<usize>() - 1) & (!size_of::<usize>() + 1);
        end = end & (!size_of::<usize>() + 1);
        if start >= end {
            panic!("Invalid memory region: start = {:#x}, end = {:#x}", start, end);
        }

        let size = end - start;
        let order = size.trailing_zeros() as usize;
        if order >= 32 {
            panic!("Memory region too large: size = {:#x}, order = {}", size, order);
        }

        // println!("1");

        let mut total = 0;
        let mut current_start = start;

        while current_start + size_of::<usize>() <= end {
            let lowbit = current_start & (!current_start + 1);
            let size = min(lowbit, prev_power_of_two(end - current_start));
            total += size;

            // self.free_list[size.trailing_zeros() as usize].push(current_start as *mut usize);
            // println!("2");
            // self.free_list[size.trailing_zeros() as usize]
            //     .get_or_insert_with(|| alloc::vec::Vec::new());
            // 在 add 方法中修改为分两步操作
            let idx = size.trailing_zeros() as usize;

            // if self.free_list[idx].is_none() {
            //     self.free_list[idx] = Some(alloc::vec::Vec::new());
            // }
            // if let Some(list) = &mut self.free_list[idx] {
            //     list.push(current_start);
            // }
            self.free_list[idx].push(current_start as *mut usize);

            current_start += size;
            // println!("2");
        }

        self.total += total;
    }

    /// Dealloc a range of memory from the heap
    pub fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), size_of::<usize>()),
        );
        let class = size.trailing_zeros() as usize;

        unsafe {
            // 将释放的内存块放回适当大小的空闲链表

            // self.free_list[class]
            //     .get_or_insert_with(|| alloc::vec::Vec::new())
            //     .push(ptr.as_ptr() as usize);

            self.free_list[class].push(ptr.as_ptr() as *mut usize);

            // 合并空闲的伙伴块
            let mut current_ptr = ptr.as_ptr() as usize;
            let mut current_class = class;
            while current_class < self.free_list.len() {
                let buddy = current_ptr ^ (1 << current_class);
                let mut flag = false;

                // 确保 free_list[current_class] 存在
                // if let Some(list) = &mut self.free_list[current_class] {
                //     // 查找伙伴块在列表中的位置
                //     if let Some(idx) = list.iter().position(|&addr| addr == buddy) {
                //         // 找到伙伴块，从列表中移除
                //         list.swap_remove(idx);
                //         flag = true;
                //     }
                // }
                for block in self.free_list[current_class].iter_mut() {
                    if block.value() as usize == buddy {
                        block.pop();
                        flag = true;
                        break;
                    }
                }

                // 找到伙伴块，需要合并
                if flag {
                    // 从当前链表中删除当前块
                    // if let Some(list) = &mut self.free_list[current_class] {
                    //     if let Some(idx) = list.iter().position(|&addr| addr == current_ptr) {
                    //         list.swap_remove(idx);
                    //     }
                    // }
                    self.free_list[current_class].pop();

                    // 合并块并移至更高级别的链表
                    current_ptr = min(current_ptr, buddy);
                    current_class += 1;

                    // self.free_list[current_class]
                    //     .get_or_insert_with(|| alloc::vec::Vec::new())
                    //     .push(current_ptr);

                    self.free_list[current_class].push(current_ptr as *mut usize);

                } else {
                    break;
                }
            }
        }

        self.user -= layout.size();
        self.allocated -= size;
    }
    /// Alloc a range of memory from the heap satifying `layout` requirements
    pub fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), size_of::<usize>()),
        );
        let class = size.trailing_zeros() as usize;

        for i in class..self.free_list.len() {
            // 检查当前大小类别是否有空闲块
            // if let Some(list) = &self.free_list[i] {
            //     if !list.is_empty() {
            //         // 分割大块为小块
            //         for j in (class + 1..i + 1).rev() {
            //             if let Some(block) = self.free_list[j].as_mut().and_then(|list| list.pop()) {
            //                 // 计算伙伴块地址
            //                 let buddy_addr = block + (1 << (j - 1));
            //
            //                 // 将分割的两块加入较小尺寸的空闲列表
            //                 self.free_list[j - 1]
            //                     .get_or_insert_with(|| alloc::vec::Vec::new())
            //                     .push(buddy_addr);
            //                 self.free_list[j - 1]
            //                     .get_or_insert_with(|| alloc::vec::Vec::new())
            //                     .push(block);
            //             } else {
            //                 return Err(());
            //             }
            //         }
            //
            //         // 从对应尺寸类别中取出一个块
            //         if let Some(block) = self.free_list[class].as_mut().and_then(|list| list.pop()) {
            //             if let Some(result) = NonNull::new(block as *mut u8) {
            //                 self.user += layout.size();
            //                 self.allocated += size;
            //                 return Ok(result);
            //             }
            //         }
            //
            //         return Err(());
            //     }
            // }

            if !self.free_list[i].is_empty() {
                // Split buffers
                for j in (class + 1..i + 1).rev() {
                    if let Some(block) = self.free_list[j].pop() {
                        unsafe {
                            self.free_list[j - 1]
                                .push((block as usize + (1 << (j - 1))) as *mut usize);
                            self.free_list[j - 1].push(block);
                        }
                    } else {
                        return Err(());
                    }
                }

                let result = NonNull::new(
                    self.free_list[class]
                        .pop()
                        .expect("current block should have free space now")
                        as *mut u8,
                );
                if let Some(result) = result {
                    self.user += layout.size();
                    self.allocated += size;
                    return Ok(result);
                } else {
                    return Err(());
                }
            }
        }

        Err(())
    }

    /// Return the number of bytes that user requests
    pub fn stats_alloc_user(&self) -> usize {
        self.user
    }

    /// Return the number of bytes that are actually allocated
    pub fn stats_alloc_actual(&self) -> usize {
        self.allocated
    }

    /// Return the total number of bytes in the heap
    pub fn stats_total_bytes(&self) -> usize {
        self.total
    }
}

pub fn prev_power_of_two(num: usize) -> usize {
    1 << (8 * (size_of::<usize>()) - num.leading_zeros() as usize - 1)
}



#[cfg(feature = "use_spin")]
pub struct LockedHeap(Mutex<Heap>);

#[cfg(feature = "use_spin")]
impl LockedHeap {
    /// Creates an empty heap
    pub const fn new() -> LockedHeap {
        LockedHeap(Mutex::new(Heap::new()))
    }

    /// Creates an empty heap
    pub const fn empty() -> LockedHeap {
        LockedHeap(Mutex::new(Heap::new()))
    }
}



#[cfg(feature = "use_spin")]
unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock()
            .alloc(layout)
            .ok()
            .map_or(0 as *mut u8, |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            self.0.lock().dealloc(NonNull::new_unchecked(ptr), layout)
        }
    }
}

#[cfg(feature = "use_spin")]
impl Deref for LockedHeap {
    type Target = Mutex<Heap>;

    fn deref(&self) -> &Mutex<Heap> {
        &self.0
    }
}