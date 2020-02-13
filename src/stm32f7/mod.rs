//! Parent module for all STM32F7 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32f730", feature="doc"))]
pub mod stm32f730;

#[cfg(any(feature="stm32f745", feature="doc"))]
pub mod stm32f745;

#[cfg(any(feature="stm32f765", feature="doc"))]
pub mod stm32f765;

#[cfg(any(feature="stm32f7x2", feature="doc"))]
pub mod stm32f7x2;

#[cfg(any(feature="stm32f7x3", feature="doc"))]
pub mod stm32f7x3;

#[cfg(any(feature="stm32f7x6", feature="doc"))]
pub mod stm32f7x6;

#[cfg(any(feature="stm32f7x7", feature="doc"))]
pub mod stm32f7x7;

#[cfg(any(feature="stm32f7x9", feature="doc"))]
pub mod stm32f7x9;

