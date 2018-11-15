//! stm32ral module for stm32l151

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::aes;
pub use super::instances::comp;
pub use super::instances::crc;
pub use super::instances::dac;
pub use super::instances::dma;
pub use super::instances::exti;
pub use super::instances::flash;
pub use super::instances::fsmc;
pub use super::instances::gpio;
pub use super::instances::i2c;
pub use super::instances::iwdg;
pub use super::instances::lcd;
pub use super::instances::opamp;
pub use super::instances::pwr;
pub use super::instances::rcc;
pub use super::instances::ri;
pub use super::instances::rtc;
pub use super::instances::sdio;
pub use super::instances::spi;
pub use super::instances::syscfg;
pub mod tim10;
pub mod tim11;
pub use super::instances::adc;
pub use super::instances::dbgmcu;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::nvic_stir;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::stk;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::tim4;
pub use super::instances::tim5;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub use super::instances::tim9;
pub use super::instances::usart;
pub use super::instances::usb;
pub use super::instances::wwdg;
