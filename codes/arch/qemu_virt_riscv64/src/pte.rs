use mmi::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};

use mmi::{MapPermission};

bitflags! {
    pub struct PTEFlags: u16 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const RWX = 0b111 << 1;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
        const O = 1 << 9; //copy on write
    }
}
impl PTEFlags{
    pub fn get_bits(&self) -> u16{
        self.bits
    }
}

impl From<MapPermission> for PTEFlags {
	fn from(v: MapPermission) -> Self {
		PTEFlags::from_bits(v.bits()).unwrap() | PTEFlags::V | PTEFlags::A | PTEFlags::D
	}
}

impl From<PTEFlags> for MapPermission {
	fn from(v: PTEFlags) -> Self {
		MapPermission::from_bits_truncate(v.bits())
	}
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

//Interface to MMK
impl From<usize> for PageTableEntry{
    fn from(a: usize) -> Self{
        PageTableEntry { bits: a }
    }
}

//Interface to MMK
impl From<PageTableEntry> for usize{
    fn from(a: PageTableEntry) -> Self{
        a.bits
    }
}

impl PageTableEntry {

    pub fn new_table(paddr: PhysAddr) -> Self {
        let attr = PTEFlags::V | PTEFlags::A | PTEFlags::D;
        Self{bits: attr.bits() as usize | (paddr.0 >> 12 << 10) as usize}
    }

    //Interface to MMK
    //Note: Empty MapPermission means a page table pages.
    pub fn new_page(paddr: PhysAddr, flags: MapPermission, is_block: bool) -> Self {
        PageTableEntry {
            bits: (paddr.0 >> 12 << 10) | PTEFlags::from(flags).bits() as usize,
        }
    }

    //Interface to MMK
    pub fn empty() -> Self {
        PageTableEntry {
            bits: 0
        }
    }


    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits_truncate(self.bits as u16).into()
    }
    
    //Interface to MMK
    pub fn ppn(&self) -> PhysPageNum {
        PhysPageNum{0: self.bits as usize >> 10 }
    }
    pub fn paddr(&self) -> PhysAddr {
        self.ppn().into()
    }

    //Interface to MMK
    pub fn perm(&self) -> MapPermission {
        self.flags().into()
    }

    //Interface to MMK
    pub fn valid(&self) -> bool {
        self.flags() & PTEFlags::V != PTEFlags::empty()
    }

    //Interface to MMK
    pub fn is_block(&self) -> bool {
        self.valid() && (self.flags() & PTEFlags::RWX != PTEFlags::empty())
    }

    //Interface to MMK
    pub fn set_perm(&mut self, flags: MapPermission) {
        let new_flags: PTEFlags = flags.into();
        self.bits = (self.bits & 0xFFFF_FFFF_FFFF_FC00) | new_flags.bits() as usize;
    }

}