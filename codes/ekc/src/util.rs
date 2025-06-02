use crate::{mmi::*, mm::pt_current};
use crate::*;
use tiny_keccak::Hasher;
use tiny_keccak::Sha3;
use alloc::format;
use crate::mmk_arch::TrapContext;


pub fn translate_from_user<T>(input: &T) -> Option<&'static T> {
    unsafe {
        let va = input as *const T as *const usize as usize;
        if let Some(pa) = nkapi_translate_va(pt_current(), va.into()){
            return Some(&mut *(pa.0 as *mut T));
        }else{
            panic!("invalid virtual address from user [{}]: {:x}", pt_current(), va);
        }
    }
    return None
}

pub fn translate_from_user_mut<T>(input: &mut T) -> Option<&'static mut T> {
    unsafe {
        let va = input as *const T as *const usize as usize;
        if let Some(pa) = nkapi_translate_va(pt_current(), va.into()){
            return Some(&mut *(pa.0 as *mut T));
        }else{
            panic!("invalid virtual address from user [{}]: {:x}", pt_current(), va);
        }
    }
    return None
}