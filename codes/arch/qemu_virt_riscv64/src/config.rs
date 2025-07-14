#[allow(unused)]

pub use crate::mmi::*;


///////////////////////////////////
//// used by mmi (extern "Rust")
////

#[no_mangle]
pub const fn arch_phys_to_virt_addr(pa: PhysAddr) -> VirtAddr {
    VirtAddr{0: pa.0}
}

#[no_mangle]
pub const fn arch_virt_to_phys_addr(va: VirtAddr) -> PhysAddr {
    PhysAddr{0: va.0}
}

#[no_mangle]
pub const fn arch_phys_to_virt(pa: usize) -> usize {
    pa
}

#[no_mangle]
pub const fn arch_virt_to_phys(va: usize) -> usize {
    va
}

///////////////////////////////////
//// riscv64 MMU config
////
/// 
pub const PAGE_SIZE: usize = 0x1000; //should not change
pub const PAGE_SIZE_BITS: usize = 0xc;

// MMU max pagewalk level is 2.
pub type VpnIndexes = [usize; 4];

// MMU pagewalk rules: [19:12]=8, [31:20]=12
pub const MMU_PAGEWALK: VpnIndexes = [9, 9, 9, 9];

//pagetable size: 8(usize)*(2^9), 4(usize)*(2^12)
pub const MMU_PAGETABLE_SIZE: VpnIndexes = [8*512, 8*512, 8*512, 8*512];


///////////////////////////////////
//// riscv64 reginfo
/// 
pub const USIZE_WIDTH: usize = 8;

pub const XREG_RA: usize = 1;
pub const XREG_SP: usize = 2;
pub const XREG_PARAM: usize = 10;

///////////////////////////////////////
//// QEMU config
/// 

pub const USER_STACK_SIZE_MIN: usize = PAGE_SIZE * 8;

pub const USER_STACK_SIZE: usize = PAGE_SIZE * 35;

pub const USER_HEAP_SIZE: usize = PAGE_SIZE * 64;

pub const NK_STACK_SIZE: usize = PAGE_SIZE * 2;

pub const NK_HEAP_SIZE: usize = PAGE_SIZE * 0x80;

pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE * 2;

pub const KERNEL_HEAP_SIZE: usize = PAGE_SIZE * 0x200;

///////////////////////////////////
//// api config

pub const SPECIAL_AREA: usize = usize::MAX - 0x1000 + 1;

pub const NK_TRAMPOLINE: usize = SPECIAL_AREA;

pub const TRAMPOLINE: usize = SPECIAL_AREA - PAGE_SIZE;
 
pub const METADATA_PAGE: usize = SPECIAL_AREA - 2*PAGE_SIZE;
 
pub const PROXY_CONTEXT: usize = METADATA_PAGE;
pub const CONFIG_DATA: usize = METADATA_PAGE + 0x400;
pub const MMKAPI_TABLE: usize = METADATA_PAGE + 0x800;


///////////////////////////////////
//// platform mem layout config
/// 
pub const SIGNAL_TRAMPOLINE: usize = 0x100000000 - PAGE_SIZE;

pub const TRAP_CONTEXT: usize = SIGNAL_TRAMPOLINE - PAGE_SIZE;

pub const USER_STACK: usize = TRAP_CONTEXT - PAGE_SIZE;

pub const NKSPACE_START: usize = 0x80200000;

pub const NKSPACE_END: usize = 0x80800000;

pub const OKSPACE_START: usize = 0x80800000;

pub const OKSPACE_END: usize = 0x88000000;

pub const CLOCK_FREQ: usize = 12500000;

pub const MMU_MAX_LEVEL: usize = 3;
//3: SV39.   4: SV48.   5: SV57.

pub const MMIO: &[(usize, usize)] = &[
    (0x10001000, 0x1000),
    (0x10000000, 0x1000),
];

