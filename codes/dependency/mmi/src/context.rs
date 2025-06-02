
#[repr(C)]
pub struct ProxyContext{
    pub nk_register: [usize; 32], //nk的寄存器
    //_+32*8
    pub outer_register: [usize; 32], //outer kernel的寄存器 注意初始化的时候把栈指针设置好
    //_+64*8
    pub nk_satp: usize, // nk的satp
    pub outer_satp: usize, // outer的satp
    //_+66*8
    pub nk_sie: usize,
    pub outer_sie: usize,

    //_+68*8
    pub nkapi_vec: [usize; 22],
    
    //_+90*8
    pub __deleted3: usize,

    //_+91*8
    pub nkapi_enable: usize,

    //_+92*8
    pub __deleted2: usize,

    pub __deleted: usize, 
    //user trap return address, currently referenced by OS, to build return context.

    //max: 512*8
}

#[repr(C)]
pub struct ConfigData{
    pub usr_trap_handler: usize,
    //address of user trap handler.

    pub kernel_trap_handler: usize,
    //address of kernel trap handler.

    pub signal_handler: usize,
    //address of signal handler.

    pub allocator_start: usize,
    //start address of outer allocator.

    pub allocator_end: usize,
    //end address of outer allocator.

    pub shared_start_vaddr: usize,
    //start address of outer allocator.

    pub shared_end_vaddr: usize
    //end address of outer allocator.
}
