//! Parent module for all STM32G0 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32g0x0", feature="doc"))]
pub mod stm32g0x0;

#[cfg(any(feature="stm32g0x1", feature="doc"))]
pub mod stm32g0x1;

