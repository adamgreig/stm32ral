#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TIM1
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::tim1::Instance;
pub use crate::stm32mp::peripherals::tim1::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::tim1::{
    TIM1_AF1, TIM1_AF2, TIM1_ARR, TIM1_BDTR, TIM1_CCER, TIM1_CCMR1ALTERNATE1,
    TIM1_CCMR2ALTERNATE17, TIM1_CCMR3, TIM1_CCR1, TIM1_CCR2, TIM1_CCR3, TIM1_CCR4, TIM1_CCR5,
    TIM1_CCR6, TIM1_CNT, TIM1_CR1, TIM1_CR2, TIM1_DCR, TIM1_DIER, TIM1_DMAR, TIM1_EGR, TIM1_PSC,
    TIM1_RCR, TIM1_SMCR, TIM1_SR, TIM1_TISEL,
};

/// Access functions for the TIM1 peripheral instance
pub mod TIM1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x44000000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TIM1
    pub const reset: ResetValues = ResetValues {
        TIM1_CR1: 0x00000000,
        TIM1_CR2: 0x00000000,
        TIM1_SMCR: 0x00000000,
        TIM1_DIER: 0x00000000,
        TIM1_SR: 0x00000000,
        TIM1_EGR: 0x00000000,
        TIM1_CCMR1ALTERNATE1: 0x00000000,
        TIM1_CCMR2ALTERNATE17: 0x00000000,
        TIM1_CCER: 0x00000000,
        TIM1_CNT: 0x00000000,
        TIM1_PSC: 0x00000000,
        TIM1_ARR: 0x0000FFFF,
        TIM1_RCR: 0x00000000,
        TIM1_CCR1: 0x00000000,
        TIM1_CCR2: 0x00000000,
        TIM1_CCR3: 0x00000000,
        TIM1_CCR4: 0x00000000,
        TIM1_BDTR: 0x00000000,
        TIM1_DCR: 0x00000000,
        TIM1_DMAR: 0x00000000,
        TIM1_CCMR3: 0x00000000,
        TIM1_CCR5: 0x00000000,
        TIM1_CCR6: 0x00000000,
        TIM1_AF1: 0x00000001,
        TIM1_AF2: 0x00000001,
        TIM1_TISEL: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TIM1_TAKEN: bool = false;

    /// Safe access to TIM1
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
            if TIM1_TAKEN {
                None
            } else {
                TIM1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TIM1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TIM1_TAKEN && inst.addr == INSTANCE.addr {
                TIM1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TIM1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TIM1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TIM1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TIM1: *const RegisterBlock = 0x44000000 as *const _;
