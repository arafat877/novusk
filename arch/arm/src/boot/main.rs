use crate::kernel::cpu::CPUINFO;

#[cfg(target_arch = "arm")]
fn boot_setup() {
    use super::setup::Arm32Boot;

    let arm_boot = Arm32Boot::new();
}

#[no_mangle]
pub unsafe extern "C" fn arm_boot_main() {
    #[cfg(target_arch = "arm")]
    boot_setup();

    #[cfg(target_arch = "aarch64")]
    crate::bits64::arm64_boot_setup();

    panic!("{}: {} bit kernel ended", CPUINFO.architecture, CPUINFO.bits);
}