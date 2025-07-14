use crate::address::*;
use crate::flags::*;
use core::arch::asm;
use super::config::NK_TRAMPOLINE;

#[macro_export]
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
