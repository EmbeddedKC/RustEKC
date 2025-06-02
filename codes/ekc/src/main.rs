#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate mmk_arch;

extern crate alloc;
extern crate bitflags;
extern crate mmi;

mod api;
#[macro_use]
mod console;
mod mm;  
mod trap;
mod service;
mod config;
mod util;

pub use console::*;
pub use api::*;
pub use crate::config::*;

use crate::mmi::*; 
use crate::mmk_arch::*;
use spin::Mutex;

use core::default;
use core::panic::PanicInfo;

fn default_trap_handler(ctx: &mut TrapContext, typ: usize){
    debug_info!("Trap occured: {:x}", typ);
    debug_info!("From address: {:x}", ctx.x[14]);
    return;
}

#[no_mangle]
pub fn mmk_main(param_from_bootloader: [usize; 5]){
    //clear_bss();
    mm::init();
    debug_info_level!(7, "mm init success.");

    let early_pthandle: usize = 0;
    nkapi_pt_init(early_pthandle, false);

    nkapi_alloc_mul(early_pthandle, VirtAddr(OKSPACE_START).into()
    , VirtAddr(OKSPACE_END).into(), MapType::Identical, 
    MapPermission::R | MapPermission::W | MapPermission::X);
    
    nkapi_set_shared_range_vaddr(OKSPACE_START, OKSPACE_END);
    // nkapi_alloc(early_pthandle, 
    //     VirtAddr(OKSPACE_START).into(),
    //     MapType::Identical, 
    //     MapPermission::R | MapPermission::W | MapPermission::X);
    
    debug_info!("payload pagetable init success.");

    let mut proxy = PROXYCONTEXT();
    proxy.nk_satp = mm::KERNEL_SPACE.lock().token();
    proxy.outer_register[XREG_RA] = OKSPACE_START as usize; //let ra be outer kernel init
    
    for a in 0..5 {
        proxy.outer_register[XREG_PARAM + a] = param_from_bootloader[a]; //let ra be outer kernel init
    }
    
    ///////////// debug code /////////////
    if true {
        debug_info!("debug code enabled.");

        nkapi_alloc_mul(early_pthandle, VirtAddr(NKSPACE_START).into()
                , VirtAddr(NKSPACE_END).into(), MapType::Identical, 
                MapPermission::R | MapPermission::W | MapPermission::X);
        let mut config = CONFIGDATA();
        config.kernel_trap_handler = default_trap_handler as usize;
        config.usr_trap_handler = default_trap_handler as usize;
    }
    ///////////// debug code /////////////

    service::service_init();
    debug_info!("service init success.");

    nkapi_activate(early_pthandle);
    
    arch_final_init();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        debug_error!("[kernel] Panicked at {}: {} {}", location.file(), location.line(), info.message().unwrap());
    } else {
        debug_error!("[kernel] Panicked: {}", info.message().unwrap());
    }
    arch_shutdown()
}
