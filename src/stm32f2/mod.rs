//! Parent module for all STM32F2 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32f215", feature="doc"))]
pub mod stm32f215;

#[cfg(any(feature="stm32f217", feature="doc"))]
pub mod stm32f217;

