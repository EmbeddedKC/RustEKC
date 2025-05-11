#include "my_stdio.h"
#include "uart.h"
#include "va_list.h"

void vSendString( const char *s )
{
	int i = 0;

	portENTER_CRITICAL();
	int cnt = 100;
	while (s[i] != '\0' && cnt > 0) {
		uartputc_sync( s[i] );
		cnt--;
		i++;
	}
	portEXIT_CRITICAL();
}

void vSendStringISR( const char *s )
{
	int i = 0;

	//portENTER_CRITICAL();
	int cnt = 100;
	while (s[i] != '\0' && cnt > 0) {
		uartputc_sync( s[i] );
		cnt--;
		i++;
	}
	//portEXIT_CRITICAL();
}

void intToHex(unsigned int num, char *hexStr) {
    // 十六进制字符表
    const char hexDigits[] = "0123456789ABCDEF";
    int i = 0;
    
    // 处理特殊情况：整数为 0
    if (num == 0) {
        hexStr[i++] = '0';
        hexStr[i] = '\0';
        return;
    }

    // 将整数转换为十六进制字符
    while (num != 0) {
        int remainder = num % 16;
        hexStr[i++] = hexDigits[remainder];
        num /= 16;
    }

    // 添加字符串终止符
    hexStr[i] = '\0';

    // 反转字符串以获得正确的十六进制表示
    for (int j = 0, k = i - 1; j < k; j++, k--) {
        char temp = hexStr[j];
        hexStr[j] = hexStr[k];
        hexStr[k] = temp;
    }
}

void printHexISR(int num) {
    vSendStringISR("0x");
    char hex[30];
    intToHex(num, hex);
    vSendStringISR(hex);
}

void printHex(int num) {
    vSendString("0x");
    char hex[30];
    intToHex(num, hex);
    vSendString(hex);
}

 #define ZEROPAD    1       /* pad with zero */
 #define SIGN   2       /* unsigned/signed long */
 #define PLUS   4       /* show plus */
 #define SPACE  8       /* space if plus */
 #define LEFT   16      /* left justified */
 #define SPECIAL    32      /* 0x */
 #define LARGE  64      /* use 'ABCDEF' instead of 'abcdef' */

 int _div(long* n,unsigned base)
 {
     int __res; 
         __res = ((unsigned long) *n) % (unsigned) base; 
         *n = ((unsigned long) *n) / (unsigned) base; 
         return __res;
 }

#define do_div(n,base) _div(&n,base)/*({ \
    int __res; \
    __res = ((unsigned long) n) % (unsigned) base; \
    n = ((unsigned long) n) / (unsigned) base; \
    __res; })*/

 static inline int isdigit(int ch)
 {
    return (ch >= '0') && (ch <= '9');
 }



 static int skip_atoi(const char **s)
 {
    int i = 0;

    while (isdigit(**s))
        i = i * 10 + *((*s)++) - '0';
    return i;
 }


