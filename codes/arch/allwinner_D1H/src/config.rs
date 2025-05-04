#[allow(unused)]

pub use crate::mmi::*;

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
//// riscv64 reginfo

pub const XREG_RA: usize = 1;
pub const XREG_SP: usize = 2;
pub const XREG_PARAM: usize = 10;

///////////////////////////////////
//// nezha config
/// 
pub const SIGNAL_TRAMPOLINE: usize = 0x100000000 - PAGE_SIZE;

pub const TRAP_CONTEXT: usize = SIGNAL_TRAMPOLINE - PAGE_SIZE;

pub const USER_STACK: usize = TRAP_CONTEXT - PAGE_SIZE;

pub const USER_STACK_SIZE_MIN: usize = PAGE_SIZE * 4;

pub const USER_STACK_SIZE: usize = PAGE_SIZE * 16;

pub const USER_HEAP_SIZE: usize = PAGE_SIZE * 32;

pub const NK_STACK_SIZE: usize = PAGE_SIZE * 2;

pub const NK_HEAP_SIZE: usize = PAGE_SIZE * 0x128;

pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE * 2;

pub const KERNEL_HEAP_SIZE: usize = PAGE_SIZE * 0x200;

pub const NKSPACE_START: usize = 0x40200000;

pub const NKSPACE_END: usize = 0x40800000;

pub const OKSPACE_START: usize = 0x40800000;

pub const OKSPACE_END: usize = 0x44000000;

pub const CLOCK_FREQ: usize = 24000000;

pub const MMU_MAX_LEVEL: usize = 3;
//3: SV39.   4: SV48.   5: SV57.

pub const MMIO: &[(usize, usize)] = &[
    (0x1000_0000, 0x1000),      /* PLIC      */
    (0x1000_1000, 0x1000),      /* PLIC      */
    (0x1000_2000, 0x1000),      /* PLIC      */
    (0x1020_0000, 0x1000),      /* PLIC      */

    (0x1400_0000, 0x1000),      /* CLINT      */

    (0x0300_0000, 0x1000),      /* SYS_CFG   */
    (0x0601_0000, 0x1000),      /* RISCV_CFG   */

    (0x0300_2000, 0x1000),      /* DMAC      */

    (0x0250_0000, 0x1000),      /* UART      */
    (0x0250_1000, 0x1000),      /* UART      */

    (0x0402_5000, 0x1000),      /* SPI0      */
    (0x0402_6000, 0x1000),      /* SPI_DBI   */
    
    (0x0205_0000, 0x1000),      /* TIMER  */
    (0x0300_8000, 0x1000),      /* HSTIMER  */
    
    (0x0200_0000, 0x1000),      /* GPIO      */
    (0x0200_1000, 0x1000),      /* GPIO      */
    (0x0203_1000, 0x1000),      /* DMIC      */
    
];


