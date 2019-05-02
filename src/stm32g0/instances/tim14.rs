#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! General purpose timers
//!
//! Used by: stm32g0x0, stm32g0x1

#[cfg(not(feature = "nosync"))]
pub use stm32g0::peripherals::tim14::Instance;
pub use stm32g0::peripherals::tim14::{RegisterBlock, ResetValues};
pub use stm32g0::peripherals::tim14::{
    ARR, CCER, CCMR1, CCR1, CNT, CR1, DIER, EGR, PSC, SR, TISEL,
};

/// Access functions for the TIM14 peripheral instance
pub mod TIM14 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40002000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TIM14
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        DIER: 0x00000000,
        SR: 0x00000000,
        EGR: 0x00000000,
        CCMR1: 0x00000000,
        CCER: 0x00000000,
        CNT: 0x00000000,
        PSC: 0x00000000,
        ARR: 0x00000000,
        CCR1: 0x00000000,
        TISEL: 0x00000000,
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
pub const TIM14: *const RegisterBlock = 0x40002000 as *const _;
