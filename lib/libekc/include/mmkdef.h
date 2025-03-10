#ifndef _MMKDEF_H_
#define _MMKDEF_H_

///////////////////////////////////
/// bitflags

#define PTE_V (1L << 0)
#define PTE_R (1L << 1)
#define PTE_W (1L << 2)
#define PTE_X (1L << 3)
#define PTE_U (1L << 4)
#define PTE_G (1L << 5)
#define PTE_A (1L << 6)
#define PTE_D (1L << 7)

#define MAP_PERM_R PTE_R
#define MAP_PERM_W PTE_W
#define MAP_PERM_X PTE_X
#define MAP_PERM_U PTE_U

///////////////////////////////////
/// map type

#define MAP_IDENTICAL 0xfffffffffffffffe
#define MAP_FRAMED 0xfffffffffffffffd
#define MAP_RAW 0xfffffffffffffffc
#define MAP_SPECIFIED(x) ((uint64_t)x)


///////////////////////////////////
/// 
/// the value below is NK call number.
/// 
#define NKAPI_TRAP_HANDLE 0
#define NKAPI_CONFIG 1

#define NKAPI_PT_INIT 2
#define NKAPI_PT_DESTROY 3
#define NKAPI_ALLOC 4
#define NKAPI_DEALLOC 5
#define NKAPI_ACTIVATE 6
#define NKAPI_TRANSLATE 7
#define NKAPI_SET_PERM 8
#define NKAPI_GET_PTE 9

#define NKAPI_WRITE 10
#define NKAPI_FORK_PTE 11
#define NKAPI_TIME 12
#define NKAPI_DEBUG 13
#define NKAPI_CURRENT_PT 14
///
///////////////////////////////////


///////////////////////////////////
/// 
/// the value below is NK_TRAP_HANDLE param.
/// 
#define NKCFG_S_DELEGATE 0
#define NKCFG_U_DELEGATE 1
#define NKCFG_SIGNAL 2
#define NKCFG_ALLOCATOR_START 3
#define NKCFG_ALLOCATOR_END 4
///
///////////////////////////////////

///////////////////////////////////
/// 
/// the macro below is MMK gate macro definition.
/// 

#define mmk_call_5(id,arg1,arg2,arg3,arg4,arg5,ret,status) asm volatile( \
		"ADD SP, SP, #-0xf0 \n\t" \
        "" \
		"STP x2, X3, [SP, #0x0] \n\t" \
        "STP x4, X5, [SP, #0x10] \n\t" \
        "STP x6, X7, [SP, #0x20] \n\t" \
        "STP x8, X9, [SP, #0x30] \n\t" \
        "STP x10, X11, [SP, #0x40] \n\t" \
        "STP x12, X13, [SP, #0x50] \n\t" \
        "STP x14, X15, [SP, #0x60] \n\t" \
        "STP x16, X17, [SP, #0x70] \n\t" \
        "STP x18, X19, [SP, #0x80] \n\t" \
        "STP x20, X21, [SP, #0x90] \n\t" \
        "STP x22, X23, [SP, #0xa0] \n\t" \
        "STP x24, X25, [SP, #0xb0] \n\t" \
        "STP x26, X27, [SP, #0xc0] \n\t" \
        "STP x28, X29, [SP, #0xd0] \n\t" \
        "STP x29, X30, [SP, #0xe0] \n\t" \
        "" \
		"MOV x28, %2 \n\t"\
        "MOV x7, %3 \n\t"\
		"MOV x0, %4 \n\t"\
		"MOV x1, %5 \n\t"\
		"MOV x2, %6 \n\t"\
		"MOV x3, %7 \n\t"\
		"MOV x4, %8 \n\t"\
        "BLR x28 \n\t"\
        "MOV %0, x0 \n\t"\
        "MOV %1, x1 \n\t"\
        "" \
		"LDP x2, X3, [SP, #0x0] \n\t" \
        "LDP x4, X5, [SP, #0x10] \n\t" \
        "LDP x6, X7, [SP, #0x20] \n\t" \
        "LDP x8, X9, [SP, #0x30] \n\t" \
        "LDP x10, X11, [SP, #0x40] \n\t" \
        "LDP x12, X13, [SP, #0x50] \n\t" \
        "LDP x14, X15, [SP, #0x60] \n\t" \
        "LDP x16, X17, [SP, #0x70] \n\t" \
        "LDP x18, X19, [SP, #0x80] \n\t" \
        "LDP x20, X21, [SP, #0x90] \n\t" \
        "LDP x22, X23, [SP, #0xa0] \n\t" \
        "LDP x24, X25, [SP, #0xb0] \n\t" \
        "LDP x26, X27, [SP, #0xc0] \n\t" \
        "LDP x28, X29, [SP, #0xd0] \n\t" \
        "LDP x29, X30, [SP, #0xe0] \n\t" \
        "" \
		"ADD SP, SP, #0xf0 \n\t" \
            : "=r" (ret), "=r" (status)\
            : "r" (-0x1000LL), "r" (id),\
            "r" (arg1), "r" (arg2), "r" (arg3), "r" (arg4), "r" (arg5)\
            : "x28","x7","x0","x1","x2","x3","x4")

#define mmk_call_4(id,arg1,arg2,arg3,arg4, ret, status) \
	mmk_call_5(id,arg1,arg2,arg3,arg4, 0, ret, status)

#define mmk_call_3(id,arg1,arg2,arg3, ret, status) \
	mmk_call_5(id,arg1,arg2,arg3,0, 0, ret, status)

#define mmk_call_2(id,arg1,arg2, ret, status) \
	mmk_call_5(id,arg1,arg2,0,0, 0, ret, status)

#define mmk_call_1(id,arg1, ret, status) \
	mmk_call_5(id,arg1,0,0,0, 0, ret, status)

#define mmk_call_0(id, ret, status) \
	mmk_call_5(id,0,0,0,0, 0, ret, status)
	
///
///////////////////////////////////

#endif

