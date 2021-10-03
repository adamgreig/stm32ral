#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TIM13
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::tim13::Instance;
pub use crate::stm32mp::peripherals::tim13::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::tim13::{
    TIM13_ARR, TIM13_CCER, TIM13_CCMR1, TIM13_CCR1, TIM13_CNT, TIM13_CR1, TIM13_DIER, TIM13_EGR,
    TIM13_PSC, TIM13_SR, TIM13_TISEL,
};

/// Access functions for the TIM13 peripheral instance
pub mod TIM13 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40007000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TIM13
    pub const reset: ResetValues = ResetValues {
        TIM13_CR1: 0x00000000,
        TIM13_DIER: 0x00000000,
        TIM13_SR: 0x00000000,
        TIM13_EGR: 0x00000000,
        TIM13_CCMR1: 0x00000000,
        TIM13_CCER: 0x00000000,
        TIM13_CNT: 0x00000000,
        TIM13_PSC: 0x00000000,
        TIM13_ARR: 0x0000FFFF,
        TIM13_CCR1: 0x00000000,
        TIM13_TISEL: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TIM13_TAKEN: bool = false;

    /// Safe access to TIM13
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
            if TIM13_TAKEN {
                None
            } else {
                TIM13_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TIM13
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TIM13_TAKEN && inst.addr == INSTANCE.addr {
                TIM13_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TIM13
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TIM13_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TIM13
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TIM13: *const RegisterBlock = 0x40007000 as *const _;
