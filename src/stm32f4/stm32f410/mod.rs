//! stm32ral module for stm32f410

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::adc1;
pub use super::instances::adc_common_f401_f410_f411_f412_f413 as adc_common;
pub use super::instances::crc;
pub use super::instances::dbgmcu_f401_f410_f411_f412_f413 as dbgmcu;
pub use super::instances::exti;
pub mod flash;
pub use super::instances::iwdg;
pub mod pwr;
pub mod rcc;
pub use super::instances::rtc_f401_f410_f411_f412_f413 as rtc;
pub mod syscfg;
pub use super::instances::dma;
pub use super::instances::tim1;
pub use super::instances::tim11_f401_f410_f411_f412_f413_f427_f429_f446_f469 as tim11;
pub use super::instances::tim5;
pub use super::instances::tim8;
pub use super::instances::tim9;
pub use super::instances::usart_f401_f410_f411 as usart;
pub use super::instances::wwdg;
pub mod gpio;
pub mod i2c;
pub mod spi;
pub use super::instances::nvic_f401_f405_f407_f410_f411_f412_f413 as nvic;
pub use super::instances::tim6;
pub mod rng;
pub use super::instances::dac;
pub mod fmpi2c4;
pub mod lptim1;
pub use super::instances::fpu;
pub use super::instances::fpu_cpacr;
pub use super::instances::mpu;
pub use super::instances::nvic_stir;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::stk;
