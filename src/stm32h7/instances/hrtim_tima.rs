#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! High Resolution Timer: TIMA
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm7, stm32h753, stm32h753v, stm32h757cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::hrtim_tima::Instance;
pub use crate::stm32h7::peripherals::hrtim_tima::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::hrtim_tima::{
    CHPAR, CMP1AR, CMP1CAR, CMP2AR, CMP3AR, CMP4AR, CNTAR, CPT1ACR, CPT1AR, CPT2ACR, CPT2AR, DTAR,
    EEFAR1, EEFAR2, FLTAR, OUTAR, PERAR, REPAR, RSTA1R, RSTA2R, RSTAR, SETA1R, SETA2R, TIMACR,
    TIMADIER5, TIMAICR, TIMAISR,
};

/// Access functions for the HRTIM_TIMA peripheral instance
pub mod HRTIM_TIMA {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40017480,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HRTIM_TIMA
    pub const reset: ResetValues = ResetValues {
        TIMACR: 0x00000000,
        TIMAISR: 0x00000000,
        TIMAICR: 0x00000000,
        TIMADIER5: 0x00000000,
        CNTAR: 0x00000000,
        PERAR: 0x0000FFFF,
        REPAR: 0x00000000,
        CMP1AR: 0x00000000,
        CMP1CAR: 0x00000000,
        CMP2AR: 0x00000000,
        CMP3AR: 0x00000000,
        CMP4AR: 0x00000000,
        CPT1AR: 0x00000000,
        CPT2AR: 0x00000000,
        DTAR: 0x00000000,
        SETA1R: 0x00000000,
        RSTA1R: 0x00000000,
        SETA2R: 0x00000000,
        RSTA2R: 0x00000000,
        EEFAR1: 0x00000000,
        EEFAR2: 0x00000000,
        RSTAR: 0x00000000,
        CHPAR: 0x00000000,
        CPT1ACR: 0x00000000,
        CPT2ACR: 0x00000000,
        OUTAR: 0x00000000,
        FLTAR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HRTIM_TIMA_TAKEN: bool = false;

    /// Safe access to HRTIM_TIMA
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
            if HRTIM_TIMA_TAKEN {
                None
            } else {
                HRTIM_TIMA_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HRTIM_TIMA
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HRTIM_TIMA_TAKEN && inst.addr == INSTANCE.addr {
                HRTIM_TIMA_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal HRTIM_TIMA
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HRTIM_TIMA_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to HRTIM_TIMA
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HRTIM_TIMA: *const RegisterBlock = 0x40017480 as *const _;
