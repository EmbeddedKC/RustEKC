
use crate::mmi::*;
use crate::*;
//use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::sync::Arc;
use lazy_static::*;
use spin::Mutex;

use core::arch::asm;
use super::{PhysAddr, PhysPageNum};
use super::PageTableRecord;

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss();
    fn ebss();
    fn sproxy();
    fn eproxy();
    fn ekernel();
    fn strampoline();
    fn snktrampoline();
}

lazy_static! {
    pub static ref KERNEL_SPACE: Arc<Mutex<MemorySet>> = Arc::new(Mutex::new(
        MemorySet::new_kernel()
    ));
}

pub struct MemorySet {
    //id: usize,   // 这个也找不到
    pub page_table: PageTableRecord,
    // areas: Vec<MapArea>,  // 常规的Maparea
    // chunks: ChunkArea,  // lazy优化，详见文档
    // stack_chunks: ChunkArea,  // check_lazy这个方法是唯一用到这两个地方的位置
    // mmap_chunks: Vec<ChunkArea>  // 用lazy做的优化
}

impl MemorySet {
    // pub fn clone_areas(&self) -> Vec<MapArea> {
    //     self.areas.clone()
    // }
    fn new_bare(id: usize) -> Self {
        let ptr = PageTableRecord::new(id);
        Self {
            //id,
            page_table: ptr,
        //     areas: Vec::new(),
        //     chunks: ChunkArea::new(MapType::Framed,
        //                         MapPermission::R | MapPermission::W | MapPermission::U),
        //     mmap_chunks: Vec::new(),
        //     stack_chunks: ChunkArea::new(MapType::Framed,
        //                         MapPermission::R | MapPermission::W | MapPermission::U)
        }
    }

    pub fn token(&self) -> usize {
        self.page_table.token()
    }

    /// Mention that trampoline is not collected by areas.
    fn map_trampoline(&mut self) {
        self.page_table.map(
            VirtAddr::from(TRAMPOLINE).into(),
            PhysAddr::from(VirtAddr::from(strampoline as usize)).into(),
            MapPermission::R | MapPermission::W | MapPermission::X,
        );
        //Yan_ice:额外为proxy context加一个跳板
        self.page_table.map(
            VirtAddr::from(NK_TRAMPOLINE).into(),
            PhysAddr::from(VirtAddr::from(snktrampoline as usize)).into(),
            MapPermission::R | MapPermission::W | MapPermission::X,
        );
        self.page_table.map(
            VirtAddr::from(PROXY_CONTEXT).into(),
            PhysAddr::from(VirtAddr::from(sproxy as usize)).into(),
            MapPermission::R | MapPermission::W | MapPermission::X,
        );
        
    }

    pub fn map_range(&mut self, begin: VirtPageNum, end: VirtPageNum, map_perm: MapPermission) {
        debug_info!("range: {:?}, {:?}",begin, end);
        let mut ptr: usize = begin.0;
        while ptr < end.0 {
            for level in (0..3).rev() {
                if level == 0{
                    self.page_table.map(ptr.into(), ptr.into(), map_perm);
                    ptr = ptr + 1;
                }else{
                    let size = 1 << 9*level;
                    if ptr & size-1 == 0 && ptr + size <= end.0{
                        self.page_table.map_huge(ptr.into(), ptr.into(), map_perm, level);
                        ptr = ptr + size;
                        break;
                    }
                }
            }
            
        }
    }

    pub fn map(&mut self, vpn: VirtPageNum, ppn: VirtPageNum, map_perm: MapPermission) {
        self.page_table.map(vpn.into(), ppn.into(), map_perm);
    }

    /// Without kernel stacks.
    pub fn new_kernel() -> Self {
        //let mut memory_set = Self::new_bare(0xff);
        let mut memory_set = Self::new_bare(0);

        // map trampoline
        memory_set.map_trampoline();  //映射trampoline

        // debug_info!("mapping .text section: {:x} {:x}",stext as usize, etext as usize);
        // memory_set.map_range(
        //     VirtAddr::from(stext as usize).into(),
        //     VirtAddr::from(etext as usize).into(),
        //     MapPermission::R | MapPermission::X | MapPermission::W,
        // );
        // debug_info!("mapping .rodata section: {:x} {:x}",srodata as usize, erodata as usize);
        // memory_set.map_range(
        //     VirtAddr::from(srodata as usize).into(),
        //     VirtAddr::from(erodata as usize).into(),
        //     MapPermission::R | MapPermission::W,
        // );
        // debug_info!("mapping .data section: {:x}",sdata as usize);
        // memory_set.map_range(
        //     VirtAddr::from(sdata as usize).into(),
        //     VirtAddr::from(edata as usize).into(),
        //     MapPermission::R | MapPermission::W | MapPermission::X,
        // );
        // debug_info!("mapping .bss section: {:x}",sbss as usize);
        // memory_set.map_range(
        //     VirtAddr::from(sbss as usize).into(),
        //     VirtAddr::from(ebss as usize).into(),
        //     MapPermission::R | MapPermission::W | MapPermission::X,
        // );
        // debug_info!("mapping nk frame memory: {:x}",ekernel as usize);
        // memory_set.map_range(
        //     VirtAddr::from(ekernel as usize).into(),
        //     VirtAddr::from(NKSPACE_END).into(),
        //     MapPermission::R | MapPermission::W,
        // );

        debug_info!("mapping mmk space: {:x} {:x}",stext as usize, NKSPACE_END as usize);
        memory_set.map_range(
            VirtAddr::from(stext as usize).into(),
            VirtAddr::from(NKSPACE_END as usize).into(),
            MapPermission::R | MapPermission::X | MapPermission::W
        );

        debug_info!("mapping outer kernel space: {:x}",OKSPACE_START);
        memory_set.map_range(
            VirtAddr::from(OKSPACE_START).into(),
            VirtAddr::from(OKSPACE_END).into(),
            MapPermission::R | MapPermission::W | MapPermission::X ,
        );

        debug_info!("mapping memory-mapped registers");
        for pair in MMIO {  // 这里是config硬编码的管脚地址
            memory_set.map_range(
                VirtAddr::from((*pair).0).into(),
                VirtAddr::from(((*pair).0 + (*pair).1)).into(),
                MapPermission::R | MapPermission::W
            );
            // memory_set.page_table.map((*pair).0.into(), (*pair).0.into(), 
            //     MapPermission::R | MapPermission::W | MapPermission::X)
        }

        debug_info!("MMK page table init.");

        memory_set
    }

    ///修改satp，切换到该页表
    pub fn activate(&self) {
        self.page_table.activate();
    }

    // pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
    //     self.page_table.translate(vpn)
    // }

    // pub fn print_pagetable(&mut self, from:usize, to:usize){
    //     self.page_table.print_pagetable(from,to);
    // }
}

// #[derive(Clone)]
// pub struct MapArea {
//     vpn_range: VPNRange,
//     map_perm: MapPermission,
// }

// impl MapArea {
//     pub fn new(
//         start_va: VirtAddr,
//         end_va: VirtAddr,
//         map_perm: MapPermission
//     ) -> Self {
//         let start_vpn: VirtPageNum = start_va.floor();
//         let end_vpn: VirtPageNum = end_va.ceil();
//         Self {
//             vpn_range: VPNRange::new(start_vpn, end_vpn),
//             map_perm,
//         }
//     }

//     // Alloc and map one page
//     pub fn map_one(&mut self, page_table: &mut PageTableRecord, vpn: VirtPageNum) {
//         let ppn: PhysPageNum = vpn.into();
        
//         page_table.map(vpn, ppn, self.map_perm);
//     }

//     // Alloc and map all pages
//     pub fn map(&mut self, page_table: &mut PageTableRecord) {
//         for vpn in self.vpn_range {
//             if vpn.0 & (1<<9)-1 == 0 {
//                 page_table.map_huge(vpn, vpn.into(), self.map_perm, 1);
//             }
                
//                 //self.map_one(page_table, vpn);
//         }
//     }
// }

