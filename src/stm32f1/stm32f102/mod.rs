//! stm32ral module for stm32f102

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::pwr;
pub mod rcc;
pub use super::instances::afio;
pub use super::instances::dma;
pub use super::instances::exti;
pub use super::instances::gpio;
pub use super::instances::i2c;
pub use super::instances::iwdg;
pub use super::instances::rtc;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::wwdg;
pub mod spi1;
pub use super::instances::usart;
pub mod adc;
pub use super::instances::crc;
pub use super::instances::flash;
pub mod dbgmcu;
pub use super::instances::bkp;
pub mod fsmc;
pub use super::instances::adc2;
pub use super::instances::can;
pub use super::instances::dac_f100_f102 as dac;
pub use super::instances::otg_fs_device;
pub use super::instances::otg_fs_global;
pub use super::instances::otg_fs_host;
pub use super::instances::otg_fs_pwrclk;
pub use super::instances::sdio;
pub use super::instances::tim10;
pub use super::instances::tim11;
pub use super::instances::tim12;
pub use super::instances::tim1_f100_f101_f102 as tim1;
pub use super::instances::tim8;
pub use super::instances::tim9;
pub use super::instances::usb;
pub mod spi;
pub mod tim4;
pub mod tim5;
pub mod uart;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::nvic_stir;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::stk;
pub use super::instances::tim13_f100_f102_f107 as tim13;
pub use super::instances::tim14_f100_f102_f107 as tim14;
pub use super::instances::tim6;
pub use super::instances::tim7;
