use soc::{SocAddr, SocInfo};

#[cfg(feature = "unknown_soc")]
#[no_mangle]
pub static mut SOC_INFO: SocInfo = SocInfo {
    known: false,
    name: "Unknown - not set",
    addresses: [
        ("Peripheral Base", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
    ]
};