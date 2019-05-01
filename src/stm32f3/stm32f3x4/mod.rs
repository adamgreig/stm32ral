//! stm32ral module for stm32f3x4

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub mod gpio;
pub use super::instances::crc;
pub use super::instances::flash_f302_f303_f3x4 as flash;
pub use super::instances::tsc;
pub mod dma1;
pub mod rcc;
pub use super::instances::tim15_f302_f303_f3x4 as tim15;
pub use super::instances::tim16_f302_f303_f3x4 as tim16;
pub use super::instances::tim17_f302_f303_f3x4 as tim17;
pub use super::instances::tim2_f302_f303_f3x4 as tim2;
pub use super::instances::usart_f301_f373_f3x4 as usart;
pub mod spi;
pub use super::instances::exti_f302_f303_f3x4 as exti;
pub mod pwr;
pub use super::instances::i2c;
pub use super::instances::iwdg;
pub use super::instances::rtc_f302_f303_f3x4 as rtc;
pub use super::instances::tim6_f302_f303_f3x4 as tim6;
pub use super::instances::tim7_f302_f303_f3x4 as tim7;
pub use super::instances::wwdg;
pub mod dac;
pub use super::instances::dbgmcu;
pub use super::instances::tim1;
pub mod adc;
pub mod syscfg;
pub use super::instances::can;
pub use super::instances::tim3_f301_f373_f3x4_f3x8 as tim3;
pub mod adc_common;
pub mod hrtim_common;
pub mod hrtim_master;
pub mod hrtim_tima;
pub mod hrtim_timb;
pub mod hrtim_timc;
pub mod hrtim_timd;
pub mod hrtim_time;
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
