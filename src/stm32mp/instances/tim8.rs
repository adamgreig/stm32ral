#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TIM8
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::tim8::Instance;
pub use crate::stm32mp::peripherals::tim8::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::tim8::{
    TIM8_AF1, TIM8_AF2, TIM8_ARR, TIM8_BDTR, TIM8_CCER, TIM8_CCMR1ALTERNATE8,
    TIM8_CCMR2ALTERNATE24, TIM8_CCMR3, TIM8_CCR1, TIM8_CCR2, TIM8_CCR3, TIM8_CCR4, TIM8_CCR5,
    TIM8_CCR6, TIM8_CNT, TIM8_CR1, TIM8_CR2, TIM8_DCR, TIM8_DIER, TIM8_DMAR, TIM8_EGR, TIM8_PSC,
    TIM8_RCR, TIM8_SMCR, TIM8_SR, TIM8_TISEL,
};

/// Access functions for the TIM8 peripheral instance
pub mod TIM8 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x44001000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TIM8
    pub const reset: ResetValues = ResetValues {
        TIM8_CR1: 0x00000000,
        TIM8_CR2: 0x00000000,
        TIM8_SMCR: 0x00000000,
        TIM8_DIER: 0x00000000,
        TIM8_SR: 0x00000000,
        TIM8_EGR: 0x00000000,
        TIM8_CCMR1ALTERNATE8: 0x00000000,
        TIM8_CCMR2ALTERNATE24: 0x00000000,
        TIM8_CCER: 0x00000000,
        TIM8_CNT: 0x00000000,
        TIM8_PSC: 0x00000000,
        TIM8_ARR: 0x0000FFFF,
        TIM8_RCR: 0x00000000,
        TIM8_CCR1: 0x00000000,
        TIM8_CCR2: 0x00000000,
        TIM8_CCR3: 0x00000000,
        TIM8_CCR4: 0x00000000,
        TIM8_BDTR: 0x00000000,
        TIM8_DCR: 0x00000000,
        TIM8_DMAR: 0x00000000,
        TIM8_CCMR3: 0x00000000,
        TIM8_CCR5: 0x00000000,
        TIM8_CCR6: 0x00000000,
        TIM8_AF1: 0x00000001,
        TIM8_AF2: 0x00000001,
        TIM8_TISEL: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TIM8_TAKEN: bool = false;

    /// Safe access to TIM8
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
            if TIM8_TAKEN {
                None
            } else {
                TIM8_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TIM8
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TIM8_TAKEN && inst.addr == INSTANCE.addr {
                TIM8_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TIM8
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TIM8_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TIM8
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TIM8: *const RegisterBlock = 0x44001000 as *const _;
