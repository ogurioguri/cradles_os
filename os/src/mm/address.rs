use crate::config::{PAGE_SIZE, PAGE_SIZE_BITS};
use super::PageTableEntry;
use core::fmt::{self, Debug, Formatter};



#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysicalAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtualAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysicalPageNum(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct VirtualPageNum(pub usize);



impl PhysicalAddr {
    pub fn page_num_floor(&self) -> PhysicalPageNum { PhysicalPageNum(self.0 / PAGE_SIZE) }
    pub fn page_num_ceil(&self) -> PhysicalPageNum { PhysicalPageNum((self.0 + PAGE_SIZE - 1) / PAGE_SIZE) }
    
}

impl PhysicalPageNum {

    ///页表项为单位，多级页表中的一个页表项
    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysicalAddr = self.clone().into();
        unsafe {
            core::slice::from_raw_parts_mut(pa.0 as *mut PageTableEntry, 512)
        }
    }
    ///字节为单位
    pub fn get_bytes_array(&self) -> &'static mut [u8] {
        let pa: PhysicalAddr = (*self).into();
        unsafe { core::slice::from_raw_parts_mut(pa.0 as *mut u8, PAGE_SIZE) }
    }
    pub fn get_mut<T>(&self) -> &'static mut T {
        let pa: PhysicalAddr = (*self).into();
        unsafe { (pa.0 as *mut T).as_mut().unwrap() }
    }
}
const PA_WIDTH_SV39: usize = 56;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;



impl From<usize> for PhysicalAddr {
    fn from(v: usize) -> Self { Self(v & ( (1 << PA_WIDTH_SV39) - 1 )) }
}
impl From<usize> for PhysicalPageNum {
    fn from(v: usize) -> Self { Self(v & ( (1 << PPN_WIDTH_SV39) - 1 )) }
}

impl From<PhysicalAddr> for usize {
    fn from(v: PhysicalAddr) -> Self { v.0 }
}
impl From<PhysicalPageNum> for usize {
    fn from(v: PhysicalPageNum) -> Self { v.0 }
}

impl PhysicalAddr {
    pub fn page_offset(&self) -> usize { self.0 & (PAGE_SIZE - 1) }
}

impl From<PhysicalAddr> for PhysicalPageNum {
    fn from(v: PhysicalAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.page_num_floor()
    }
}

impl From<PhysicalPageNum> for PhysicalAddr {
    fn from(v: PhysicalPageNum) -> Self { Self(v.0 << PAGE_SIZE_BITS) }
}

impl VirtualPageNum {
    pub fn indexes(&self) -> [usize; 3] {
        let mut vpn = self.0;
        let mut idx = [0usize; 3];
        for i in (0..3).rev() {
            idx[i] = vpn & 511;
            vpn >>= 9;
        }
        idx
    }
}
pub trait StepByOne {
    fn step(&mut self);
}
impl StepByOne for VirtualPageNum {
    fn step(&mut self) {
        self.0 += 1;
    }
}

#[derive(Copy, Clone)]
/// a simple range structure for type T
pub struct SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    l: T,
    r: T,
}
impl<T> SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    pub fn new(start: T, end: T) -> Self {
        assert!(start <= end, "start {:?} > end {:?}!", start, end);
        Self { l: start, r: end }
    }
    pub fn get_start(&self) -> T {
        self.l
    }
    pub fn get_end(&self) -> T {
        self.r
    }
}
impl<T> IntoIterator for SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    type Item = T;
    type IntoIter = SimpleRangeIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        SimpleRangeIterator::new(self.l, self.r)
    }
}
/// iterator for the simple range structure
pub struct SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    current: T,
    end: T,
}
impl<T> SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    pub fn new(l: T, r: T) -> Self {
        Self { current: l, end: r }
    }
}
impl<T> Iterator for SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let t = self.current;
            self.current.step();
            Some(t)
        }
    }
}
pub type VPNRange = SimpleRange<VirtualPageNum>;

