#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TIM4
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::tim4::Instance;
pub use crate::stm32mp::peripherals::tim4::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::tim4::{
    TIM4_ARR, TIM4_BDTR, TIM4_CCER, TIM4_CCMR1ALTERNATE4, TIM4_CCMR2ALTERNATE20, TIM4_CCMR3,
    TIM4_CCR1, TIM4_CCR2, TIM4_CCR3, TIM4_CCR4, TIM4_CCR5, TIM4_CCR6, TIM4_CNT, TIM4_CR1, TIM4_CR2,
    TIM4_DCR, TIM4_DIER, TIM4_DMAR, TIM4_EGR, TIM4_PSC, TIM4_RCR, TIM4_SMCR, TIM4_SR,
};

/// Access functions for the TIM4 peripheral instance
pub mod TIM4 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40002000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TIM4
    pub const reset: ResetValues = ResetValues {
        TIM4_CR1: 0x00000000,
        TIM4_CR2: 0x00000000,
        TIM4_SMCR: 0x00000000,
        TIM4_DIER: 0x00000000,
        TIM4_SR: 0x00000000,
        TIM4_EGR: 0x00000000,
        TIM4_CCMR1ALTERNATE4: 0x00000000,
        TIM4_CCMR2ALTERNATE20: 0x00000000,
        TIM4_CCER: 0x00000000,
        TIM4_CNT: 0x00000000,
        TIM4_PSC: 0x00000000,
        TIM4_ARR: 0x0000FFFF,
        TIM4_RCR: 0x00000000,
        TIM4_CCR1: 0x00000000,
        TIM4_CCR2: 0x00000000,
        TIM4_CCR3: 0x00000000,
        TIM4_CCR4: 0x00000000,
        TIM4_BDTR: 0x00000000,
        TIM4_DCR: 0x00000000,
        TIM4_DMAR: 0x00000000,
        TIM4_CCMR3: 0x00000000,
        TIM4_CCR5: 0x00000000,
        TIM4_CCR6: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TIM4_TAKEN: bool = false;

    /// Safe access to TIM4
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
            if TIM4_TAKEN {
                None
            } else {
                TIM4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TIM4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TIM4_TAKEN && inst.addr == INSTANCE.addr {
                TIM4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TIM4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TIM4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TIM4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TIM4: *const RegisterBlock = 0x40002000 as *const _;
