#![no_std]
#![no_main]

use core::arch::global_asm;

//use cortex_a::registers::{CNTFRQ_EL0, CNTPCT_EL0, CNTP_CTL_EL0, CNTP_TVAL_EL0};
//use cortex_a::registers::VBAR_EL1;

use tock_registers::interfaces::{Readable, Writeable};
use crate::config::*;
use crate::arch_debug_info;
use spin::Mutex;
use crate::print_raw;

mod gicv2;

use gicv2::*;

const PHYS_TIMER_IRQ_NUM: usize = 30;
const TICKS_PER_SEC: usize = 60;
const MSEC_PER_SEC: u64 = 1000;

#[derive(Debug, Eq, PartialEq)]
pub enum IrqHandlerResult {
    Reschedule,
    NoReschedule,
}

pub fn set_next_trigger() {
    
    //CNTP_TVAL_EL0.set(() as u64);
    unsafe{
    core::arch::asm!("
        MCR     p15, 0, r8, c14, c2, 0
    ",in("r8") (CLOCK_FREQ / TICKS_PER_SEC));
    }
}


#[repr(C)]
#[derive(Debug, Clone)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sepc: usize,
    pub sstatus: usize
    
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
            // sstatus::set_spp(SPP::User);
            // let sstatus = sstatus::read();
            let mut cx = Self {
                x: [0; 32],
                sepc: entry,
                sstatus: 0
            };
            cx.set_sp(sp);
            cx
        }
        
    }

}


//called by trap.S
#[no_mangle]
//fn mmk_trap_handler(tf: &mut TrapContext, kind: TrapKind, source: TrapSource){
fn mmk_trap_handler(tf: &mut TrapContext, kind: usize){
    arch_debug_info!("Default trap occur in MMK: kind=[{:?}]", kind);
    //arch_debug_info!("sp_el0={:x}, sepc={:x}", tf.x[31], tf.sepc);
    //panic!("trap panic.");
    return;
}


#[no_mangle]
fn invalid_exception(){ }
#[no_mangle]
fn handle_sync_exception(){ }
#[no_mangle]
fn handle_irq_exception(){ }

pub fn init(){

    //enable CNTP_CTL (Counter-timer Physical Timer Control Register)
    unsafe{
        core::arch::asm!("
            MOV     R0, #1         ;
            MCR     p15, 0, R0, c14, c2, 1   ;
        ")
    }
    //CNTP_CTL_EL0.write(CNTP_CTL_EL0::ENABLE::SET);

    extern "C" {
        fn mmk_exception_vector_base();
    }

    crate::arch_set_INTR_handler(mmk_exception_vector_base as usize);
    //VBAR_EL1.set(mmk_exception_vector_base as usize as _);
    //VBAR_EL1.set(TRAMPOLINE as usize as _);

    //set_next_trigger();
    
    //gicv2::init();
    //gicv2::irq_set_mask(PHYS_TIMER_IRQ_NUM, false);
    print_raw("trap init success.\n");
    // unsafe {
	// let cfg = CONFIGDATA();
    //     cfg.kernel_trap_handler = default_delegate as usize;
	//     stvec::write(TRAMPOLINE-(__alltraps as usize)+(_ktrap as usize), stvec::TrapMode::Direct);
    //     PROXYCONTEXT().__deleted = TRAMPOLINE - __alltraps as usize + __restore as usize;
    
    // }

    //for debug use
    //crate::arch_set_INTR_handler(mmk_debug_handler as usize);
}

