#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate novuskinc;

use novuskinc::module::*;

pub mod config;

use kinfo::info::FS;

pub fn _init_fscheck(km: &mut KernelModule) {
    let mut fs = config::get_fs();

    if fs == "None" || fs == "" {
        return;
    }
}

module_init!(ModuleType::InKernel, fscheck);

pub fn _end_fscheck(km: &mut KernelModule) {

}

module_end!(ModuleType::InKernel, fscheck);
