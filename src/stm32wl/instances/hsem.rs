#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Hardware semaphore
//!
//! Used by: stm32wl5x_cm0p, stm32wl5x_cm4

#[cfg(not(feature = "nosync"))]
pub use crate::stm32wl::peripherals::hsem::Instance;
pub use crate::stm32wl::peripherals::hsem::{RegisterBlock, ResetValues};
pub use crate::stm32wl::peripherals::hsem::{
    C1ICR, C1IER, C1ISR, C1MISR, C2ICR, C2IER, C2ISR, C2MISR, CR, KEYR, R0, R1, R10, R11, R12, R13,
    R14, R15, R2, R3, R4, R5, R6, R7, R8, R9, RLR0, RLR1, RLR10, RLR11, RLR12, RLR13, RLR14, RLR15,
    RLR2, RLR3, RLR4, RLR5, RLR6, RLR7, RLR8, RLR9,
};

/// Access functions for the HSEM peripheral instance
pub mod HSEM {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58001400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HSEM
    pub const reset: ResetValues = ResetValues {
        R0: 0x00000000,
        R1: 0x00000000,
        R2: 0x00000000,
        R3: 0x00000000,
        R4: 0x00000000,
        R5: 0x00000000,
        R6: 0x00000000,
        R7: 0x00000000,
        R8: 0x00000000,
        R9: 0x00000000,
        R10: 0x00000000,
        R11: 0x00000000,
        R12: 0x00000000,
        R13: 0x00000000,
        R14: 0x00000000,
        R15: 0x00000000,
        RLR0: 0x00000000,
        RLR1: 0x00000000,
        RLR2: 0x00000000,
        RLR3: 0x00000000,
        RLR4: 0x00000000,
        RLR5: 0x00000000,
        RLR6: 0x00000000,
        RLR7: 0x00000000,
        RLR8: 0x00000000,
        RLR9: 0x00000000,
        RLR10: 0x00000000,
        RLR11: 0x00000000,
        RLR12: 0x00000000,
        RLR13: 0x00000000,
        RLR14: 0x00000000,
        RLR15: 0x00000000,
        C1IER: 0x00000000,
        C1ICR: 0x00000000,
        C1ISR: 0x00000000,
        C1MISR: 0x00000000,
        C2IER: 0x00000000,
        C2ICR: 0x00000000,
        C2ISR: 0x00000000,
        C2MISR: 0x00000000,
        CR: 0x00000000,
        KEYR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HSEM_TAKEN: bool = false;

    /// Safe access to HSEM
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
            if HSEM_TAKEN {
                None
            } else {
                HSEM_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HSEM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HSEM_TAKEN && inst.addr == INSTANCE.addr {
                HSEM_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal HSEM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HSEM_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to HSEM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HSEM: *const RegisterBlock = 0x58001400 as *const _;
