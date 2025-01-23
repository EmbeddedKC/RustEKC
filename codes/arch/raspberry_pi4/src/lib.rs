#![no_std]
#![feature(naked_functions)]

extern crate mmi;
extern crate alloc;

mod util;
mod trap;
pub mod pte;
mod entry;

pub mod config;
use core::fmt::{self, Write};
use core::arch::global_asm;
//global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("top.S"));

pub use config::*;
pub use pte::*;
use crate::entry::t_breakpoint; 
//use aarch64_cpu::registers::{TTBR0_EL1, Writeable};
use cortex_a::registers::{TTBR0_EL1, TTBR1_EL1};
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

pub use trap::TrapContext;


pub use util::pl011::console_putchar as arch_putchar;
pub use util::pl011::console_getchar as arch_getchar;
pub use util::console::print as arch_print;
pub use util::psci::shutdown as arch_shutdown;
pub use util::console::print_raw;
pub use config::arch_phys_to_virt as arch_phys_to_virt;
pub use config::arch_virt_to_phys as arch_virt_to_phys;


extern "C" {
    pub fn nk_gate();
    pub fn nk_exit(hart: usize);
}


pub fn arch_get_root_pt(pt_id: usize, ppn: PhysPageNum) -> usize{
    //let token: usize = ((pt_id & 0xff) << 48) | PhysAddr::from(ppn).0;
    let token: usize = PhysAddr::from(ppn).0;
    //currently not use ASID (ASID = 0).
    
    return token;
}

// This function only set the page table of MMK.
// Be aware of flushing TLB after switching page table.
pub fn arch_set_root_pt(pt_id: usize, ppn: PhysPageNum){
    //let token: usize = ((pt_id & 0xff) << 48) | PhysAddr::from(ppn).0;
    let token: usize = PhysAddr::from(ppn).0;
    //currently not use ASID (ASID = 0).
    TTBR0_EL1.set(token as u64);
    TTBR1_EL1.set(token as u64);

    //arch_flush_tlb(pt_id);
}


pub fn arch_get_cpu_id() -> usize{
    let cpu_id: usize;
    unsafe {
        core::arch::asm!("MRS x28, mpidr_el1", 
                        out("x28") cpu_id);
    }
    return cpu_id & 0xff;
}

pub fn arch_get_cpu_time() -> usize {
    // unsafe{
    //         core::arch::asm!(
    //             "rdtime a0",
    //             inout("a0") time
    //         );
    //     }
    0
}

pub fn arch_flush_tlb(uid: usize) {
    unsafe { 
        //core::arch::asm!("tlbi alle1; dsb sy; isb");

        //currently not use ASID. (ASID=0)
        core::arch::asm!("tlbi aside1, x28; dsb sy; isb", in("x28") 0); 
        //core::arch::asm!("tlbi aside1, x28; dsb sy; isb", in("x28") uid); 
    }
}


//this func is called before mmk_main()!
#[no_mangle]
pub fn arch_early_init() -> usize{
    //util::gpio::gpio_init();
    //util::pl011::pl011_init();
    util::pl011::uart_init();
    print_raw("Hello, world!\n");
    let core = arch_get_cpu_id();
    if core != 0 {
        loop{}
    }
    trap::init();

    print_raw("arch: raspberry_pi_4b.\n");
    0
}


//this func is called after mmk_main()!
#[no_mangle]
pub fn arch_final_init() -> usize{
    PROXYCONTEXT().__deleted3 = t_breakpoint as usize;
    //util::init();
    let target: usize = mmi::config::NK_TRAMPOLINE - nk_gate as usize + nk_exit as usize;
    arch_debug_info!("Ready jump to payload: {:x}", target);
    
    unsafe{
        core::arch::asm!("BR x28", 
        //in("x31") nk_exit as usize,
        in("x28") target,
        in("x0") 0 );
        panic!("not reachable");
    }
}

pub fn arch_scan_instruction(pa: PhysAddr) {
    // TODO: not implemented yet.
    // currently only implemented for risc-v
}