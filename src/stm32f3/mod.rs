//! Parent module for all STM32F3 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32f301", feature="doc"))]
pub mod stm32f301;

#[cfg(any(feature="stm32f302", feature="doc"))]
pub mod stm32f302;

#[cfg(any(feature="stm32f303", feature="doc"))]
pub mod stm32f303;

#[cfg(any(feature="stm32f373", feature="doc"))]
pub mod stm32f373;

#[cfg(any(feature="stm32f3x4", feature="doc"))]
pub mod stm32f3x4;

#[cfg(any(feature="stm32f3x8", feature="doc"))]
pub mod stm32f3x8;

