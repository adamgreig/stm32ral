//! stm32ral module for stm32f301

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub mod gpio;
pub use super::instances::crc;
pub use super::instances::tsc;
pub mod flash;
pub mod rcc;
pub use super::instances::dma;
pub mod tim2;
pub use super::instances::tim15_f301_f373_f3x8 as tim15;
pub use super::instances::tim16_f301_f373_f3x8 as tim16;
pub use super::instances::tim17_f301_f373_f3x8 as tim17;
pub use super::instances::tim19;
pub use super::instances::tim3_f301_f373_f3x4_f3x8 as tim3;
pub use super::instances::tim4_f301_f373_f3x8 as tim4;
pub use super::instances::tim5;
pub use super::instances::usart_f301_f373_f3x4 as usart;
pub mod adc;
pub mod spi;
pub use super::instances::can;
pub use super::instances::cec;
pub use super::instances::dac1;
pub use super::instances::dac2;
pub use super::instances::dbgmcu;
pub use super::instances::exti_f301_f3x8 as exti;
pub use super::instances::i2c;
pub use super::instances::iwdg;
pub use super::instances::pwr_f301_f373 as pwr;
pub use super::instances::rtc_f301_f373_f3x8 as rtc;
pub use super::instances::sdadc;
pub use super::instances::tim12;
pub use super::instances::tim13;
pub use super::instances::tim14;
pub use super::instances::tim18;
pub use super::instances::tim6_f301_f373_f3x8 as tim6;
pub use super::instances::tim7_f301_f373_f3x8 as tim7;
pub use super::instances::usb;
pub use super::instances::wwdg;
pub mod syscfg;
pub use super::instances::fpu;
pub use super::instances::fpu_cpacr;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::nvic_stir;
pub use super::instances::opamp_f301_f3x4 as opamp;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::stk;
pub mod comp;
