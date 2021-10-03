#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CRC1
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::crc::Instance;
pub use crate::stm32mp::peripherals::crc::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::crc::{CRC_CR, CRC_DR, CRC_IDR, CRC_INIT, CRC_POL};

/// Access functions for the CRC1 peripheral instance
pub mod CRC1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58009000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CRC1
    pub const reset: ResetValues = ResetValues {
        CRC_DR: 0xFFFFFFFF,
        CRC_IDR: 0x00000000,
        CRC_CR: 0x00000000,
        CRC_INIT: 0xFFFFFFFF,
        CRC_POL: 0x04C11DB7,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CRC1_TAKEN: bool = false;

    /// Safe access to CRC1
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CRC1_TAKEN {
                None
            } else {
                CRC1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CRC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CRC1_TAKEN && inst.addr == INSTANCE.addr {
                CRC1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CRC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CRC1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CRC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CRC1: *const RegisterBlock = 0x58009000 as *const _;

/// Access functions for the CRC2 peripheral instance
pub mod CRC2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4c004000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CRC2
    pub const reset: ResetValues = ResetValues {
        CRC_DR: 0xFFFFFFFF,
        CRC_IDR: 0x00000000,
        CRC_CR: 0x00000000,
        CRC_INIT: 0xFFFFFFFF,
        CRC_POL: 0x04C11DB7,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CRC2_TAKEN: bool = false;

    /// Safe access to CRC2
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CRC2_TAKEN {
                None
            } else {
                CRC2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CRC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CRC2_TAKEN && inst.addr == INSTANCE.addr {
                CRC2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CRC2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CRC2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CRC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CRC2: *const RegisterBlock = 0x4c004000 as *const _;
