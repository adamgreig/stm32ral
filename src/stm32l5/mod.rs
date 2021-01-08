//! Parent module for all STM32L5 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32l552", feature="doc"))]
pub mod stm32l552;

#[cfg(any(feature="stm32l562", feature="doc"))]
pub mod stm32l562;

