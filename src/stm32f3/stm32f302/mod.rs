//! stm32ral module for stm32f302

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::crc;
pub use super::instances::flash_f302_f303_f3x4 as flash;
pub use super::instances::gpio;
pub use super::instances::tsc;
pub mod rcc;
pub use super::instances::adc;
pub use super::instances::adc_common;
pub use super::instances::can;
pub use super::instances::dac;
pub use super::instances::dbgmcu;
pub use super::instances::dma;
pub use super::instances::exti_f302_f303_f3x4 as exti;
pub use super::instances::i2c_f302_f303_f3x4_f3x8 as i2c;
pub use super::instances::iwdg;
pub use super::instances::pwr_f302_f303 as pwr;
pub use super::instances::rtc_f302_f303_f3x4 as rtc;
pub use super::instances::spi;
pub use super::instances::tim1;
pub use super::instances::tim15_f302_f303_f3x4 as tim15;
pub use super::instances::tim16_f302_f303_f3x4 as tim16;
pub use super::instances::tim17_f302_f303_f3x4 as tim17;
pub use super::instances::tim20;
pub use super::instances::tim2_f302_f303_f3x4 as tim2;
pub use super::instances::tim3_f302_f303 as tim3;
pub use super::instances::tim4_f302_f303 as tim4;
pub use super::instances::tim6_f302_f303_f3x4 as tim6;
pub use super::instances::tim7_f302_f303_f3x4 as tim7;
pub use super::instances::tim8;
pub use super::instances::usart_f302_f303_f3x8 as usart;
pub use super::instances::usb_fs;
pub use super::instances::wwdg;
pub mod fmc;
pub use super::instances::fpu;
pub use super::instances::fpu_cpacr;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::nvic_stir;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::stk;
pub use super::instances::syscfg_comp_opamp;
