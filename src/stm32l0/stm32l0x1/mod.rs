//! stm32ral module for stm32l0x1

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 3;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub mod aes;
pub use super::instances::dma1;
pub mod crc;
pub mod gpio;
pub mod lptim;
pub mod rtc;
pub use super::instances::iwdg;
pub use super::instances::usart;
pub use super::instances::wwdg;
pub mod fw;
pub mod rcc;
pub mod spi;
pub mod syscfg;
pub use super::instances::i2c;
pub mod adc;
pub mod dbg;
pub mod exti;
pub mod flash;
pub mod pwr;
pub mod tim2;
pub mod tim3;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub mod tim21;
pub mod tim22;
pub use super::instances::lpuart1;
pub use super::instances::nvic;
