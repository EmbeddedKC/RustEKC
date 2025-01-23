
use super::{
    frame_alloc, frame_dealloc, VirtAddr, PhysAddr, PhysPageNum, PageTableEntry
};
use crate::{debug_warn, debug_info};
use crate::mmi::*;
use mmi::PAGE_SIZE;
use alloc::{vec::Vec, boxed::Box};
use bitflags::*;
use mm::outer_frame_dealloc;
use crate::mm::get_pte_array;
use spin::Mutex;
use crate::*;

#[derive(Copy, Clone)]
pub struct PageTable {
    pt_id: usize,
    root_ppn: PhysPageNum,
}

    pub fn pte_is_valid(pte: &PageTableEntry) -> bool {
        pte.valid()
    }
    pub fn pte_is_cow(pte: &PageTableEntry) -> bool {
        (pte.perm() & MapPermission::O) != MapPermission::empty()
    }
    pub fn pte_readable(pte: &PageTableEntry) -> bool {
        (pte.perm() & MapPermission::R) != MapPermission::empty()
    }
    pub fn pte_writable(pte: &PageTableEntry) -> bool {
        (pte.perm() & MapPermission::W) != MapPermission::empty()
    }
    pub fn pte_executable(pte: &PageTableEntry) -> bool {
        (pte.perm() & MapPermission::X) != MapPermission::empty()
    }
    pub fn pte_is_block(pte: &PageTableEntry) -> bool {
        pte.is_block()
    }


pub struct PageTableRecord {
    pub pt_id: usize,
    pub root_ppn: PhysPageNum,
    frames: Vec<PhysPageNum>,
    //pub pages: Vec<PhysPageNum>
}

impl From<&PageTableRecord> for PageTable{
    fn from(pt: &PageTableRecord) -> Self {
        PageTable {
            pt_id: pt.pt_id,
            root_ppn: pt.root_ppn
        }
    }
}
impl From<&mut PageTableRecord> for PageTable{
    fn from(pt: &mut PageTableRecord) -> Self {
        PageTable {
            pt_id: pt.pt_id,
            root_ppn: pt.root_ppn
        }
    }
}

/// Assume that it won't oom when creating/mapping.
impl PageTableRecord {
    pub fn id(&self) -> usize{
        return self.pt_id;
    }
    
    pub fn new(id: usize) -> Self {
        let ppn = frame_alloc().unwrap();

        PageTableRecord {
            pt_id: id,
            root_ppn: ppn,
            frames: Vec::new(),
            //pages: Vec::new()
        }
    }

    pub fn destroy(mut self){

        for mapped_frame in self.frames.into_iter(){
            frame_dealloc(mapped_frame); 
            //dealloc page table
        }

        frame_dealloc(self.root_ppn);//dealloc root
        self.pt_id = usize::MAX;
        self.root_ppn = 0.into();
    }

    fn find_pte_create(&mut self, vpn: VirtPageNum) -> &mut PageTableEntry {
        self.find_pte_create_level(vpn, MMU_MAX_LEVEL)
    }
    pub fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        self.find_pte_level(vpn, MMU_MAX_LEVEL)
    }

    // level = {1,2,3}, 1 is the highest.
    pub fn find_pte_level(&self, vpn:VirtPageNum, level:usize) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut pa = self.root_ppn.into();
        let mut result: Option<&mut PageTableEntry> = None;
        for i in ((MMU_MAX_LEVEL-level)..MMU_MAX_LEVEL).rev() {
            if i == 4 {
                panic!("i = 4.");
            }
            let pte = &mut get_pte_array(pa)[idxs[i]];
            if i == (MMU_MAX_LEVEL-level) {
                result = Some(pte);
                break;
            }
            if !pte_is_valid(pte) {
                return None;
            }
            if pte_is_block(pte) {
                debug_warn!("found block in find_pte_level.");
                return None;
            }

            pa = pte.paddr();
        }
        result
    }

    pub fn find_pte_create_level(&mut self, vpn:VirtPageNum, level:usize) -> &mut PageTableEntry {
        let idxs = vpn.indexes();
        let mut pa: PhysAddr = self.root_ppn.into();

        let mut pte: &mut PageTableEntry = &mut get_pte_array(pa)[idxs[0]];
        
        for i in ((MMU_MAX_LEVEL-level)..MMU_MAX_LEVEL).rev() {
            if i == 4 {
                panic!("i = 4.");
            }
            pte = &mut get_pte_array(pa)[idxs[i]];
            if i == (MMU_MAX_LEVEL-level) {
                return pte;
                break;
            }
            if !pte_is_valid(pte) {
                let ppn = frame_alloc().unwrap();

                *pte = PageTableEntry::new_table(ppn.into());
                self.frames.push(ppn);
                
            }
            // if pte_is_block(pte) {
            //     panic!("found block in find_pte_create_level.");
            // }
            pa = pte.paddr();
        }

        // if self.pt_id != 0{
        //     debug_info!("create page table for pt: {}, vpn: 0x{:x}, ppn: 0x{:x}", self.pt_id, vpn.0, ppn.0);
        // }
        panic!("not reachable.");
    }
    
    #[cfg(feature="debug")]
    pub fn trace_address(&self, va: VirtAddr){
        let vpn = va.floor();

        let idxs = vpn.indexes();
        let mut pa: PhysAddr = self.root_ppn.into();
        let mut result: Option<&PageTableEntry> = None;
        print!("root pt address is {:x}. ",pa.0);

        print!("Tracing translation for {:?}:\n",va);
        
        for i in (0..MMU_MAX_LEVEL).rev() {
            let pte = &get_pte_array(pa)[idxs[i]];
            print!("==> finding next pte from pa={:x}, index={:x}.\n", pa.0, idxs[i]);
            if !pte_is_valid(pte) {
                print!("INVALID\n");
                debug_info!("Trace failed. {:?} -> X", va);
                return;
            }

            pa = pte.paddr();
            print!("==> next_pte={:x} (pa={:x}, flag={:?})\n", pte.bits, pa.0, pte.flags());
            
            if pte.is_block() {
                print!("The pa {:x} is the target physical address (block).\n", pa.0);
                debug_info!("Trace finished. {:?} -> 0x{:x} {:?}", va, pte.paddr().0, pte.perm());
                return;
            }
            if i == 0 {
                print!("The pa {:x} is the target physical address.\n", pa.0);
                debug_info!("Trace finished. {:?} -> 0x{:x} {:?}", va, pte.paddr().0, pte.perm());
                return;
            }

        }
    }

    
    #[allow(unused)]
    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: MapPermission) {
        let pte = self.find_pte_create_level(vpn, MMU_MAX_LEVEL);
        *pte = PageTableEntry::new_page(ppn.into(), flags, false);
        //debug_info!("mapping {:?}",vpn);
    }

    #[allow(unused)]
    pub fn map_huge(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: MapPermission, level: usize) {
        assert!(level > 0 && level < MMU_MAX_LEVEL);
        assert!(vpn.0 & (1 << 9*level ) - 1 == 0);
        assert!(ppn.0 & (1 << 9*level ) - 1 == 0);
        //debug_info!("mapping huge {:?}",vpn);
        let pte = self.find_pte_create_level(vpn, MMU_MAX_LEVEL-level);
        *pte = PageTableEntry::new_page(ppn.into(), flags, true);
    }

    #[allow(unused)]
    pub fn remap_cow(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, former_ppn: PhysPageNum) {
        let pte = self.find_pte_create(vpn); // former ppn

        *pte = PageTableEntry::new_page(ppn.into(), pte.perm() & !MapPermission::O | MapPermission::W , false);
        ppn.get_bytes_array().copy_from_slice(former_ppn.get_bytes_array());
    }
    #[allow(unused)]
    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find_pte_create(vpn);
        assert!(pte_is_valid(pte), "vpn {:?} is invalid before unmapping", vpn);
        let ppn = pte.ppn();
        *pte = PageTableEntry::empty();
        // if vpn.0 != ppn.0 {
        //     self.pages.retain(|x|{x.0!=ppn.0});
        // }
        
    }
    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        let p = self.find_pte(vpn)
            .map(|pte| {pte.clone()});
        p
    }
    pub fn translate_va(&self, va: VirtAddr) -> Option<PhysAddr> {
        if let Some(pte) = self.find_pte(va.clone().floor()) {
            if pte_is_valid(pte) {
                let pa: PhysAddr = PhysAddr{0: pte.ppn().0*PAGE_SIZE + va.page_offset()};
                return Some(pa);
            }
        }
        None
        
    }

    pub fn set_perm(&mut self, vpn: VirtPageNum, flags: MapPermission) {
        self.find_pte_create(vpn).set_perm(flags);
    }

    fn map_share(&mut self, kernel_pagetable: &mut PageTableRecord, 
        kernel_vpn: VirtPageNum, level: usize){
        let pte_kernel = kernel_pagetable.find_pte_level(kernel_vpn, level);
        let pte = self.find_pte_create_level(kernel_vpn, level);
        if let Some(src) = pte_kernel {
            *pte = *src;
        } 
    }

   pub fn map_kernel_shared(&mut self, kernel_pagetable: &mut PageTableRecord){

        // insert shared pte of os
        let idex_begin: usize = NKSPACE_END / PAGE_SIZE;
        let idex_end: usize = OKSPACE_END / PAGE_SIZE;
        for i in idex_begin..idex_end{
            if i % 0x1000 == 0 {
                self.map_share(kernel_pagetable, i.into(), 1);
            }
        }

        //share trampoline and kernel stack.
        self.map_share(kernel_pagetable, (TRAMPOLINE / PAGE_SIZE).into(), 1);
        //self.map_share(kernel_pagetable, (NK_TRAMPOLINE / PAGE_SIZE).into(), 3);
        //self.map_share(kernel_pagetable, (PROXY_CONTEXT / PAGE_SIZE).into(), 3);
        //self.map_share(kernel_pagetable, (SIGNAL_TRAMPOLINE / PAGE_SIZE).into(), 3);
        
    }

    //Yan_ice： 这个是satp！
    pub fn token(&self) -> usize {
        return arch_get_root_pt(self.pt_id, self.root_ppn);
    }

    ///修改satp，切换到该页表
    pub fn activate(&self) {
        crate::arch_set_root_pt(self.pt_id, 
            self.root_ppn);
        crate::arch_flush_tlb(self.pt_id);
        debug_info!("MMK pagetable activated.");
    }

}

