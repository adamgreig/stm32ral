//! Parent module for all STM32L1 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32l100", feature="doc"))]
pub mod stm32l100;

#[cfg(any(feature="stm32l151", feature="doc"))]
pub mod stm32l151;

#[cfg(any(feature="stm32l162", feature="doc"))]
pub mod stm32l162;

