//! Parent module for all STM32L0 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32l0x1", feature="doc"))]
pub mod stm32l0x1;

#[cfg(any(feature="stm32l0x2", feature="doc"))]
pub mod stm32l0x2;

#[cfg(any(feature="stm32l0x3", feature="doc"))]
pub mod stm32l0x3;

