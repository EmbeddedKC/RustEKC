use crate::address::*;
use crate::flags::*;
use core::arch::asm;
use super::config::NK_TRAMPOLINE;

///////////////////////////////////
/// 
/// the value below is NK call number.
/// 

pub const MMKAPI_TRAP_HANDLE: usize = 0;
pub const MMKAPI_CONFIG: usize = 1;
pub const MMKAPI_PT_INIT: usize = 2;
pub const MMKAPI_PT_DESTROY: usize = 3;
pub const MMKAPI_ALLOC: usize = 4;
pub const MMKAPI_DEALLOC: usize = 5;
pub const MMKAPI_ACTIVATE: usize = 6;
pub const MMKAPI_TRANSLATE: usize = 7;
pub const MMKAPI_SET_PERM: usize = 8;
pub const MMKAPI_GET_PTE: usize = 9;
pub const MMKAPI_WRITE: usize = 10;
pub const MMKAPI_FORK_PTE: usize = 11;
pub const MMKAPI_TIME: usize = 12;
pub const MMKAPI_DEBUG: usize = 13;

pub const MMKAPI_CURRENT_PT: usize = 14;

pub const MMKAPI_MEMBLOCK_SET_RANGE: usize = 14;
pub const MMKAPI_MEMBLOCK_ALLOC_RANGE: usize = 15;
pub const MMKAPI_MEMBLOCK_SET_FLAG: usize = 16;
pub const MMKAPI_INQUIRE_MEMBLOCK: usize = 17;

///
///////////////////////////////////

///////////////////////////////////
/// 
/// the value below is NK_TRAP_HANDLE param.
/// 


pub const MMKCFG_S_DELEGATE: usize = 0;
pub const MMKCFG_U_DELEGATE: usize = 1; 
pub const MMKCFG_SIGNAL: usize = 2;
pub const MMKCFG_ALLOCATOR_START: usize = 3;
pub const MMKCFG_ALLOCATOR_END: usize = 4;

pub const MMKCFG_MIN_PFN: usize = 5;

///
///////////////////////////////////
/// 

#[cfg(target_arch = "riscv64")]
macro_rules! entry_gate {
    ($tar:expr,$retval0: expr, $retval1: expr) => {
        unsafe{
            asm!(
                "jalr x1, x31, 0",
                in("x31") NK_TRAMPOLINE,
                in("x17") $tar as usize,
                lateout("x10") $retval0,
                lateout("x11") $retval1,
            );
            asm!("fence.i");
        }

    };
    ($tar:expr,$t1:expr,$retval0: expr, $retval1: expr) => {
        unsafe{
            asm!(
                "jalr x1, x31, 0",
                in("x31") NK_TRAMPOLINE,
                in("x17") $tar as usize,
                in("x10") usize::from($t1),
                lateout("x10") $retval0,
                lateout("x11") $retval1,
            );
            asm!("fence.i");
        }
    };
    ($tar:expr,$t1:expr,$t2:expr,$retval0: expr, $retval1: expr) => {
        unsafe{
            asm!(
                "jalr x1, x31, 0",
                in("x31") NK_TRAMPOLINE,
                in("x17") $tar as usize,
                in("x10") usize::from($t1),
                in("x11") usize::from($t2),
                lateout("x10") $retval0,
                lateout("x11") $retval1,
            );
            asm!("fence.i");
        }
    };
    ($tar:expr,$t1:expr,$t2:expr,$t3:expr,$retval0: expr, $retval1: expr) => {
        unsafe{
            asm!(
                "jalr x1, x31, 0",
                in("x31") NK_TRAMPOLINE,
                in("x17") $tar as usize,
                in("x10") usize::from($t1),
                in("x11") usize::from($t2),
                in("x12") usize::from($t3),
                lateout("x10") $retval0,
                lateout("x11") $retval1,
            );
            asm!("fence.i");
        }
    };
    ($tar:expr,$t1:expr,$t2:expr,$t3:expr,$t4:expr,$retval0: expr, $retval1: expr) => {
        unsafe{            
            asm!(
                "jalr x1, x31, 0",
                in("x31") NK_TRAMPOLINE,
                in("x17") $tar as usize,
                in("x10") usize::from($t1),
                in("x11") usize::from($t2),
                in("x12") usize::from($t3),
                in("x13") usize::from($t4),
                lateout("x10") $retval0,
                lateout("x11") $retval1,
            );
            asm!("fence.i");
        }
    };
    ($tar:expr,$t1:expr,$t2:expr,$t3:expr,$t4:expr,$t5:expr,$retval0: expr, $retval1: expr) => {
        unsafe{
            asm!(
                "jalr x1, x31, 0",
                in("x31") NK_TRAMPOLINE,
                in("x17") $tar as usize,
                in("x10") usize::from($t1),
                in("x11") usize::from($t2),
                in("x12") usize::from($t3),
                in("x13") usize::from($t4),
                in("x14") usize::from($t5),
                lateout("x10") $retval0,
                lateout("x11") $retval1,
            );
            asm!("fence.i");
        }
    };
}


#[cfg(target_arch = "aarch64")]
macro_rules! entry_gate {
    ($tar:expr,$retval0: expr, $retval1: expr) => {
        unsafe{
            asm!(
                "BLR x28",
                in("x28") NK_TRAMPOLINE,
                in("x7") $tar as usize,
                lateout("x0") $retval0,
                lateout("x1") $retval1,
            );
        }

    };
    ($tar:expr,$t1:expr,$retval0: expr, $retval1: expr) => {
        unsafe{
            asm!(
                "BLR x28",
                in("x28") NK_TRAMPOLINE,
                in("x7") $tar as usize,
                in("x0") usize::from($t1),
                lateout("x0") $retval0,
                lateout("x1") $retval1,
            );
        }
    };
    ($tar:expr,$t1:expr,$t2:expr,$retval0: expr, $retval1: expr) => {
        unsafe{
            asm!(
                "BLR x28",
                in("x28") NK_TRAMPOLINE,
                in("x7") $tar as usize,
                in("x0") usize::from($t1),
                in("x1") usize::from($t2),
                lateout("x0") $retval0,
                lateout("x1") $retval1,
            );
        }
    };
    ($tar:expr,$t1:expr,$t2:expr,$t3:expr,$retval0: expr, $retval1: expr) => {
        unsafe{
            asm!(
                "BLR x28",
                in("x28") NK_TRAMPOLINE,
                in("x7") $tar as usize,
                in("x0") usize::from($t1),
                in("x1") usize::from($t2),
                in("x2") usize::from($t3),
                lateout("x0") $retval0,
                lateout("x1") $retval1,
            );
        }
    };
    ($tar:expr,$t1:expr,$t2:expr,$t3:expr,$t4:expr,$retval0: expr, $retval1: expr) => {
        unsafe{            
            asm!(
                "BLR x28",
                in("x28") NK_TRAMPOLINE,
                in("x7") $tar as usize,
                in("x0") usize::from($t1),
                in("x1") usize::from($t2),
                in("x2") usize::from($t3),
                in("x3") usize::from($t4),
                lateout("x0") $retval0,
                lateout("x1") $retval1,
            );
        }
    };
    ($tar:expr,$t1:expr,$t2:expr,$t3:expr,$t4:expr,$t5:expr,$retval0: expr, $retval1: expr) => {
        unsafe{
            asm!(
                "BLR x28",
                in("x28") NK_TRAMPOLINE,
                in("x7") $tar as usize,
                in("x0") usize::from($t1),
                in("x1") usize::from($t2),
                in("x2") usize::from($t3),
                in("x3") usize::from($t4),
                in("x4") usize::from($t5),
                lateout("x0") $retval0,
                lateout("x1") $retval1,
            );
        }
    };
}

pub fn nkapi_time() -> usize{
    let retval0: usize;
    let retval1: usize;
    entry_gate!(MMKAPI_TIME, retval0, retval1);
    if retval1 != 0 {
        panic!("Error occurs.");
    }
    return retval0;
}

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

pub fn nkapi_set_allocator_start(begin: usize){
    let mut retval0: usize;
    let mut retval1: usize;
    entry_gate!(MMKAPI_CONFIG, MMKCFG_ALLOCATOR_START, begin,
        retval0, retval1);
    if retval1 != 0 {
        panic!("Error occurs.");
    }
}

pub fn nkapi_set_allocator_end(end: usize){
    let mut retval0: usize;
    let mut retval1: usize;
    entry_gate!(MMKAPI_CONFIG, MMKCFG_ALLOCATOR_END, end,
        retval0, retval1);
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



