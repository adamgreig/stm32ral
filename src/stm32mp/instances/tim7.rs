#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TIM7
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::tim7::Instance;
pub use crate::stm32mp::peripherals::tim7::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::tim7::{
    TIM7_ARR, TIM7_BDTR, TIM7_CCER, TIM7_CCMR1ALTERNATE7, TIM7_CCMR2ALTERNATE23, TIM7_CCMR3,
    TIM7_CCR1, TIM7_CCR2, TIM7_CCR3, TIM7_CCR4, TIM7_CCR5, TIM7_CCR6, TIM7_CNT, TIM7_CR1, TIM7_CR2,
    TIM7_DCR, TIM7_DIER, TIM7_DMAR, TIM7_EGR, TIM7_PSC, TIM7_RCR, TIM7_SMCR, TIM7_SR,
};

/// Access functions for the TIM7 peripheral instance
pub mod TIM7 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40005000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TIM7
    pub const reset: ResetValues = ResetValues {
        TIM7_CR1: 0x00000000,
        TIM7_CR2: 0x00000000,
        TIM7_SMCR: 0x00000000,
        TIM7_DIER: 0x00000000,
        TIM7_SR: 0x00000000,
        TIM7_EGR: 0x00000000,
        TIM7_CCMR1ALTERNATE7: 0x00000000,
        TIM7_CCMR2ALTERNATE23: 0x00000000,
        TIM7_CCER: 0x00000000,
        TIM7_CNT: 0x00000000,
        TIM7_PSC: 0x00000000,
        TIM7_ARR: 0x0000FFFF,
        TIM7_RCR: 0x00000000,
        TIM7_CCR1: 0x00000000,
        TIM7_CCR2: 0x00000000,
        TIM7_CCR3: 0x00000000,
        TIM7_CCR4: 0x00000000,
        TIM7_BDTR: 0x00000000,
        TIM7_DCR: 0x00000000,
        TIM7_DMAR: 0x00000000,
        TIM7_CCMR3: 0x00000000,
        TIM7_CCR5: 0x00000000,
        TIM7_CCR6: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TIM7_TAKEN: bool = false;

    /// Safe access to TIM7
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
            if TIM7_TAKEN {
                None
            } else {
                TIM7_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TIM7
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TIM7_TAKEN && inst.addr == INSTANCE.addr {
                TIM7_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TIM7
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TIM7_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TIM7
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TIM7: *const RegisterBlock = 0x40005000 as *const _;
