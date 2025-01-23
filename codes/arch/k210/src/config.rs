#[allow(unused)]

pub use crate::mmi::*;

// used by mmi. (extern "C")
#[no_mangle]
pub const fn arch_phys_to_virt(pa: PhysAddr) -> VirtAddr {
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
//// k210 config
/// 
pub const SIGNAL_TRAMPOLINE: usize = 0x100000000 - PAGE_SIZE;

pub const TRAP_CONTEXT: usize = SIGNAL_TRAMPOLINE - PAGE_SIZE;

pub const USER_STACK: usize = TRAP_CONTEXT - PAGE_SIZE;

pub const USER_STACK_SIZE_MIN: usize = PAGE_SIZE * 4;

pub const USER_STACK_SIZE: usize = PAGE_SIZE * 16;

pub const USER_HEAP_SIZE: usize = PAGE_SIZE * 32;

pub const NK_STACK_SIZE: usize = PAGE_SIZE * 2;

pub const NK_HEAP_SIZE: usize = PAGE_SIZE * 0x30;

pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE * 2;

pub const KERNEL_HEAP_SIZE: usize = PAGE_SIZE * 0x200;

pub const NKSPACE_START: usize = 0x80020000;

pub const NKSPACE_END: usize = 0x80200000;

pub const OKSPACE_START: usize = 0x80200000;

pub const OKSPACE_END: usize = 0x80500000;

pub const CLOCK_FREQ: usize = 403000000 / 62;

pub const MMU_MAX_LEVEL: usize = 3;
//3: SV39.   4: SV48.   5: SV57.

pub const MMIO: &[(usize, usize)] = &[

    (0x0C00_0000, 0x1000),      /* PLIC      */
    (0x0C00_1000, 0x1000),      /* PLIC      */
    (0x0C00_2000, 0x1000),      /* PLIC      */
    (0x0C20_0000, 0x1000),      /* PLIC      */
    (0x3800_0000, 0x1000),      /* UARTHS    */
    (0x3800_1000, 0x1000),      /* GPIOHS    */
    (0x5020_0000, 0x1000),      /* GPIO      */
    (0x5024_0000, 0x1000),      /* SPI_SLAVE */
    (0x502B_0000, 0x1000),      /* FPIOA     */
    (0x502D_0000, 0x1000),      /* TIMER0    */
    (0x502E_0000, 0x1000),      /* TIMER1    */
    (0x502F_0000, 0x1000),      /* TIMER2    */
    (0x5044_0000, 0x1000),      /* SYSCTL    */
    (0x5200_0000, 0x1000),      /* SPI0      */
    (0x5300_0000, 0x1000),      /* SPI1      */
    (0x5400_0000, 0x1000),      /* SPI2      */
];

