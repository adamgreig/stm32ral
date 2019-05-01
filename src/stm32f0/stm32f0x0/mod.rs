//! stm32ral module for stm32f0x0

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 3;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::crc;
pub mod gpio;
pub mod pwr;
pub mod spi;
pub use super::instances::dma1;
pub use super::instances::exti;
pub use super::instances::i2c;
pub use super::instances::iwdg;
pub use super::instances::nvic;
pub use super::instances::tim1;
pub use super::instances::tim14;
pub use super::instances::tim3;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub use super::instances::wwdg;
pub mod adc;
pub mod rcc;
pub mod syscfg;
pub mod usart;
pub use super::instances::rtc;
pub use super::instances::tim15;
pub use super::instances::tim16;
pub use super::instances::tim17;
pub mod dbgmcu;
pub mod flash;
pub use super::instances::scb;
pub use super::instances::stk;
pub use super::instances::usb;
