use alloc::sync::Arc;
use alloc::format;
use tiny_keccak::{Sha3, Hasher};

use crate::arch_get_cpu_time;
use crate::{nkapi, nkapi_return_err, nkapi_return_ok};

use crate::{debug_warn, debug_info, 
    mm::frame_alloc};
use crate::service::register_mmkapi;
use mmk_arch::config::*;

pub const LOG_BASE_ADDRESS: usize = OKSPACE_END - 0x80000;
pub const LOG_END_ADDRESS: usize = LOG_BASE_ADDRESS + 0x80000;

pub static mut ALLOCED_ADDRESS: usize = LOG_BASE_ADDRESS;

fn alloc_next() {
    unsafe {
        if ALLOCED_ADDRESS >= LOG_END_ADDRESS {
            debug_warn!("logging buffer is FULL!");
            return;
        }
        let log_vpn: usize = ALLOCED_ADDRESS >> 12;
        let log_ppn: usize = usize::from(frame_alloc().unwrap()); 
        crate::mm::KERNEL_SPACE.lock().map(log_vpn.into(),log_ppn.into(), MapPermission::R | MapPermission::W);
        //debug_info!("new logging pageframe: {:x} >> {:x}", log_vpn, log_ppn);
    
        ALLOCED_ADDRESS = ALLOCED_ADDRESS + 0x1000;
    }

}

pub fn app_init(){
    alloc_next();
    unsafe {
        *(LOG_BASE_ADDRESS as *mut usize) = LOG_BASE_ADDRESS + 8;
    }
    register_mmkapi(24, app_handler as usize);
}

nkapi!{
    fn app_handler(id: usize, para1: usize, para2: usize) -> usize{
        
        match id {
            1 => {
                let time: usize = append(para1,para2);
                nkapi_return_ok!(time);
            }
            2 => {
                gethash(para1);
            }
            3 => {
                let time: usize = getall(para1, para2);
                nkapi_return_ok!(time);
            }
            4 => {
                printall();
            }
            5 => {
                //only for debug and test use.
                //should not available in release.
                unsafe{
                    *(LOG_BASE_ADDRESS as *mut usize) = LOG_BASE_ADDRESS + 8;
                }
            }
            
            _ => {
                debug_warn!("unsupported operation in logging!");
            }

        }
        nkapi_return_ok!(0 as usize);
    }
}

pub fn append(data_ptr: usize, data_len: usize) -> usize{

    unsafe{
        if(arch_phys_to_virt_addr(data_ptr.into()).0 < NKSPACE_END){
            return 0;
        }
        let mut current: usize = *(LOG_BASE_ADDRESS as *mut usize);
        //debug_info!("ptr={:x}, len={:x}, cur={:x}",data_ptr,data_len,current);
        while current + data_len >= ALLOCED_ADDRESS {
            alloc_next();
        }
        *(LOG_BASE_ADDRESS as *mut usize) = current + data_len;
        return rust_mmcpy(data_ptr, current, data_len);
    } 
}

pub fn getall(out_buf: usize, mut size: usize) -> usize{
    if(arch_phys_to_virt_addr(out_buf.into()).0 < NKSPACE_END){
        return 0;
    }
    
    unsafe{
        let mut current: usize = *(LOG_BASE_ADDRESS as *mut usize);
        if size > current - LOG_BASE_ADDRESS - 8 {
            size = current - LOG_BASE_ADDRESS - 8;
        }
        return rust_mmcpy(LOG_BASE_ADDRESS + 8, out_buf, size);

    }
}

// return time usage for evaluation
pub fn rust_mmcpy(in_buf: usize, out_buf: usize, mut size: usize) -> usize{
    let time_start: usize = arch_get_cpu_time();
    unsafe{
        (out_buf as *mut u8).copy_from(in_buf as *const u8, size);
    }
    let time_end: usize = arch_get_cpu_time();
    return time_end - time_start;
}

pub fn printall(){
    unsafe{
        let mut current: usize = *(LOG_BASE_ADDRESS as *mut usize);
        for a in (LOG_BASE_ADDRESS + 8)..current {
            crate::mmk_arch::arch_putchar(*(a as *mut u8) as usize);
        }
    }
}

pub fn gethash(out_ptr: usize){
    if(arch_phys_to_virt_addr(out_ptr.into()).0 < NKSPACE_END){
        return;
    }
    let mut hasher = Sha3::v256();
    unsafe{
        let mut current: usize = *(LOG_BASE_ADDRESS as *mut usize);
        for a in (LOG_BASE_ADDRESS + 4)..current {
            //hasher.update((&format!("{}",&*(out_ptr as *mut u8) )).as_bytes() );
        }
        let mut output: [u8; 32] = [0; 32];
        hasher.finalize(&mut output);
        //(out_ptr as *mut u8).copy_from((&output[0]) as *const u8, 32);
    }
}
