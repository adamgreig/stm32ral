#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! High Resolution Timer: TIMB
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v, stm32h757cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::hrtim_timb::Instance;
pub use crate::stm32h7::peripherals::hrtim_timb::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::hrtim_timb::{
    CHPBR, CMP1BR, CMP1CBR, CMP2BR, CMP3BR, CMP4BR, CNTR, CPT1BCR, CPT1BR, CPT2BCR, CPT2BR, DTBR,
    EEFBR1, EEFBR2, FLTBR, OUTBR, PERBR, REPBR, RSTB1R, RSTB2R, RSTBR, SETB1R, SETB2R, TIMBCR,
    TIMBDIER5, TIMBICR, TIMBISR,
};

/// Access functions for the HRTIM_TIMB peripheral instance
pub mod HRTIM_TIMB {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40017500,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HRTIM_TIMB
    pub const reset: ResetValues = ResetValues {
        TIMBCR: 0x00000000,
        TIMBISR: 0x00000000,
        TIMBICR: 0x00000000,
        TIMBDIER5: 0x00000000,
        CNTR: 0x00000000,
        PERBR: 0x0000FFFF,
        REPBR: 0x00000000,
        CMP1BR: 0x00000000,
        CMP1CBR: 0x00000000,
        CMP2BR: 0x00000000,
        CMP3BR: 0x00000000,
        CMP4BR: 0x00000000,
        CPT1BR: 0x00000000,
        CPT2BR: 0x00000000,
        DTBR: 0x00000000,
        SETB1R: 0x00000000,
        RSTB1R: 0x00000000,
        SETB2R: 0x00000000,
        RSTB2R: 0x00000000,
        EEFBR1: 0x00000000,
        EEFBR2: 0x00000000,
        RSTBR: 0x00000000,
        CHPBR: 0x00000000,
        CPT1BCR: 0x00000000,
        CPT2BCR: 0x00000000,
        OUTBR: 0x00000000,
        FLTBR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HRTIM_TIMB_TAKEN: bool = false;

    /// Safe access to HRTIM_TIMB
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
            if HRTIM_TIMB_TAKEN {
                None
            } else {
                HRTIM_TIMB_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HRTIM_TIMB
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HRTIM_TIMB_TAKEN && inst.addr == INSTANCE.addr {
                HRTIM_TIMB_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal HRTIM_TIMB
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HRTIM_TIMB_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to HRTIM_TIMB
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HRTIM_TIMB: *const RegisterBlock = 0x40017500 as *const _;
