#[macro_export]
macro_rules! entry_gate {
    ($tar:expr,$retval0: expr, $retval1: expr) => {
        unsafe{
            asm!(
                "BLR x28",
                in("x28") crate::NK_TRAMPOLINE,
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
                in("x28") crate::NK_TRAMPOLINE,
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
                in("x28") crate::NK_TRAMPOLINE,
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
                in("x28") crate::NK_TRAMPOLINE,
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
                in("x28") crate::NK_TRAMPOLINE,
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
                in("x28") crate::NK_TRAMPOLINE,
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

