//! stm32ral module for stm32l4x3

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::comp_l4x3_l4x5_l4x6 as comp;
pub use super::instances::crc;
pub use super::instances::dac1;
pub use super::instances::dma;
pub use super::instances::firewall;
pub use super::instances::flash;
pub use super::instances::i2c_l4x3_l4x5 as i2c;
pub use super::instances::iwdg;
pub use super::instances::lcd;
pub use super::instances::tsc;
pub use super::instances::wwdg;
pub mod rcc;
pub use super::instances::adc;
pub use super::instances::aes;
pub use super::instances::gpio;
pub use super::instances::lptim;
pub use super::instances::pwr;
pub use super::instances::rng;
pub use super::instances::sai1;
pub use super::instances::syscfg;
pub use super::instances::tim1;
pub use super::instances::tim15;
pub use super::instances::tim16;
pub use super::instances::tim2;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub mod usart;
pub use super::instances::can1_l4x3_l4x5 as can1;
pub use super::instances::crs;
pub use super::instances::dbgmcu_l4x1_l4x2_l4x3 as dbgmcu;
pub use super::instances::exti;
pub use super::instances::fpu;
pub use super::instances::fpu_cpacr;
pub use super::instances::lpuart1;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::nvic_stir;
pub use super::instances::opamp;
pub use super::instances::rtc;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::sdmmc;
pub use super::instances::spi_l4x3_l4x5_l4x6 as spi;
pub use super::instances::stk;
pub use super::instances::swpmi1;
pub use super::instances::usb_l4x3_l4x5 as usb;
pub use super::instances::vrefbuf;
