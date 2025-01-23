#[allow(unused)]

pub const PAGE_SIZE: usize = 0x1000; //should not change
pub const PAGE_SIZE_BITS: usize = 0xc;

pub const SPECIAL_AREA: usize = 0xffff_ffff_ffff_f000;

pub const NK_TRAMPOLINE: usize = SPECIAL_AREA;

pub const TRAMPOLINE: usize = SPECIAL_AREA - PAGE_SIZE;
 
pub const METADATA_PAGE: usize = SPECIAL_AREA - 2*PAGE_SIZE;
 
pub const PROXY_CONTEXT: usize = METADATA_PAGE;
pub const CONFIG_DATA: usize = METADATA_PAGE + 0x400;
pub const MMKAPI_TABLE: usize = METADATA_PAGE + 0x800;
