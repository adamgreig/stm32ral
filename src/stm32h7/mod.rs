//! Parent module for all STM32H7 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32h743", feature="doc"))]
pub mod stm32h743;

#[cfg(any(feature="stm32h743v", feature="doc"))]
pub mod stm32h743v;

#[cfg(any(feature="stm32h747cm4", feature="doc"))]
pub mod stm32h747cm4;

#[cfg(any(feature="stm32h747cm7", feature="doc"))]
pub mod stm32h747cm7;

#[cfg(any(feature="stm32h753", feature="doc"))]
pub mod stm32h753;

#[cfg(any(feature="stm32h753v", feature="doc"))]
pub mod stm32h753v;

#[cfg(any(feature="stm32h7b3", feature="doc"))]
pub mod stm32h7b3;

