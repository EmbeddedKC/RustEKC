//! PL011 UART.

use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::register_structs;
use tock_registers::registers::{ReadOnly, ReadWrite};

use crate::mmi::{PhysAddr, VirtAddr};
use spin::Mutex;
use crate::config::*;
use lazy_static::lazy_static;

const UART_BASE: PhysAddr = PhysAddr{0: 0x0900_0000};

//static UART: Mutex<Pl011Uart> = Mutex::new(Pl011Uart::new(arch_phys_to_virt_addr(UART_BASE)));

lazy_static! {
    static ref UART: Pl011Uart = Pl011Uart::new(arch_phys_to_virt_addr(UART_BASE));
}

register_structs! {
    Pl011UartRegs {
        /// Data Register.
        (0x00 => dr: ReadWrite<u32>),
        (0x04 => _reserved0),
        /// Flag Register.
        (0x18 => fr: ReadOnly<u32>),
        (0x1c => @END),
    }
}

struct Pl011Uart {
    base_vaddr: VirtAddr,
}

impl Pl011Uart {
    const fn new(base_vaddr: VirtAddr) -> Self {
        Self { base_vaddr }
    }

    const fn regs(&self) -> &Pl011UartRegs {
        unsafe { &*(self.base_vaddr.0 as *const usize as *const _) }
    }

    fn putchar(&self, c: u8) {
        while self.regs().fr.get() & (1 << 5) != 0 {}
        self.regs().dr.set(c as u32);
    }

    fn getchar(&self) -> Option<u8> {
        if self.regs().fr.get() & (1 << 4) == 0 {
            Some(self.regs().dr.get() as u8)
        } else {
            None
        }
    }
}

pub fn console_putchar(c: usize) {
    UART.putchar(c as u8);
}

pub fn console_getchar() -> Option<u8> {
    UART.getchar()
}
