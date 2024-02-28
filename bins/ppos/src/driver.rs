use cpu::mmio::{AUX_MMIO_BASE, GPIO_MMIO_BASE};
use device::device_driver::DeviceDriver;
use device::gpio::GPIO;
use device::mini_uart::MiniUart;
use library::console;

static MINI_UART: MiniUart = unsafe { MiniUart::new(AUX_MMIO_BASE) };
static GPIO: GPIO = unsafe { GPIO::new(GPIO_MMIO_BASE) };

pub unsafe fn init() -> Result<(), &'static str> {
    GPIO.init()?;
    GPIO.setup_for_mini_uart();
    MINI_UART.init()?;
    console::register_console(&MINI_UART);
    Ok(())
}
