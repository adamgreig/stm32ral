//! stm32ral module for stm32f101

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::fsmc_f101_f103_f107 as fsmc;
pub use super::instances::pwr;
pub mod rcc;
pub use super::instances::afio;
pub use super::instances::bkp;
pub use super::instances::crc;
pub use super::instances::dac_f101_f103_f107 as dac;
pub use super::instances::dbg;
pub use super::instances::dma;
pub use super::instances::exti;
pub use super::instances::flash;
pub use super::instances::gpio;
pub use super::instances::i2c;
pub use super::instances::iwdg;
pub use super::instances::rtc;
pub use super::instances::spi;
pub use super::instances::tim10;
pub use super::instances::tim11;
pub use super::instances::tim12;
pub use super::instances::tim13_f101_f103 as tim13;
pub use super::instances::tim14_f101_f103 as tim14;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::tim4;
pub use super::instances::tim5;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub use super::instances::tim9;
pub use super::instances::uart4;
pub use super::instances::uart5;
pub use super::instances::usart;
pub use super::instances::wwdg;
pub mod adc1;
pub use super::instances::otg_fs_device;
pub use super::instances::otg_fs_global;
pub use super::instances::otg_fs_host;
pub use super::instances::otg_fs_pwrclk;
pub mod can;
pub use super::instances::ethernet_dma;
pub use super::instances::ethernet_mac;
pub use super::instances::ethernet_mmc;
pub use super::instances::ethernet_ptp;
pub use super::instances::usb;
pub mod adc;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::nvic_stir;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::sdio_f101_f103 as sdio;
pub use super::instances::stk;
pub use super::instances::tim1_f100_f101_f102 as tim1;
pub use super::instances::tim8;
