#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Tamper and backup registers
//!
//! Used by: stm32l552, stm32l562

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l5::peripherals::tamp::Instance;
pub use crate::stm32l5::peripherals::tamp::{RegisterBlock, ResetValues};
pub use crate::stm32l5::peripherals::tamp::{
    ATCR1, ATCR2, ATOR, ATSEEDR, BKP0R, BKP10R, BKP11R, BKP12R, BKP13R, BKP14R, BKP15R, BKP16R,
    BKP17R, BKP18R, BKP19R, BKP1R, BKP20R, BKP21R, BKP22R, BKP23R, BKP24R, BKP25R, BKP26R, BKP27R,
    BKP28R, BKP29R, BKP2R, BKP30R, BKP31R, BKP3R, BKP4R, BKP5R, BKP6R, BKP7R, BKP8R, BKP9R, CFGR,
    COUNTR, CR1, CR2, CR3, FLTCR, IER, MISR, PRIVCR, SCR, SMCR, SMISR, SR,
};

/// Access functions for the SEC_TAMP peripheral instance
pub mod SEC_TAMP {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50003400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_TAMP
    pub const reset: ResetValues = ResetValues {
        CR1: 0xFFFF0000,
        CR2: 0x00000000,
        CR3: 0x00000000,
        FLTCR: 0x00000000,
        ATCR1: 0x00070000,
        ATSEEDR: 0x00000000,
        ATOR: 0x00000000,
        ATCR2: 0x00000000,
        SMCR: 0x00000000,
        PRIVCR: 0x00000000,
        IER: 0x00000000,
        SR: 0x00000000,
        MISR: 0x00000000,
        SMISR: 0x00000000,
        SCR: 0x00000000,
        COUNTR: 0x00000000,
        CFGR: 0x00000000,
        BKP0R: 0x00000000,
        BKP1R: 0x00000000,
        BKP2R: 0x00000000,
        BKP3R: 0x00000000,
        BKP4R: 0x00000000,
        BKP5R: 0x00000000,
        BKP6R: 0x00000000,
        BKP7R: 0x00000000,
        BKP8R: 0x00000000,
        BKP9R: 0x00000000,
        BKP10R: 0x00000000,
        BKP11R: 0x00000000,
        BKP12R: 0x00000000,
        BKP13R: 0x00000000,
        BKP14R: 0x00000000,
        BKP15R: 0x00000000,
        BKP16R: 0x00000000,
        BKP17R: 0x00000000,
        BKP18R: 0x00000000,
        BKP19R: 0x00000000,
        BKP20R: 0x00000000,
        BKP21R: 0x00000000,
        BKP22R: 0x00000000,
        BKP23R: 0x00000000,
        BKP24R: 0x00000000,
        BKP25R: 0x00000000,
        BKP26R: 0x00000000,
        BKP27R: 0x00000000,
        BKP28R: 0x00000000,
        BKP29R: 0x00000000,
        BKP30R: 0x00000000,
        BKP31R: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_TAMP_TAKEN: bool = false;

    /// Safe access to SEC_TAMP
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
            if SEC_TAMP_TAKEN {
                None
            } else {
                SEC_TAMP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_TAMP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_TAMP_TAKEN && inst.addr == INSTANCE.addr {
                SEC_TAMP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_TAMP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_TAMP_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_TAMP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_TAMP: *const RegisterBlock = 0x50003400 as *const _;

/// Access functions for the TAMP peripheral instance
pub mod TAMP {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40003400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TAMP
    pub const reset: ResetValues = ResetValues {
        CR1: 0xFFFF0000,
        CR2: 0x00000000,
        CR3: 0x00000000,
        FLTCR: 0x00000000,
        ATCR1: 0x00070000,
        ATSEEDR: 0x00000000,
        ATOR: 0x00000000,
        ATCR2: 0x00000000,
        SMCR: 0x00000000,
        PRIVCR: 0x00000000,
        IER: 0x00000000,
        SR: 0x00000000,
        MISR: 0x00000000,
        SMISR: 0x00000000,
        SCR: 0x00000000,
        COUNTR: 0x00000000,
        CFGR: 0x00000000,
        BKP0R: 0x00000000,
        BKP1R: 0x00000000,
        BKP2R: 0x00000000,
        BKP3R: 0x00000000,
        BKP4R: 0x00000000,
        BKP5R: 0x00000000,
        BKP6R: 0x00000000,
        BKP7R: 0x00000000,
        BKP8R: 0x00000000,
        BKP9R: 0x00000000,
        BKP10R: 0x00000000,
        BKP11R: 0x00000000,
        BKP12R: 0x00000000,
        BKP13R: 0x00000000,
        BKP14R: 0x00000000,
        BKP15R: 0x00000000,
        BKP16R: 0x00000000,
        BKP17R: 0x00000000,
        BKP18R: 0x00000000,
        BKP19R: 0x00000000,
        BKP20R: 0x00000000,
        BKP21R: 0x00000000,
        BKP22R: 0x00000000,
        BKP23R: 0x00000000,
        BKP24R: 0x00000000,
        BKP25R: 0x00000000,
        BKP26R: 0x00000000,
        BKP27R: 0x00000000,
        BKP28R: 0x00000000,
        BKP29R: 0x00000000,
        BKP30R: 0x00000000,
        BKP31R: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TAMP_TAKEN: bool = false;

    /// Safe access to TAMP
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
            if TAMP_TAKEN {
                None
            } else {
                TAMP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TAMP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TAMP_TAKEN && inst.addr == INSTANCE.addr {
                TAMP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TAMP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TAMP_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TAMP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TAMP: *const RegisterBlock = 0x40003400 as *const _;
