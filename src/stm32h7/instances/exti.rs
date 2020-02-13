#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! External interrupt/event controller
//!
//! Used by: stm32h743, stm32h743v, stm32h753, stm32h753v

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::exti::Instance;
pub use crate::stm32h7::peripherals::exti::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::exti::{
    CPUEMR1, CPUEMR2, CPUEMR3, CPUIMR1, CPUIMR2, CPUIMR3, CPUPR1, CPUPR2, CPUPR3, D3PCR1H, D3PCR1L,
    D3PCR2H, D3PCR2L, D3PCR3H, D3PMR1, D3PMR2, D3PMR3, FTSR1, FTSR2, FTSR3, RTSR1, RTSR2, RTSR3,
    SWIER1, SWIER2, SWIER3,
};

/// Access functions for the EXTI peripheral instance
pub mod EXTI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58000000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in EXTI
    pub const reset: ResetValues = ResetValues {
        RTSR1: 0x00000000,
        FTSR1: 0x00000000,
        SWIER1: 0x00000000,
        D3PMR1: 0x00000000,
        D3PCR1L: 0x00000000,
        D3PCR1H: 0x00000000,
        RTSR2: 0x00000000,
        FTSR2: 0x00000000,
        SWIER2: 0x00000000,
        D3PMR2: 0x00000000,
        D3PCR2L: 0x00000000,
        D3PCR2H: 0x00000000,
        RTSR3: 0x00000000,
        FTSR3: 0x00000000,
        SWIER3: 0x00000000,
        D3PMR3: 0x00000000,
        D3PCR3H: 0x00000000,
        CPUIMR1: 0xFFC00000,
        CPUEMR1: 0x00000000,
        CPUPR1: 0x00000000,
        CPUIMR2: 0x00000000,
        CPUEMR2: 0x00000000,
        CPUPR2: 0x00000000,
        CPUIMR3: 0x00000000,
        CPUEMR3: 0x00000000,
        CPUPR3: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut EXTI_TAKEN: bool = false;

    /// Safe access to EXTI
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
            if EXTI_TAKEN {
                None
            } else {
                EXTI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to EXTI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if EXTI_TAKEN && inst.addr == INSTANCE.addr {
                EXTI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal EXTI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        EXTI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to EXTI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const EXTI: *const RegisterBlock = 0x58000000 as *const _;
