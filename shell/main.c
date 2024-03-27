#include "header/uart.h"
#include "header/shell.h"
#include "header/dtb.h"
#include "header/utils.h"

extern void *_dtb_ptr;
void main()
{

    // set up serial console
    uart_init();
	
    // say hello
    uart_send_string("\nIn shell main!\n");
	fdt_traverse(get_cpio_addr,_dtb_ptr);
    uart_send_string("Type in `help` to get instruction menu!\n");
    //echo everything back
	shell();
}
//ldr     x0, =0x2eff7600