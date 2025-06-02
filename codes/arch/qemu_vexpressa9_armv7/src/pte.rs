use mmi::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use mmi::MapPermission;
use crate::config::*;

// bitflags! {
//     pub struct PTEFlags: u16 {
//         // const V = 1 << 0;
//         const R = 1 << 0;
//         const W = 1 << 1;
//         const X = 1 << 2;
//         const U = 1 << 3;
//         const D = 1 << 4;
//         const O = 1 << 9; //copy on write
//     }
// }


bitflags::bitflags! {
    /// Memory attribute fields in the VMSAv8-64 translation table format descriptors.
    pub struct PTEFlags: usize {
        // Attribute fields in stage 1 VMSAv8-64 Block and Page descriptors:
        
        /// The execute-never field.
        const XN =         1 <<  0;

        /// Whether the descriptor is valid.
        const VALID_PAGETABLE_PAGE =       0b01 << 0;

        const VALID_PAGEFRAME_PAGE =       0b10 << 0;

        const BUFFERABLE =  1 << 2;

        const CACHEABLE =   1 << 3;

        const AP_BOTH_FA =  0b01 << 4;

        const AP_KERN_FA  =  0b10 << 4;

        const AP_BOTH_RO  =  0b11 << 4;

        /// No use. (Should be Zero)
        const SBZ  =  0b111 << 6;

        /// Memory attributes index field.
        const AP3    = 1 << 9;

        /// Memory attributes index field.
        const SHARED_BY_CORE    = 1 << 10;

        /// The not global bit.
        const NG =          1 << 11;

    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum MemType {
    Device = 0,
    Normal = 1,
}

impl PTEFlags {

    const fn from_mem_type(mem_type: MemType) -> Self {
        let mut bits = Self::VALID_PAGEFRAME_PAGE.bits();
        if matches!(mem_type, MemType::Normal) {
            //bits |= Self::CACHEABLE.bits();
        }else{
            //bits |= Self::BUFFERABLE.bits();
        }
        Self::from_bits_truncate(bits)
    }

    fn mem_type(&self) -> MemType {
        let idx = (self.bits() & Self::CACHEABLE.bits()) == 0;
        match idx {
            true => MemType::Device,
            false => MemType::Normal,
            _ => panic!("Invalid memory attribute index"),
        }
    }
}


impl From<PTEFlags> for MapPermission {
    fn from(attr: PTEFlags) -> Self {
        let mut mp = Self::empty();
        if !attr.contains(PTEFlags::VALID_PAGEFRAME_PAGE) &&
            !attr.contains(PTEFlags::VALID_PAGETABLE_PAGE){
            return mp;
        }

        if attr.contains(PTEFlags::AP_BOTH_RO) {
            mp |= Self::R;
        } else
        if attr.contains(PTEFlags::AP_KERN_FA) {
            mp |= Self::R | Self::W;
        }else
        if attr.contains(PTEFlags::AP_BOTH_FA) {
            mp |= Self::R | Self::W | Self::U;
        }

        if !attr.contains(PTEFlags::XN) {
            mp |= Self::X;
        }

        if attr.mem_type() == MemType::Device {
            mp |= Self::D;
        }
        mp
    }
}

impl From<MapPermission> for PTEFlags {
    fn from(flags: MapPermission) -> Self {
        let mut attr = if flags.contains(MapPermission::D) {
            Self::from_mem_type(MemType::Device)
        } else {
            Self::from_mem_type(MemType::Normal)
        };

        if !flags.contains(MapPermission::W) {
            attr |= Self::AP_BOTH_RO;
        }else if !flags.contains(MapPermission::U) {
            attr |= Self::AP_KERN_FA;
        }else {
            attr |= Self::AP_BOTH_FA;
        }

        if !flags.contains(MapPermission::X) {
            attr |= Self::XN;
        }

        attr
    }
}



#[derive(Copy, Clone)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl From<usize> for PageTableEntry{
    fn from(a: usize) -> Self{
        PageTableEntry { bits: a as usize }
    }
}
impl From<PageTableEntry> for usize{
    fn from(a: PageTableEntry) -> Self{
        a.bits
    }
}

impl PageTableEntry {
    const PHYS_ADDR_MASK: usize = 0xfffff000;

    pub fn new_page(paddr: PhysAddr, flags: MapPermission, is_block: bool) -> Self {
        let mut attr = PTEFlags::from(flags) | PTEFlags::VALID_PAGEFRAME_PAGE;
        if is_block {
            panic!("block not supported.");
        }
        Self{bits: attr.bits() | (paddr.0 & Self::PHYS_ADDR_MASK) as usize}
    }

    pub fn new_table(paddr: PhysAddr) -> Self {
        let attr = PTEFlags::VALID_PAGETABLE_PAGE;
        Self{bits: attr.bits() | (paddr.0 & Self::PHYS_ADDR_MASK) as usize}
    }

    // //Interface to MMK
    // //Note: Empty MapPermission means a page table pages.
    // pub fn new(pa: PhysAddr, flags: MapPermission, is_block: bool) -> Self {
        
    //     let mut attr = PTEFlags::from(flags) | PTEFlags::AF | PTEFlags::VALID;

    //     if !is_block {
    //         attr |= PTEFlags::NON_BLOCK;
    //     }
        

    //     if (flags & MapPermission::RWX) != MapPermission::empty() {
    //         attr |= PTEFlags::INNER;
    //     }

    //     if (flags & MapPermission::RWX) == MapPermission::empty() {
    //         attr = PTEFlags::VALID | PTEFlags::NON_BLOCK;
    //     }

    //     Self{bits: attr.bits | (pa.0 & Self::PHYS_ADDR_MASK) as u64}
    // }

    //Interface to MMK
    pub const fn empty() -> Self {
        PageTableEntry {
            bits: 0
        }
    }

    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits_truncate(self.bits).into()
    }
    
    //Interface to MMK
    pub fn ppn(&self) -> PhysPageNum {
        PhysAddr{0: self.bits as usize & Self::PHYS_ADDR_MASK}.into()
    }
    pub fn paddr(&self) -> PhysAddr {
        PhysAddr{0: self.bits as usize & Self::PHYS_ADDR_MASK}
    }

    //Interface to MMK
    pub fn perm(&self) -> MapPermission {
        self.flags().into()
    }

    //Interface to MMK
    pub fn valid(&self) -> bool {
        self.flags() & PTEFlags{bits: 0b11} != PTEFlags::empty()
    }

    //Interface to MMK
    pub fn is_block(&self) -> bool {
        false
    }

    //Interface to MMK
    pub fn set_perm(&mut self, flags: MapPermission) {
        let new_flags: PTEFlags = flags.into();
        self.bits = (self.bits & Self::PHYS_ADDR_MASK as usize) as usize | new_flags.bits();
    }

}
