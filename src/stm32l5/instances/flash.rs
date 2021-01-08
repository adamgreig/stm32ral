#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Flash
//!
//! Used by: stm32l552, stm32l562

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l5::peripherals::flash::Instance;
pub use crate::stm32l5::peripherals::flash::{RegisterBlock, ResetValues};
pub use crate::stm32l5::peripherals::flash::{
    ACR, ECCR, LVEKEYR, NSBOOTADD0R, NSBOOTADD1R, NSCR, NSKEYR, NSSR, OPTKEYR, OPTR, PDKEYR,
    PRIVCFGR, SECBB1R1, SECBB1R2, SECBB1R3, SECBB1R4, SECBB2R1, SECBB2R2, SECBB2R3, SECBB2R4,
    SECBOOTADD0R, SECCR, SECHDPCR, SECKEYR, SECSR, SECWM1R1, SECWM1R2, SECWM2R1, SECWM2R2, WRP1AR,
    WRP1BR, WRP2AR, WRP2BR,
};

/// Access functions for the FLASH peripheral instance
pub mod FLASH {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40022000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FLASH
    pub const reset: ResetValues = ResetValues {
        ACR: 0x00000000,
        PDKEYR: 0x00000000,
        NSKEYR: 0x00000000,
        SECKEYR: 0x00000000,
        OPTKEYR: 0x00000000,
        LVEKEYR: 0x00000000,
        NSSR: 0x00000000,
        SECSR: 0x00000000,
        NSCR: 0xC0000000,
        SECCR: 0x80000000,
        ECCR: 0x00000000,
        OPTR: 0x00000000,
        NSBOOTADD0R: 0x0000000F,
        NSBOOTADD1R: 0x0000000F,
        SECBOOTADD0R: 0x00000000,
        SECWM1R1: 0xFF00FF00,
        SECWM1R2: 0x0F000F00,
        WRP1AR: 0xFF00FF00,
        WRP1BR: 0xFF00FF00,
        SECWM2R1: 0xFF00FF00,
        SECWM2R2: 0x0F000F00,
        WRP2AR: 0xFF00FF00,
        WRP2BR: 0xFF00FF00,
        SECBB1R1: 0x00000000,
        SECBB1R2: 0x00000000,
        SECBB1R3: 0x00000000,
        SECBB1R4: 0x00000000,
        SECBB2R1: 0x00000000,
        SECBB2R2: 0x00000000,
        SECBB2R3: 0x00000000,
        SECBB2R4: 0x00000000,
        SECHDPCR: 0x00000000,
        PRIVCFGR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut FLASH_TAKEN: bool = false;

    /// Safe access to FLASH
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
            if FLASH_TAKEN {
                None
            } else {
                FLASH_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to FLASH
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if FLASH_TAKEN && inst.addr == INSTANCE.addr {
                FLASH_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal FLASH
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        FLASH_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to FLASH
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FLASH: *const RegisterBlock = 0x40022000 as *const _;

/// Access functions for the SEC_FLASH peripheral instance
pub mod SEC_FLASH {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50022000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_FLASH
    pub const reset: ResetValues = ResetValues {
        ACR: 0x00000000,
        PDKEYR: 0x00000000,
        NSKEYR: 0x00000000,
        SECKEYR: 0x00000000,
        OPTKEYR: 0x00000000,
        LVEKEYR: 0x00000000,
        NSSR: 0x00000000,
        SECSR: 0x00000000,
        NSCR: 0xC0000000,
        SECCR: 0x80000000,
        ECCR: 0x00000000,
        OPTR: 0x00000000,
        NSBOOTADD0R: 0x0000000F,
        NSBOOTADD1R: 0x0000000F,
        SECBOOTADD0R: 0x00000000,
        SECWM1R1: 0xFF00FF00,
        SECWM1R2: 0x0F000F00,
        WRP1AR: 0xFF00FF00,
        WRP1BR: 0xFF00FF00,
        SECWM2R1: 0xFF00FF00,
        SECWM2R2: 0x0F000F00,
        WRP2AR: 0xFF00FF00,
        WRP2BR: 0xFF00FF00,
        SECBB1R1: 0x00000000,
        SECBB1R2: 0x00000000,
        SECBB1R3: 0x00000000,
        SECBB1R4: 0x00000000,
        SECBB2R1: 0x00000000,
        SECBB2R2: 0x00000000,
        SECBB2R3: 0x00000000,
        SECBB2R4: 0x00000000,
        SECHDPCR: 0x00000000,
        PRIVCFGR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_FLASH_TAKEN: bool = false;

    /// Safe access to SEC_FLASH
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
            if SEC_FLASH_TAKEN {
                None
            } else {
                SEC_FLASH_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_FLASH
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_FLASH_TAKEN && inst.addr == INSTANCE.addr {
                SEC_FLASH_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_FLASH
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_FLASH_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_FLASH
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_FLASH: *const RegisterBlock = 0x50022000 as *const _;
