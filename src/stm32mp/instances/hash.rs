#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! HASH register block
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::hash::Instance;
pub use crate::stm32mp::peripherals::hash::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::hash::{
    HASH_CR, HASH_CSR0, HASH_CSR1, HASH_CSR10, HASH_CSR11, HASH_CSR12, HASH_CSR13, HASH_CSR14,
    HASH_CSR15, HASH_CSR16, HASH_CSR17, HASH_CSR18, HASH_CSR19, HASH_CSR2, HASH_CSR20, HASH_CSR21,
    HASH_CSR22, HASH_CSR23, HASH_CSR24, HASH_CSR25, HASH_CSR26, HASH_CSR27, HASH_CSR28, HASH_CSR29,
    HASH_CSR3, HASH_CSR30, HASH_CSR31, HASH_CSR32, HASH_CSR33, HASH_CSR34, HASH_CSR35, HASH_CSR36,
    HASH_CSR37, HASH_CSR38, HASH_CSR39, HASH_CSR4, HASH_CSR40, HASH_CSR41, HASH_CSR42, HASH_CSR43,
    HASH_CSR44, HASH_CSR45, HASH_CSR46, HASH_CSR47, HASH_CSR48, HASH_CSR49, HASH_CSR5, HASH_CSR50,
    HASH_CSR51, HASH_CSR52, HASH_CSR53, HASH_CSR6, HASH_CSR7, HASH_CSR8, HASH_CSR9, HASH_DIN,
    HASH_HR0, HASH_HR1, HASH_HR2, HASH_HR3, HASH_HR4, HASH_HR5, HASH_HR6, HASH_HR7, HASH_HWCFGR,
    HASH_IMR, HASH_IPIDR, HASH_MID, HASH_SR, HASH_STR, HASH_VERR,
};

/// Access functions for the HASH1 peripheral instance
pub mod HASH1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x54002000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HASH1
    pub const reset: ResetValues = ResetValues {
        HASH_CR: 0x00000000,
        HASH_DIN: 0x00000000,
        HASH_STR: 0x00000000,
        HASH_HR0: 0x00000000,
        HASH_HR1: 0x00000000,
        HASH_HR2: 0x00000000,
        HASH_HR3: 0x00000000,
        HASH_HR4: 0x00000000,
        HASH_IMR: 0x00000000,
        HASH_SR: 0x00000001,
        HASH_CSR0: 0x00000002,
        HASH_CSR1: 0x00000000,
        HASH_CSR2: 0x00000000,
        HASH_CSR3: 0x00000000,
        HASH_CSR4: 0x00000000,
        HASH_CSR5: 0x00000000,
        HASH_CSR6: 0x00000000,
        HASH_CSR7: 0x00000000,
        HASH_CSR8: 0x00000000,
        HASH_CSR9: 0x00000000,
        HASH_CSR10: 0x00000000,
        HASH_CSR11: 0x00000000,
        HASH_CSR12: 0x00000000,
        HASH_CSR13: 0x00000000,
        HASH_CSR14: 0x00000000,
        HASH_CSR15: 0x00000000,
        HASH_CSR16: 0x00000000,
        HASH_CSR17: 0x00000000,
        HASH_CSR18: 0x00000000,
        HASH_CSR19: 0x00000000,
        HASH_CSR20: 0x00000000,
        HASH_CSR21: 0x00000000,
        HASH_CSR22: 0x00000000,
        HASH_CSR23: 0x00000000,
        HASH_CSR24: 0x00000000,
        HASH_CSR25: 0x00000000,
        HASH_CSR26: 0x00000000,
        HASH_CSR27: 0x00000000,
        HASH_CSR28: 0x00000000,
        HASH_CSR29: 0x00000000,
        HASH_CSR30: 0x00000000,
        HASH_CSR31: 0x00000000,
        HASH_CSR32: 0x00000000,
        HASH_CSR33: 0x00000000,
        HASH_CSR34: 0x00000000,
        HASH_CSR35: 0x00000000,
        HASH_CSR36: 0x00000000,
        HASH_CSR37: 0x00000000,
        HASH_CSR38: 0x00000000,
        HASH_CSR39: 0x00000000,
        HASH_CSR40: 0x00000000,
        HASH_CSR41: 0x00000000,
        HASH_CSR42: 0x00000000,
        HASH_CSR43: 0x00000000,
        HASH_CSR44: 0x00000000,
        HASH_CSR45: 0x00000000,
        HASH_CSR46: 0x00000000,
        HASH_CSR47: 0x00000000,
        HASH_CSR48: 0x00000000,
        HASH_CSR49: 0x00000000,
        HASH_CSR50: 0x00000000,
        HASH_CSR51: 0x00000000,
        HASH_CSR52: 0x00000000,
        HASH_CSR53: 0x00000000,
        HASH_HR5: 0x00000000,
        HASH_HR6: 0x00000000,
        HASH_HR7: 0x00000000,
        HASH_HWCFGR: 0x00000001,
        HASH_VERR: 0x00000023,
        HASH_IPIDR: 0x00170031,
        HASH_MID: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HASH1_TAKEN: bool = false;

    /// Safe access to HASH1
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
            if HASH1_TAKEN {
                None
            } else {
                HASH1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HASH1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HASH1_TAKEN && inst.addr == INSTANCE.addr {
                HASH1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal HASH1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HASH1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to HASH1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HASH1: *const RegisterBlock = 0x54002000 as *const _;

/// Access functions for the HASH2 peripheral instance
pub mod HASH2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4c002000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HASH2
    pub const reset: ResetValues = ResetValues {
        HASH_CR: 0x00000000,
        HASH_DIN: 0x00000000,
        HASH_STR: 0x00000000,
        HASH_HR0: 0x00000000,
        HASH_HR1: 0x00000000,
        HASH_HR2: 0x00000000,
        HASH_HR3: 0x00000000,
        HASH_HR4: 0x00000000,
        HASH_IMR: 0x00000000,
        HASH_SR: 0x00000001,
        HASH_CSR0: 0x00000002,
        HASH_CSR1: 0x00000000,
        HASH_CSR2: 0x00000000,
        HASH_CSR3: 0x00000000,
        HASH_CSR4: 0x00000000,
        HASH_CSR5: 0x00000000,
        HASH_CSR6: 0x00000000,
        HASH_CSR7: 0x00000000,
        HASH_CSR8: 0x00000000,
        HASH_CSR9: 0x00000000,
        HASH_CSR10: 0x00000000,
        HASH_CSR11: 0x00000000,
        HASH_CSR12: 0x00000000,
        HASH_CSR13: 0x00000000,
        HASH_CSR14: 0x00000000,
        HASH_CSR15: 0x00000000,
        HASH_CSR16: 0x00000000,
        HASH_CSR17: 0x00000000,
        HASH_CSR18: 0x00000000,
        HASH_CSR19: 0x00000000,
        HASH_CSR20: 0x00000000,
        HASH_CSR21: 0x00000000,
        HASH_CSR22: 0x00000000,
        HASH_CSR23: 0x00000000,
        HASH_CSR24: 0x00000000,
        HASH_CSR25: 0x00000000,
        HASH_CSR26: 0x00000000,
        HASH_CSR27: 0x00000000,
        HASH_CSR28: 0x00000000,
        HASH_CSR29: 0x00000000,
        HASH_CSR30: 0x00000000,
        HASH_CSR31: 0x00000000,
        HASH_CSR32: 0x00000000,
        HASH_CSR33: 0x00000000,
        HASH_CSR34: 0x00000000,
        HASH_CSR35: 0x00000000,
        HASH_CSR36: 0x00000000,
        HASH_CSR37: 0x00000000,
        HASH_CSR38: 0x00000000,
        HASH_CSR39: 0x00000000,
        HASH_CSR40: 0x00000000,
        HASH_CSR41: 0x00000000,
        HASH_CSR42: 0x00000000,
        HASH_CSR43: 0x00000000,
        HASH_CSR44: 0x00000000,
        HASH_CSR45: 0x00000000,
        HASH_CSR46: 0x00000000,
        HASH_CSR47: 0x00000000,
        HASH_CSR48: 0x00000000,
        HASH_CSR49: 0x00000000,
        HASH_CSR50: 0x00000000,
        HASH_CSR51: 0x00000000,
        HASH_CSR52: 0x00000000,
        HASH_CSR53: 0x00000000,
        HASH_HR5: 0x00000000,
        HASH_HR6: 0x00000000,
        HASH_HR7: 0x00000000,
        HASH_HWCFGR: 0x00000001,
        HASH_VERR: 0x00000023,
        HASH_IPIDR: 0x00170031,
        HASH_MID: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HASH2_TAKEN: bool = false;

    /// Safe access to HASH2
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
            if HASH2_TAKEN {
                None
            } else {
                HASH2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HASH2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HASH2_TAKEN && inst.addr == INSTANCE.addr {
                HASH2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal HASH2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HASH2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to HASH2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HASH2: *const RegisterBlock = 0x4c002000 as *const _;
