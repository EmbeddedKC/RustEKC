#![no_std]
#![no_main]

use core::arch::global_asm;

use mmi::{CONFIGDATA, PROXYCONTEXT, PROXY_CONTEXT, TRAMPOLINE};
use riscv::register::{satp, sepc, stvec, scause, stval};
use riscv::register::sstatus::{Sstatus, self, SPP};
use crate::config::*;


#[repr(C)]
#[derive(Debug, Clone)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
    //pub kernel_satp: usize,
    pub kernel_sp: usize,
    pub trap_handler: usize,
    
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) { self.x[2] = sp; }
    pub fn get_sp(& self)->usize { self.x[2] }

    pub fn app_init_context(
        // 只有三个函数调用过这个方法，在初始化的时候
        //，也就是从elf得到pcb的时候，trap context要一起初始化，这里初始化为elf的header
        entry: usize, // trap之前的上一条指令
        sp: usize, // 当前用户栈的栈顶
        //kernel_satp: usize,  // 未理解的内核页表，这东西在干啥
        kernel_sp: usize  // 内核栈栈顶
    ) -> Self {
        // set CPU privilege to User after trapping back
        //Yan_ice: If it is supervisor mode, it can use sret successfully.
        unsafe{
            sstatus::set_spp(SPP::User);
            let sstatus = sstatus::read();
            let mut cx = Self {
                x: [0; 32],
                sstatus,
                sepc: entry,
                kernel_sp,
                trap_handler: TRAMPOLINE
            };
            cx.set_sp(sp);
            cx
        }
        
    }

}

extern "C"{
    fn __signal_trampoline();
    fn __alltraps();
    fn __restore();
    fn _ktrap();
    fn _kreturn();
}

fn default_delegate(){
    let sepc = sepc::read();
    let scause = scause::read();
    let stval = stval::read();
    panic!("Default trap occur in MMK from 0x{:x}. stval:0x{:x} cause:0x{:x}", sepc, stval, scause.bits());
}


pub fn init(){
    unsafe {
	let cfg = CONFIGDATA();
        cfg.kernel_trap_handler = default_delegate as usize;
	    stvec::write(TRAMPOLINE-(__alltraps as usize)+(_ktrap as usize), stvec::TrapMode::Direct);
        PROXYCONTEXT().__deleted = TRAMPOLINE - __alltraps as usize + __restore as usize;
    
    }
}

