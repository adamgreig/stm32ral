#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SDMMC1
//!
//! Used by: stm32l552, stm32l562

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l5::peripherals::sdmmc1::Instance;
pub use crate::stm32l5::peripherals::sdmmc1::{RegisterBlock, ResetValues};
pub use crate::stm32l5::peripherals::sdmmc1::{
    SDMMC_ACKTIMER, SDMMC_ARGR, SDMMC_CLKCR, SDMMC_CMDR, SDMMC_DCNTR, SDMMC_DCTRL, SDMMC_DLENR,
    SDMMC_DTIMER, SDMMC_FIFOR, SDMMC_ICR, SDMMC_ID, SDMMC_IDMABASE0R, SDMMC_IDMABASE1R,
    SDMMC_IDMABSIZER, SDMMC_IDMACTRLR, SDMMC_MASKR, SDMMC_POWER, SDMMC_RESP1R, SDMMC_RESP2R,
    SDMMC_RESP3R, SDMMC_RESP4R, SDMMC_RESPCMDR, SDMMC_STAR, SDMMC_VER,
};

/// Access functions for the SDMMC1 peripheral instance
pub mod SDMMC1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x420c8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SDMMC1
    pub const reset: ResetValues = ResetValues {
        SDMMC_POWER: 0x00000000,
        SDMMC_CLKCR: 0x00000000,
        SDMMC_ARGR: 0x00000000,
        SDMMC_CMDR: 0x00000000,
        SDMMC_RESP1R: 0x00000000,
        SDMMC_RESP2R: 0x00000000,
        SDMMC_RESP3R: 0x00000000,
        SDMMC_RESP4R: 0x00000000,
        SDMMC_DTIMER: 0x00000000,
        SDMMC_DLENR: 0x00000000,
        SDMMC_DCTRL: 0x00000000,
        SDMMC_DCNTR: 0x00000000,
        SDMMC_STAR: 0x00000000,
        SDMMC_ICR: 0x00000000,
        SDMMC_MASKR: 0x00000000,
        SDMMC_ACKTIMER: 0x00000000,
        SDMMC_IDMACTRLR: 0x00000000,
        SDMMC_IDMABSIZER: 0x00000000,
        SDMMC_IDMABASE0R: 0x00000000,
        SDMMC_IDMABASE1R: 0x00000000,
        SDMMC_FIFOR: 0x00000000,
        SDMMC_VER: 0x00000010,
        SDMMC_ID: 0x00140022,
        SDMMC_RESPCMDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SDMMC1_TAKEN: bool = false;

    /// Safe access to SDMMC1
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
            if SDMMC1_TAKEN {
                None
            } else {
                SDMMC1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SDMMC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SDMMC1_TAKEN && inst.addr == INSTANCE.addr {
                SDMMC1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SDMMC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SDMMC1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SDMMC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SDMMC1: *const RegisterBlock = 0x420c8000 as *const _;

/// Access functions for the SEC_SDMMC1 peripheral instance
pub mod SEC_SDMMC1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x520c8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_SDMMC1
    pub const reset: ResetValues = ResetValues {
        SDMMC_POWER: 0x00000000,
        SDMMC_CLKCR: 0x00000000,
        SDMMC_ARGR: 0x00000000,
        SDMMC_CMDR: 0x00000000,
        SDMMC_RESP1R: 0x00000000,
        SDMMC_RESP2R: 0x00000000,
        SDMMC_RESP3R: 0x00000000,
        SDMMC_RESP4R: 0x00000000,
        SDMMC_DTIMER: 0x00000000,
        SDMMC_DLENR: 0x00000000,
        SDMMC_DCTRL: 0x00000000,
        SDMMC_DCNTR: 0x00000000,
        SDMMC_STAR: 0x00000000,
        SDMMC_ICR: 0x00000000,
        SDMMC_MASKR: 0x00000000,
        SDMMC_ACKTIMER: 0x00000000,
        SDMMC_IDMACTRLR: 0x00000000,
        SDMMC_IDMABSIZER: 0x00000000,
        SDMMC_IDMABASE0R: 0x00000000,
        SDMMC_IDMABASE1R: 0x00000000,
        SDMMC_FIFOR: 0x00000000,
        SDMMC_VER: 0x00000010,
        SDMMC_ID: 0x00140022,
        SDMMC_RESPCMDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_SDMMC1_TAKEN: bool = false;

    /// Safe access to SEC_SDMMC1
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
            if SEC_SDMMC1_TAKEN {
                None
            } else {
                SEC_SDMMC1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_SDMMC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_SDMMC1_TAKEN && inst.addr == INSTANCE.addr {
                SEC_SDMMC1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_SDMMC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_SDMMC1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_SDMMC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_SDMMC1: *const RegisterBlock = 0x520c8000 as *const _;
