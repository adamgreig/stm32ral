//! stm32ral module for stm32f0x2

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 3;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::crc;
pub use super::instances::dac;
pub use super::instances::gpio;
pub use super::instances::i2c;
pub use super::instances::iwdg;
pub use super::instances::pwr;
pub use super::instances::spi;
pub use super::instances::tim1;
pub use super::instances::tim14;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub use super::instances::wwdg;
pub mod exti;
pub use super::instances::dma1;
pub use super::instances::nvic;
pub mod rcc;
pub use super::instances::adc;
pub use super::instances::syscfg_comp;
pub mod usart;
pub use super::instances::can;
pub use super::instances::cec;
pub use super::instances::crs;
pub use super::instances::dbgmcu;
pub use super::instances::flash;
pub use super::instances::rtc;
pub use super::instances::scb;
pub use super::instances::stk;
pub use super::instances::tim15;
pub use super::instances::tim16;
pub use super::instances::tim17;
pub use super::instances::tsc;
pub use super::instances::usb;
