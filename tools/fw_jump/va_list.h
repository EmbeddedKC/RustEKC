typedef struct _va_list {
    char* addr;
    int para_num;
} my_va_list;

inline unsigned long va_read_param(my_va_list *va){
    unsigned long val = (unsigned long)-1;
    if(va->para_num == 1){
        __asm__ volatile("MOV %0, x1":"=r"(val));
    }else if(va->para_num == 2){
        __asm__ volatile("MOV %0, x2":"=r"(val));
    }else if(va->para_num == 3){
        __asm__ volatile("MOV %0, x3":"=r"(val));
    }else if(va->para_num == 4){
        __asm__ volatile("MOV %0, x4":"=r"(val));
    }else if(va->para_num == 5){
        __asm__ volatile("MOV %0, x5":"=r"(val));
    }else if(va->para_num == 6){
        __asm__ volatile("MOV %0, x6":"=r"(val));
    }else if(va->para_num == 7){
        __asm__ volatile("MOV %0, x7":"=r"(val));
    }
    va->para_num++;
    return val;
}

#ifdef  __cplusplus
    #define _ADDRESSOF(v)   ( &reinterpret_cast<const char &>(v) )
#else
    #define _ADDRESSOF(v)   ( &(v) )
#endif

//int-byte aligned.
#define PARAM_ALIGN sizeof(int)

#define _INTSIZEOF(n)   ( (sizeof(n) + PARAM_ALIGN - 1) & ~(PARAM_ALIGN - 1) )

#define my_va_start(va, args) { \
        va.addr = (char*)_ADDRESSOF(args) + _INTSIZEOF(args); \
        va.para_num = 8; \
    }

#define my_va_end(va) {va.addr = 0; va.para_num = 0; }

#define my_va_arg(va, type) va.para_num < 8 ? \
        (type)va_read_param(&va) : \
        ( *(type *)((va.addr += _INTSIZEOF(type)) - _INTSIZEOF(type)) )
        


// #define va_list my_va_list
// #define va_start my_va_start
// #define va_arg my_va_arg
// #define va_end my_va_end

#define va_list __builtin_va_list
#define va_start __builtin_va_start
#define va_arg __builtin_va_arg
#define va_end __builtin_va_end