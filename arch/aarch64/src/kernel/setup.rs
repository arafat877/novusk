use core::ptr::write_volatile;
use novuskinc::platform::device_init;
use printk::printk_init;
use crate::early_printk;
use super::cpu::irq::aarch64_irq_setup;
use setup::{ArchKernelSetup, SetupReturn};
use crate::include::dif::DIF;

static mut AARCH64_KERNEL: Aarch64Kernel = Aarch64Kernel::new();

pub struct Aarch64Kernel {
    pub early_kernel: bool,
}

impl Aarch64Kernel {
    pub const fn new() -> Self {
        return Aarch64Kernel {
            early_kernel: true,
        }
    }
    
    pub fn setup(&self) {
        self.test_memory();

        let irq = self.irq_setup();
        let dev = unsafe { self.device_init() };

        if irq.0.is_err() {
            panic!("{}", irq.1);
        } else if dev.0.is_err() {
            panic!("{}", dev.1);
        }

        early_printk!("{}\n", irq.1);
        early_printk!("{}\n", dev.1);
    }

    fn test_memory(&self) {
        let mut test_vec = vec![0];
        test_vec.push(1);

        for i in 0..1024 {
            test_vec.push(i);
        }
    }
}

impl ArchKernelSetup for Aarch64Kernel {
    fn irq_setup(&self) -> SetupReturn {
        unsafe { aarch64_irq_setup(); }
        return (Ok(()), "IRQ successfully setup");
    }

    fn device_init(&self) -> SetupReturn {
        unsafe { device_init(); }

        (Ok(()), "Device initialized")
    }
}

#[no_mangle]
pub unsafe extern "C" fn setup_arch() {
    early_printk!("\nStarting Aarch64 kernel...\n");
    early_printk!("Early and main kernel printing for Aarch64 initialized\n");
    early_printk!("\nSetting up kernel...\n");

    AARCH64_KERNEL.setup();
}
