#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! High Resolution Timer: Common functions
//!
//! Used by: stm32h747cm4, stm32h747cm7, stm32h757cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::hrtim_common_v2::Instance;
pub use crate::stm32h7::peripherals::hrtim_common_v2::{
    BDTxUPR, ADC1R, ADC2R, ADC3R, ADC4R, BDMADR, BDMUPDR, BMCMPR6, BMCR, BMPER, BMTRG, CR1, CR2,
    DISR, DLLCR, EECR1, EECR2, EECR3, FLTINR1, FLTINR2, ICR, IER, ISR, ODSR, OENR,
};
pub use crate::stm32h7::peripherals::hrtim_common_v2::{RegisterBlock, ResetValues};

/// Access functions for the HRTIM_Common peripheral instance
pub mod HRTIM_Common {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40017780,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HRTIM_Common
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        ISR: 0x00000000,
        ICR: 0x00000000,
        IER: 0x00000000,
        OENR: 0x00000000,
        DISR: 0x00000000,
        ODSR: 0x00000000,
        BMCR: 0x00000000,
        BMTRG: 0x00000000,
        BMCMPR6: 0x00000000,
        BMPER: 0x00000000,
        EECR1: 0x00000000,
        EECR2: 0x00000000,
        EECR3: 0x00000000,
        ADC1R: 0x00000000,
        ADC2R: 0x00000000,
        ADC3R: 0x00000000,
        ADC4R: 0x00000000,
        DLLCR: 0x00000000,
        FLTINR1: 0x00000000,
        FLTINR2: 0x00000000,
        BDMUPDR: 0x00000000,
        BDTxUPR: 0x00000000,
        BDMADR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HRTIM_Common_TAKEN: bool = false;

    /// Safe access to HRTIM_Common
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
            if HRTIM_Common_TAKEN {
                None
            } else {
                HRTIM_Common_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HRTIM_Common
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HRTIM_Common_TAKEN && inst.addr == INSTANCE.addr {
                HRTIM_Common_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal HRTIM_Common
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HRTIM_Common_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to HRTIM_Common
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HRTIM_Common: *const RegisterBlock = 0x40017780 as *const _;
