
use core::arch::asm;

//use aarch64_cpu::{asm, asm::barrier, registers::*};
//use aarch64_cpu::registers::{ReadWriteable, Readable, Writeable};

use cortex_a::{asm, asm::barrier, registers::*};
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

use crate::config::BOOT_KERNEL_STACK_SIZE;
use crate::pte::{PageTableEntry};
use crate::mmi::*;

use mmi::MapPermission;

#[link_section = ".bss.stack"]
static mut BOOT_STACK: [u8; BOOT_KERNEL_STACK_SIZE] = [0; BOOT_KERNEL_STACK_SIZE];

#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_L0: [PageTableEntry; 512] = [PageTableEntry::empty(); 512];

#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_L1: [PageTableEntry; 512] = [PageTableEntry::empty(); 512];

#[no_mangle]
unsafe fn switch_to_el1() {
    let lr: u64 = _start as u64; //return address of this func
    SPSel.write(SPSel::SP::ELx);
    let current_el = CurrentEL.read(CurrentEL::EL);
    if current_el >= 2 {
        if current_el == 3 {
            // Set EL2 to 64bit and enable the HVC instruction.
            SCR_EL3.write(
                SCR_EL3::NS::NonSecure + SCR_EL3::HCE::HvcEnabled + SCR_EL3::RW::NextELIsAarch64,
            );
            // Set the return address and exception level.
            SPSR_EL3.write(
                SPSR_EL3::M::EL1h
                    + SPSR_EL3::D::Masked
                    + SPSR_EL3::A::Masked
                    + SPSR_EL3::I::Masked
                    + SPSR_EL3::F::Masked,
            );
            ELR_EL3.set(lr);
        }
        // Disable EL1 timer traps and the timer offset.
        CNTHCTL_EL2.modify(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);
        CNTVOFF_EL2.set(0);
        // Set EL1 to 64bit.
        HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);
        // Set the return address and exception level.
        SPSR_EL2.write(
            SPSR_EL2::M::EL1h
                + SPSR_EL2::D::Masked
                + SPSR_EL2::A::Masked
                + SPSR_EL2::I::Masked
                + SPSR_EL2::F::Masked,
        );

        SP_EL1.set(BOOT_STACK.as_ptr_range().end as u64);

        //SP_EL1.set(boot_stack_top as u64);
        
        ELR_EL2.set(lr);
        asm::eret();
    }
}
#[no_mangle]
unsafe fn init_mmu() {

    // Device-nGnRE memory
    let attr0 = MAIR_EL1::Attr0_Device::nonGathering_nonReordering_EarlyWriteAck;
    // Normal memory
    let attr1 = MAIR_EL1::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc
        + MAIR_EL1::Attr1_Normal_Outer::WriteBack_NonTransient_ReadWriteAlloc;
    MAIR_EL1.write(attr0 + attr1); // 0xff_04

    // Enable TTBR0 and TTBR1 walks, page size = 4K, vaddr size = 48 bits, paddr size = 40 bits.
    let tcr_flags0 = TCR_EL1::EPD0::EnableTTBR0Walks
        + TCR_EL1::TG0::KiB_4
        + TCR_EL1::SH0::Inner
        + TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        + TCR_EL1::IRGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        + TCR_EL1::T0SZ.val(16); //sv48
    let tcr_flags1 = TCR_EL1::EPD1::EnableTTBR1Walks
        + TCR_EL1::TG1::KiB_4
        + TCR_EL1::SH1::Inner
        + TCR_EL1::ORGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        + TCR_EL1::IRGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        + TCR_EL1::T1SZ.val(16); //sv48
    TCR_EL1.write(TCR_EL1::IPS::Bits_40 + tcr_flags0 + tcr_flags1);
    barrier::isb(barrier::SY);

    // Set both TTBR0 and TTBR1
    let root_paddr = PhysAddr::new(BOOT_PT_L0.as_ptr() as _);
    //TTBR0_EL1.set(root_paddr.0 as _);
    //TTBR1_EL1.set(root_paddr.0 as _);
    crate::arch_set_root_pt(0,root_paddr.into());
    
    // Flush TLB
    crate::arch_flush_tlb(0);

    // Enable the MMU and turn on I-cache and D-cache
    SCTLR_EL1.modify(SCTLR_EL1::M::Enable + SCTLR_EL1::C::Cacheable + SCTLR_EL1::I::Cacheable);
    barrier::isb(barrier::SY);
    
}
#[no_mangle]
unsafe fn init_boot_page_table() {
    // 0x0000_0000_0000 ~ 0x0080_0000_0000, table
    BOOT_PT_L0[0] = PageTableEntry::new_table(VirtAddr::new(BOOT_PT_L1.as_ptr() as usize).into());
    // 0x0000_0000_0000..0x0000_4000_0000, block, device memory
    BOOT_PT_L1[0] = PageTableEntry::new_page(
        PhysAddr::new(0),
        MapPermission::R | MapPermission::W | MapPermission::G,
        true,
    );
    // 0x0000_4000_0000..0x0000_8000_0000, block, normal memory
    BOOT_PT_L1[1] = PageTableEntry::new_page(
        PhysAddr::new(0x4000_0000),
        MapPermission::R | MapPermission::W | MapPermission::X,
        true,
    );
}

#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    // PC = 0x4008_0000
    asm!("
        adrp    x8, boot_stack_top
        mov     sp, x8

        str x3, [sp, -8]! 
        str x2, [sp, -8]! 
        str x1, [sp, -8]! 
        str x0, [sp, -8]! 
        mov x0, sp

        bl      {switch_to_el1}

        bl      {init_boot_page_table}
        bl      {init_mmu}

        ldr     x8, ={arch_early_init}
        blr     x8  
        
        mov x0, sp
        ldr     x8, =mmk_main
        br      x8 

        b       .",
        switch_to_el1 = sym switch_to_el1,
        arch_early_init = sym crate::arch_early_init,
        init_boot_page_table = sym init_boot_page_table,
        init_mmu = sym init_mmu,
        options(noreturn),
    )
}
