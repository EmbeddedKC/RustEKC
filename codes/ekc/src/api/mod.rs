//pub mod flags;
#[macro_use]
pub mod nkapi;

//pub use flags::*;
pub use nkapi::*;

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

///////////////////////////////////


//global_asm!(include_str!("nk_gate.S"));
