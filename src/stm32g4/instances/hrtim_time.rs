#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! High Resolution Timer: TIME
//!
//! Used by: stm32g474, stm32g484

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g4::peripherals::hrtim_time::Instance;
pub use crate::stm32g4::peripherals::hrtim_time::{RegisterBlock, ResetValues};
pub use crate::stm32g4::peripherals::hrtim_time::{
    CHPER, CMP1CER, CMP1ER, CMP2ER, CMP3ER, CMP4ER, CNTER, CPT1ECR, CPT1ER, CPT2ECR, CPT2ER, DTER,
    EEEFR3, EEFER1, EEFER2, FLTER, OUTER, PERER, REPER, RSTE1R, RSTE2R, RSTER, SETE1R, SETE2R,
    TIMECR, TIMECR2, TIMEDIER, TIMEICR, TIMEISR,
};

/// Access functions for the HRTIM_TIME peripheral instance
pub mod HRTIM_TIME {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40016a80,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HRTIM_TIME
    pub const reset: ResetValues = ResetValues {
        TIMECR: 0x00000000,
        TIMEISR: 0x00000000,
        TIMEICR: 0x00000000,
        TIMEDIER: 0x00000000,
        CNTER: 0x00000000,
        PERER: 0x0000FFFF,
        REPER: 0x00000000,
        CMP1ER: 0x00000000,
        CMP1CER: 0x00000000,
        CMP2ER: 0x00000000,
        CMP3ER: 0x00000000,
        CMP4ER: 0x00000000,
        CPT1ER: 0x00000000,
        CPT2ER: 0x00000000,
        DTER: 0x00000000,
        SETE1R: 0x00000000,
        RSTE1R: 0x00000000,
        SETE2R: 0x00000000,
        RSTE2R: 0x00000000,
        EEFER1: 0x00000000,
        EEFER2: 0x00000000,
        RSTER: 0x00000000,
        CHPER: 0x00000000,
        CPT1ECR: 0x00000000,
        CPT2ECR: 0x00000000,
        OUTER: 0x00000000,
        FLTER: 0x00000000,
        TIMECR2: 0x00000000,
        EEEFR3: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HRTIM_TIME_TAKEN: bool = false;

    /// Safe access to HRTIM_TIME
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
            if HRTIM_TIME_TAKEN {
                None
            } else {
                HRTIM_TIME_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HRTIM_TIME
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HRTIM_TIME_TAKEN && inst.addr == INSTANCE.addr {
                HRTIM_TIME_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal HRTIM_TIME
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HRTIM_TIME_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to HRTIM_TIME
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HRTIM_TIME: *const RegisterBlock = 0x40016a80 as *const _;
