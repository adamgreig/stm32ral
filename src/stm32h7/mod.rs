//! Parent module for all STM32H7 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32h7x3", feature="doc"))]
pub mod stm32h7x3;

