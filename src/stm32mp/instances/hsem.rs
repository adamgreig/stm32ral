#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! HSEM
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::hsem::Instance;
pub use crate::stm32mp::peripherals::hsem::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::hsem::{
    HSEM_C1ICR, HSEM_C1IER, HSEM_C1ISR, HSEM_C1MISR, HSEM_C2ICR, HSEM_C2IER, HSEM_C2ISR,
    HSEM_C2MISR, HSEM_CR, HSEM_HWCFGR1, HSEM_HWCFGR2, HSEM_IPIDR, HSEM_KEYR, HSEM_R0, HSEM_R1,
    HSEM_R10, HSEM_R11, HSEM_R12, HSEM_R13, HSEM_R14, HSEM_R15, HSEM_R16, HSEM_R17, HSEM_R18,
    HSEM_R19, HSEM_R2, HSEM_R20, HSEM_R21, HSEM_R22, HSEM_R23, HSEM_R24, HSEM_R25, HSEM_R26,
    HSEM_R27, HSEM_R28, HSEM_R29, HSEM_R3, HSEM_R30, HSEM_R31, HSEM_R4, HSEM_R5, HSEM_R6, HSEM_R7,
    HSEM_R8, HSEM_R9, HSEM_RLR0, HSEM_RLR1, HSEM_RLR10, HSEM_RLR11, HSEM_RLR12, HSEM_RLR13,
    HSEM_RLR14, HSEM_RLR15, HSEM_RLR16, HSEM_RLR17, HSEM_RLR18, HSEM_RLR19, HSEM_RLR2, HSEM_RLR20,
    HSEM_RLR21, HSEM_RLR22, HSEM_RLR23, HSEM_RLR24, HSEM_RLR25, HSEM_RLR26, HSEM_RLR27, HSEM_RLR28,
    HSEM_RLR29, HSEM_RLR3, HSEM_RLR30, HSEM_RLR31, HSEM_RLR4, HSEM_RLR5, HSEM_RLR6, HSEM_RLR7,
    HSEM_RLR8, HSEM_RLR9, HSEM_SIDR, HSEM_VERR,
};

/// Access functions for the HSEM peripheral instance
pub mod HSEM {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4c000000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HSEM
    pub const reset: ResetValues = ResetValues {
        HSEM_R0: 0x00000000,
        HSEM_R1: 0x00000000,
        HSEM_R2: 0x00000000,
        HSEM_R3: 0x00000000,
        HSEM_R4: 0x00000000,
        HSEM_R5: 0x00000000,
        HSEM_R6: 0x00000000,
        HSEM_R7: 0x00000000,
        HSEM_R8: 0x00000000,
        HSEM_R9: 0x00000000,
        HSEM_R10: 0x00000000,
        HSEM_R11: 0x00000000,
        HSEM_R12: 0x00000000,
        HSEM_R13: 0x00000000,
        HSEM_R14: 0x00000000,
        HSEM_R15: 0x00000000,
        HSEM_R16: 0x00000000,
        HSEM_R17: 0x00000000,
        HSEM_R18: 0x00000000,
        HSEM_R19: 0x00000000,
        HSEM_R20: 0x00000000,
        HSEM_R21: 0x00000000,
        HSEM_R22: 0x00000000,
        HSEM_R23: 0x00000000,
        HSEM_R24: 0x00000000,
        HSEM_R25: 0x00000000,
        HSEM_R26: 0x00000000,
        HSEM_R27: 0x00000000,
        HSEM_R28: 0x00000000,
        HSEM_R29: 0x00000000,
        HSEM_R30: 0x00000000,
        HSEM_R31: 0x00000000,
        HSEM_RLR0: 0x00000000,
        HSEM_RLR1: 0x00000000,
        HSEM_RLR2: 0x00000000,
        HSEM_RLR3: 0x00000000,
        HSEM_RLR4: 0x00000000,
        HSEM_RLR5: 0x00000000,
        HSEM_RLR6: 0x00000000,
        HSEM_RLR7: 0x00000000,
        HSEM_RLR8: 0x00000000,
        HSEM_RLR9: 0x00000000,
        HSEM_RLR10: 0x00000000,
        HSEM_RLR11: 0x00000000,
        HSEM_RLR12: 0x00000000,
        HSEM_RLR13: 0x00000000,
        HSEM_RLR14: 0x00000000,
        HSEM_RLR15: 0x00000000,
        HSEM_RLR16: 0x00000000,
        HSEM_RLR17: 0x00000000,
        HSEM_RLR18: 0x00000000,
        HSEM_RLR19: 0x00000000,
        HSEM_RLR20: 0x00000000,
        HSEM_RLR21: 0x00000000,
        HSEM_RLR22: 0x00000000,
        HSEM_RLR23: 0x00000000,
        HSEM_RLR24: 0x00000000,
        HSEM_RLR25: 0x00000000,
        HSEM_RLR26: 0x00000000,
        HSEM_RLR27: 0x00000000,
        HSEM_RLR28: 0x00000000,
        HSEM_RLR29: 0x00000000,
        HSEM_RLR30: 0x00000000,
        HSEM_RLR31: 0x00000000,
        HSEM_C1IER: 0x00000000,
        HSEM_C1ICR: 0x00000000,
        HSEM_C1ISR: 0x00000000,
        HSEM_C1MISR: 0x00000000,
        HSEM_C2IER: 0x00000000,
        HSEM_C2ICR: 0x00000000,
        HSEM_C2ISR: 0x00000000,
        HSEM_C2MISR: 0x00000000,
        HSEM_CR: 0x00000000,
        HSEM_KEYR: 0x00000000,
        HSEM_HWCFGR2: 0x00000021,
        HSEM_HWCFGR1: 0x00000220,
        HSEM_VERR: 0x00000020,
        HSEM_IPIDR: 0x00100072,
        HSEM_SIDR: 0xA3C5DD01,
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
pub const HSEM: *const RegisterBlock = 0x4c000000 as *const _;
