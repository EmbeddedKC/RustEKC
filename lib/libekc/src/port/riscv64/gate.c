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
		"fence.i \n\t"
		"mv x31, %2 \n\t"
		"mv x17, %3 \n\t"
		"mv x10, %4 \n\t"
		"mv x11, %5 \n\t"
		"mv x12, %6 \n\t"
		"mv x13, %7 \n\t"
		"mv x14, %8 \n\t"
        "jalr x1, x31, 0 \n\t"
        "mv %0, a0 \n\t"
        "mv %1, a1 \n\t"
        "fence.i \n\t"
                : "=r" (ret), "=r" (status)
                : "r" (-0x1000), "r" (id),
                "r" (vec[0]), "r" (vec[1]), "r" (vec[2]), "r" (vec[3]), "r" (vec[4])
                : "x10","x11","x12","x13","x14","x17","x31"
            );
        *retval = ret;
        return status;
}


