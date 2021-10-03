#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TIM3
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::tim3::Instance;
pub use crate::stm32mp::peripherals::tim3::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::tim3::{
    TIM3_ARR, TIM3_BDTR, TIM3_CCER, TIM3_CCMR1ALTERNATE3, TIM3_CCMR2ALTERNATE19, TIM3_CCMR3,
    TIM3_CCR1, TIM3_CCR2, TIM3_CCR3, TIM3_CCR4, TIM3_CCR5, TIM3_CCR6, TIM3_CNT, TIM3_CR1, TIM3_CR2,
    TIM3_DCR, TIM3_DIER, TIM3_DMAR, TIM3_EGR, TIM3_PSC, TIM3_RCR, TIM3_SMCR, TIM3_SR,
};

/// Access functions for the TIM3 peripheral instance
pub mod TIM3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40001000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TIM3
    pub const reset: ResetValues = ResetValues {
        TIM3_CR1: 0x00000000,
        TIM3_CR2: 0x00000000,
        TIM3_SMCR: 0x00000000,
        TIM3_DIER: 0x00000000,
        TIM3_SR: 0x00000000,
        TIM3_EGR: 0x00000000,
        TIM3_CCMR1ALTERNATE3: 0x00000000,
        TIM3_CCMR2ALTERNATE19: 0x00000000,
        TIM3_CCER: 0x00000000,
        TIM3_CNT: 0x00000000,
        TIM3_PSC: 0x00000000,
        TIM3_ARR: 0x0000FFFF,
        TIM3_RCR: 0x00000000,
        TIM3_CCR1: 0x00000000,
        TIM3_CCR2: 0x00000000,
        TIM3_CCR3: 0x00000000,
        TIM3_CCR4: 0x00000000,
        TIM3_BDTR: 0x00000000,
        TIM3_DCR: 0x00000000,
        TIM3_DMAR: 0x00000000,
        TIM3_CCMR3: 0x00000000,
        TIM3_CCR5: 0x00000000,
        TIM3_CCR6: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TIM3_TAKEN: bool = false;

    /// Safe access to TIM3
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
            if TIM3_TAKEN {
                None
            } else {
                TIM3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TIM3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TIM3_TAKEN && inst.addr == INSTANCE.addr {
                TIM3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TIM3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TIM3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TIM3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TIM3: *const RegisterBlock = 0x40001000 as *const _;
