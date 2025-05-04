#[allow(unused)]

pub use crate::mmi::*;

pub const BOOT_KERNEL_STACK_SIZE: usize = 4096 * 4; // 16K

// used by mmi. (extern "C")
#[no_mangle]
pub const fn arch_phys_to_virt_addr(pa: PhysAddr) -> VirtAddr {
    VirtAddr{0: pa.0}
}

#[no_mangle]
pub const fn arch_virt_to_phys(va: VirtAddr) -> PhysAddr {
    PhysAddr{0: va.0}
}

///////////////////////////////////
//// aarch64 reginfo

pub const XREG_RA: usize = 30;
pub const XREG_SP: usize = 0;
pub const XREG_PARAM: usize = 0;

///////////////////////////////////
//// aarch64 config

pub const SIGNAL_TRAMPOLINE: usize = 0x0ffff000;

pub const TRAP_CONTEXT: usize = SIGNAL_TRAMPOLINE - PAGE_SIZE;

pub const USER_STACK: usize = TRAP_CONTEXT - PAGE_SIZE;

pub const USER_STACK_SIZE_MIN: usize = PAGE_SIZE * 4;

pub const USER_STACK_SIZE: usize = PAGE_SIZE * 16;

pub const USER_HEAP_SIZE: usize = PAGE_SIZE * 32;

pub const NK_STACK_SIZE: usize = PAGE_SIZE * 2;

pub const NK_HEAP_SIZE: usize = PAGE_SIZE * 0x200;

pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE * 2;

pub const KERNEL_HEAP_SIZE: usize = PAGE_SIZE * 0x200;

pub const NKSPACE_START: usize = arch_phys_to_virt(0x10000); //0xffff000040080000

pub const NKSPACE_END: usize = arch_phys_to_virt(0x100000);

pub const OKSPACE_START: usize = 0x100000;

pub const OKSPACE_END: usize = 0x890000;

pub const CLOCK_FREQ: usize = 24000000;

///////////////////////////////////
//// pagewalk config

pub const MMU_PAGE_WALK: [usize; 4] = [9,9,9,9]
pub const MMU_MAX_LEVEL: usize = 4;
//3: SV39.   4: SV48.   5: SV57.

pub const fn arch_phys_to_virt(phys: usize) -> usize {
    return (phys);
}

pub const fn arch_phys_to_virt_addr(phys: PhysAddr) -> VirtAddr {
    return VirtAddr{0: phys.0};
}

pub const MMIO: &[(usize, usize)] = &[
    (phys_to_virt(0x0900_0000), 0x1_0000),   // PL011 UART (console!)
    (phys_to_virt(0x0800_0000), 0x2_0000), // GICv2 (trap!)

    (phys_to_virt(0x1000_0000), 0x1000),      /* PLIC      */
    (phys_to_virt(0x1000_1000), 0x1000),      /* PLIC      */
    (phys_to_virt(0x1000_2000), 0x1000),      /* PLIC      */
    (phys_to_virt(0x1020_0000), 0x1000),      /* PLIC      */

    (phys_to_virt(0x1400_0000), 0x1000),      /* CLINT      */

    (phys_to_virt(0x0300_0000), 0x1000),      /* SYS_CFG   */

    (phys_to_virt(0x0300_2000), 0x1000),      /* DMAC      */

    (phys_to_virt(0x0205_0000), 0x1000),      /* TIMER  */
    (phys_to_virt(0x0300_8000), 0x1000),      /* HSTIMER  */

    (phys_to_virt(0x0203_1000), 0x1000),      /* DMIC      */
];


