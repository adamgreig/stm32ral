#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TIM14
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::tim14::Instance;
pub use crate::stm32mp::peripherals::tim14::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::tim14::{
    TIM14_ARR, TIM14_CCER, TIM14_CCMR1, TIM14_CCR1, TIM14_CNT, TIM14_CR1, TIM14_DIER, TIM14_EGR,
    TIM14_PSC, TIM14_SR, TIM14_TISEL,
};

/// Access functions for the TIM14 peripheral instance
pub mod TIM14 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40008000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TIM14
    pub const reset: ResetValues = ResetValues {
        TIM14_CR1: 0x00000000,
        TIM14_DIER: 0x00000000,
        TIM14_SR: 0x00000000,
        TIM14_EGR: 0x00000000,
        TIM14_CCMR1: 0x00000000,
        TIM14_CCER: 0x00000000,
        TIM14_CNT: 0x00000000,
        TIM14_PSC: 0x00000000,
        TIM14_ARR: 0x0000FFFF,
        TIM14_CCR1: 0x00000000,
        TIM14_TISEL: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TIM14_TAKEN: bool = false;

    /// Safe access to TIM14
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
            if TIM14_TAKEN {
                None
            } else {
                TIM14_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TIM14
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TIM14_TAKEN && inst.addr == INSTANCE.addr {
                TIM14_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TIM14
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TIM14_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TIM14
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TIM14: *const RegisterBlock = 0x40008000 as *const _;
