use core::fmt;
use core::fmt::Write;

use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

use crate::mmio_deref_wrapper::MMIODerefWrapper;

use super::device_driver::DeviceDriver;
use library::{console, sync::mutex::Mutex};

struct MiniUartInner {
    registers: MMIODerefWrapper<Registers>,
}

pub struct MiniUart {
    inner: Mutex<MiniUartInner>,
}

register_bitfields![
    u32,
    AUX_ENB  [
        SPI2_ENABLE OFFSET(2) NUMBITS(1) [
            DISABLE = 0,
            ENABLE = 1,
        ],
        SPI1_ENABLE OFFSET(1) NUMBITS(1) [
            DISABLE = 0,
            ENABLE = 1,
        ],
        MINI_UART_ENABLE OFFSET(0) NUMBITS(1) [
            DISABLE = 0,
            ENABLE = 1,
        ]
    ],
    AUX_MU_IER [
        ENABLE_RECEIVE_INTERRUPT OFFSET(1) NUMBITS(1) [
            DISABLE = 0,
            ENABLE = 1,
        ],
        ENABLE_TRANSMIT_INTERRUPT OFFSET(0) NUMBITS(1) [
            DISABLE = 0,
            ENABLE = 1,
        ]
    ],
    AUX_MU_IIR [
        FIFO_CLEAR_BITS OFFSET(1) NUMBITS(2) [
            CLEAR_RECEIVE_FIFO = 0b01,
            CLEAR_TRANSMIT_FIFO = 0b10,
        ],
    ],
    AUX_MU_LCR [
        DATA_SIZE OFFSET(0) NUMBITS(1) [
            SEVEN_BITS = 0,
            EIGHT_BITS = 1,
        ]
    ],
    AUX_MU_MCR [
        RTS OFFSET(1) NUMBITS(1) [
            HIGH = 0,
            LOW = 1,
        ]
    ],
    AUX_MU_LSR [
        TRANSMITTER_IDLE OFFSET(6) NUMBITS(1) [
            IDLE = 1,
        ],
        TRANSMITTER_EMPTY OFFSET(5) NUMBITS(1) [
            EMPTY = 1,
        ],
        RECEIVER_OVERRUN OFFSET(1) NUMBITS(1) [
            OVERRUN = 1,
        ],
        DATA_READY OFFSET(0) NUMBITS(1) [
            READY = 1,
        ]
    ],
    AUX_MU_CNTL [
        TRANSMIT_AUTO_FLOW_CONTROL OFFSET(3) NUMBITS(1) [
            DISABLE = 0,
            ENABLE = 1
        ],
        RECEIVE_AUTO_FLOW_CONTROL OFFSET(2) NUMBITS(1) [
            DISABLE = 0,
            ENABLE = 1,
        ],
        TRANSMITTER_ENABLE OFFSET(1) NUMBITS(1) [
            DISABLE = 0,
            ENABLE = 1,
        ],
        RECEIVER_ENABLE OFFSET(0) NUMBITS(1) [
            DISABLE = 0,
            ENABLE = 1,
        ]
    ]
];

register_structs! {
    Registers {
        (0x00 => _reserved1),
        (0x04 => enable: ReadWrite<u32, AUX_ENB::Register>),
        (0x08 => _reserved2),
        (0x40 => data: ReadWrite<u32>),
        (0x44 => interrupt_enable: ReadWrite<u32, AUX_MU_IER::Register>),
        (0x48 => interrupt_identify: ReadWrite<u32, AUX_MU_IIR::Register>),
        (0x4c => line_controll: ReadWrite<u32, AUX_MU_LCR::Register>),
        (0x50 => modem_controll: ReadWrite<u32, AUX_MU_MCR::Register>),
        (0x54 => line_status: ReadOnly<u32, AUX_MU_LSR::Register>),
        (0x58 => _reserved3),
        (0x60 => controll: ReadWrite<u32, AUX_MU_CNTL::Register>),
        (0x64 => _reserved4),
        (0x68 => baudrate: ReadWrite<u32>),
        (0x6c => _reserved5),
        (0xd8 => @END),
    }
}

impl MiniUartInner {
    /**
     * # Safety
     *
     * - The user must ensure to provide a correct MMIO start address.
     */
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            registers: MMIODerefWrapper::new(mmio_start_addr),
        }
    }

    fn init(&self) {
        self.registers
            .enable
            .modify(AUX_ENB::MINI_UART_ENABLE::ENABLE);
        // disable transmitter and receiver during configuration
        self.registers.controll.modify(
            AUX_MU_CNTL::TRANSMITTER_ENABLE::DISABLE + AUX_MU_CNTL::RECEIVER_ENABLE::DISABLE,
        );
        // disable interrupt which is not needed currently
        self.registers.interrupt_enable.modify(
            AUX_MU_IER::ENABLE_TRANSMIT_INTERRUPT::DISABLE
                + AUX_MU_IER::ENABLE_RECEIVE_INTERRUPT::DISABLE,
        );
        // set the data size to 8 bit
        self.registers
            .line_controll
            .modify(AUX_MU_LCR::DATA_SIZE::EIGHT_BITS);
        // disable auto flow control
        self.registers.modem_controll.set(0);
        // set baud rate to 115200
        self.registers.baudrate.set(270);
        // disable FIFO
        self.registers.interrupt_identify.modify(
            AUX_MU_IIR::FIFO_CLEAR_BITS::CLEAR_TRANSMIT_FIFO
                + AUX_MU_IIR::FIFO_CLEAR_BITS::CLEAR_RECEIVE_FIFO,
        );
        // enable transmitter and receiver
        self.registers
            .controll
            .modify(AUX_MU_CNTL::TRANSMITTER_ENABLE::ENABLE + AUX_MU_CNTL::RECEIVER_ENABLE::ENABLE);
    }

    /**
     * Check if data is available to read
     */
    fn is_readable(&self) -> bool {
        self.registers.line_status.is_set(AUX_MU_LSR::DATA_READY)
    }

    /**
     * Check if data is a available to write
     */
    fn is_writable(&self) -> bool {
        self.registers
            .line_status
            .is_set(AUX_MU_LSR::TRANSMITTER_EMPTY)
    }

    pub fn read_byte(&self) -> u8 {
        while !self.is_readable() {}
        self.registers.data.get() as u8
    }

    pub fn write_byte(&self, value: u8) {
        while !self.is_writable() {}
        self.registers.data.set(value as u32);
    }
}

impl MiniUart {
    /**
     * # Safety
     *
     * - The user must ensure to provide a correct MMIO start address.
     */
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            inner: Mutex::new(MiniUartInner::new(mmio_start_addr)),
        }
    }
}

impl fmt::Write for MiniUartInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_byte(c as u8);
        }
        Ok(())
    }
}

impl console::Read for MiniUart {
    fn read_char(&self) -> char {
        let inner = self.inner.lock().unwrap();
        inner.read_byte() as char
    }
}

impl console::Write for MiniUart {
    fn write_char(&self, c: char) {
        let inner = self.inner.lock().unwrap();
        inner.write_byte(c as u8);
    }

    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result {
        let mut inner = self.inner.lock().unwrap();
        inner.write_fmt(args)
    }
}

impl console::ReadWrite for MiniUart {}

impl DeviceDriver for MiniUart {
    unsafe fn init(&self) -> Result<(), &'static str> {
        let inner = self.inner.lock().unwrap();
        inner.init();
        Ok(())
    }
}
