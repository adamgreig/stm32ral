//! Parent module for all STM32G0 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32g030", feature="doc"))]
pub mod stm32g030;

#[cfg(any(feature="stm32g031", feature="doc"))]
pub mod stm32g031;

#[cfg(any(feature="stm32g041", feature="doc"))]
pub mod stm32g041;

#[cfg(any(feature="stm32g07x", feature="doc"))]
pub mod stm32g07x;

#[cfg(any(feature="stm32g081", feature="doc"))]
pub mod stm32g081;

