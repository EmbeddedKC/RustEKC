//! ARM Generic Interrupt Controller v2.

#![allow(dead_code)]

use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::register_structs;
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};

use crate::mmi::{PhysAddr, VirtAddr};
use crate::trap::IrqHandlerResult;
use crate::arch_phys_to_virt_addr;
use lazy_static::*;
use spin::Mutex;

const GIC_BASE: usize = 0x0800_0000;
const GICD_BASE: PhysAddr = PhysAddr::new(GIC_BASE);
const GICC_BASE: PhysAddr = PhysAddr::new(GIC_BASE + 0x10000);

const PPI_BASE: usize = 16;
const SPI_BASE: usize = 32;


lazy_static! {
    static ref GIC: Mutex<Gic> = Mutex::new(
        Gic::new()
    );
}

register_structs! {
    #[allow(non_snake_case)]
    GicDistributorRegs {
        /// Distributor Control Register.
        (0x0000 => CTLR: ReadWrite<u32>),
        /// Interrupt Controller Type Register.
        (0x0004 => TYPER: ReadOnly<u32>),
        /// Distributor Implementer Identification Register.
        (0x0008 => IIDR: ReadOnly<u32>),
        (0x000c => _reserved_0),
        /// Interrupt Group Registers.
        (0x0080 => IGROUPR: [ReadWrite<u32>; 0x20]),
        /// Interrupt Set-Enable Registers.
        (0x0100 => ISENABLER: [ReadWrite<u32>; 0x20]),
        /// Interrupt Clear-Enable Registers.
        (0x0180 => ICENABLER: [ReadWrite<u32>; 0x20]),
        /// Interrupt Set-Pending Registers.
        (0x0200 => ISPENDR: [ReadWrite<u32>; 0x20]),
        /// Interrupt Clear-Pending Registers.
        (0x0280 => ICPENDR: [ReadWrite<u32>; 0x20]),
        /// Interrupt Set-Active Registers.
        (0x0300 => ISACTIVER: [ReadWrite<u32>; 0x20]),
        /// Interrupt Clear-Active Registers.
        (0x0380 => ICACTIVER: [ReadWrite<u32>; 0x20]),
        /// Interrupt Priority Registers.
        (0x0400 => IPRIORITYR: [ReadWrite<u32>; 0x100]),
        /// Interrupt Processor Targets Registers.
        (0x0800 => ITARGETSR: [ReadWrite<u32>; 0x100]),
        /// Interrupt Configuration Registers.
        (0x0c00 => ICFGR: [ReadWrite<u32>; 0x40]),
        (0x0d00 => _reserved_1),
        /// Software Generated Interrupt Register.
        (0x0f00 => SGIR: WriteOnly<u32>),
        (0x1000 => @END),
    }
}

register_structs! {
    #[allow(non_snake_case)]
    GicCpuInterfaceRegs {
        /// CPU Interface Control Register.
        (0x0000 => CTLR: ReadWrite<u32>),
        /// Interrupt Priority Mask Register.
        (0x0004 => PMR: ReadWrite<u32>),
        /// Binary Point Register.
        (0x0008 => BPR: ReadWrite<u32>),
        /// Interrupt Acknowledge Register.
        (0x000c => IAR: ReadOnly<u32>),
        /// End of Interrupt Register.
        (0x0010 => EOIR: WriteOnly<u32>),
        /// Running Priority Register.
        (0x0014 => RPR: ReadOnly<u32>),
        /// Highest Priority Pending Interrupt Register.
        (0x0018 => HPPIR: ReadOnly<u32>),
        (0x001c => _reserved_1),
        /// CPU Interface Identification Register.
        (0x00fc => IIDR: ReadOnly<u32>),
        (0x0100 => _reserved_2),
        /// Deactivate Interrupt Register.
        (0x1000 => DIR: WriteOnly<u32>),
        (0x2000 => @END),
    }
}

enum TriggerMode {
    Edge = 0,
    Level = 1,
}

enum Polarity {
    ActiveHigh = 0,
    ActiveLow = 1,
}

struct Gic {
    gicd_base: VirtAddr,
    gicc_base: VirtAddr,
    max_irqs: usize,
}

impl Gic {
    fn new() -> Self {
        let mut gic = Self {
            gicd_base: 0.into(),
            gicc_base: 0.into(),
            max_irqs: 0
        };
        gic
    }

    const fn gicd(&self) -> &GicDistributorRegs {
        unsafe { &*(self.gicd_base.0 as *const usize as *const _) }
    }

    const fn gicc(&self) -> &GicCpuInterfaceRegs {
        unsafe { &*(self.gicc_base.0 as *const usize as *const _) }
    }

    fn cpu_num(&self) -> usize {
        ((self.gicd().TYPER.get() as usize >> 5) & 0b111) + 1
    }

    fn configure_interrupt(&self, vector: usize, tm: TriggerMode, pol: Polarity) {
        // Only configurable for SPI interrupts
        assert!(vector < self.max_irqs);
        assert!(vector >= SPI_BASE);
        // TODO: polarity should actually be configure through a GPIO controller
        assert!(matches!(pol, Polarity::ActiveHigh));

        // type is encoded with two bits, MSB of the two determine type
        // 16 irqs encoded per ICFGR register
        let reg_ndx = vector >> 4;
        let bit_shift = ((vector & 0xf) << 1) + 1;
        let mut reg_val = self.gicd().ICFGR[reg_ndx].get();
        match tm {
            TriggerMode::Edge => reg_val |= 1 << bit_shift,
            TriggerMode::Level => reg_val &= !(1 << bit_shift),
        }
        self.gicd().ICFGR[reg_ndx].set(reg_val);
    }

    fn set_enable(&self, vector: usize, enable: bool) {
        assert!(vector < self.max_irqs);
        let reg = vector / 32;
        let mask = 1 << (vector % 32);
        if enable {
            self.gicd().ISENABLER[reg].set(mask);
        } else {
            self.gicd().ICENABLER[reg].set(mask);
        }
    }

    fn pending_irq(&self) -> Option<usize> {
        let iar = self.gicc().IAR.get();
        if iar >= 0x3fe {
            // spurious
            None
        } else {
            Some(iar as _)
        }
    }

    fn eoi(&self, vector: usize) {
        self.gicc().EOIR.set(vector as _);
    }

    fn init(&mut self, gicd_base: VirtAddr, gicc_base: VirtAddr) {
        self.gicd_base = gicd_base;
        self.gicc_base = gicc_base;
        self.max_irqs = ((self.gicd().TYPER.get() as usize & 0b11111) + 1) * 32;
        
        let gicd = self.gicd();
        let gicc = self.gicc();

        for i in (0..self.max_irqs).step_by(32) {
            gicd.ICENABLER[i / 32].set(u32::MAX);
            gicd.ICPENDR[i / 32].set(u32::MAX);
        }
        if self.cpu_num() > 1 {
            for i in (SPI_BASE..self.max_irqs).step_by(4) {
                // Set external interrupts to target cpu 0
                gicd.ITARGETSR[i / 4].set(0x01_01_01_01);
            }
        }
        // Initialize all the SPIs to edge triggered
        for i in SPI_BASE..self.max_irqs {
            self.configure_interrupt(i, TriggerMode::Edge, Polarity::ActiveHigh);
        }

        // enable GIC
        gicd.CTLR.set(1);
        gicc.CTLR.set(1);
        // unmask interrupts at all priority levels
        gicc.PMR.set(0xff);
    }
}

pub fn irq_set_mask(vector: usize, masked: bool) {
    GIC.lock().set_enable(vector, !masked);
}

pub fn handle_irq() -> IrqHandlerResult {
    if let Some(vector) = GIC.lock().pending_irq() {
        let res = match vector {
            30 => {
                super::set_next_trigger();
                IrqHandlerResult::Reschedule
            }
            _ => IrqHandlerResult::NoReschedule,
        };
        GIC.lock().eoi(vector);
        res
    } else {
        IrqHandlerResult::NoReschedule
    }
}

pub fn init() {
    GIC.lock().init(
        arch_phys_to_virt_addr(GICD_BASE), 
        arch_phys_to_virt_addr(GICC_BASE)
    );
}
