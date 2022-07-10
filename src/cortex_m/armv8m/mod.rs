//! stm32ral module for armv8m

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct CorePeripherals {}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct CorePeripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl CorePeripherals {
    pub unsafe fn steal() -> Self {
        CorePeripherals {}
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl CorePeripherals {
    pub fn steal() -> Self {
        CorePeripherals {}
    }
}
