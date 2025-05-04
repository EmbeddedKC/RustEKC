#![no_std]
#![feature(naked_functions)]

extern crate mmi;
extern crate alloc;

mod util;
mod trap;
pub mod pte;
mod entry;

#[macro_use]
pub mod gate;

pub mod config;
use core::fmt::{self, Write};
use core::arch::global_asm;
//global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("top.S"));

pub use config::*;
pub use pte::*;

//use aarch64_cpu::registers::{TTBR0_EL1, Writeable};
use cortex_a::registers::{TTBR0_EL1, TTBR1_EL1};
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

pub use trap::TrapContext;


pub use util::pl011::console_putchar as arch_putchar;
pub use util::pl011::console_getchar as arch_getchar;
pub use util::console::print as arch_print;
pub use util::psci::shutdown as arch_shutdown;
pub use util::console::print_raw;
pub use config::arch_phys_to_virt_addr as arch_phys_to_virt_addr;
pub use config::arch_virt_to_phys as arch_virt_to_phys;

#[no_mangle]
pub fn t_breakpoint(){
    arch_debug_info!("breakpoint");
    panic!("breakpoint reached.");
}


extern "C" {
    pub fn nk_gate();
    pub fn nk_exit(hart: usize);
}

pub fn arch_get_device_priv_key(buf: &mut [u64]){
    for a in 0..16 {
        buf[a] = 42;
    }
}

pub fn arch_get_root_pt(pt_id: usize, ppn: PhysPageNum) -> usize{
    let token: usize = ((pt_id & 0xff) << 48) | PhysAddr::from(ppn).0;
    return token;
}

// This function only set the page table of MMK.
pub fn arch_set_root_pt(pt_id: usize, ppn: PhysPageNum){
    let token: usize = ((pt_id & 0xff) << 48) | PhysAddr::from(ppn).0;
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
    let time: usize;
    unsafe{
            core::arch::asm!(
                "dsb sy; isb; mrs x0, cntvct_el0",
                out("x0") time
            );
        }
    time
}

pub fn arch_flush_tlb(uid: usize) {
    unsafe { 
        //core::arch::asm!("tlbi vlle1; dsb sy; isb");
        core::arch::asm!("tlbi aside1, x28; dsb sy; isb", in("x28") uid); 
    }
}

//this func is called before mmk_main()!
#[no_mangle]
pub fn arch_early_init() -> usize{
    print_raw("Hello, world!\n");
    let core = arch_get_cpu_id();
    if core != 0 {
        loop{}
    }
    trap::init();

    print_raw("arch: qemu_virt_aarch64.\n");
    0
}


//this func is called after mmk_main()!
#[no_mangle]
pub fn arch_final_init() -> usize{
    
    //util::init();
    arch_debug_info!("Ready jump to payload.");
    
    unsafe{
        core::arch::asm!("BR x28", 
        //in("x31") nk_exit as usize,
        in("x28") crate::config::NK_TRAMPOLINE - nk_gate as usize + nk_exit as usize,
        in("x0") 0 );
        panic!("not reachable");
    }
}

pub fn arch_scan_instruction(pa: PhysAddr) {
    let time_start: usize = arch_get_cpu_time();
    unsafe{
        let data: &mut [u32; 1024] = &mut *(pa.0 as *mut [u32; 1024]);
        for instruction in 0..1024 {
            let csr = data[instruction] >> 20;
            let opcode = data[instruction] & 0b1111111;
            if opcode == 0x73 {
                if csr == 0x180 {
                    arch_debug_info!("modify satp instruction found in 0x{:x}. Removed.",pa.0);
                    data[instruction] = 0b0010011; //addi zero, zero, 0
                    
                }else if csr == 0x105 {
                    arch_debug_info!("modify stvec instruction found in 0x{:x}. Removed.",pa.0);
                    data[instruction] = 0b0010011; //addi zero, zero, 0
                    
                }
            }
        }
    }
    
    let time_end: usize = arch_get_cpu_time();

    arch_debug_info!("instruction scan time: {}", time_end - time_start);
    // TODO: not implemented yet.
    // currently only implemented for risc-v

}