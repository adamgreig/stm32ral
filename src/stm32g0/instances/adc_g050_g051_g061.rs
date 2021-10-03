#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ADC address block description
//!
//! Used by: stm32g050, stm32g051, stm32g061

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g0::peripherals::adc_v2::Instance;
pub use crate::stm32g0::peripherals::adc_v2::{RegisterBlock, ResetValues};
pub use crate::stm32g0::peripherals::adc_v2::{
    ADC_AWD1TR, ADC_AWD2CR, ADC_AWD2TR, ADC_AWD3CR, ADC_AWD3TR, ADC_CALFACT, ADC_CCR, ADC_CFGR1,
    ADC_CFGR2, ADC_CHSELRMOD, ADC_CR, ADC_DR, ADC_IER, ADC_ISR, ADC_SMPR,
};

/// Access functions for the ADC peripheral instance
pub mod ADC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40012400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC
    pub const reset: ResetValues = ResetValues {
        ADC_ISR: 0x00000000,
        ADC_IER: 0x00000000,
        ADC_CR: 0x00000000,
        ADC_CFGR1: 0x00000000,
        ADC_CFGR2: 0x00000000,
        ADC_SMPR: 0x00000000,
        ADC_AWD1TR: 0x0FFF0000,
        ADC_AWD2TR: 0x0FFF0000,
        ADC_CHSELRMOD: 0x00000000,
        ADC_AWD3TR: 0x0FFF0000,
        ADC_DR: 0x00000000,
        ADC_AWD2CR: 0x00000000,
        ADC_AWD3CR: 0x00000000,
        ADC_CALFACT: 0x00000000,
        ADC_CCR: 0x00000000,
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
pub const ADC: *const RegisterBlock = 0x40012400 as *const _;
