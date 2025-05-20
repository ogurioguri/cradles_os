use super::{FrameTracker, frame_alloc};
use super::{PTEFlags, PageTable, PageTableEntry};
use super::{PhysicalAddr, PhysicalPageNum, VirtualAddr, VirtualPageNum};
use super::{StepByOne, VPNRange};
use crate::config::{MEMORY_END,  PAGE_SIZE, TRAMPOLINE, TRAP_CONTEXT, USER_STACK_SIZE};
use crate::sync::UPSafeCell;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::arch::asm;
use lazy_static::*;
use riscv::register::satp;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MapType {
    Identical,
    Framed,
}

pub struct MapArea {
    vpn_range: VPNRange,
    data_frames: BTreeMap<VirtualPageNum, FrameTracker>,
    map_type: MapType,
    map_perm: MapPermission,
}