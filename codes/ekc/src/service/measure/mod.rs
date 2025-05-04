use alloc::sync::Arc;
use alloc::format;
use crate::service::register_mmkapi;
use crate::{debug_warn, debug_info, 
    mm::frame_alloc};
use lazy_static::lazy_static;
use spin::Mutex;
use mmk_arch::config::*;
use mmk_arch::arch_get_cpu_time;
use alloc::vec::Vec;
use core::hash::Hasher;
use rs_sha256::{HasherContext, Sha256Hasher};

lazy_static! {
    static ref MEASUREMENT: Mutex<[u8; 32]> = Mutex::new([0;32]);
    static ref RPOS_MEASUREMENT: Mutex<[u8; 32]> = Mutex::new([0;32]);
}

const RPOS_IMG_SIZE: usize = 256; //KB

const PAYLOAD_IMG_SIZE: usize = 1024; //KB


pub fn app_init(){

    let mut measurement: [u8;32] = [0;32];

    //let start = arch_get_cpu_time();
    //gethash_rpos(&mut measurement);
    //let end = arch_get_cpu_time();
    
    //debug_warn!("rpos hash - cost: {}",end-start);

    //let mut b = RPOS_MEASUREMENT.lock();
    //for a in 0..32 {
    //    b[a] = measurement[a];
    //}

    let start = arch_get_cpu_time();
    gethash(&mut measurement);
    let end = arch_get_cpu_time();

    debug_warn!("payload hash - cost: {}",end-start);

    let mut b = MEASUREMENT.lock();
    for a in 0..32 {
        b[a] = measurement[a];
    }

    register_mmkapi(23, app_handler as usize);
}


pub fn app_handler(id: usize, buf: &mut [u8; 32]){
    match id {
        1 => {
            let b = MEASUREMENT.lock();
            for a in 0..32 {
                buf[a] = b[a];
            }
        }
        _ => {
            debug_warn!("unsupported operation in getting measurement!");
        }

    }
}

pub fn gethash(out_ptr: &mut [u8; 32]){
    //let mut hasher = Sha3::v256();
    unsafe{

        // while a < target{
        //     let blk = &*(a as *mut [u8; 32]);
        //     out_ptr
        //     hasher.update(blk);
        //     a = a + 32;
        // }
        let blk = &*(OKSPACE_START as *mut [u8; PAYLOAD_IMG_SIZE*1024]);
        
        use rs_sha256::{HasherContext, Sha256Hasher};

        let mut sha256hasher = Sha256Hasher::default();
        sha256hasher.write(blk);

        let u64result = sha256hasher.finish();
        let bytes_result = HasherContext::finish(&mut sha256hasher);
        for a in 0..32 {
            out_ptr[a] = bytes_result[a];
        }
    }
}

pub fn gethash_rpos(out_ptr: &mut [u8; 32]){
    //let mut hasher = Sha3::v256();
    unsafe{

        let blk = &*(NKSPACE_START as *mut [u8; RPOS_IMG_SIZE*1024]);
        
        use rs_sha256::{HasherContext, Sha256Hasher};

        let mut sha256hasher = Sha256Hasher::default();
        sha256hasher.write(blk);

        let u64result = sha256hasher.finish();
        let bytes_result = HasherContext::finish(&mut sha256hasher);
        for a in 0..32 {
            out_ptr[a] = bytes_result[a];
        }
    }
}