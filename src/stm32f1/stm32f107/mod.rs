//! stm32ral module for stm32f107

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::pwr;
pub mod rcc;
pub use super::instances::gpio;
pub mod afio;
pub mod exti;
pub use super::instances::bkp;
pub use super::instances::dma;
pub use super::instances::iwdg;
pub use super::instances::rtc;
pub use super::instances::tim1_f103_f107 as tim1;
pub use super::instances::wwdg;
pub mod tim2;
pub mod tim3;
pub mod tim4;
pub mod tim5;
pub use super::instances::adc1;
pub use super::instances::adc2;
pub use super::instances::i2c;
pub use super::instances::spi;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub use super::instances::usart;
pub mod can1;
pub mod can2;
pub use super::instances::ethernet_dma;
pub use super::instances::ethernet_mac;
pub use super::instances::ethernet_mmc;
pub use super::instances::ethernet_ptp;
pub mod usb_otg_device;
pub mod usb_otg_global;
pub mod usb_otg_host;
pub mod usb_otg_pwrclk;
pub use super::instances::dac_f101_f103_f107 as dac;
pub mod dbg;
pub use super::instances::crc;
pub use super::instances::flash;
pub use super::instances::fsmc_f101_f103_f107 as fsmc;
pub use super::instances::sdio_f102_f107 as sdio;
pub use super::instances::uart4;
pub use super::instances::uart5;
pub mod tim10;
pub mod tim11;
pub use super::instances::tim12;
pub use super::instances::tim9;
pub mod adc3;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::nvic_stir;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::stk;
pub use super::instances::tim13_f100_f102_f107 as tim13;
pub use super::instances::tim14_f100_f102_f107 as tim14;
pub use super::instances::tim8;
pub use super::instances::usb;
