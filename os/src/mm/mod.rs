mod address;
mod page_table;
mod heap_allocator;
mod frame_allocator;
mod memory_set;

pub use address::{PhysicalAddr, PhysicalPageNum, VirtualAddr, VirtualPageNum};
use address::{StepByOne, VPNRange};
pub use frame_allocator::{FrameTracker, frame_alloc};
pub use memory_set::remap_test;
pub use memory_set::{KERNEL_SPACE, MapPermission, MemorySet};
use page_table::{PTEFlags, PageTable};
pub use page_table::{PageTableEntry, translated_byte_buffer, translated_refmut, translated_str};





pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}