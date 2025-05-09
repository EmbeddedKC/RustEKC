pub use crate::mmk_arch::config::*;

pub fn PROXYCONTEXT() -> &'static mut ProxyContext{

    let _PROXY_CONTEXT: usize = METADATA_PAGE;
    unsafe{ 
        &mut *(_PROXY_CONTEXT as usize 
        as *mut usize 
        as *mut ProxyContext) 
    }
}

pub fn CONFIGDATA() -> &'static mut ConfigData{

    let _CONFIG_DATA: usize = METADATA_PAGE + 0x400;
    unsafe{ 
        &mut *(_CONFIG_DATA as usize 
        as *mut usize 
        as *mut ConfigData) 
    }
}

//MMK API always need 8 byte per address
pub fn MMK_API(id: usize) -> *mut usize{

    let _MMKAPI_TABLE: usize = METADATA_PAGE + 0x800;

    if id >= 128 {
        panic!("receive an illegal mmk api id.");
    }
    let addr = (_MMKAPI_TABLE + id*8) as *mut usize;
    return addr;
}