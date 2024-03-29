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

#[cfg(any(feature="stm32g050", feature="doc"))]
pub mod stm32g050;

#[cfg(any(feature="stm32g051", feature="doc"))]
pub mod stm32g051;

#[cfg(any(feature="stm32g061", feature="doc"))]
pub mod stm32g061;

#[cfg(any(feature="stm32g070", feature="doc"))]
pub mod stm32g070;

#[cfg(any(feature="stm32g071", feature="doc"))]
pub mod stm32g071;

#[cfg(any(feature="stm32g07x", feature="doc"))]
pub mod stm32g07x;

#[cfg(any(feature="stm32g081", feature="doc"))]
pub mod stm32g081;

#[cfg(any(feature="stm32g0b0", feature="doc"))]
pub mod stm32g0b0;

#[cfg(any(feature="stm32g0b1", feature="doc"))]
pub mod stm32g0b1;

#[cfg(any(feature="stm32g0c1", feature="doc"))]
pub mod stm32g0c1;

