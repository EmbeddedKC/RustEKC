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
    pub struct PTEFlags: u64 {
        // Attribute fields in stage 1 VMSAv8-64 Block and Page descriptors:

        /// Whether the descriptor is valid.
        const VALID =       1 << 0;
        /// The descriptor gives the address of the next level of translation table or 4KB page.
        /// (not a 2M, 1G block)
        const NON_BLOCK =   1 << 1;
        /// Memory attributes index field.
        const ATTR_INDX =   0b111 << 2;
        /// Non-secure bit. For memory accesses from Secure state, specifies whether the output
        /// address is in Secure or Non-secure memory.
        const NS =          1 << 5;
        /// Access permission: accessable at EL0.
        const AP_EL0 =      1 << 6;
        /// Access permission: read-only.
        const AP_RO =       1 << 7;
        /// Shareability: Inner Shareable (otherwise Outer Shareable).
        const INNER =       1 << 8;
        /// Shareability: Inner or Outer Shareable (otherwise Non-shareable).
        const SHAREABLE =   1 << 9;
        /// The Access flag.
        const AF =          1 << 10;
        /// The not global bit.
        const NG =          1 << 11;
        /// Indicates that 16 adjacent translation table entries point to contiguous memory regions.
        const CONTIGUOUS =  1 <<  52;
        /// The Privileged execute-never field.
        const PXN =         1 <<  53;
        /// The Execute-never or Unprivileged execute-never field.
        const UXN =         1 <<  54;

        // Next-level attributes in stage 1 VMSAv8-64 Table descriptors:

        /// PXN limit for subsequent levels of lookup.
        const PXN_TABLE =           1 << 59;
        /// XN limit for subsequent levels of lookup.
        const XN_TABLE =            1 << 60;
        /// Access permissions limit for subsequent levels of lookup: access at EL0 not permitted.
        const AP_NO_EL0_TABLE =     1 << 61;
        /// Access permissions limit for subsequent levels of lookup: write access not permitted.
        const AP_NO_WRITE_TABLE =   1 << 62;
        /// For memory accesses from Secure state, specifies the Security state for subsequent
        /// levels of lookup.
        const NS_TABLE =            1 << 63;
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum MemType {
    Device = 0,
    Normal = 1,
}

impl PTEFlags {
    const ATTR_INDEX_MASK: u64 = 0b111_00;

    const fn from_mem_type(mem_type: MemType) -> Self {
        let mut bits = ((mem_type as u64) << 2) | PTEFlags::VALID.bits;
        if matches!(mem_type, MemType::Normal) {
            bits |= Self::INNER.bits() | Self::SHAREABLE.bits();
        }
        Self::from_bits_truncate(bits)
    }

    fn mem_type(&self) -> MemType {
        let idx = (self.bits() & Self::ATTR_INDEX_MASK) >> 2;
        match idx {
            0 => MemType::Device,
            1 => MemType::Normal,
            _ => panic!("Invalid memory attribute index"),
        }
    }
}

// bitflags::bitflags! {
//     pub struct MemFlags: usize {
//         const READ          = 1 << 0;
//         const WRITE         = 1 << 1;
//         const EXECUTE       = 1 << 2;
//         const USER          = 1 << 3;
//         const DEVICE        = 1 << 4;
//     }
// }


impl From<PTEFlags> for MapPermission {
    fn from(attr: PTEFlags) -> Self {
        let mut mp = Self::empty();
        if !attr.contains(PTEFlags::VALID){
            return mp;
        }
        if attr.contains(PTEFlags::VALID) {
            mp |= Self::R;
        }
        if !attr.contains(PTEFlags::AP_RO) {
            mp |= Self::W;
        }
        if attr.contains(PTEFlags::AP_EL0) {
            mp |= Self::U;
            if !attr.contains(PTEFlags::UXN) {
                mp |= Self::X;
            }
        } else if !attr.intersects(PTEFlags::PXN) {
            mp |= Self::X;
        }
        if attr.mem_type() == MemType::Device {
            mp |= Self::G;
        }
        mp
    }
}

impl From<MapPermission> for PTEFlags {
    fn from(flags: MapPermission) -> Self {
        let mut attr = if flags.contains(MapPermission::G) {
            Self::from_mem_type(MemType::Device)
        } else {
            Self::from_mem_type(MemType::Normal)
        };

        if !flags.contains(MapPermission::W) {
            attr |= Self::AP_RO;
        }
        if flags.contains(MapPermission::U) {
            attr |= Self::AP_EL0 | Self::PXN;
            if !flags.contains(MapPermission::X) {
                attr |= Self::UXN;
            }
        } else {
            attr |= Self::UXN;
            if !flags.contains(MapPermission::X) {
                attr |= Self::PXN;
            }
        }
        attr
    }
}



#[derive(Copy, Clone)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: u64,
}

impl From<u64> for PageTableEntry{
    fn from(a: u64) -> Self{
        PageTableEntry { bits: a as u64 }
    }
}
impl From<PageTableEntry> for u64{
    fn from(a: PageTableEntry) -> Self{
        a.bits
    }
}

impl PageTableEntry {
    const PHYS_ADDR_MASK: usize = 0x007ffffff000;

    pub fn new_page(paddr: PhysAddr, flags: MapPermission, is_block: bool) -> Self {
        let mut attr = PTEFlags::from(flags) | PTEFlags::AF;
        if !is_block {
            attr |= PTEFlags::NON_BLOCK;
        }
        Self{bits: attr.bits() | (paddr.0 & Self::PHYS_ADDR_MASK) as u64}
    }
    pub fn new_table(paddr: PhysAddr) -> Self {
        let attr = PTEFlags::NON_BLOCK | PTEFlags::VALID;
        Self{bits: attr.bits() | (paddr.0 & Self::PHYS_ADDR_MASK) as u64}
    }

    //Interface to MMK
    //Note: Empty MapPermission means a page table pages.
    pub fn new(pa: PhysAddr, flags: MapPermission, is_block: bool) -> Self {
        
        let mut attr = PTEFlags::from(flags) | PTEFlags::AF | PTEFlags::VALID;

        if !is_block {
            attr |= PTEFlags::NON_BLOCK;
        }
        

        if (flags & MapPermission::RWX) != MapPermission::empty() {
            attr |= PTEFlags::INNER;
        }

        if (flags & MapPermission::RWX) == MapPermission::empty() {
            attr = PTEFlags::VALID | PTEFlags::NON_BLOCK;
        }

        Self{bits: attr.bits | (pa.0 & Self::PHYS_ADDR_MASK) as u64}
    }

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
        self.flags() & PTEFlags::VALID != PTEFlags::empty()
    }

    //Interface to MMK
    pub fn is_block(&self) -> bool {
        self.valid() && (self.flags() & PTEFlags::NON_BLOCK == PTEFlags::empty())
    }

    //Interface to MMK
    pub fn set_perm(&mut self, flags: MapPermission) {
        let new_flags: PTEFlags = flags.into();
        self.bits = (self.bits & Self::PHYS_ADDR_MASK as u64) as u64 | new_flags.bits();
    }

}
