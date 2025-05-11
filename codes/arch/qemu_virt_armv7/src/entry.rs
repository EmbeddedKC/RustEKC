
use core::arch::asm;

//use aarch64_cpu::{asm, asm::barrier, registers::*};
//use aarch64_cpu::registers::{ReadWriteable, Readable, Writeable};

// use cortex_a::{asm, asm::barrier, registers::*};
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

use crate::pte::{PageTableEntry};
use crate::mmi::*;

use mmi::MapPermission;

#[link_section = ".bss.stack"]
static mut BOOT_STACK: [u8; 4096] = [0; 4096];

#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_L0: [usize; 4096] = [0; 4096];

#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_L1: [PageTableEntry; 256*64] = [PageTableEntry::empty(); 256*64];

#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_L1_DEV: [PageTableEntry; 256] = [PageTableEntry::empty(); 256];


#[no_mangle]
unsafe fn switch_to_el1(){

    // infinite loop for CPUID not 0
    core::arch::asm!("                    
        mrc p15, 0, r0, c0, c0, 5  
        and r0, r0, #0b11   
        cmp r0, #0
        bne switch_to_el1
    ");

    // 清除 mode bits, 设置为SVC mode (0b10011)
    // Also can be System mode (0b11111)
    // core::arch::asm!("    
    //     mrs r0, cpsr
    //     bic r0, r0, #0x1F 
    //     orr r0, r0, #0x13
    //     msr cpsr_c, r0
    // ");

    //crate::arch_set_INTR_handler(debug_handler as usize);
}


#[no_mangle]
unsafe fn init_boot_page_table(){

    // 0x400 00 000 ~ 0x401 00 000,
    //BOOT_PT_L0[index] = phys_addr | SECTION_FLAGS;

    for i in 0..64 {
        BOOT_PT_L0[(0x4000_0000 >> 20) + i] = (BOOT_PT_L1.as_ptr() as usize + 4*i*256) + 0x1; //01 is page table
    }
    
    BOOT_PT_L0[(0x0900_0000 >> 20)] = (BOOT_PT_L1_DEV.as_ptr() as usize) + 0x1; //01 is page table

    for i in 0..256 {
        // 0x400 00 000 ~ 0x401 00 000,
        for j in 0..64 {
            BOOT_PT_L1[j*256 + i] = PageTableEntry::new_page(
                PhysAddr::new(0x4000_0000 + (j<<20) + i*0x1000),
                MapPermission::R | MapPermission::W | MapPermission::X,
                false
            );
        }
        
        // 0x090 00 000 ~ 0x091 00 000,
        BOOT_PT_L1_DEV[i] = PageTableEntry::new_page(
            PhysAddr::new(0x0900_0000  + i*0x1000),
            MapPermission::R | MapPermission::W,
            false
        );
    }
}

#[no_mangle]
unsafe fn init_mmu() {

    //barrier::isb(barrier::SY);
    crate::arch_barrier();

    // Set root pt
    let root_paddr = PhysAddr::new(BOOT_PT_L0.as_ptr() as _);
    crate::arch_set_root_pt(0,root_paddr.into());
    
    // Flush TLB
    crate::arch_flush_tlb(0);

    // Enable the MMU and turn on I-cache and D-cache
    //SCTLR_EL1.modify(SCTLR_EL1::M::Enable + SCTLR_EL1::C::Cacheable + SCTLR_EL1::I::Cacheable);
    
    //enable MMU + cache + buffer

    asm!("MRC p15, 0, R0, c1, c0, 0 
        ORR R0, R0, #(1 << 0)  
        ORR R0, R0, #(1 << 2) 
        ORR R0, R0, #(1 << 12)
        MCR p15, 0, R0, c1, c0, 0 
    ");

    crate::arch_barrier();
}

extern "Rust" {
    fn mmk_main();
}

use crate::arch_early_init;

// #[naked]
// #[no_mangle]
// #[link_section = ".text._start"]
// unsafe extern "C" fn _start() -> ! {
//     // PC = 0x4008_0000
//     asm!("
//         cpsid if

//         mrs r0, cpsr
//         bic r0, r0, #0x1F 
//         orr r0, r0, #0x13 
//         msr cpsr_c, r0

//         LDR     R0, =0xFFFFFFFF
//         MCR     p15, 0, R0, c3, c0, 0

//         MOV     R0, 0

//         ldr     sp, =boot_stack_top
//         mov     r11, sp

//         bl      estart", 
//         options(noreturn),
//     )

// }
#[no_mangle]
unsafe fn estart() {
    switch_to_el1();
    init_boot_page_table();
    init_mmu();
    arch_early_init();
    mmk_main();
    panic!("unreachable");
}