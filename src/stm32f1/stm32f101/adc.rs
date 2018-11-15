#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Analog to digital converter

#[cfg(not(feature = "nosync"))]
pub use stm32f1::peripherals::adc::Instance;
pub use stm32f1::peripherals::adc::{RegisterBlock, ResetValues};
pub use stm32f1::peripherals::adc::{
    CR1, CR2, DR, HTR, JDR1, JDR2, JDR3, JDR4, JOFR1, JOFR2, JOFR3, JOFR4, JSQR, LTR, SMPR1, SMPR2,
    SQR1, SQR2, SQR3, SR,
};

/// Access functions for the ADC2 peripheral instance
pub mod ADC2 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40012800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC2
    pub const reset: ResetValues = ResetValues {
        SR: 0x00000000,
        CR1: 0x00000000,
        CR2: 0x00000000,
        SMPR1: 0x00000000,
        SMPR2: 0x00000000,
        JOFR1: 0x00000000,
        JOFR2: 0x00000000,
        JOFR3: 0x00000000,
        JOFR4: 0x00000000,
        HTR: 0x00000FFF,
        LTR: 0x00000000,
        SQR1: 0x00000000,
        SQR2: 0x00000000,
        SQR3: 0x00000000,
        JSQR: 0x00000000,
        JDR1: 0x00000000,
        JDR2: 0x00000000,
        JDR3: 0x00000000,
        JDR4: 0x00000000,
        DR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut ADC2_TAKEN: bool = false;

    /// Safe access to ADC2
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
            if ADC2_TAKEN {
                None
            } else {
                ADC2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ADC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ADC2_TAKEN && inst.addr == INSTANCE.addr {
                ADC2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to ADC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC2: *const RegisterBlock = 0x40012800 as *const _;

/// Access functions for the ADC3 peripheral instance
pub mod ADC3 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40013c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC3
    pub const reset: ResetValues = ResetValues {
        SR: 0x00000000,
        CR1: 0x00000000,
        CR2: 0x00000000,
        SMPR1: 0x00000000,
        SMPR2: 0x00000000,
        JOFR1: 0x00000000,
        JOFR2: 0x00000000,
        JOFR3: 0x00000000,
        JOFR4: 0x00000000,
        HTR: 0x00000FFF,
        LTR: 0x00000000,
        SQR1: 0x00000000,
        SQR2: 0x00000000,
        SQR3: 0x00000000,
        JSQR: 0x00000000,
        JDR1: 0x00000000,
        JDR2: 0x00000000,
        JDR3: 0x00000000,
        JDR4: 0x00000000,
        DR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut ADC3_TAKEN: bool = false;

    /// Safe access to ADC3
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
            if ADC3_TAKEN {
                None
            } else {
                ADC3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ADC3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ADC3_TAKEN && inst.addr == INSTANCE.addr {
                ADC3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to ADC3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC3: *const RegisterBlock = 0x40013c00 as *const _;
