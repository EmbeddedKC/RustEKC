#[macro_export]
macro_rules! entry_gate {
    ($tar:expr,$retval0: expr, $retval1: expr) => {
        unsafe{
            asm!(
                "BLX r8",
                in("r8") crate::NK_TRAMPOLINE,
                in("r7") $tar as usize,
                lateout("r0") $retval0,
                lateout("r1") $retval1,
            );
        }

    };
    ($tar:expr,$t1:expr,$retval0: expr, $retval1: expr) => {
        unsafe{
            asm!(
                "BLX r8",
                in("r8") crate::NK_TRAMPOLINE,
                in("r7") $tar as usize,
                in("r0") usize::from($t1),
                lateout("r0") $retval0,
                lateout("r1") $retval1,
            );
        }
    };
    ($tar:expr,$t1:expr,$t2:expr,$retval0: expr, $retval1: expr) => {
        unsafe{
            asm!(
                "BLX r8",
                in("r8") crate::NK_TRAMPOLINE,
                in("r7") $tar as usize,
                in("r0") usize::from($t1),
                in("r1") usize::from($t2),
                lateout("r0") $retval0,
                lateout("r1") $retval1,
            );
        }
    };
    ($tar:expr,$t1:expr,$t2:expr,$t3:expr,$retval0: expr, $retval1: expr) => {
        unsafe{
            asm!(
                "BLX r8",
                in("r8") crate::NK_TRAMPOLINE,
                in("r7") $tar as usize,
                in("r0") usize::from($t1),
                in("r1") usize::from($t2),
                in("r2") usize::from($t3),
                lateout("r0") $retval0,
                lateout("r1") $retval1,
            );
        }
    };
    ($tar:expr,$t1:expr,$t2:expr,$t3:expr,$t4:expr,$retval0: expr, $retval1: expr) => {
        unsafe{            
            asm!(
                "BLX r8",
                in("r8") crate::NK_TRAMPOLINE,
                in("r7") $tar as usize,
                in("r0") usize::from($t1),
                in("r1") usize::from($t2),
                in("r2") usize::from($t3),
                in("r3") usize::from($t4),
                lateout("r0") $retval0,
                lateout("r1") $retval1,
            );
        }
    };
    ($tar:expr,$t1:expr,$t2:expr,$t3:expr,$t4:expr,$t5:expr,$retval0: expr, $retval1: expr) => {
        unsafe{
            asm!(
                "BLX r8",
                in("r8") crate::NK_TRAMPOLINE,
                in("r7") $tar as usize,
                in("r0") usize::from($t1),
                in("r1") usize::from($t2),
                in("r2") usize::from($t3),
                in("r3") usize::from($t4),
                in("r4") usize::from($t5),
                lateout("r0") $retval0,
                lateout("r1") $retval1,
            );
        }
    };
}

