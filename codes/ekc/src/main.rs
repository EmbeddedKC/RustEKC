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


pub use console::*;
pub use api::*;
pub use crate::config::*;

use crate::mmi::*; 
use crate::mmk_arch::*;
use spin::Mutex;

use core::default;
use core::panic::PanicInfo;

// fn clear_bss() {
//     extern "C" {
//         fn sbss_no_stack();
//         fn ebss();
//     }
//     (sbss_no_stack as usize..ebss as usize).for_each(|a| {
//         unsafe { (a as *mut u8).write_volatile(0) }
//     });
// }

#[no_mangle]
pub fn mmk_main(){
    debug_info_level!(7, "Hello MMK.");
    //clear_bss();
    mm::init();
    debug_info_level!(7, "mm init success.");

    let early_pthandle: usize = 0;
    nkapi_pt_init(early_pthandle, false);

    nkapi_alloc_mul(early_pthandle, VirtAddr(OKSPACE_START).into()
    , VirtAddr(OKSPACE_END).into(), MapType::Identical, 
    MapPermission::R | MapPermission::W | MapPermission::X);

    // nkapi_alloc(early_pthandle, 
    //     VirtAddr(OKSPACE_START).into(),
    //     MapType::Identical, 
    //     MapPermission::R | MapPermission::W | MapPermission::X);
    
    debug_info!("payload pagetable init success.");

    let mut proxy = PROXYCONTEXT();
    proxy.nk_satp = mm::KERNEL_SPACE.lock().token();
    proxy.outer_register[XREG_RA] = OKSPACE_START as usize; //let ra be outer kernel init

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
