//! stm32ral module for armv6m

pub mod cpuid;
pub use super::instances::dcb;
pub use super::instances::dwt;
pub use super::instances::syst;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct CorePeripherals {
    pub CPUID: cpuid::Instance,
    pub DCB: dcb::Instance,
    pub DWT: dwt::Instance,
    pub SYST: syst::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct CorePeripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl CorePeripherals {
    pub unsafe fn steal() -> Self {
        CorePeripherals {
            CPUID: cpuid::CPUID::steal(),
            DCB: dcb::DCB::steal(),
            DWT: dwt::DWT::steal(),
            SYST: syst::SYST::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl CorePeripherals {
    pub fn steal() -> Self {
        CorePeripherals {}
    }
}
