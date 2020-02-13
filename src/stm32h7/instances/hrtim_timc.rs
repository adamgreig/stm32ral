#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! High Resolution Timer: TIMC
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v, stm32h757cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::hrtim_timc::Instance;
pub use crate::stm32h7::peripherals::hrtim_timc::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::hrtim_timc::{
    CHPCR, CMP1CCR, CMP1CR, CMP2CR, CMP3CR, CMP4CR, CNTCR, CPT1CCR, CPT1CR, CPT2CCR, CPT2CR, DTCR,
    EEFCR1, EEFCR2, FLTCR, OUTCR, PERCR, REPCR, RSTC1R, RSTC2R, RSTCR, SETC1R, SETC2R, TIMCCR,
    TIMCDIER5, TIMCICR, TIMCISR,
};

/// Access functions for the HRTIM_TIMC peripheral instance
pub mod HRTIM_TIMC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40017580,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HRTIM_TIMC
    pub const reset: ResetValues = ResetValues {
        TIMCCR: 0x00000000,
        TIMCISR: 0x00000000,
        TIMCICR: 0x00000000,
        TIMCDIER5: 0x00000000,
        CNTCR: 0x00000000,
        PERCR: 0x0000FFFF,
        REPCR: 0x00000000,
        CMP1CR: 0x00000000,
        CMP1CCR: 0x00000000,
        CMP2CR: 0x00000000,
        CMP3CR: 0x00000000,
        CMP4CR: 0x00000000,
        CPT1CR: 0x00000000,
        CPT2CR: 0x00000000,
        DTCR: 0x00000000,
        SETC1R: 0x00000000,
        RSTC1R: 0x00000000,
        SETC2R: 0x00000000,
        RSTC2R: 0x00000000,
        EEFCR1: 0x00000000,
        EEFCR2: 0x00000000,
        RSTCR: 0x00000000,
        CHPCR: 0x00000000,
        CPT1CCR: 0x00000000,
        CPT2CCR: 0x00000000,
        OUTCR: 0x00000000,
        FLTCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HRTIM_TIMC_TAKEN: bool = false;

    /// Safe access to HRTIM_TIMC
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
            if HRTIM_TIMC_TAKEN {
                None
            } else {
                HRTIM_TIMC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HRTIM_TIMC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HRTIM_TIMC_TAKEN && inst.addr == INSTANCE.addr {
                HRTIM_TIMC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal HRTIM_TIMC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HRTIM_TIMC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to HRTIM_TIMC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HRTIM_TIMC: *const RegisterBlock = 0x40017580 as *const _;
