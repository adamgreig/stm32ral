//! Parent module for all STM32F4 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32f401", feature="doc"))]
pub mod stm32f401;

#[cfg(any(feature="stm32f405", feature="doc"))]
pub mod stm32f405;

#[cfg(any(feature="stm32f407", feature="doc"))]
pub mod stm32f407;

#[cfg(any(feature="stm32f410", feature="doc"))]
pub mod stm32f410;

#[cfg(any(feature="stm32f411", feature="doc"))]
pub mod stm32f411;

#[cfg(any(feature="stm32f412", feature="doc"))]
pub mod stm32f412;

#[cfg(any(feature="stm32f413", feature="doc"))]
pub mod stm32f413;

#[cfg(any(feature="stm32f427", feature="doc"))]
pub mod stm32f427;

#[cfg(any(feature="stm32f429", feature="doc"))]
pub mod stm32f429;

#[cfg(any(feature="stm32f446", feature="doc"))]
pub mod stm32f446;

#[cfg(any(feature="stm32f469", feature="doc"))]
pub mod stm32f469;

