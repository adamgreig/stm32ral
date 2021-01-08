#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Power control
//!
//! Used by: stm32l552, stm32l562

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l5::peripherals::pwr::Instance;
pub use crate::stm32l5::peripherals::pwr::{RegisterBlock, ResetValues};
pub use crate::stm32l5::peripherals::pwr::{
    CR1, CR2, CR3, CR4, PDCRA, PDCRB, PDCRC, PDCRD, PDCRE, PDCRF, PDCRG, PDCRH, PRIVCFGR, PUCRA,
    PUCRB, PUCRC, PUCRD, PUCRE, PUCRF, PUCRG, PUCRH, SCR, SECCFGR, SR1, SR2,
};

/// Access functions for the PWR peripheral instance
pub mod PWR {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40007000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in PWR
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000400,
        CR2: 0x00000000,
        CR3: 0x00000000,
        CR4: 0x00000000,
        SR1: 0x00000000,
        SR2: 0x00000000,
        SCR: 0x00000000,
        PUCRA: 0x00000000,
        PDCRA: 0x00000000,
        PUCRB: 0x00000000,
        PDCRB: 0x00000000,
        PUCRC: 0x00000000,
        PDCRC: 0x00000000,
        PUCRD: 0x00000000,
        PDCRD: 0x00000000,
        PUCRE: 0x00000000,
        PDCRE: 0x00000000,
        PUCRF: 0x00000000,
        PDCRF: 0x00000000,
        PUCRG: 0x00000000,
        PDCRG: 0x00000000,
        PUCRH: 0x00000000,
        PDCRH: 0x00000000,
        SECCFGR: 0x00000000,
        PRIVCFGR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut PWR_TAKEN: bool = false;

    /// Safe access to PWR
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
            if PWR_TAKEN {
                None
            } else {
                PWR_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to PWR
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if PWR_TAKEN && inst.addr == INSTANCE.addr {
                PWR_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal PWR
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        PWR_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to PWR
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PWR: *const RegisterBlock = 0x40007000 as *const _;

/// Access functions for the SEC_PWR peripheral instance
pub mod SEC_PWR {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50007000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_PWR
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000400,
        CR2: 0x00000000,
        CR3: 0x00000000,
        CR4: 0x00000000,
        SR1: 0x00000000,
        SR2: 0x00000000,
        SCR: 0x00000000,
        PUCRA: 0x00000000,
        PDCRA: 0x00000000,
        PUCRB: 0x00000000,
        PDCRB: 0x00000000,
        PUCRC: 0x00000000,
        PDCRC: 0x00000000,
        PUCRD: 0x00000000,
        PDCRD: 0x00000000,
        PUCRE: 0x00000000,
        PDCRE: 0x00000000,
        PUCRF: 0x00000000,
        PDCRF: 0x00000000,
        PUCRG: 0x00000000,
        PDCRG: 0x00000000,
        PUCRH: 0x00000000,
        PDCRH: 0x00000000,
        SECCFGR: 0x00000000,
        PRIVCFGR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_PWR_TAKEN: bool = false;

    /// Safe access to SEC_PWR
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
            if SEC_PWR_TAKEN {
                None
            } else {
                SEC_PWR_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_PWR
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_PWR_TAKEN && inst.addr == INSTANCE.addr {
                SEC_PWR_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_PWR
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_PWR_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_PWR
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_PWR: *const RegisterBlock = 0x50007000 as *const _;
