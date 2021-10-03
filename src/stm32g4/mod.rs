//! Parent module for all STM32G4 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32g431", feature="doc"))]
pub mod stm32g431;

#[cfg(any(feature="stm32g441", feature="doc"))]
pub mod stm32g441;

#[cfg(any(feature="stm32g471", feature="doc"))]
pub mod stm32g471;

#[cfg(any(feature="stm32g473", feature="doc"))]
pub mod stm32g473;

#[cfg(any(feature="stm32g474", feature="doc"))]
pub mod stm32g474;

#[cfg(any(feature="stm32g483", feature="doc"))]
pub mod stm32g483;

#[cfg(any(feature="stm32g484", feature="doc"))]
pub mod stm32g484;

#[cfg(any(feature="stm32g491", feature="doc"))]
pub mod stm32g491;

#[cfg(any(feature="stm32g4a1", feature="doc"))]
pub mod stm32g4a1;

