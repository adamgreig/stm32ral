#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TIM15
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::tim15::Instance;
pub use crate::stm32mp::peripherals::tim15::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::tim15::{
    TIMx_BDTR, TIMx_CCMR1, TIMx_EGR, TIMx_SMCR, TIM15_AF1, TIM15_ARR, TIM15_CCER, TIM15_CCR1,
    TIM15_CCR2, TIM15_CNT, TIM15_CR1, TIM15_CR2, TIM15_DCR, TIM15_DIER, TIM15_DMAR, TIM15_PSC,
    TIM15_RCR, TIM15_SR, TIM15_TISEL,
};

/// Access functions for the TIM15 peripheral instance
pub mod TIM15 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x44006000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TIM15
    pub const reset: ResetValues = ResetValues {
        TIM15_CR1: 0x00000000,
        TIM15_CR2: 0x00000000,
        TIMx_SMCR: 0x00000000,
        TIM15_DIER: 0x00000000,
        TIM15_SR: 0x00000000,
        TIMx_EGR: 0x00000000,
        TIMx_CCMR1: 0x00000000,
        TIM15_CCER: 0x00000000,
        TIM15_CNT: 0x00000000,
        TIM15_PSC: 0x00000000,
        TIM15_ARR: 0x0000FFFF,
        TIM15_RCR: 0x00000000,
        TIM15_CCR1: 0x00000000,
        TIM15_CCR2: 0x00000000,
        TIMx_BDTR: 0x00000000,
        TIM15_DCR: 0x00000000,
        TIM15_DMAR: 0x00000000,
        TIM15_AF1: 0x00000001,
        TIM15_TISEL: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TIM15_TAKEN: bool = false;

    /// Safe access to TIM15
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
            if TIM15_TAKEN {
                None
            } else {
                TIM15_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TIM15
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TIM15_TAKEN && inst.addr == INSTANCE.addr {
                TIM15_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TIM15
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TIM15_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TIM15
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TIM15: *const RegisterBlock = 0x44006000 as *const _;
