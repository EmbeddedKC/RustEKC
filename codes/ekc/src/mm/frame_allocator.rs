use super::{PhysAddr, PhysPageNum};
use alloc::vec::{self, Vec};
use spin::Mutex;
use crate::*;
use lazy_static::*;
use alloc::collections::BTreeMap;
use crate::debug_info;
use crate::mm::get_pte_array;

pub trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self, src: u8) -> Option<PhysPageNum>;
    fn alloc_multiple(&mut self, owner: u8, blksize: usize, align: usize) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum, src: u8);
    fn dealloc_multiple(&mut self, ppn: PhysPageNum, src: u8, blksize: usize);
    fn add_ref(&mut self, ppn: PhysPageNum, src: u8);
    fn fork(&mut self, ppn: PhysPageNum, src: u8, dst: u8);
    fn enquire_ref(&mut self, ppn: PhysPageNum) -> Vec<u8>;
}

pub struct StackFrameAllocator {
    current: PhysPageNum,
    end: PhysPageNum,
    recycled: Vec<PhysPageNum>,
    refcounter: BTreeMap<PhysPageNum, Vec<u8>>,
}

impl StackFrameAllocator {
    pub fn init(&mut self, l: VirtPageNum, r: VirtPageNum) {
        self.current = l.into();
        self.end = r.into();
        debug_info!("[{:x} - {:x}] last {} Physical Frames.", self.current.0, self.end.0 , self.end.0 - self.current.0);
    }
}
impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        Self {
            current: PhysPageNum(0),
            end: PhysPageNum(0),
            recycled: Vec::new(),
            refcounter: BTreeMap::new(),
        }
    }
    fn alloc_multiple(&mut self, owner: u8, mut blksize: usize, mut align: usize) -> Option<PhysPageNum>  {
            if blksize <= 0 {
                blksize = 1;
            }
            if align <= 0 {
                align = 1;
            }
            // 排序并去重，防止重复值干扰判断
            let mut sorted = self.recycled.clone();
            sorted.sort_unstable();
            sorted.dedup();

            //alloc recycled block
            for window in sorted.windows(blksize.into()) {

                if window[0].0 % align != 0 {
                    continue;
                }

                let mut fail = false;
                for i in 0 .. window.len()-1 {
                    if window[i].0 + 1 != window[i+1].0 {
                        fail = true;
                        break;
                    }
                }
                
                if !fail {
                    let ret = window[0].clone();

                    for a in 0..blksize {
                        if let Some(pos) = self.recycled.iter().position(|&x| x == window[a as usize]) {
                            self.recycled.remove(pos);
                        }
                    }
                    return Some(ret);
                }
            }
        
            //alloc new block
            let mut record = self.current;
            while record.0 % align != align - 1 {
                self.current.step();
                record = self.current;
                self.recycled.push(record);
            }

            for i in 0..blksize {
                if self.current == self.end {
                    debug_warn!("No enough free page!");
                    return None;
                } else {
                    self.current.step();
                    
                    self.refcounter.insert(self.current, alloc::vec![owner]);
                    //debug_info_level!(1,"allocated ppn: {:x}", self.current.0);
                }
            }
            Some((record.0 + 1).into())
    }
    fn alloc(&mut self, owner: u8) -> Option<PhysPageNum> {
        //debug_info_level!(6,"allocator start.");
            
        if let Some(ppn) = self.recycled.pop() {
            debug_info_level!(1,"alloced recycled ppn: {:x}", ppn.0);
            self.refcounter.insert(ppn, alloc::vec![owner]);
            //debug_info_level!(6,"allocator_return");
                
            Some(ppn.into())
        } else {
            if self.current == self.end {
                debug_warn!("No enough free page!");
                
                None
            } else {
                self.current.step();
                
                self.refcounter.insert(self.current, alloc::vec![owner]);
                //debug_info_level!(1,"allocated ppn: {:x}", self.current.0);
                Some(self.current)
            }
        }
    }

    fn dealloc_multiple(&mut self, ppn: PhysPageNum, user: u8, blksize: usize) {
        // if self.refcounter.contains_key(&ppn) {
        // let no_ref = false;
        for i in 0..(blksize as usize) {
            self.dealloc(PhysPageNum{0: ppn.0+i}, user);
        }  
    }

    fn dealloc(&mut self, ppn: PhysPageNum, user: u8) {
        // if self.refcounter.contains_key(&ppn) {
        // let no_ref = false;
        if let Some(ref_times) = self.refcounter.get_mut(&ppn) {
            ref_times.retain(|x|{*x != user});

            //debug_info!{"dealloced ppn: {:X}", ppn}
                
            // debug_info!{"the refcount of {:X} decrease to {}", ppn, ref_times}
            if ref_times.is_empty() {
                self.refcounter.remove(&ppn);
                self.recycled.push(ppn);
                return;
            }

            if ref_times[0] == 0 && ref_times.len() == 2 {
                ref_times.remove(0);
            }
        }      
    }

    fn fork(&mut self, ppn: PhysPageNum, src: u8, dst: u8){
        if let Some(ref_times) = self.refcounter.get_mut(&ppn) {
            if ref_times[0] == 0 || ref_times[0] == src{
                if ref_times[0] != 0 {
                    ref_times.insert(0, 0);
                }
                ref_times.push(dst);
            }else{
                debug_info!{"only the owner can fork pages! {:X}", ppn.0}
            }
        }      
    }

    fn add_ref(&mut self, ppn: PhysPageNum, src: u8) {
        //debug_info!("adding ref: {:x}",ppn.0);
        let ref_user = self.refcounter.get_mut(&ppn).unwrap();
        ref_user.push(src);
    }


    fn enquire_ref(&mut self, ppn: PhysPageNum) -> Vec<u8>{
        if let Some(ref_times) = self.refcounter.get_mut(&ppn) {
            if ref_times[0] == 0 && ref_times.len() == 2 {
                ref_times.remove(0);
            }
    
            return (*ref_times).to_vec().clone();
        }
        return Vec::new();
    }

}

type FrameAllocatorImpl = StackFrameAllocator;

lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocatorImpl> =
        Mutex::new(FrameAllocatorImpl::new());
    pub static ref OUTER_FRAME_ALLOCATOR: Mutex<FrameAllocatorImpl> =
        Mutex::new(FrameAllocatorImpl::new()); 
}

extern "C" {
    fn ekernel();
}

pub fn init_frame_allocator() {

    FRAME_ALLOCATOR
        .lock()
        .init(VirtAddr::from(ekernel as usize).ceil(), VirtAddr::from(NKSPACE_END).floor());
    
}


pub fn outer_frame_alloc(owner: u8) -> Option<PhysPageNum> {
    
    let mut outer_allocator = OUTER_FRAME_ALLOCATOR.lock();
    
    if outer_allocator.current.0 == 0 {
        let st: PhysPageNum = PhysAddr::from(CONFIGDATA().allocator_start).ceil();
        let ed: PhysPageNum = PhysAddr::from(CONFIGDATA().allocator_end).floor();
        debug_warn!("Allocator config: {:?} - {:?}", st, ed);

        outer_allocator.init(st.into(), ed.into());
        
    }

    let pn = outer_allocator.alloc(owner);
    
    if let Some(ppn) = pn{
        unsafe{
            for i in 0..512 {
                let adr = ((ppn.0<<12) + i*8) as *mut usize;
                *adr = 0;
            }
        }
    }
    pn
    
}

pub fn outer_frame_dealloc(ppn: PhysPageNum, user: u8) {
    OUTER_FRAME_ALLOCATOR.lock().dealloc(ppn, user);
}

pub fn frame_alloc() -> Option<PhysPageNum> {
    let pn = FRAME_ALLOCATOR
        .lock()
        .alloc(0);
    
    // if let Some(ppn) = pn {
    //     unsafe{
    //         for i in 0..PAGE_SIZE/4 {
    //             let adr = ((ppn.0<<12) + i*8) as *mut u64;
    //             *adr = 0;
    //         }
    //     }
    // }else{
    //     debug_error!("No enough space for Page Table!");
    // }
    pn
}
pub fn frame_alloc_multiple(blksiz: usize, align: usize) -> Option<PhysPageNum> {
    let pn = FRAME_ALLOCATOR
        .lock()
        .alloc_multiple(0, blksiz, align);
    
    // if let Some(ppn) = pn {
    //     unsafe{
    //         for i in 0..512 {
    //             let adr = ((ppn.0<<12) + i*8) as *mut usize;
    //             *adr = 0;
    //         }
    //     }
    // }else{
    //     debug_error!("No enough space for Page Table!");
    // }
    pn
}


pub fn permanent_frame_alloc() -> Option<PhysPageNum> {
    let pn = FRAME_ALLOCATOR
        .lock()
        .alloc(0);
    
    if let Some(ppn) = pn{
        unsafe{
            for i in 0..512 {
                let adr = ((ppn.0<<12) + i*8) as *mut usize;
                *adr = 0;
            }
        }
    }else{
        debug_error!("Permanent: No enough space for Page Table!");
    }
    pn
}

pub fn outer_fork(ppn: PhysPageNum, user: u8, target: u8) {
    OUTER_FRAME_ALLOCATOR
        .lock()
        .fork(ppn, user, target);
}

pub fn outer_frame_add_ref(ppn: PhysPageNum, user: u8) {
    OUTER_FRAME_ALLOCATOR
        .lock()
        .add_ref(ppn, user);
}

pub fn frame_dealloc_multiple(ppn: PhysPageNum, blksiz: usize) {
    FRAME_ALLOCATOR
        .lock()
        .dealloc_multiple(ppn, 0, blksiz);
}

pub fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR
        .lock()
        .dealloc(ppn, 0);
}

pub fn enquire_ref(ppn: PhysPageNum) -> Vec<u8> {
    OUTER_FRAME_ALLOCATOR
        .lock()
        .enquire_ref(ppn)
}

pub fn add_free(ppn: PhysPageNum){
    FRAME_ALLOCATOR.lock().recycled.push(ppn);
}


// #[allow(unused)]
// pub fn frame_allocator_test() {
//     let mut v: Vec<FrameTracker> = Vec::new();
//     for i in 0..5 {
//         let frame = frame_alloc().unwrap();
//         debug_info!("{:?}", frame);
//         v.push(frame);
//     }
//     v.clear();
//     for i in 0..5 {
//         let frame = frame_alloc().unwrap();
//         debug_info!("{:?}", frame);
//         v.push(frame);
//     }
//     drop(v);
//     debug_info!("frame_allocator_test passed!");
// }
