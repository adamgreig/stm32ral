//! stm32ral module for stm32f100

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub mod fsmc;
pub use super::instances::pwr;
pub mod rcc;
pub use super::instances::gpio;
pub mod afio;
pub mod exti;
pub use super::instances::dma;
pub use super::instances::rtc;
pub mod bkp;
pub use super::instances::iwdg;
pub use super::instances::tim1_f100_f101_f102 as tim1;
pub use super::instances::wwdg;
pub mod tim12;
pub mod tim2;
pub mod tim3;
pub mod tim4;
pub mod tim5;
pub use super::instances::i2c;
pub use super::instances::tim13_f100_f102_f107 as tim13;
pub use super::instances::tim14_f100_f102_f107 as tim14;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub mod spi;
pub use super::instances::usart;
pub mod adc1;
pub use super::instances::crc;
pub use super::instances::dac_f100_f102 as dac;
pub use super::instances::dbgmcu;
pub use super::instances::uart;
pub mod cec;
pub mod flash;
pub mod tim15;
pub mod tim16;
pub mod tim17;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::nvic_stir;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::stk;
