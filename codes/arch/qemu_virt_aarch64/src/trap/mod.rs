#![no_std]
#![no_main]

use core::arch::global_asm;

use cortex_a::registers::{CNTFRQ_EL0, CNTPCT_EL0, CNTP_CTL_EL0, CNTP_TVAL_EL0};
use cortex_a::registers::VBAR_EL1;

use tock_registers::interfaces::{Readable, Writeable};
use crate::config::*;
use crate::arch_debug_info;
use spin::Mutex;


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
    CNTP_TVAL_EL0.set((CLOCK_FREQ / TICKS_PER_SEC) as u64);
}


#[repr(C)]
#[derive(Debug, Clone)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sepc: usize,
    pub sstatus: usize
    
}

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
enum TrapKind {
    Synchronous = 0,
    Irq = 1,
    Fiq = 2,
    SError = 3,
}

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
enum TrapSource {
    CurrentSpEl0 = 0,
    CurrentSpElx = 1,
    LowerAArch64 = 2,
    LowerAArch32 = 3,
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
fn mmk_trap_handler(tf: &mut TrapContext, kind: TrapKind, source: TrapSource){
    arch_debug_info!("Default trap occur in MMK: kind=[{:?}] source=[{:?}]", kind, source);
    arch_debug_info!("sp_el0={:x}, sepc={:x}", tf.x[31], tf.sepc);
    panic!("panic.");
    return;
}

#[no_mangle]
fn invalid_exception(){ }
#[no_mangle]
fn handle_sync_exception(){ }
#[no_mangle]
fn handle_irq_exception(){ }

pub fn init(){
    CNTP_CTL_EL0.write(CNTP_CTL_EL0::ENABLE::SET);
    extern "C" {
        fn mmk_exception_vector_base();
    }
    VBAR_EL1.set(mmk_exception_vector_base as usize as _);
    //VBAR_EL1.set(TRAMPOLINE as usize as _);

    //set_next_trigger();
    
    //gicv2::init();
    //gicv2::irq_set_mask(PHYS_TIMER_IRQ_NUM, false);
    arch_debug_info!("trap init success.");

}

