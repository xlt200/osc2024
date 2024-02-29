use core::ptr::write_volatile;
use cpu::mmio::PERIPHERAL_MMIO_BASE;
use device::mailbox::Mailbox;
use library::{console, print, println, string::String};

pub struct Shell {
    input: String,
}

impl Default for Shell {
    fn default() -> Self {
        Shell::new()
    }
}

impl Shell {
    pub fn new() -> Self {
        Self {
            input: String::from(""),
        }
    }

    fn shell_hint(&self) {
        print!("# ");
    }

    pub fn run(&mut self) -> ! {
        self.shell_hint();
        loop {
            let c = console::console().read_char();
            match c {
                '\r' | '\n' => {
                    self.execute_command();
                    self.shell_hint();
                }
                '\x08' | '\x7f' => self.backspace(),
                ' '..='~' => self.press_key(c),
                _ => (),
            }
        }
    }

    fn help(&self) {
        println!("help\t: print this help menu");
        println!("hello\t: print Hello World!");
        println!("reboot\t: reboot the device");
        println!("board-revision\t: get board revision");
    }

    fn reboot(&self) {
        const PM_PASSWORD: u32 = 0x5a000000;
        const PM_RSTC_REG_ADDR: *mut u32 = (PERIPHERAL_MMIO_BASE + 0x0010001c) as *mut u32;
        const PM_WDOG_REG_ADDR: *mut u32 = (PERIPHERAL_MMIO_BASE + 0x00100024) as *mut u32;
        unsafe {
            write_volatile(PM_RSTC_REG_ADDR, PM_PASSWORD | 0x20);
            write_volatile(PM_WDOG_REG_ADDR, PM_PASSWORD);
        }
    }

    fn hello(&self) {
        println!("Hello World!");
    }

    fn get_board_revision(&self) {
        println!("board revision: {:#08x}", Mailbox::get_board_revision());
    }

    fn execute_command(&mut self) {
        println!();
        match self.input.trim() {
            "help" => self.help(),
            "hello" => self.hello(),
            "reboot" => self.reboot(),
            "board-revision" => self.get_board_revision(),
            "" => (),
            cmd => println!("{}: command not found", cmd),
        }
        self.input.clear();
    }

    fn press_key(&mut self, key: char) {
        self.input.push(key);
        print!("{}", key);
    }

    fn backspace(&mut self) {
        if self.input.is_empty() {
            return;
        }
        self.input.pop();
        // move the cursor to the previous character and overwrite it with a space
        // then move the cursor back again
        print!("\x08 \x08");
    }
}
