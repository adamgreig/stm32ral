//! Parent module for all STM32L4 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32l4x1", feature="doc"))]
pub mod stm32l4x1;

#[cfg(any(feature="stm32l4x2", feature="doc"))]
pub mod stm32l4x2;

#[cfg(any(feature="stm32l4x3", feature="doc"))]
pub mod stm32l4x3;

#[cfg(any(feature="stm32l4x5", feature="doc"))]
pub mod stm32l4x5;

#[cfg(any(feature="stm32l4x6", feature="doc"))]
pub mod stm32l4x6;

