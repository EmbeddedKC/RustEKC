mod heap_allocator;
#[macro_use]
mod frame_allocator;
mod page_table;
mod memory_set;

use crate::{debug_info, mm::frame_allocator::{OUTER_FRAME_ALLOCATOR, outer_fork}};

use crate::mmi::context::*;
use core::arch::asm;

use alloc::{boxed::Box};
use lazy_static::*;
use spin::Mutex;

use crate::util::translate_from_user;

use alloc::vec::Vec;

// use crate::task::{current_task, Signals};

use page_table::*;

use mmk_arch::pte::*;
use super::trap::nk_trap_handler;
use crate::mmi::*;
use crate::config::*;

use crate::*;
pub use frame_allocator::{
    StackFrameAllocator, 
    FrameAllocator,
    add_free, 
    outer_frame_add_ref, 
    enquire_ref,

    frame_alloc,
    frame_dealloc,
    frame_alloc_multiple,
    frame_dealloc_multiple,
    outer_frame_alloc,
    outer_frame_dealloc
};

pub use memory_set::{MemorySet, KERNEL_SPACE};
pub use heap_allocator::HEAP_ALLOCATOR;
pub use frame_allocator::FRAME_ALLOCATOR;

extern "C" {
    fn sproxy();
    fn strampoline();
    fn snktrampoline();
    fn ssignaltrampoline();
}

pub fn get_pte_array(pa: PhysAddr, len: usize) -> &'static mut [PageTableEntry] {
    let va: VirtAddr = pa.into();
    unsafe {
        core::slice::from_raw_parts_mut(va.0 as *mut PageTableEntry, len)
    }
}

pub fn init() {
    //debug_info!("nktrampoline pa: {:x}", snktrampoline as usize);
    heap_allocator::init_heap();  // 堆空间分配器
    frame_allocator::init_frame_allocator();  // 物理页帧分配器
    // KERNEL_SPACE是个lazy启动的，启动时将pagetable等数据写好

    KERNEL_SPACE.lock().activate();  // 切换页表
    //crate::arch_flush_tlb(0);
    debug_info!("kernel table activated and init success.");

    init_vec();
    debug_info!("EKC API vector table initialized.");
}

lazy_static! {
    pub static ref PAGE_TABLE_LIST: Mutex<Vec<PageTableRecord>> = Mutex::new(
        Vec::<PageTableRecord>::new()
    );
    pub static ref CURRENT_PT: Mutex<Box<usize>> = Mutex::new(Box::new(999999));
    pub static ref LATE_DESTROY: Mutex<Box<usize>> = Mutex::new(Box::new(0));

}

macro_rules! pt_operate {
    ($handle:expr, $target:ident, $oper:block) => {
        let mut _find = false;
        for tar in PAGE_TABLE_LIST.lock().iter_mut(){
            if (tar.id() == $handle) || 
                    ($handle==usize::MAX && tar.id() == CURRENT_PT.lock().as_ref().clone()) {
                _find = true;
                let $target: &mut PageTableRecord = tar;
                //debug_info!("find page table with pt handle [{}]", $handle);
                $oper
            }
        }
        if !_find {
            debug_error!("Cannot find pagetable with handle [{:x}]!",$handle);
            nkapi_return_err!(2 as usize);
        }
    };
}


use crate::service::register_mmkapi;
pub fn init_vec(){
    let proxy = PROXYCONTEXT();
    proxy.nkapi_enable = 1; //0xffffd160
        register_mmkapi(MMKAPI_TRAP_HANDLE, nk_trap_handler as usize);

        register_mmkapi(MMKAPI_CONFIG,nkapi_config as usize);
        register_mmkapi(MMKAPI_PT_INIT,nkapi_pt_init as usize);
        register_mmkapi(MMKAPI_PT_DESTROY,nkapi_pt_destroy as usize);
        register_mmkapi(MMKAPI_ALLOC,nkapi_alloc as usize);
        register_mmkapi(MMKAPI_DEALLOC,nkapi_dealloc as usize);
        register_mmkapi(MMKAPI_ACTIVATE,nkapi_activate as usize);
        register_mmkapi(MMKAPI_WRITE,nkapi_write as usize);
        register_mmkapi(MMKAPI_TRANSLATE,nkapi_translate as usize);
        register_mmkapi(MMKAPI_GET_PTE,nkapi_get_pte as usize);
        register_mmkapi(MMKAPI_FORK_PTE,nkapi_fork_pte as usize);
        register_mmkapi(MMKAPI_SET_PERM,nkapi_set_permission as usize);
        register_mmkapi(MMKAPI_TIME,nkapi_time as usize);
        register_mmkapi(MMKAPI_DEBUG,nkapi_print_pt as usize);
        
        register_mmkapi(MMKAPI_CURRENT_PT,nkapi_current_pt as usize);

}

fn check_valid(owner: u8, ppn: PhysPageNum, perm: MapPermission) -> bool{
    //arch_scan_instruction(ppn.into());
    return true;
    //TODO: currently skip checking,
    
    //NK SPACE can never be access.
    if ppn.0 >= NKSPACE_START && ppn.0 < NKSPACE_END{
        debug_error!("No permission to access nk space {:x}", ppn.0);
        return false;
    }

    if perm.contains(MapPermission::W | MapPermission::X) &&
    !perm.contains(MapPermission::U) {
        debug_error!("PTE with flag WX!U are not allowed: 0x{:x}", ppn.0);
    }
    
    if perm.contains(MapPermission::X) && !perm.contains(MapPermission::U) {
        let pa: PhysAddr = ppn.into();
             KERNEL_SPACE.lock().page_table.map(ppn.0.into(), 
                ppn, MapPermission::R | MapPermission::W);
            arch_scan_instruction(ppn.into());
            debug_info!("check finish.");
    }

    if enquire_ref(ppn.into()).len() == 0 {
        return true;
    }

    return true;
    //only owner can access with write perm.
    //if perm.contains(MapPermission::W){
    //    if enquire_ref(ppn.into())[0] != owner {
    //        debug_error!("Only owner can have write permission {:x}", ppn.0);
    //        return false;
    //    }
    //}

    //only user can operate it.
    //for usr in enquire_ref(ppn.into()){
    //    if usr == owner {
    //        return true;
    //    }
    //}
    //debug_error!("Only page user can operate this {:x}", ppn.0);
    //return false;
}

pub fn pt_current() -> usize {
    CURRENT_PT.lock().as_ref().clone()
}


/**
 * the function below would expose to OS outer kernel.
*/
nkapi!{
    fn nkapi_time() -> usize {
        debug_info_level!(3,"ekcapi_time()");
        let mut time:usize = arch_get_cpu_time();
        
        nkapi_return_ok!(time);
    }
}

nkapi!{
    fn nkapi_config(t: usize, val1: usize, val2: usize, val3: usize){
        //debug_info_level!(3,"ekcapi_config({:x}, {:x})", t, val);
        let conf = CONFIGDATA();
        match t{
            MMKCFG_S_DELEGATE =>{
                conf.kernel_trap_handler = val1;
            }
            MMKCFG_U_DELEGATE =>{
                conf.usr_trap_handler = val1;
            }
            MMKCFG_SIGNAL => {
                conf.signal_handler = val1;
            }
            MMKCFG_ALLOCATOR => {
                conf.allocator_start = val1;
                conf.allocator_end = val2;
                //proxy.allocator_start = val;
            }
            MMKCFG_ALLOCATOR_END => {
                conf.allocator_end = val2;
                //proxy.allocator_end = val;
            }
            MMKCFG_SHARED => {
                conf.shared_start_vaddr = val1;
                conf.shared_end_vaddr = val2;
                //proxy.allocator_end = val;
            }
            10 => {
                //Print a string with size less than 512.
                //FIXME: Only for debug use.
                //It may cause unknown risks.
                unsafe{
                    let str_ptr: &[u8; 512] = &*(val1 as *const [u8; 512]);
                    if let Some(ker_ptr) = translate_from_user(str_ptr) {
                        crate::mmk_arch::print_raw_chars(ker_ptr);
                    }
                }
                nkapi_return_ok!();
            }
            _ => {
                debug_info!("Unknown config ID: {}", t);
                nkapi_return_err!();
            }
        }
        debug_info_level!(1,"config {:x} success: {:x} {:x}.", t, val1, val2);
    }
    
}

nkapi!{
    fn nkapi_print_pt(pt_handle: usize, from: usize, to: usize){
        debug_info_level!(3,"ekcapi_print_pt({:x}, {:x}, {:x})", pt_handle, from, to);
        //pt_operate! (pt_handle, target_pt, {
            // debug_info!("=========[print pt {}]==========", pt_handle);
            //target_pt.trace_address((from<<12).into());
        //});
    }
}

nkapi!{
    fn nkapi_fork_pte(pt_handle: usize, pt_child: usize, vpn_o: VirtPageNum, size: usize, cow: usize) -> PhysPageNum{
        debug_info_level!(3,"ekcapi_fork_pte({:x}, {:x}, {:x}, {:x}, {:x})", pt_handle, pt_child, vpn_o.0, size, cow);
        
        let cow = cow!=0;

        for offset in 0..size {
            let vpn: VirtPageNum = (vpn_o.0 + offset).into();
                        
            pt_operate! (pt_child, target_pt, {
                let dst_pte = target_pt.find_pte(vpn);
                if dst_pte.is_some() {
                    debug_error!("fork_pte: target pte already exists: {:?}", vpn);
    
                    nkapi_return_err!();
                    //Yan_ice: temp not err, for MMIO.
                }
            }); 
    
            let mut perm: MapPermission = MapPermission::empty();
                pt_operate! (pt_handle, target_pt, {
                    let src_pte = target_pt.find_pte(vpn);
                    if src_pte.is_none() {
                        debug_error!("fork_pte: source pte is invalid!");
                        nkapi_return_err!();
                    }
                    perm = src_pte.unwrap().perm();
                });


            if cow{
                debug_warn!("COW is enabled.");
                perm = perm & !MapPermission::W | MapPermission::O;
    
                let mut src_pte = None;
                pt_operate! (pt_handle, target_pt, {
                    target_pt.set_perm(vpn, perm);
                    src_pte = Some(target_pt.find_pte(vpn).unwrap().clone());
                });
                //debug_info!("forking pte: {:?} -> {:?}",vpn, src_pte.unwrap().ppn());
                pt_operate! (pt_child, target_pt, {
                    target_pt.map(vpn, src_pte.unwrap().ppn(), src_pte.unwrap().perm());
                });
                outer_fork(src_pte.unwrap().ppn(), pt_handle as u8, pt_child as u8);
                nkapi_return_ok!(src_pte.unwrap().ppn());
                
            }else{
                let (src_ppn, state2) = nkapi_translate(pt_handle, vpn.into(), 0);
                
                let (dst_ppn, state1) = nkapi_alloc(pt_child, vpn.0, 1, 
                    MapType::Framed.into(), (perm.bits() as usize).into());
                if state1 == 0 && state2 == 0 {
                    unsafe {
                        for offset2 in 0..512 {
                            let src_addr = ((src_ppn<<PAGE_SIZE_BITS) + offset2*8) as *const u64;
                            let dst_addr = ((dst_ppn<<PAGE_SIZE_BITS) + offset2*8) as *mut u64;
                            *dst_addr = *src_addr;
                        }
                    }
                }else {
                    nkapi_return_err!(22);
                }
            }
        }
        
        nkapi_return_ok!();
    }  
}

nkapi!{
    fn nkapi_pt_init(pt_handle: usize, re_gen: usize) -> usize {
        debug_info_level!(3,"ekcapi_pt_init({:x}, {:x})", pt_handle, re_gen);
        
        if pt_handle == 0xff {
            debug_warn!("Uid {:x} is preserved. Create failed.",pt_handle);
            nkapi_return_err!(2);
        }
        let mut re_generate = re_gen;

        let __ret = |status_code: &mut usize|-> usize { {return *status_code;} }(&mut 0);

        let mut re_activate = pt_current() == pt_handle;
        //if re_activate {
            //debug_warn!("re-init a current-activate page table.");
            //it might be a "exec" syscall.
        //}
        
        let late_destroy = LATE_DESTROY.lock().as_ref().clone();

        if late_destroy != 0 && late_destroy != pt_handle {
            debug_info!("Destroy pt [{}] triggered.",late_destroy);
            nkapi_pt_destroy(late_destroy);

            *LATE_DESTROY.lock().as_mut() = 0;
        }
        if late_destroy != 0 && late_destroy == pt_handle {
            re_generate = 1;
            *LATE_DESTROY.lock().as_mut() = 0;
        }

        debug_info_level!(0, "Creating pt [{}].",pt_handle);
        
        {
            let mut ptlist = PAGE_TABLE_LIST.lock();
            for tar in 0..ptlist.len(){
                if ptlist[tar].id() == pt_handle {
                    if re_generate != 0 && pt_handle != 0{
                        debug_info_level!(7,"Pagetable [{}] already exists, destroy and re-generating.",pt_handle);
                        ptlist.remove(tar).destroy();
                        break;
                        //debug_info!("Pagetable [{}] already exists, would not do anything.",pt_handle);
                        //return Ok(pt_handle);
                    }else{
                        debug_warn!("Pagetable [{}] already exists. Ignored.",pt_handle);
                        nkapi_return_err!(1);
                    }
                }
            }
            arch_flush_tlb(pt_handle);
        }

        //Yan_ice: here we create a new pagetable,
        let mut pt = PageTableRecord::new(pt_handle);
        
        if pt_handle != 0{
            pt_operate!(0,pt_kernel,{
                pt.map_kernel_shared(pt_kernel);
            });
        }else{

            pt.map(VirtAddr::from(SIGNAL_TRAMPOLINE).into(),
            PhysAddr::from(ssignaltrampoline as usize).into(),
            MapPermission::R | MapPermission::X | MapPermission::U);
            // mapping trampoline
            pt.map(VirtAddr::from(TRAMPOLINE).into(), 
                PhysAddr::from(strampoline as usize).into(),
                MapPermission::R | MapPermission::X | MapPermission::W);
            pt.map(VirtAddr::from(NK_TRAMPOLINE).into(), 
                PhysAddr::from(snktrampoline as usize).into(),
                MapPermission::R | MapPermission::X);
            pt.map(VirtAddr::from(PROXY_CONTEXT).into(),
                PhysAddr::from(sproxy as usize).into(),
                MapPermission::R | MapPermission::W);
        }

        PAGE_TABLE_LIST.lock().push(pt);
    
        if re_activate {
            nkapi_activate(pt_handle);
        }

        //debug_info!("Creating user PageTable [{}] finished.",pt_handle);
        
        nkapi_return_ok!(pt_handle);
    }
}


nkapi!{
    fn nkapi_set_permission(pt_handle: usize, vpn: VirtPageNum, flags: usize){
        debug_info_level!(3,"ekcapi_set_permission({:x}, {:x}, {:x})", pt_handle, vpn.0, flags);
        
        // find target pagetable
        pt_operate! (pt_handle, target_pt, {
            if target_pt.translate(vpn).is_none() {
                debug_warn!("PTE with {:?} not valid while setting permission.", vpn);
            }
            
            let mut pte_perm = MapPermission::from_bits(flags as u16).unwrap();
        
            if check_valid(pt_handle as u8, target_pt.translate(vpn).unwrap().ppn(),
                    pte_perm) {
                nkapi_return_err!(1);
            }
            target_pt.set_perm(vpn, pte_perm);
        });
    }
}

extern "C"{
    fn _ktrap();
}

nkapi!{
    fn nkapi_pt_destroy(pt_handle: usize) {
        debug_info_level!(3,"ekcapi_pt_destroy({:x})", pt_handle);
        
        if pt_handle == 0 {
            debug_error!("Cannot destroy pt [0]");
            nkapi_return_err!(1);
        }

        if pt_current() == pt_handle {
            //debug_info!("Destroying current_pt [{}], it would be destroyed later when de-activated.",pt_handle);
            *LATE_DESTROY.lock().as_mut() = pt_handle;
            nkapi_return_ok!();
        }else{
            //debug_info!("Destroy pt [{}].", pt_handle);
        }
        
        let mut ptlist = PAGE_TABLE_LIST.lock();
        for tar in 0..ptlist.len(){
            if ptlist[tar].id() == pt_handle {
                ptlist.remove(tar).destroy();
                break;
            }
        }
        arch_flush_tlb(pt_handle);
        
    }
}

nkapi!{
    fn nkapi_alloc(pt_handle: usize, root_vpn: VirtPageNum, size: usize, 
        map_type_u: usize, perm: MapPermission) -> PhysPageNum{
        debug_info_level!(3,"ekcapi_alloc({:x}, {:x}, {:x}, {:x}, {:?}({:x}))", pt_handle, root_vpn.0, size, map_type_u, perm, perm.bits());

        let mut pte_perm = perm.clone();
        let map_type = MapType::from(map_type_u);
    
        //Yan_ice: pte of pt_handle 0 is shared.
        if pt_handle == 0 {
            pte_perm = pte_perm | MapPermission::G;
        }
        pt_operate! (pt_handle, target_pt, {

            let mut first_ppn: PhysPageNum = PhysPageNum(0);

            for i in 0..size {

                let vpn = VirtPageNum{0: root_vpn.0 + i};
                let target_ppn: PhysPageNum;
                match map_type{
                    MapType::Framed => {
                        if let Some(ppn) = outer_frame_alloc(pt_handle as u8){
                            //debug_info!("outer allocating pt: {:x} ppn: 0x{:x}", pt_handle, ppn.0);
                            target_ppn = ppn;
                        }else{
                            panic!("No more memory in Outer Kernel!");
                        }
                    }
                    MapType::Raw => {
                        debug_info_level!(1, "RAW is used but not checked.");
                        if let Some(ppn) = outer_frame_alloc(pt_handle as u8){
                            target_ppn = ppn;
                            // debug_info!("outer allocating: {:?}", target_ppn);
                            // debug_info!("vpn is 0x{:x}", vpn.0);
                        }else{
                            panic!("No more memory in Outer Kernel!");
                        }

                        // debug_info!("in pt {} map ppn 0x{:x} to vpn 0x{:x}", pt_handle, target_ppn.0, VirtPageNum(target_ppn.0).0);
                        target_pt.map(VirtPageNum(target_ppn.0), target_ppn, pte_perm);

                        // debug_info!("in pt {} map ppn 0x{:x} to vpn 0x{:x}", pt_handle, target_ppn.0, vpn.0);
                        // target_pt.map(vpn, target_ppn, pte_flags);
                        nkapi_return_ok!(target_ppn);
                    }
                    MapType::Identical => {
                        target_ppn = PhysPageNum::from(vpn.0).into();
                    }
                    MapType::Specified(ppn) => {
                        target_ppn = (ppn.0 + i).into();
                    }
                }
                if i == 0{
                    first_ppn = target_ppn.into();
                }

                //debug_info!("in pt {} map ppn 0x{:x} to vpn 0x{:x}", pt_handle, target_ppn.0, vpn.0);

                if !check_valid(pt_handle as u8, target_ppn, perm) {
                    debug_error!("Invalid allocation!");
                    nkapi_return_err!(1);
                }

                target_pt.map(vpn, target_ppn, pte_perm);
            }
            nkapi_return_ok!(first_ppn);

        });
        debug_info!("ekcapi_alloc: cannot find pagetable!");
        nkapi_return_err!(1);
    }
}

nkapi!{
    fn nkapi_dealloc(pt_handle: usize, vpn_n: usize, size: usize){
        debug_info_level!(3,"ekcapi_dealloc({:x}, {:x}, {:x})", pt_handle, vpn_n, size);
        
        pt_operate! (pt_handle, target_pt, {
            for offset in 0..size {
                let vpn: VirtPageNum = VirtPageNum{0: vpn_n + offset};
                if let Some(pte) = target_pt.translate(vpn) {
                
                    // if !check_valid(pt_handle as u8, pte.ppn(), MapPermission::R) {
                    //     debug_info!("deallocate failed: invalid");
                    //     nkapi_return_err!(1);
                    // }

                    target_pt.unmap(vpn);
                    //debug_info!("dealloc vpn {:?} = ppn {:x}",vpn, pte.ppn().0);
                    outer_frame_dealloc(pte.ppn(),pt_handle as u8);
                }
                // else{
                //     debug_warn!("[{}] deallocate failed: vpn {:x} not found",pt_handle, vpn.0);
                //     nkapi_return_err!(1);
                // }
            }
            nkapi_return_ok!();
        });
    }
}


nkapi!{
    // while translating COW with write==True, it would start alloc and copy.

    fn nkapi_current_pt() -> usize {
        debug_info_level!(3,"ekcapi_current_pt()");
        
        nkapi_return_ok!(pt_current());
    }
}

nkapi!{
    // while translating COW with write==True, it would start alloc and copy.

    fn nkapi_translate(pt_handle: usize, vpn: VirtPageNum, write: usize) -> PhysPageNum {
        //debug_info_level!(3,"ekcapi_translate({:x}, {:x}, {:x})", pt_handle, vpn.0, write);
        
        let write = write!=0;
        pt_operate! (pt_handle, target_pt, {
            if let Some(pte) = target_pt.translate(vpn){
                if write && pte_is_valid(&pte) && pte_is_cow(&pte){
                    let former_ppn = pte.ppn();
                    let usrs = enquire_ref(former_ppn);
                    if usrs.len() == 1 && usrs[0] == pt_handle as u8{
                        // change the flags of the src_pte
                        target_pt.set_perm(
                            vpn, pte.perm() & !MapPermission::O | MapPermission::W
                        );
                    }else{
                        let ppn = outer_frame_alloc(pt_handle as u8).unwrap();
                        target_pt.remap_cow(vpn, ppn, former_ppn);
                    }
                }
                nkapi_return_ok!(pte.ppn());
            }
            debug_info!("WARN: cannot translate {:?}", vpn);
        });
        nkapi_return_err!(1);
    }
}

nkapi!{
    fn nkapi_translate_va(pt_handle: usize, va: VirtAddr) -> PhysAddr{
        //debug_info_level!(3,"ekcapi_translate_va({:x}, {:x})", pt_handle, va.0);
        
        pt_operate! (pt_handle, target_pt, {
            if let Some(pa) = target_pt.translate_va(va){
                nkapi_return_ok!(pa);
            }
        });
        nkapi_return_err!(2);
    }
}

nkapi!{
    fn nkapi_get_pte(pt_handle: usize, vpn: VirtPageNum) -> PageTableEntry{
        debug_info_level!(3,"ekcapi_get_pte({:x}, {:x})", pt_handle, vpn.0);
        
        pt_operate! (pt_handle, target_pt, {
            if let Some(pte) = target_pt.find_pte(vpn) {
                nkapi_return_ok!(pte.bits as usize);
            }
        });
        nkapi_return_err!();
    }
    
}


nkapi!{
    fn nkapi_write(pt_handle: usize, vpn: VirtPageNum, data_ptr: usize, offset:usize){
        debug_info_level!(3,"ekcapi_write({:x}, {:x}, {:x}, {:x})", pt_handle, vpn.0, data_ptr, offset);
        
        unsafe{
            let result: (usize, usize) = nkapi_translate_va(pt_current(), data_ptr.into());
            if result.1 == 0 {
                debug_info_level!(0, "ekcapi_write begin.");
                let former_pa = PhysAddr(result.0);
                pt_operate! (pt_handle, target_pt, {
                    let data = &*(former_pa.0 as *const usize as *mut [u8; PAGE_SIZE]);
                    let pte = &mut target_pt.translate(vpn).unwrap();
                    debug_info_level!(0, "mmk_write: copying data from pa {:x}(va {:x}) to ppn {:x}(vpn {:x})", 
                            former_pa.0, data_ptr, pte.ppn().0, vpn.0);
                    if pte_is_valid(pte) && pte_is_cow(&pte){
                        debug_info!("copy on write");
    
                        let former_ppn = pte.ppn();
                        let usrs = enquire_ref(former_ppn);
                        if usrs.len() == 1 && usrs[0] == pt_handle as u8{
                            // change the flags of the src_pte
                            target_pt.set_perm(
                                vpn, pte.perm() & !MapPermission::O | MapPermission::W
                            );
                        }else{
                            let ppn = outer_frame_alloc(pt_handle as u8).unwrap();
                            target_pt.remap_cow(vpn, ppn, former_ppn);
                        }
                        nkapi_return_ok!();
                    }
                    
                    let src = &data[0..(PAGE_SIZE - offset)];
                    let dst = &mut pte.ppn().get_bytes_array()[offset..PAGE_SIZE];
                    dst.copy_from_slice(src);

                });
                debug_info_level!(0, "nkapi write finish.");
                nkapi_return_ok!();
            }else{
                debug_info_level!(5,"ekcapi_write: not found.");
                nkapi_return_err!(1);
            }
        }
    }
}

nkapi!{
    fn nkapi_activate(pt_handle: usize){
        debug_info_level!(3,"ekcapi_activate({:x})", pt_handle);
        
        pt_operate! (pt_handle, target_pt, {
            let satp = target_pt.token();
            // nk_entry_gate();
            // unsafe {
            //     satp::write(satp);
            //     llvm_asm!("sfence.vma" :::: "volatile");
            // }
    
            // debug_info!("outer kernel's table switch.");
            debug_info_level!(5,"nkapi: pagetable [{}] activated.", pt_handle);
            *CURRENT_PT.lock().as_mut() = pt_handle;
            PROXYCONTEXT().outer_satp = satp;
        });

        let late_destroy = LATE_DESTROY.lock().as_ref().clone();

            if late_destroy != 0 && late_destroy != pt_handle {
                debug_info_level!(7,"Destroy pt [{}] triggered.",late_destroy);
                
                nkapi_pt_destroy(late_destroy);
                
                arch_flush_tlb(pt_handle);

                debug_info_level!(7,"Destroy pt [{}] finished.",late_destroy);
                *LATE_DESTROY.lock().as_mut() = 0;
            }
    }
}
