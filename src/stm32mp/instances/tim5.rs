#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TIM5
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::tim5::Instance;
pub use crate::stm32mp::peripherals::tim5::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::tim5::{
    TIM5_ARR, TIM5_BDTR, TIM5_CCER, TIM5_CCMR1ALTERNATE5, TIM5_CCMR2ALTERNATE21, TIM5_CCMR3,
    TIM5_CCR1, TIM5_CCR2, TIM5_CCR3, TIM5_CCR4, TIM5_CCR5, TIM5_CCR6, TIM5_CNT, TIM5_CR1, TIM5_CR2,
    TIM5_DCR, TIM5_DIER, TIM5_DMAR, TIM5_EGR, TIM5_PSC, TIM5_RCR, TIM5_SMCR, TIM5_SR,
};

/// Access functions for the TIM5 peripheral instance
pub mod TIM5 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40003000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TIM5
    pub const reset: ResetValues = ResetValues {
        TIM5_CR1: 0x00000000,
        TIM5_CR2: 0x00000000,
        TIM5_SMCR: 0x00000000,
        TIM5_DIER: 0x00000000,
        TIM5_SR: 0x00000000,
        TIM5_EGR: 0x00000000,
        TIM5_CCMR1ALTERNATE5: 0x00000000,
        TIM5_CCMR2ALTERNATE21: 0x00000000,
        TIM5_CCER: 0x00000000,
        TIM5_CNT: 0x00000000,
        TIM5_PSC: 0x00000000,
        TIM5_ARR: 0x0000FFFF,
        TIM5_RCR: 0x00000000,
        TIM5_CCR1: 0x00000000,
        TIM5_CCR2: 0x00000000,
        TIM5_CCR3: 0x00000000,
        TIM5_CCR4: 0x00000000,
        TIM5_BDTR: 0x00000000,
        TIM5_DCR: 0x00000000,
        TIM5_DMAR: 0x00000000,
        TIM5_CCMR3: 0x00000000,
        TIM5_CCR5: 0x00000000,
        TIM5_CCR6: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TIM5_TAKEN: bool = false;

    /// Safe access to TIM5
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
            if TIM5_TAKEN {
                None
            } else {
                TIM5_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TIM5
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TIM5_TAKEN && inst.addr == INSTANCE.addr {
                TIM5_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TIM5
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TIM5_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TIM5
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TIM5: *const RegisterBlock = 0x40003000 as *const _;
