//! PL011 UART.

use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::register_structs;
use tock_registers::registers::{ReadOnly, ReadWrite};

use crate::mmi::{PhysAddr, VirtAddr};
use spin::Mutex;
use crate::config::*;
use lazy_static::lazy_static;

const UART_BASE: PhysAddr = PhysAddr{0: 0x10009000};

//static UART: Mutex<Pl011Uart> = Mutex::new(Pl011Uart::new(arch_phys_to_virt_addr(UART_BASE)));

lazy_static! {
    static ref UART: Pl011Uart = Pl011Uart::new(arch_phys_to_virt_addr(UART_BASE));
    //static ref UART: Mutex<Pl011Uart> = Mutex::new(Pl011Uart::new(arch_phys_to_virt_addr(UART_BASE)));

}

pub fn b_warn_pl011full() -> usize{
    return 0;
}

register_structs! {
    Pl011UartRegs {
        /// Data Register.
        (0x00 => dr: ReadWrite<u32>),
        (0x04 => _reserved0),
        /// Flag Register.
        (0x18 => fr: ReadOnly<u32>),
        (0x24 => ibrd: ReadWrite<u32>),
        (0x28 => fbrd: ReadWrite<u32>),
        (0x2c => lcrh: ReadWrite<u32>),
        (0x30 => cr: ReadWrite<u32>),
        (0x34 => @END),
    }
}

struct Pl011Uart {
    base_vaddr: VirtAddr,
}

impl Pl011Uart {
    const fn new(base_vaddr: VirtAddr) -> Self {
        let ua: Pl011Uart = Self { base_vaddr };
        ua
    }

    const fn regs(&self) -> &Pl011UartRegs {
        unsafe { &*(self.base_vaddr.0 as *const usize as *const _) }
    }

    fn putchar(&self, c: u8) {
        let mut cnt: usize = 0;
        while (self.regs().fr.get() & (1 << 5) != 0) && cnt < 0x10000{
            cnt = cnt + 1;
        }
        if cnt < 0x10000 {
            self.regs().dr.set(c as u32);
        }else{
            b_warn_pl011full();
        }
    }

    fn getchar(&self) -> Option<u8> {
        if self.regs().fr.get() & (1 << 4) == 0 {
            Some(self.regs().dr.get() as u8)
        } else {
            None
        }
    }

    fn init(&self) {
        self.regs().cr.set(0);
        // self.regs().ibrd.set(13);
        // self.regs().fbrd.set(1);
        // self.regs().lcrh.set((3<<5)|(1<<4));
        self.regs().cr.set(0x300);
    }
}

pub fn pl011_init() {
    UART.init();
    //UART.lock().init();
}

pub fn console_putchar(c: usize) {
    
    UART.putchar(c as u8);
    //UART.lock().putchar(c as u8);
}


pub fn console_getchar() -> Option<u8> {
    UART.getchar()
    //UART.lock().getchar()
}
