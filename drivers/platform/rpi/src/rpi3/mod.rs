use novuskinc::kernel::types::KernelFunctionName;
use crate::DEVICE_DRIVERS;

pub mod console;
pub(self) mod drivers;
pub mod fb;
pub mod gpio;

use drivers::set_rpi3_drivers;
use fb::get_rpi3_fb;

unsafe fn rpi3_init() -> u8 {
    let mut ret = 0;

    if get_rpi3_fb().is_none() {
        ret += 1;
    }

    set_rpi3_drivers();

    0
}

define_kernel_function!(KernelFunctionName::device_init, -> u8, rpi3_init);

#[no_mangle]
pub unsafe extern "C" fn device_indicate_panic() {

}
