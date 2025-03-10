
#ifndef _MMK_H_
#define _MMK_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include "mmkdef.h"

///////////////////////////////////
/// type declaration

typedef uint64_t PtHandle;

typedef uint64_t PhysAddr;
typedef uint64_t VirtAddr;
typedef uint64_t PhysPageNum;
typedef uint64_t VirtPageNum;
typedef uint64_t PageTableEntry;

typedef uint16_t PTEFlags;
typedef uint16_t MapPermission;

typedef uint64_t MapType;

int mmk_call(unsigned long id, unsigned long *args, unsigned int arglen, uint64_t *retval);

int nkapi_time(unsigned long *time);

int nkapi_translate(unsigned long pt_handle, VirtPageNum vpn, unsigned char write, PhysPageNum *ppn);
int nkapi_translate_va(unsigned long pt_handle, VirtAddr va, PhysAddr *pa);
int nkapi_get_pte(unsigned long pt_handle, VirtPageNum vpn, unsigned long *pte);
int nkapi_fork_pte(unsigned long pt_handle, unsigned long pt_child, VirtPageNum vpn, unsigned char cow, PhysPageNum *ppn);
int nkapi_alloc(unsigned long pt_handle, VirtPageNum vpn, 
	MapType map_type, MapPermission map_perm, PhysPageNum *ppn);
int nkapi_dealloc(unsigned long pt_handle, VirtPageNum vpn);
int nkapi_pt_init(unsigned long pt_handle, unsigned char regenerate);
int nkapi_pt_destroy(unsigned long pt_handle);
int nkapi_activate(unsigned long pt_handle);
int nkapi_write(unsigned long pt_handle, VirtPageNum vpn,  uint8_t *data, unsigned long len, unsigned long offset);
int nkapi_set_permission(unsigned long pt_handle, VirtPageNum vpn, MapPermission map_perm);
int nkapi_print_pt(unsigned long pt_handle, unsigned long from, unsigned long to);

int nkapi_config_kernel_delegate_handler(unsigned long entry);
int nkapi_config_user_delegate_handler(unsigned long entry);
int nkapi_config_signal_handler(unsigned long entry);
int nkapi_config_allocator_range(unsigned long begin, unsigned long end);


#endif
