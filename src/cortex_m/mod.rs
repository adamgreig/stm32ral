//! Parent module for all CORTEX_M devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="armv6_m", feature="doc"))]
pub mod armv6_m;

#[cfg(any(feature="armv7e_m", feature="doc"))]
pub mod armv7e_m;

#[cfg(any(feature="armv7_m", feature="doc"))]
pub mod armv7_m;

