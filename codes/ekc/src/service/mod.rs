
//#[cfg(feature = "logging")]
pub mod logging;

#[cfg(feature = "pkcs")]
pub mod pkcs;

//#[cfg(feature = "aescbc")]
pub mod aescbc;

//#[cfg(feature = "measure")]
pub mod measure;

use crate::context::MMK_API;

pub fn service_init(){

    //#[cfg(feature = "logging")]
    logging::app_init();

    #[cfg(feature = "pkcs")]
    pkcs::app_init();

    //#[cfg(feature = "aescbc")]
    aescbc::app_init();

    measure::app_init();
}

#[macro_export]
macro_rules! nkapi_return_ok {
    () => {
        debug_info_level!(3, "nkapi_exit_ok(0)");
        return (0,0)
    };
    ($ret: expr) => {
        debug_info_level!(3, "nkapi_exit_ok({:x})",usize::from($ret));
        return ($ret.into(), 0 as usize)
    }
}
#[macro_export]
macro_rules! nkapi_return_err {
    () => {
        debug_info_level!(10, "nkapi_exit_err(0)");
        return (0,1)
    };
    ($ret: expr) => {
        debug_info_level!(10, "nkapi_exit_err({:x})",$ret as usize);
        return ($ret as usize, 1 as usize)
    }
}
#[macro_export]
macro_rules! nkapi{
    //no return value
    ( $(#[$attr:meta])* fn $NAME:ident ( $($arg:ident : $tt:ty),+ ) $blk:block ) => {  

        #[warn(unreachable_code)]
        pub fn $NAME( $($arg:usize),+ ) -> (usize, usize){
            $( let $arg: $tt = <$tt>::from($arg); )+

            $blk;
            nkapi_return_ok!();
        }
    };

    //no params, has return value
    ( $(#[$attr:meta])* fn $NAME:ident () -> $ret:ty $blk:block ) => {
        #[warn(unreachable_code)]
        pub fn $NAME() -> (usize, usize){

            $blk;
        }
    };

    //has params, has return value
    ( $(#[$attr:meta])* fn $NAME:ident ( $($arg:ident : $tt:ty),+ ) -> $ret:ty $blk:block ) => {
        pub fn $NAME( $($arg:usize),+ ) -> (usize, usize){
            $( let $arg: $tt = <$tt>::from($arg); )+

            $blk;
        }
    };
}

pub fn register_mmkapi(id: usize, function: usize){
    if id >= 128 {
        panic!("receive an illegal mmk api id.");
    }
    unsafe{
        let addr = MMK_API(id);
        if *addr != 0 {
            debug_info!("WARN: MMKAPI with id {:x} registered twice.", id);
        }
        *addr = function;
    }
    
}