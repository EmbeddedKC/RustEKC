use crate::address::*;
use crate::flags::*;
use core::arch::asm;

#[macro_use]
use mmk_arch::entry_gate;
use super::*;

pub fn nkapi_time() -> usize{
    let retval0: usize;
    let retval1: usize;
    entry_gate!(MMKAPI_TIME, retval0, retval1);
    if retval1 != 0 {
        panic!("Error occurs.");
    }
    return retval0;
}
use crate::debug_info;

pub fn nkapi_translate(pt_handle: usize, vpn:VirtPageNum, write: bool) -> Option<PhysPageNum>{
    let retval0: usize;
    let retval1: usize;
    entry_gate!(MMKAPI_TRANSLATE,pt_handle,vpn,write, retval0, retval1);
    if retval1 == 0{
        return Some(retval0.into());
    }
    return None;

}

pub fn nkapi_translate_va(pt_handle: usize, va:VirtAddr) -> Option<PhysAddr>{
    if let Some(ppn) = nkapi_translate(pt_handle, va.floor(), false) {
        return Some(PhysAddr((ppn.0<<12) + va.page_offset()));
    }
    None
}

pub fn nkapi_get_pte(pt_handle: usize, vpn: VirtPageNum) -> Option<usize>{
    // if let Some(ppn) = nkapi_translate(pt_handle,va.clone().floor(),false) {
    //     let pa: PhysAddr = PhysAddr{0: ppn.0*crate::config::PAGE_SIZE + va.page_offset()};
    //     return Some(pa);
    // }
    // None
    let retval0: usize;
    let retval1: usize;
    entry_gate!(MMKAPI_GET_PTE,pt_handle,vpn, retval0, retval1);
    if retval1 == 0{
        return Some(retval0.into());
    }
    return None;

}

pub fn nkapi_fork_pte(pt_handle: usize, pt_child: usize, vpn: VirtPageNum, cow: bool) -> Option<PhysPageNum> {
    let retval0: usize;
    let retval1: usize;
    entry_gate!(MMKAPI_FORK_PTE, pt_handle, pt_child, vpn, cow, retval0, retval1);
    if retval1 == 0{
        return Some(retval0.into());
    }
    return None;

}

pub fn nkapi_alloc(pt_handle: usize, vpn: VirtPageNum, map_type: MapType, perm: MapPermission)-> PhysPageNum{
    let retval0: usize;
    let retval1: usize;
    entry_gate!(MMKAPI_ALLOC, pt_handle, vpn, 1 as usize, usize::from(map_type), perm, 
    retval0, retval1);
    return retval0.into();
}

pub fn nkapi_alloc_mul(pt_handle: usize, vpn_start: VirtPageNum, vpn_end: VirtPageNum, map_type: MapType, perm: MapPermission)-> PhysPageNum{
    let retval0: usize;
    let retval1: usize;
    let size = vpn_end.0 - vpn_start.0 + 1;
    entry_gate!(MMKAPI_ALLOC, pt_handle, vpn_start, size, usize::from(map_type), perm, 
    retval0, retval1);
    return retval0.into();
}

pub fn nkapi_pt_init(pt_handle: usize, regenerate: bool){
    let retval0: usize;
    let retval1: usize;

    entry_gate!(MMKAPI_PT_INIT,pt_handle, regenerate, retval0, retval1);
}

pub fn nkapi_pt_destroy(pt_handle: usize){
    let retval0: usize;
    let retval1: usize;

    entry_gate!(MMKAPI_PT_DESTROY,pt_handle, retval0, retval1);
}

pub fn nkapi_dealloc(pt_handle: usize, vpn: VirtPageNum){
    let retval0: usize;
    let retval1: usize;
    entry_gate!(MMKAPI_DEALLOC, pt_handle, vpn,retval0, retval1);
}

pub fn nkapi_activate(pt_handle: usize){
    let retval0: usize;
    let retval1: usize;
    entry_gate!(MMKAPI_ACTIVATE, pt_handle ,retval0, retval1);
}

pub fn nkapi_write(pt_handle: usize, mut current_vpn: VirtPageNum, data: &[u8], len: usize, offset:usize){
    let retval0: usize;
    let retval1: usize;
    entry_gate!(MMKAPI_WRITE,pt_handle, current_vpn, 
        data as *const [u8] as *const usize as usize, len, offset, retval0, retval1);
}

pub fn nkapi_set_user_delegate_handler(entry: usize){
    let retval0: usize;
    let retval1: usize;
    entry_gate!(MMKAPI_CONFIG, MMKCFG_U_DELEGATE, entry,
        retval0, retval1);
}

pub fn nkapi_set_kernel_delegate_handler(entry: usize){
    let retval0: usize;
    let retval1: usize;
    entry_gate!(MMKAPI_CONFIG, MMKCFG_S_DELEGATE, entry,
        retval0, retval1);
}

pub fn nkapi_set_signal_handler(entry: usize){
    let retval0: usize;
    let retval1: usize;
    entry_gate!(MMKAPI_CONFIG, MMKCFG_SIGNAL, entry,
        retval0, retval1);
}

pub fn nkapi_set_allocator_start(begin: usize, end: usize){
    let mut retval0: usize;
    let mut retval1: usize;
    entry_gate!(MMKAPI_CONFIG, MMKCFG_ALLOCATOR, begin, end,
        retval0, retval1);
    if retval1 != 0 {
        panic!("Error occurs.");
    }
}

pub fn nkapi_set_shared_range_vaddr(begin: usize, end: usize){
    let mut retval0: usize;
    let mut retval1: usize;
    entry_gate!(MMKAPI_CONFIG, MMKCFG_SHARED, begin, end,
        retval0, retval1);
    // if retval1 != 0 {
    //     panic!("Error occurs.");
    // }
}


pub fn nkapi_set_permission(pt_handle: usize, vpn:VirtPageNum, flags: usize){
    let retval0: usize;
    let retval1: usize;
    entry_gate!(MMKAPI_SET_PERM, pt_handle, vpn, flags,retval0, retval1);
}

pub fn nkapi_print_pt(pt_handle: usize, from: usize, to: usize){
    let retval0: usize;
    let retval1: usize;
    entry_gate!(MMKAPI_DEBUG, pt_handle, from, to ,retval0, retval1);
}



