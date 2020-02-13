#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! HSEM
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v, stm32h757cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::hsem::Instance;
pub use crate::stm32h7::peripherals::hsem::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::hsem::{
    CR, ICR, IER, ISR, KEYR, MISR, R0, R1, R10, R11, R12, R13, R14, R15, R16, R17, R18, R19, R2,
    R20, R21, R22, R23, R24, R25, R26, R27, R28, R29, R3, R30, R31, R4, R5, R6, R7, R8, R9, RLR0,
    RLR1, RLR10, RLR11, RLR12, RLR13, RLR14, RLR15, RLR16, RLR17, RLR18, RLR19, RLR2, RLR20, RLR21,
    RLR22, RLR23, RLR24, RLR25, RLR26, RLR27, RLR28, RLR29, RLR3, RLR30, RLR31, RLR4, RLR5, RLR6,
    RLR7, RLR8, RLR9,
};

/// Access functions for the HSEM peripheral instance
pub mod HSEM {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58026400,
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
        R16: 0x00000000,
        R17: 0x00000000,
        R18: 0x00000000,
        R19: 0x00000000,
        R20: 0x00000000,
        R21: 0x00000000,
        R22: 0x00000000,
        R23: 0x00000000,
        R24: 0x00000000,
        R25: 0x00000000,
        R26: 0x00000000,
        R27: 0x00000000,
        R28: 0x00000000,
        R29: 0x00000000,
        R30: 0x00000000,
        R31: 0x00000000,
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
        RLR16: 0x00000000,
        RLR17: 0x00000000,
        RLR18: 0x00000000,
        RLR19: 0x00000000,
        RLR20: 0x00000000,
        RLR21: 0x00000000,
        RLR22: 0x00000000,
        RLR23: 0x00000000,
        RLR24: 0x00000000,
        RLR25: 0x00000000,
        RLR26: 0x00000000,
        RLR27: 0x00000000,
        RLR28: 0x00000000,
        RLR29: 0x00000000,
        RLR30: 0x00000000,
        RLR31: 0x00000000,
        IER: 0x00000000,
        ICR: 0x00000000,
        ISR: 0x00000000,
        MISR: 0x00000000,
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
pub const HSEM: *const RegisterBlock = 0x58026400 as *const _;
