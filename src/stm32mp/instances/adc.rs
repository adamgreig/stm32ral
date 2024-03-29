#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ADC
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::adc::Instance;
pub use crate::stm32mp::peripherals::adc::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::adc::{
    ADC_AWD2CR, ADC_AWD3CR, ADC_CALFACT, ADC_CALFACT2, ADC_CFGR, ADC_CFGR2, ADC_CR, ADC_DIFSEL,
    ADC_DR, ADC_HTR1, ADC_HTR2, ADC_HTR3, ADC_IER, ADC_ISR, ADC_JDR1, ADC_JDR2, ADC_JDR3, ADC_JDR4,
    ADC_JSQR, ADC_LTR1, ADC_LTR2, ADC_LTR3, ADC_OFR1, ADC_OFR2, ADC_OFR3, ADC_OFR4, ADC_PCSEL,
    ADC_SMPR1, ADC_SMPR2, ADC_SQR1, ADC_SQR2, ADC_SQR3, ADC_SQR4,
};

/// Access functions for the ADC peripheral instance
pub mod ADC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x48003000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC
    pub const reset: ResetValues = ResetValues {
        ADC_ISR: 0x00000000,
        ADC_IER: 0x00000000,
        ADC_CR: 0x20000000,
        ADC_CFGR: 0x80000000,
        ADC_CFGR2: 0x00000000,
        ADC_SMPR1: 0x00000000,
        ADC_SMPR2: 0x00000000,
        ADC_PCSEL: 0x00000000,
        ADC_LTR1: 0x00000000,
        ADC_HTR1: 0x03FFFFFF,
        ADC_SQR1: 0x00000000,
        ADC_SQR2: 0x00000000,
        ADC_SQR3: 0x00000000,
        ADC_SQR4: 0x00000000,
        ADC_DR: 0x00000000,
        ADC_JSQR: 0x00000000,
        ADC_OFR1: 0x00000000,
        ADC_OFR2: 0x00000000,
        ADC_OFR3: 0x00000000,
        ADC_OFR4: 0x00000000,
        ADC_JDR1: 0x00000000,
        ADC_JDR2: 0x00000000,
        ADC_JDR3: 0x00000000,
        ADC_JDR4: 0x00000000,
        ADC_AWD2CR: 0x00000000,
        ADC_AWD3CR: 0x00000000,
        ADC_LTR2: 0x00000000,
        ADC_HTR2: 0x03FFFFFF,
        ADC_LTR3: 0x00000000,
        ADC_HTR3: 0x03FFFFFF,
        ADC_DIFSEL: 0x00000000,
        ADC_CALFACT: 0x00000000,
        ADC_CALFACT2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ADC_TAKEN: bool = false;

    /// Safe access to ADC
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
            if ADC_TAKEN {
                None
            } else {
                ADC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ADC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ADC_TAKEN && inst.addr == INSTANCE.addr {
                ADC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ADC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ADC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ADC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC: *const RegisterBlock = 0x48003000 as *const _;
