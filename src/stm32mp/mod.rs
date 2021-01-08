//! Parent module for all STM32MP devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32mp157", feature="doc"))]
pub mod stm32mp157;

