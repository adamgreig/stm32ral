#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Advanced-timers
//!
//! Used by: stm32g431, stm32g441, stm32g471, stm32g473, stm32g474, stm32g483, stm32g484, stm32g491, stm32g4a1

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g4::peripherals::tim4::Instance;
pub use crate::stm32g4::peripherals::tim4::{
    CCMR3_Output, AF1, AF2, ARR, BDTR, CCER, CCMR1, CCMR2, CCR1, CCR2, CCR3, CCR4, CCR5, CCR6, CNT,
    CR1, CR2, DCR, DIER, DMAR, DTR2, ECR, EGR, PSC, RCR, SMCR, SR, TISEL,
};
pub use crate::stm32g4::peripherals::tim4::{RegisterBlock, ResetValues};

/// Access functions for the TIM3 peripheral instance
pub mod TIM3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40000400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TIM3
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        SMCR: 0x00000000,
        DIER: 0x00000000,
        SR: 0x00000000,
        EGR: 0x00000000,
        CCMR1: 0x00000000,
        CCMR2: 0x00000000,
        CCER: 0x00000000,
        CNT: 0x00000000,
        PSC: 0x00000000,
        ARR: 0xFFFFFFFF,
        RCR: 0x00000000,
        CCR1: 0x00000000,
        CCR2: 0x00000000,
        CCR3: 0x00000000,
        CCR4: 0x00000000,
        BDTR: 0x00000000,
        CCR5: 0x00000000,
        CCR6: 0x00000000,
        CCMR3_Output: 0x00000000,
        DTR2: 0x00000000,
        ECR: 0x00000000,
        TISEL: 0x00000000,
        AF1: 0x00000000,
        AF2: 0x00000000,
        DCR: 0x00000000,
        DMAR: 0x00000000,
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
pub const TIM3: *const RegisterBlock = 0x40000400 as *const _;
