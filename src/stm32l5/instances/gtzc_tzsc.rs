#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GTZC_TZSC
//!
//! Used by: stm32l552, stm32l562

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l5::peripherals::gtzc_tzsc::Instance;
pub use crate::stm32l5::peripherals::gtzc_tzsc::{RegisterBlock, ResetValues};
pub use crate::stm32l5::peripherals::gtzc_tzsc::{
    TZSC_CR, TZSC_MPCWM1_NSWMR1, TZSC_MPCWM1_NSWMR2, TZSC_MPCWM2_NSWMR1, TZSC_MPCWM2_NSWMR2,
    TZSC_MPCWM3_NSWMR1, TZSC_PRIVCFGR1, TZSC_PRIVCFGR2, TZSC_SECCFGR1, TZSC_SECCFGR2,
};

/// Access functions for the GTZC_TZSC peripheral instance
pub mod GTZC_TZSC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40032400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GTZC_TZSC
    pub const reset: ResetValues = ResetValues {
        TZSC_CR: 0x00000000,
        TZSC_SECCFGR1: 0x00000000,
        TZSC_SECCFGR2: 0x00000000,
        TZSC_PRIVCFGR1: 0x00000000,
        TZSC_PRIVCFGR2: 0x00000000,
        TZSC_MPCWM1_NSWMR1: 0x00000000,
        TZSC_MPCWM1_NSWMR2: 0x00000000,
        TZSC_MPCWM2_NSWMR1: 0x00000000,
        TZSC_MPCWM3_NSWMR1: 0x00000000,
        TZSC_MPCWM2_NSWMR2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GTZC_TZSC_TAKEN: bool = false;

    /// Safe access to GTZC_TZSC
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
            if GTZC_TZSC_TAKEN {
                None
            } else {
                GTZC_TZSC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GTZC_TZSC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GTZC_TZSC_TAKEN && inst.addr == INSTANCE.addr {
                GTZC_TZSC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GTZC_TZSC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GTZC_TZSC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GTZC_TZSC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GTZC_TZSC: *const RegisterBlock = 0x40032400 as *const _;

/// Access functions for the SEC_GTZC_TZSC peripheral instance
pub mod SEC_GTZC_TZSC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50032400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_GTZC_TZSC
    pub const reset: ResetValues = ResetValues {
        TZSC_CR: 0x00000000,
        TZSC_SECCFGR1: 0x00000000,
        TZSC_SECCFGR2: 0x00000000,
        TZSC_PRIVCFGR1: 0x00000000,
        TZSC_PRIVCFGR2: 0x00000000,
        TZSC_MPCWM1_NSWMR1: 0x00000000,
        TZSC_MPCWM1_NSWMR2: 0x00000000,
        TZSC_MPCWM2_NSWMR1: 0x00000000,
        TZSC_MPCWM3_NSWMR1: 0x00000000,
        TZSC_MPCWM2_NSWMR2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_GTZC_TZSC_TAKEN: bool = false;

    /// Safe access to SEC_GTZC_TZSC
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
            if SEC_GTZC_TZSC_TAKEN {
                None
            } else {
                SEC_GTZC_TZSC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_GTZC_TZSC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_GTZC_TZSC_TAKEN && inst.addr == INSTANCE.addr {
                SEC_GTZC_TZSC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_GTZC_TZSC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_GTZC_TZSC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_GTZC_TZSC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_GTZC_TZSC: *const RegisterBlock = 0x50032400 as *const _;
