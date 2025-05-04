#![no_std]
#![no_main]

extern crate mmi;
#[macro_use]
extern crate bitflags;

mod util;
mod trap;

pub mod pte;
pub mod config;

use core::fmt::{self, Write};
use core::arch::{global_asm, asm};
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("top.S"));

pub use config::*;
pub use pte::*;

use riscv::register::satp;
use riscv::register::scause;
use riscv::register::sepc;
use riscv::register::stval;

pub use util::sbi::console_putchar as arch_putchar;
pub use util::sbi::console_getchar as arch_getchar;
pub use util::sbi::shutdown as arch_shutdown;
pub use util::console::print as arch_print;
pub use config::arch_phys_to_virt_addr as arch_phys_to_virt_addr;
pub use config::arch_virt_to_phys as arch_virt_to_phys;

pub use trap::TrapContext;

extern "C" {
    pub fn nk_entry();
    pub fn nk_exit(hart: usize);
}

pub fn arch_get_root_pt(pt_id: usize, ppn: PhysPageNum) -> usize{
    let token: usize = (8usize << 60) | ((pt_id & 0xff) << 44) | (ppn.0);
    return token;
}

pub fn arch_set_root_pt(pt_id: usize, ppn: PhysPageNum){
    let token: usize = (8usize << 60) | ((pt_id & 0xff) << 44) | (ppn.0);
    satp::write(token);
    arch_flush_tlb(0);
}



pub fn arch_get_cpu_id() -> usize{
    let mut cpu_id: usize = 0;
    unsafe {
        core::arch::asm!("mv {0}, tp", 
                        out(reg) cpu_id);
    }
    cpu_id
}
pub fn arch_get_cpu_time() -> usize {
    let mut time = 0;
    unsafe{
            core::arch::asm!(
                "rdtime a0",
                inout("a0") time
            );
        }
    time
}

pub fn arch_flush_tlb(uid: usize) {
    unsafe{
        asm!("sfence.vma zero, t0",in("t0") uid);
    }
}

//this func is called before mmk_main()!
#[no_mangle]
pub fn arch_early_init() -> usize{
    let core = arch_get_cpu_id();
    if core != 0 {
        loop{}
    }
    arch_debug_info!("arch: qemu_virt_riscv64");
    0
}


//this func is called after mmk_main()!
#[no_mangle]
pub fn arch_final_init() -> usize{
    
    trap::init();

    arch_debug_info!("Ready jump to payload.");
    
    unsafe{
        core::arch::asm!("jr x31", 
        //in("x31") nk_exit as usize,
        in("x31") mmi::config::NK_TRAMPOLINE - nk_entry as usize + nk_exit as usize,
        in("x10") 0 );
        panic!("not reachable");
    }
}


pub fn arch_scan_instruction(pa: PhysAddr) {
    let data: &mut [u32; 1024] = &mut *(pa.0 as *mut [u32; 1024]);
        for instruction in 0..1024 {
            let csr = data[instruction] >> 20;
            let opcode = data[instruction] & 0b1111111;
            if opcode == 0x73 {
                if csr == 0x180 {
                    debug_error!("modify satp instruction found in 0x{:x}. Removed.",ppn.0);
                    data[instruction] = 0b0010011; //addi zero, zero, 0
                    
                }else if csr == 0x105 {
                    debug_error!("modify stvec instruction found in 0x{:x}. Removed.",ppn.0);
                    data[instruction] = 0b0010011; //addi zero, zero, 0
                    
                }
            }
        }
}