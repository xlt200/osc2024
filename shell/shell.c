#include "header/shell.h"
#include "header/uart.h"
#include "header/utils.h"
#include "header/mailbox.h"
#include "header/reboot.h"
#include "header/cpio.h"
#include "header/allocator.h"
#include "header/dtb.h"
#define BUFFER_MAX_SIZE 256u

extern void *_dtb_ptr;

void read_command(char* buffer) {
	int index = 0;
	while(1) {
		buffer[index] = uart_get_char();
		if(buffer[index] == '\n') {
			buffer[index] = '\0';
			// buffer[index+1] = '\n';
			//uart_send_string("\n");
			break;
		}
		else 
		{
			uart_send_char(buffer[index]);
		}
		index++;
	}
}

void shell(){
  //char array_space[256];
  //char* input_string = array_space;
  // char buffer[BUFFER_MAX_SIZE];
  char shelp[5] = "help";
  char shello[6] = "hello";
  char sinfo[5] = "info";
  char sreboot[7] = "reboot";
  char sls[3] = "ls";
  char scat[4] = "cat";
  char smalloc[7] = "malloc";
  char sdtb[4] = "dtb";
  while(1) {
	*(AUX_MU_LSR) &= 0;
	char buffer[BUFFER_MAX_SIZE];
    uart_send_string("# ");
	read_command(buffer);
    char * input_string = buffer;
	uart_send_string("\n");
	// uart_send_string(input_string);
    if(ut_string_compare(input_string,shelp)) {
       uart_send_string("help	:print this help menu\n");
       uart_send_string("hello	:print Hello World!\n");
       uart_send_string("info	:Get the hardware's information\n");
       uart_send_string("reboot	:reboot the device\n");
	   uart_send_string("ls	:list the file\n");
	   uart_send_string("cat	:print file content\n");
	   uart_send_string("malloc	:give dynamic memory space\n");
	   uart_send_string("dtb	:print device tree\n");

	} else if (ut_string_compare(input_string,shello)) {
		uart_send_string("");
        uart_send_string("Hello World!\n");
	   // uart_send_string("Hello World!\n");
    } else if (ut_string_compare(input_string,sinfo)) {
           get_board_revision();
           get_arm_mem();
    } else if (ut_string_compare(input_string,sreboot)) {
           uart_send_string("Rebooting....\n");
           reset(1000);
    } else if (ut_string_compare(input_string,sls)) {
	       uart_send_string("file list:\n");
		   cpio_ls();
		   // uart_send_string("it is ls\n");
     } else if (ut_string_compare(input_string,scat)){
		   uart_send_string("Filename: ");
		   char filename[BUFFER_MAX_SIZE];
		   read_command(filename);
           uart_send_string(filename);
		   uart_send_string("\n");
		   cpio_cat(filename);
	} else if (ut_string_compare(input_string,smalloc)){
		char *a = mem_alloc(sizeof("9876"));
		char *b = mem_alloc(sizeof("345"));
		a[0] = '9';
		a[1] = '8';
		a[2] = '7';
		a[3] = '6';
		a[4] = '\0';
		b[0] = '3';
		b[1] = '4';
		b[2] = '5';
		b[3] = '\0';
		uart_send_string(a);
		uart_send_string("\n");
	    uart_send_string(b);
		uart_send_string("\n");	 
	}	else if (ut_string_compare(input_string,sdtb)) {
		fdt_traverse(print_dtb,_dtb_ptr);
	}  else {
		uart_send_string("The instruct is not exist.\n");
	}
  }
}