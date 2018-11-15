//! Parent module for all STM32F0 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32f0x0", feature="doc"))]
pub mod stm32f0x0;

#[cfg(any(feature="stm32f0x1", feature="doc"))]
pub mod stm32f0x1;

#[cfg(any(feature="stm32f0x2", feature="doc"))]
pub mod stm32f0x2;

#[cfg(any(feature="stm32f0x8", feature="doc"))]
pub mod stm32f0x8;

