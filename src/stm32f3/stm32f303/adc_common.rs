#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Analog-to-Digital Converter

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f3::peripherals::adc1_2::Instance;
pub use crate::stm32f3::peripherals::adc1_2::{RegisterBlock, ResetValues};
pub use crate::stm32f3::peripherals::adc1_2::{CCR, CDR, CSR};

/// Access functions for the ADC1_2 peripheral instance
pub mod ADC1_2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000300,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC1_2
    pub const reset: ResetValues = ResetValues {
        CSR: 0x00000000,
        CCR: 0x00000000,
        CDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ADC1_2_TAKEN: bool = false;

    /// Safe access to ADC1_2
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
            if ADC1_2_TAKEN {
                None
            } else {
                ADC1_2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ADC1_2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ADC1_2_TAKEN && inst.addr == INSTANCE.addr {
                ADC1_2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ADC1_2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ADC1_2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ADC1_2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC1_2: *const RegisterBlock = 0x50000300 as *const _;

/// Access functions for the ADC3_4 peripheral instance
pub mod ADC3_4 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000700,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC3_4
    pub const reset: ResetValues = ResetValues {
        CSR: 0x00000000,
        CCR: 0x00000000,
        CDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ADC3_4_TAKEN: bool = false;

    /// Safe access to ADC3_4
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
            if ADC3_4_TAKEN {
                None
            } else {
                ADC3_4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ADC3_4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ADC3_4_TAKEN && inst.addr == INSTANCE.addr {
                ADC3_4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ADC3_4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ADC3_4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ADC3_4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC3_4: *const RegisterBlock = 0x50000700 as *const _;
