use core::sync::atomic::{compiler_fence, Ordering};
use crate::board::RaspberryPi;
use crate::common::RpiMb;
use mailbox::MailBox;

pub mod gpio;
pub mod led;

#[no_mangle]
// #[export_name = "device_init"]
pub extern "C" fn device_init() -> (Result<(), &'static str>, &'static str) {
    let mut pi = Rpi3::new();
    pi.init();

    if pi.error.0 {
        return (Err(pi.error.1), "RPi 3");
    } else { return (Ok(()), "RPi 3"); }
}

pub struct Rpi3 {
    pub error: (bool, &'static str),
    pub gpio: gpio::Rpi3Gpio,
    pub led: led::Rpi3Led,
    pub mb: RpiMb,
}

impl Rpi3 {
    pub fn new() -> Self {
        return Rpi3 {
            error: (false, ""),
            gpio: gpio::Rpi3Gpio::new(),
            led: led::Rpi3Led::new(),
            mb: RpiMb::new(),
        };
    }

    pub fn init(&mut self) {
        self.gpio_init();
        self.mb.init();
    }
}

impl RaspberryPi for Rpi3 {
    fn gpio_init(&mut self) {
        use core::ops::Deref;

        let gpio_deref = self.gpio.deref();

        // Check GPIO values
        if gpio_deref.__GPFSEL0 != 0 || gpio_deref.__GPFSEL1 != 73728 || gpio_deref.__GPFSEL3 != 0 || gpio_deref.__GPFSEL4 != 0 || gpio_deref.__GPFSEL5 != 0 {
            self.error = (true, "A GPIO value it wrong");
        }

        // Initialize anything that uses gpio pins
        self.led.init();
    }

    fn led_on(&self) {
        self.led.led_on();
    }

    fn led_off(&self) {
        self.led.led_off();
    }

    fn mailbox_init(&mut self) {
        self.mb.init();
    }
}