#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Hash processor
//!
//! Used by: stm32f405, stm32f407, stm32f427, stm32f429, stm32f469

#[cfg(not(feature = "nosync"))]
pub use stm32f4::peripherals::hash::Instance;
pub use stm32f4::peripherals::hash::{RegisterBlock, ResetValues};
pub use stm32f4::peripherals::hash::{
    CR, CSR0, CSR1, CSR10, CSR11, CSR12, CSR13, CSR14, CSR15, CSR16, CSR17, CSR18, CSR19, CSR2,
    CSR20, CSR21, CSR22, CSR23, CSR24, CSR25, CSR26, CSR27, CSR28, CSR29, CSR3, CSR30, CSR31,
    CSR32, CSR33, CSR34, CSR35, CSR36, CSR37, CSR38, CSR39, CSR4, CSR40, CSR41, CSR42, CSR43,
    CSR44, CSR45, CSR46, CSR47, CSR48, CSR49, CSR5, CSR50, CSR51, CSR52, CSR53, CSR6, CSR7, CSR8,
    CSR9, DIN, HASH_HR0, HASH_HR1, HASH_HR2, HASH_HR3, HASH_HR4, HASH_HR5, HASH_HR6, HASH_HR7, HR0,
    HR1, HR2, HR3, HR4, IMR, SR, STR,
};

/// Access functions for the HASH peripheral instance
pub mod HASH {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50060400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HASH
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        DIN: 0x00000000,
        STR: 0x00000000,
        HR0: 0x00000000,
        HR1: 0x00000000,
        HR2: 0x00000000,
        HR3: 0x00000000,
        HR4: 0x00000000,
        IMR: 0x00000000,
        SR: 0x00000001,
        CSR0: 0x00000000,
        CSR1: 0x00000000,
        CSR2: 0x00000000,
        CSR3: 0x00000000,
        CSR4: 0x00000000,
        CSR5: 0x00000000,
        CSR6: 0x00000000,
        CSR7: 0x00000000,
        CSR8: 0x00000000,
        CSR9: 0x00000000,
        CSR10: 0x00000000,
        CSR11: 0x00000000,
        CSR12: 0x00000000,
        CSR13: 0x00000000,
        CSR14: 0x00000000,
        CSR15: 0x00000000,
        CSR16: 0x00000000,
        CSR17: 0x00000000,
        CSR18: 0x00000000,
        CSR19: 0x00000000,
        CSR20: 0x00000000,
        CSR21: 0x00000000,
        CSR22: 0x00000000,
        CSR23: 0x00000000,
        CSR24: 0x00000000,
        CSR25: 0x00000000,
        CSR26: 0x00000000,
        CSR27: 0x00000000,
        CSR28: 0x00000000,
        CSR29: 0x00000000,
        CSR30: 0x00000000,
        CSR31: 0x00000000,
        CSR32: 0x00000000,
        CSR33: 0x00000000,
        CSR34: 0x00000000,
        CSR35: 0x00000000,
        CSR36: 0x00000000,
        CSR37: 0x00000000,
        CSR38: 0x00000000,
        CSR39: 0x00000000,
        CSR40: 0x00000000,
        CSR41: 0x00000000,
        CSR42: 0x00000000,
        CSR43: 0x00000000,
        CSR44: 0x00000000,
        CSR45: 0x00000000,
        CSR46: 0x00000000,
        CSR47: 0x00000000,
        CSR48: 0x00000000,
        CSR49: 0x00000000,
        CSR50: 0x00000000,
        CSR51: 0x00000000,
        CSR52: 0x00000000,
        CSR53: 0x00000000,
        HASH_HR0: 0x00000000,
        HASH_HR1: 0x00000000,
        HASH_HR2: 0x00000000,
        HASH_HR3: 0x00000000,
        HASH_HR4: 0x00000000,
        HASH_HR5: 0x00000000,
        HASH_HR6: 0x00000000,
        HASH_HR7: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut HASH_TAKEN: bool = false;

    /// Safe access to HASH
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
            if HASH_TAKEN {
                None
            } else {
                HASH_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HASH
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HASH_TAKEN && inst.addr == INSTANCE.addr {
                HASH_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to HASH
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HASH: *const RegisterBlock = 0x50060400 as *const _;
