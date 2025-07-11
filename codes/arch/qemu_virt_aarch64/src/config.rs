#[allow(unused)]

pub use crate::mmi::*;

pub const BOOT_KERNEL_STACK_SIZE: usize = 4096 * 4; // 16K

///////////////////////////////////
//// used by mmi (extern "Rust")
////

#[no_mangle]
pub const fn arch_phys_to_virt_addr(pa: PhysAddr) -> VirtAddr {
    VirtAddr{0: pa.0 | 0xffff_0000_0000_0000}
}

#[no_mangle]
pub const fn arch_virt_to_phys_addr(va: VirtAddr) -> PhysAddr {
    PhysAddr{0: va.0 & 0x0000_ffff_ffff_ffff}
}

#[no_mangle]
pub const fn arch_phys_to_virt(pa: usize) -> usize {
    pa | 0xffff_0000_0000_0000
}

#[no_mangle]
pub const fn arch_virt_to_phys(va: usize) -> usize {
    va & 0x0000_ffff_ffff_ffff
}


///////////////////////////////////
//// aarch64 MMU config
////
/// 
pub const PAGE_SIZE: usize = 0x1000; //should not change
pub const PAGE_SIZE_BITS: usize = 0xc;

// MMU max pagewalk level is 4.
pub type VpnIndexes = [usize; 4];

// MMU pagewalk rules: [20:12]=9, [29-21]=9,....
pub const MMU_PAGEWALK: VpnIndexes = [9,9,9,9];

//pagetable size: 8(usize)*(2^9)
pub const MMU_PAGETABLE_SIZE: VpnIndexes = [8*512, 8*512, 8*512, 8*512];

///////////////////////////////////
//// aarch64 platform config
////
/// 
pub const SIGNAL_TRAMPOLINE: usize = 0x100000000 - PAGE_SIZE;

pub const TRAP_CONTEXT: usize = SIGNAL_TRAMPOLINE - PAGE_SIZE;

pub const USER_STACK: usize = TRAP_CONTEXT - PAGE_SIZE;

pub const USER_STACK_SIZE_MIN: usize = PAGE_SIZE * 4;

pub const USER_STACK_SIZE: usize = PAGE_SIZE * 16;

pub const USER_HEAP_SIZE: usize = PAGE_SIZE * 32;

pub const NK_STACK_SIZE: usize = PAGE_SIZE * 2;

pub const NK_HEAP_SIZE: usize = PAGE_SIZE * 0x200;

pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE * 2;

pub const KERNEL_HEAP_SIZE: usize = PAGE_SIZE * 0x200;

pub const NKSPACE_START: usize = arch_phys_to_virt(0x40000000); //0xffff000040080000

pub const NKSPACE_END: usize = arch_phys_to_virt(0x40400000);

pub const OKSPACE_START: usize = 0x40400000;

pub const OKSPACE_END: usize = 0x40800000;

pub const CLOCK_FREQ: usize = 24000000;


pub const MMIO: &[(usize, usize)] = &[
    (arch_phys_to_virt(0x0900_0000), 0x1_0000),   // PL011 UART (console!)
    (arch_phys_to_virt(0x0800_0000), 0x2_0000), // GICv2 (trap!)

    (arch_phys_to_virt(0x1000_0000), 0x1000),      /* PLIC      */
    (arch_phys_to_virt(0x1000_1000), 0x1000),      /* PLIC      */
    (arch_phys_to_virt(0x1000_2000), 0x1000),      /* PLIC      */
    (arch_phys_to_virt(0x1020_0000), 0x1000),      /* PLIC      */

    (arch_phys_to_virt(0x1400_0000), 0x1000),      /* CLINT      */

    (arch_phys_to_virt(0x0300_0000), 0x1000),      /* SYS_CFG   */

    (arch_phys_to_virt(0x0300_2000), 0x1000),      /* DMAC      */

    (arch_phys_to_virt(0x0205_0000), 0x1000),      /* TIMER  */
    (arch_phys_to_virt(0x0300_8000), 0x1000),      /* HSTIMER  */

    (arch_phys_to_virt(0x0203_1000), 0x1000),      /* DMIC      */
];


///////////////////////////////////
//// aarch64 api config

pub const SPECIAL_AREA: usize = 0xffff_ffff_ffff_f000;

pub const NK_TRAMPOLINE: usize = SPECIAL_AREA;

pub const TRAMPOLINE: usize = SPECIAL_AREA - PAGE_SIZE;
 
pub const METADATA_PAGE: usize = SPECIAL_AREA - 2*PAGE_SIZE;
 
pub const PROXY_CONTEXT: usize = METADATA_PAGE;
pub const CONFIG_DATA: usize = METADATA_PAGE + 0x400;
pub const MMKAPI_TABLE: usize = METADATA_PAGE + 0x800;


///////////////////////////////////
//// aarch64 reginfo

pub const XREG_RA: usize = 30;
pub const XREG_SP: usize = 0;
pub const XREG_PARAM: usize = 0;
