//! Parent module for all STM32WB devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32wb55", feature="doc"))]
pub mod stm32wb55;

