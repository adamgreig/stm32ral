#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TAMP
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::tamp::Instance;
pub use crate::stm32mp::peripherals::tamp::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::tamp::{
    TAMP_ATCR1, TAMP_ATOR, TAMP_ATSEEDR, TAMP_BKP0R, TAMP_BKP10R, TAMP_BKP11R, TAMP_BKP12R,
    TAMP_BKP13R, TAMP_BKP14R, TAMP_BKP15R, TAMP_BKP16R, TAMP_BKP17R, TAMP_BKP18R, TAMP_BKP19R,
    TAMP_BKP1R, TAMP_BKP20R, TAMP_BKP21R, TAMP_BKP22R, TAMP_BKP23R, TAMP_BKP24R, TAMP_BKP25R,
    TAMP_BKP26R, TAMP_BKP27R, TAMP_BKP28R, TAMP_BKP29R, TAMP_BKP2R, TAMP_BKP30R, TAMP_BKP31R,
    TAMP_BKP3R, TAMP_BKP4R, TAMP_BKP5R, TAMP_BKP6R, TAMP_BKP7R, TAMP_BKP8R, TAMP_BKP9R, TAMP_CFGR,
    TAMP_COUNTR, TAMP_CR1, TAMP_CR2, TAMP_FLTCR, TAMP_HWCFGR1, TAMP_HWCFGR2, TAMP_IER, TAMP_IPIDR,
    TAMP_MISR, TAMP_SCR, TAMP_SIDR, TAMP_SMCR, TAMP_SMISR, TAMP_SR, TAMP_VERR,
};

/// Access functions for the TAMP peripheral instance
pub mod TAMP {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5c00a000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TAMP
    pub const reset: ResetValues = ResetValues {
        TAMP_CR1: 0xFFFF0000,
        TAMP_CR2: 0x00000000,
        TAMP_FLTCR: 0x00000000,
        TAMP_ATCR1: 0x00070000,
        TAMP_ATSEEDR: 0x00000000,
        TAMP_ATOR: 0x00000000,
        TAMP_SMCR: 0x80000000,
        TAMP_IER: 0x00000000,
        TAMP_SR: 0x00000000,
        TAMP_MISR: 0x00000000,
        TAMP_SMISR: 0x00000000,
        TAMP_SCR: 0x00000000,
        TAMP_COUNTR: 0x00000000,
        TAMP_CFGR: 0x00000000,
        TAMP_BKP0R: 0x00000000,
        TAMP_BKP1R: 0x00000000,
        TAMP_BKP2R: 0x00000000,
        TAMP_BKP3R: 0x00000000,
        TAMP_BKP4R: 0x00000000,
        TAMP_BKP5R: 0x00000000,
        TAMP_BKP6R: 0x00000000,
        TAMP_BKP7R: 0x00000000,
        TAMP_BKP8R: 0x00000000,
        TAMP_BKP9R: 0x00000000,
        TAMP_BKP10R: 0x00000000,
        TAMP_BKP11R: 0x00000000,
        TAMP_BKP12R: 0x00000000,
        TAMP_BKP13R: 0x00000000,
        TAMP_BKP14R: 0x00000000,
        TAMP_BKP15R: 0x00000000,
        TAMP_BKP16R: 0x00000000,
        TAMP_BKP17R: 0x00000000,
        TAMP_BKP18R: 0x00000000,
        TAMP_BKP19R: 0x00000000,
        TAMP_BKP20R: 0x00000000,
        TAMP_BKP21R: 0x00000000,
        TAMP_BKP22R: 0x00000000,
        TAMP_BKP23R: 0x00000000,
        TAMP_BKP24R: 0x00000000,
        TAMP_BKP25R: 0x00000000,
        TAMP_BKP26R: 0x00000000,
        TAMP_BKP27R: 0x00000000,
        TAMP_BKP28R: 0x00000000,
        TAMP_BKP29R: 0x00000000,
        TAMP_BKP30R: 0x00000000,
        TAMP_BKP31R: 0x00000000,
        TAMP_HWCFGR2: 0x00000101,
        TAMP_HWCFGR1: 0x009D1320,
        TAMP_VERR: 0x00000010,
        TAMP_IPIDR: 0x00121033,
        TAMP_SIDR: 0xA3C5DD01,
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
pub const TAMP: *const RegisterBlock = 0x5c00a000 as *const _;
