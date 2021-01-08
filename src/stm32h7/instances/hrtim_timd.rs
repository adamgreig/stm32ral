#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! High Resolution Timer: TIMD
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v, stm32h7b3

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::hrtim_timd::Instance;
pub use crate::stm32h7::peripherals::hrtim_timd::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::hrtim_timd::{
    CHPDR, CMP1CDR, CMP1DR, CMP2DR, CMP3DR, CMP4DR, CNTDR, CPT1DCR, CPT1DR, CPT2DCR, CPT2DR, DTDR,
    EEFDR1, EEFDR2, FLTDR, OUTDR, PERDR, REPDR, RSTD1R, RSTD2R, RSTDR, SETD1R, SETD2R, TIMDCR,
    TIMDDIER5, TIMDICR, TIMDISR,
};

/// Access functions for the HRTIM_TIMD peripheral instance
pub mod HRTIM_TIMD {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40017600,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HRTIM_TIMD
    pub const reset: ResetValues = ResetValues {
        TIMDCR: 0x00000000,
        TIMDISR: 0x00000000,
        TIMDICR: 0x00000000,
        TIMDDIER5: 0x00000000,
        CNTDR: 0x00000000,
        PERDR: 0x0000FFFF,
        REPDR: 0x00000000,
        CMP1DR: 0x00000000,
        CMP1CDR: 0x00000000,
        CMP2DR: 0x00000000,
        CMP3DR: 0x00000000,
        CMP4DR: 0x00000000,
        CPT1DR: 0x00000000,
        CPT2DR: 0x00000000,
        DTDR: 0x00000000,
        SETD1R: 0x00000000,
        RSTD1R: 0x00000000,
        SETD2R: 0x00000000,
        RSTD2R: 0x00000000,
        EEFDR1: 0x00000000,
        EEFDR2: 0x00000000,
        RSTDR: 0x00000000,
        CHPDR: 0x00000000,
        CPT1DCR: 0x00000000,
        CPT2DCR: 0x00000000,
        OUTDR: 0x00000000,
        FLTDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HRTIM_TIMD_TAKEN: bool = false;

    /// Safe access to HRTIM_TIMD
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
            if HRTIM_TIMD_TAKEN {
                None
            } else {
                HRTIM_TIMD_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HRTIM_TIMD
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HRTIM_TIMD_TAKEN && inst.addr == INSTANCE.addr {
                HRTIM_TIMD_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal HRTIM_TIMD
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HRTIM_TIMD_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to HRTIM_TIMD
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HRTIM_TIMD: *const RegisterBlock = 0x40017600 as *const _;
