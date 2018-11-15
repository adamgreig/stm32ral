//! Parent module for all STM32F1 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32f100", feature="doc"))]
pub mod stm32f100;

#[cfg(any(feature="stm32f101", feature="doc"))]
pub mod stm32f101;

#[cfg(any(feature="stm32f102", feature="doc"))]
pub mod stm32f102;

#[cfg(any(feature="stm32f103", feature="doc"))]
pub mod stm32f103;

#[cfg(any(feature="stm32f107", feature="doc"))]
pub mod stm32f107;

