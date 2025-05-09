//! ARM Power State Coordination Interface.

use core::arch::asm;

const PSCI_SYSTEM_OFF: u32 = 0x8400_0008;

fn psci_hvc_call(func: u32, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    unsafe {
        // asm!(
        //     "hvc #0",
        //     inlateout("r0") func as usize => ret,
        //     in("r1") arg0,
        //     in("r2") arg1,
        //     in("r3") arg2,
        // );
        ret = 0;
    }
    ret
}

pub fn shutdown() -> ! {
    psci_hvc_call(PSCI_SYSTEM_OFF, 0, 0, 0);
    while(true) {}
    unreachable!("It should shutdown!");
}
