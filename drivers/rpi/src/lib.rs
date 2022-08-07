#![no_std]

extern crate nmallocator;
#[macro_use] extern crate tock_registers;

pub mod board;
pub use board::RaspberryPi;
pub mod common;
pub use common::*;

#[macro_use]
#[path = "../../../kernel/irq.rs"]
pub mod irq;

#[cfg(feature = "rpi2")]
pub mod rpi2;

#[cfg(feature = "rpi3")]
pub mod rpi3;

#[cfg(feature = "rpi2")]
pub use rpi2::Rpi2;

#[cfg(feature = "rpi3")]
pub use rpi3::Rpi3;

#[cfg(feature = "rpi2")]
pub use rpi2::registers::*;

#[cfg(feature = "rpi3")]
pub use rpi3::*;

fn irq_init() {

}

define_dev_irq_init!(irq_init);
