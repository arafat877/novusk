use core::fmt::{Arguments, Write};
pub use uefi_services::system_table;

extern "C" {
    fn x86_main() -> !;
}

pub(crate) unsafe fn start_novusk() -> ! {
    #[cfg(target_arch = "x86_64")]
    x86_main();
}

#[no_mangle]
pub unsafe extern "C" fn _efi_println(fmt: Arguments) {
    let stdout = system_table().as_ref().stdout();
    writeln!(stdout, "{}", fmt);
}
