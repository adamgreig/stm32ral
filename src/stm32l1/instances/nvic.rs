#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Nested Vectored Interrupt Controller
//!
//! Used by: stm32l100, stm32l151, stm32l162

#[cfg(not(feature = "nosync"))]
pub use stm32l1::peripherals::nvic::Instance;
pub use stm32l1::peripherals::nvic::{RegisterBlock, ResetValues};
pub use stm32l1::peripherals::nvic::{
    IABR0, IABR1, ICER0, ICER1, ICPR0, ICPR1, IPR0, IPR1, IPR10, IPR11, IPR12, IPR13, IPR2, IPR3,
    IPR4, IPR5, IPR6, IPR7, IPR8, IPR9, ISER0, ISER1, ISPR0, ISPR1,
};

/// Access functions for the NVIC peripheral instance
pub mod NVIC {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe000e100,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in NVIC
    pub const reset: ResetValues = ResetValues {
        ISER0: 0x00000000,
        ISER1: 0x00000000,
        ICER0: 0x00000000,
        ICER1: 0x00000000,
        ISPR0: 0x00000000,
        ISPR1: 0x00000000,
        ICPR0: 0x00000000,
        ICPR1: 0x00000000,
        IABR0: 0x00000000,
        IABR1: 0x00000000,
        IPR0: 0x00000000,
        IPR1: 0x00000000,
        IPR2: 0x00000000,
        IPR3: 0x00000000,
        IPR4: 0x00000000,
        IPR5: 0x00000000,
        IPR6: 0x00000000,
        IPR7: 0x00000000,
        IPR8: 0x00000000,
        IPR9: 0x00000000,
        IPR10: 0x00000000,
        IPR11: 0x00000000,
        IPR12: 0x00000000,
        IPR13: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut NVIC_TAKEN: bool = false;

    /// Safe access to NVIC
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
            if NVIC_TAKEN {
                None
            } else {
                NVIC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to NVIC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if NVIC_TAKEN && inst.addr == INSTANCE.addr {
                NVIC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to NVIC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const NVIC: *const RegisterBlock = 0xe000e100 as *const _;
