#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Analog-to-digital converter
//!
//! Used by: stm32f0x0, stm32f0x1, stm32f0x2, stm32f0x8

#[cfg(not(feature = "nosync"))]
pub use stm32f0::peripherals::adc::Instance;
pub use stm32f0::peripherals::adc::{RegisterBlock, ResetValues};
pub use stm32f0::peripherals::adc::{CCR, CFGR1, CFGR2, CHSELR, CR, DR, IER, ISR, SMPR, TR};

/// Access functions for the ADC peripheral instance
pub mod ADC {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

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
        ISR: 0x00000000,
        IER: 0x00000000,
        CR: 0x00000000,
        CFGR1: 0x00000000,
        CFGR2: 0x00008000,
        SMPR: 0x00000000,
        TR: 0x00000FFF,
        CHSELR: 0x00000000,
        DR: 0x00000000,
        CCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
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
