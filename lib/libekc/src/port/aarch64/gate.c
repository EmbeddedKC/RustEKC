#include "mmk.h"

// MMK gate function definition.
int mmk_call(unsigned long id, unsigned long *args, unsigned int arglen, uint64_t *retval)
{
	unsigned long vec[5] = {0,0,0,0,0};
	for(int a = 0;a<5;a++){
		if(a < arglen){
			vec[a] = args[a];
		}
	}
		
	unsigned long ret = 0;
	unsigned long status = 0;
	asm volatile(
		"ADD SP, SP, #-0x10 \n\t"
		"STP x30, X29, [SP, #0] \n\t"
		"MOV x28, %2 \n\t"
		"MOV x7, %3 \n\t"
		"MOV x0, %4 \n\t"
		"MOV x1, %5 \n\t"
		"MOV x2, %6 \n\t"
		"MOV x3, %7 \n\t"
		"MOV x4, %8 \n\t"
        	"BLR x28 \n\t"
        	"MOV %0, x0 \n\t"
        	"MOV %1, x1 \n\t"
        	"LDP X30, X29, [SP, #0x0] \n\t"
			"ADD SP, SP, #0x10 \n\t"
                : "=r" (ret), "=r" (status)
                : "r" (-0x1000LL), "r" (id),
                "r" (vec[0]), "r" (vec[1]), "r" (vec[2]), "r" (vec[3]), "r" (vec[4])
                : "x28","x7","x0","x1","x2","x3","x4"
            );
        *retval = ret;
        return status;
}
