use core::ptr::{read_volatile, write_volatile};

use bsp::memory::PERIPHERAL_MMIO_BASE;

const MAILBOX_READ_REG_ADDR: *mut u32 = (PERIPHERAL_MMIO_BASE + 0xb880) as *mut u32;
const MAILBOX_STATUS_REG_ADDR: *mut u32 = (PERIPHERAL_MMIO_BASE + 0xb898) as *mut u32;
const MAILBOX_WRITE_REG_ADDR: *mut u32 = (PERIPHERAL_MMIO_BASE + 0xb8a0) as *mut u32;

#[repr(u32)]
enum BufferRequestCode {
    ProcessRequest = 0,
}

#[repr(u32)]
enum BufferResponseCode {
    RequestSuccessful = 0x80000000,
    ErrorParsingRequestBuffer = 0x80000001,
}

#[repr(u32)]
enum TagIdentifier {
    GetBoardRevision = 0x00010002,
}

pub struct Mailbox {}

impl Mailbox {
    fn is_writable() -> bool {
        unsafe { (read_volatile(MAILBOX_STATUS_REG_ADDR) & (1 << 31)) == 0 }
    }

    fn is_readable() -> bool {
        unsafe { read_volatile(MAILBOX_STATUS_REG_ADDR) & (1 << 30) == 0 }
    }

    fn read_from_reg() -> u32 {
        unsafe { read_volatile(MAILBOX_READ_REG_ADDR) }
    }

    fn write_to_reg(v: u32) {
        unsafe { write_volatile(MAILBOX_WRITE_REG_ADDR, v) }
    }

    pub fn read(channel: u8) -> *mut u32 {
        loop {
            while !Self::is_readable() {}
            let data = Self::read_from_reg();
            // get 4 LSB
            let data_channel = (data & 0b1111) as u8;
            if data_channel == channel {
                // get 28 MSB
                return (data & !(0b1111)) as *mut u32;
            }
        }
    }

    pub fn write(channel: u8, buffer_addr: *mut u32) {
        while !Self::is_writable() {}
        // use 28 MSB
        let message_addr = buffer_addr as u32 & !(0b1111);
        Self::write_to_reg(message_addr | channel as u32);
    }

    pub fn call(buffer_addr: *mut u32) -> *mut u32 {
        Self::write(8, buffer_addr);
        Self::read(8)
    }

    pub fn get_board_revision() -> u32 {
        let mut buffer = [0; 7];
        // set buffer length (in bytes)
        buffer[0] = 7 * 4;
        // set request code
        buffer[1] = BufferRequestCode::ProcessRequest as u32;
        // set tag tag identifier
        buffer[2] = TagIdentifier::GetBoardRevision as u32;
        // set value buffer length (in bytes)
        buffer[3] = 4;
        // set tag request code for request
        buffer[4] = 0;
        // set value buffer
        buffer[5] = 0;
        // set end tag bits
        buffer[6] = 0;
        Self::call(buffer.as_mut_ptr());
        buffer[5]
    }
}
