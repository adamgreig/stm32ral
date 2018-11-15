//! stm32ral module for stm32f412

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::adc1;
pub use super::instances::adc_common_f401_f410_f411_f412_f413 as adc_common;
pub use super::instances::crc;
pub use super::instances::dbg_f401_f410_f411_f412_f413 as dbg;
pub use super::instances::exti;
pub use super::instances::flash_f401_f405_f411_f412_f413 as flash;
pub use super::instances::iwdg;
pub use super::instances::pwr_f401_f411_f412_f413 as pwr;
pub mod rcc;
pub use super::instances::rtc_f401_f410_f411_f412_f413 as rtc;
pub use super::instances::sdio;
pub use super::instances::syscfg_f412_f413 as syscfg;
pub use super::instances::tim1;
pub use super::instances::tim10_f401_f411_f412_f413_f427_f429_f446_f469 as tim10;
pub use super::instances::tim11_f401_f410_f411_f412_f413_f427_f429_f446_f469 as tim11;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::tim4;
pub use super::instances::tim5;
pub use super::instances::tim8;
pub use super::instances::tim9;
pub mod usart;
pub use super::instances::dma;
pub use super::instances::wwdg;
pub mod gpio;
pub use super::instances::i2c_f401_f405_f407_f411_f412 as i2c;
pub use super::instances::nvic_f401_f405_f407_f410_f411_f412_f413 as nvic;
pub use super::instances::spi_f411_f412 as spi;
pub mod dfsdm;
pub mod tim6;
pub mod tim7;
pub use super::instances::can;
pub use super::instances::rng;
pub use super::instances::tim12_f412_f413 as tim12;
pub use super::instances::tim13_f412_f413 as tim13;
pub use super::instances::tim14_f412_f413 as tim14;
pub mod fmpi2c4;
pub mod fsmc;
pub use super::instances::fpu;
pub use super::instances::fpu_cpacr;
pub use super::instances::mpu;
pub use super::instances::nvic_stir;
pub use super::instances::otg_fs_device;
pub use super::instances::otg_fs_global_f412_f413 as otg_fs_global;
pub use super::instances::otg_fs_host;
pub use super::instances::otg_fs_pwrclk;
pub use super::instances::quadspi;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::stk;
