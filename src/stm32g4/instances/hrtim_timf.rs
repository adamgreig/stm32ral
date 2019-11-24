#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! High Resolution Timer: TIMF
//!
//! Used by: stm32g474, stm32g484

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g4::peripherals::hrtim_timf::Instance;
pub use crate::stm32g4::peripherals::hrtim_timf::{RegisterBlock, ResetValues};
pub use crate::stm32g4::peripherals::hrtim_timf::{
    CHPFR, CMP1CFR, CMP1FR, CMP2FR, CMP3FR, CMP4FR, CNTFR, CPT1FCR, CPT1FR, CPT2FCR, CPT2FR, DTFR,
    EEFFR1, EEFFR2, FEEFR3, FLTFR, OUTFR, PERFR, REPFR, RSTE1R, RSTF2R, RSTFR, SETF1R, SETF2R,
    TIMFCR, TIMFCR2, TIMFDIER, TIMFICR, TIMFISR,
};

/// Access functions for the HRTIM_TIMF peripheral instance
pub mod HRTIM_TIMF {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40016b00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HRTIM_TIMF
    pub const reset: ResetValues = ResetValues {
        TIMFCR: 0x00000000,
        TIMFISR: 0x00000000,
        TIMFICR: 0x00000000,
        TIMFDIER: 0x00000000,
        CNTFR: 0x00000000,
        PERFR: 0x0000FFFF,
        REPFR: 0x00000000,
        CMP1FR: 0x00000000,
        CMP1CFR: 0x00000000,
        CMP2FR: 0x00000000,
        CMP3FR: 0x00000000,
        CMP4FR: 0x00000000,
        CPT1FR: 0x00000000,
        CPT2FR: 0x00000000,
        DTFR: 0x00000000,
        SETF1R: 0x00000000,
        RSTE1R: 0x00000000,
        SETF2R: 0x00000000,
        RSTF2R: 0x00000000,
        EEFFR1: 0x00000000,
        EEFFR2: 0x00000000,
        RSTFR: 0x00000000,
        CHPFR: 0x00000000,
        CPT1FCR: 0x00000000,
        CPT2FCR: 0x00000000,
        OUTFR: 0x00000000,
        FLTFR: 0x00000000,
        TIMFCR2: 0x00000000,
        FEEFR3: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HRTIM_TIMF_TAKEN: bool = false;

    /// Safe access to HRTIM_TIMF
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
            if HRTIM_TIMF_TAKEN {
                None
            } else {
                HRTIM_TIMF_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HRTIM_TIMF
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HRTIM_TIMF_TAKEN && inst.addr == INSTANCE.addr {
                HRTIM_TIMF_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal HRTIM_TIMF
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HRTIM_TIMF_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to HRTIM_TIMF
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HRTIM_TIMF: *const RegisterBlock = 0x40016b00 as *const _;
