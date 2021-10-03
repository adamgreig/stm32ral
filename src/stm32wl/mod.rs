//! Parent module for all STM32WL devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32wl5x_cm0p", feature="doc"))]
pub mod stm32wl5x_cm0p;

#[cfg(any(feature="stm32wl5x_cm4", feature="doc"))]
pub mod stm32wl5x_cm4;

#[cfg(any(feature="stm32wle5", feature="doc"))]
pub mod stm32wle5;

