/*
 * FreeRTOS V202212.00
 * Copyright (C) 2020 Amazon.com, Inc. or its affiliates.  All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * https://www.FreeRTOS.org
 * https://www.github.com/FreeRTOS
 *
 */
#include "param.h"

	.section .init
	.globl _start
_start:

	// Continue primary hart
	MOV	x0, #0
	MOV	x1, #PRIM_HART

	CMP	x0, x1
	B.NE secondary

	// Primary hart
	ADR x28, _stack_top
	MOV sp, x28
	
	B down_el1

//////////////////////////////////
// if exception level higher than EL1, down to EL1

down_el3:
	MOV x0, #0x3C0
	MSR spsr_el3, x0
	ADR x0, down_el2
	MSR elr_el3, x0
	eret
down_el2:
	MOV x0, #0x3C0
	MSR spsr_el2, x0
	ADR x0, down_el1
	MSR elr_el2, x0
	eret
down_el1:

//////////////////////////////////
// Load data section

	ADR x0, _data_lma
	ADR x1, _data
	ADR x2, _edata

	CMP x1, x2
	B.GE while1_end

while1:
	LOAD x28, [x0]
	STOR x28, [x1]
	ADD x0, x0, REGSIZE
	ADD x1, x1, REGSIZE
	
	CMP x1, x2
	B.LT while1
while1_end:

//////////////////////////////////


//////////////////////////////////
// Clear bss section

	ADR x0, _bss
	ADR x1, _ebss

	CMP x0, x1
	B.GE while2_end
while2:
	STOR XZR, [x0]
	ADD x0, x0, REGSIZE
	
	CMP x0, x1
	B.LE while2
while2_end:

//////////////////////////////////

	// argc, argv, envp is 0
	MOV  x0, #0
	MOV  x1, #0
	MOV  x2, #0
	B main
	
deadlock:
	MOV x0, #0x233
	B deadlock

secondary:
	// TODO: Multicore is not supported
	MOV x0, #0x665
	B secondary
