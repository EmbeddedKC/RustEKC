#ifndef PARAM_H_
#define PARAM_H_

#define FW_JUMP_ADDR	0x60800000

/////////////////////////////
//aarch64 reg define.
//used by .S file

#define REGSIZE		8
#define REGSHIFT	3
#define LOAD		LDR
#define STOR		STR

#define PRIM_HART   0

/////////////////////////////


#endif /* PARAM_H_ */
