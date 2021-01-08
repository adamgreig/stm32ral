#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GTZC_TZIC
//!
//! Used by: stm32l552, stm32l562

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l5::peripherals::gtzc_tzic::Instance;
pub use crate::stm32l5::peripherals::gtzc_tzic::{RegisterBlock, ResetValues};
pub use crate::stm32l5::peripherals::gtzc_tzic::{
    FCR1, FCR2, FCR3, IER1, IER2, IER3, SR1, SR2, SR3,
};

/// Access functions for the GTZC_TZIC peripheral instance
pub mod GTZC_TZIC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40032800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GTZC_TZIC
    pub const reset: ResetValues = ResetValues {
        IER1: 0x00000000,
        IER2: 0x00000000,
        IER3: 0x00000000,
        SR1: 0x00000000,
        SR2: 0x00000000,
        SR3: 0x00000000,
        FCR1: 0x00000000,
        FCR2: 0x00000000,
        FCR3: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GTZC_TZIC_TAKEN: bool = false;

    /// Safe access to GTZC_TZIC
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
            if GTZC_TZIC_TAKEN {
                None
            } else {
                GTZC_TZIC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GTZC_TZIC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GTZC_TZIC_TAKEN && inst.addr == INSTANCE.addr {
                GTZC_TZIC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GTZC_TZIC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GTZC_TZIC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GTZC_TZIC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GTZC_TZIC: *const RegisterBlock = 0x40032800 as *const _;

/// Access functions for the SEC_GTZC_TZIC peripheral instance
pub mod SEC_GTZC_TZIC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50032800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_GTZC_TZIC
    pub const reset: ResetValues = ResetValues {
        IER1: 0x00000000,
        IER2: 0x00000000,
        IER3: 0x00000000,
        SR1: 0x00000000,
        SR2: 0x00000000,
        SR3: 0x00000000,
        FCR1: 0x00000000,
        FCR2: 0x00000000,
        FCR3: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_GTZC_TZIC_TAKEN: bool = false;

    /// Safe access to SEC_GTZC_TZIC
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
            if SEC_GTZC_TZIC_TAKEN {
                None
            } else {
                SEC_GTZC_TZIC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_GTZC_TZIC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_GTZC_TZIC_TAKEN && inst.addr == INSTANCE.addr {
                SEC_GTZC_TZIC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_GTZC_TZIC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_GTZC_TZIC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_GTZC_TZIC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_GTZC_TZIC: *const RegisterBlock = 0x50032800 as *const _;
