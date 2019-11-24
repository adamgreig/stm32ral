#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Embedded Flash Memory
//!
//! Used by: stm32h747cm7, stm32h757cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::flash_v2::Instance;
pub use crate::stm32h7::peripherals::flash_v2::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::flash_v2::{
    ACR, BOOT4_CURR, BOOT4_PRGR, BOOT7_CURR, BOOT7_PRGR, CCR1, CCR2, CR1, CR2, CRCCR1, CRCCR2,
    CRCDATAR, CRCEADDR1, CRCEADDR2, CRCSADDR1, CRCSADDR2, FAR1, FAR2, KEYR1, KEYR2, OPTCCR, OPTCR,
    OPTKEYR, OPTSR_CUR, OPTSR_PRG, PRAR_CUR1, PRAR_CUR2, PRAR_PRG1, PRAR_PRG2, SCAR_CUR1,
    SCAR_CUR2, SCAR_PRG1, SCAR_PRG2, SR1, SR2, WPSN_CURR1, WPSN_CURR2, WPSN_PRGR1, WPSN_PRGR2,
};

/// Access functions for the FLASH peripheral instance
pub mod FLASH {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x52002000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FLASH
    pub const reset: ResetValues = ResetValues {
        ACR: 0x00000037,
        OPTKEYR: 0x00000000,
        OPTCR: 0x00000001,
        OPTSR_CUR: 0x00000000,
        OPTSR_PRG: 0x00000000,
        OPTCCR: 0x00000000,
        BOOT7_CURR: 0x00000000,
        BOOT7_PRGR: 0x00000000,
        BOOT4_CURR: 0x00000000,
        BOOT4_PRGR: 0x00000000,
        CRCDATAR: 0x00000000,
        KEYR1: 0x00000000,
        CR1: 0x00000031,
        SR1: 0x00000000,
        CCR1: 0x00000000,
        PRAR_CUR1: 0x00000000,
        PRAR_PRG1: 0x00000000,
        SCAR_CUR1: 0x00000000,
        SCAR_PRG1: 0x00000000,
        WPSN_CURR1: 0x00000000,
        WPSN_PRGR1: 0x00000000,
        CRCCR1: 0x001C0000,
        CRCSADDR1: 0x00000000,
        CRCEADDR1: 0x00000000,
        FAR1: 0x00000000,
        KEYR2: 0x00000000,
        CR2: 0x00000031,
        SR2: 0x00000000,
        CCR2: 0x00000000,
        PRAR_CUR2: 0x00000000,
        PRAR_PRG2: 0x00000000,
        SCAR_CUR2: 0x00000000,
        SCAR_PRG2: 0x00000000,
        WPSN_CURR2: 0x00000000,
        WPSN_PRGR2: 0x00000000,
        CRCCR2: 0x001C0000,
        CRCSADDR2: 0x00000000,
        CRCEADDR2: 0x00000000,
        FAR2: 0x00000000,
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
pub const FLASH: *const RegisterBlock = 0x52002000 as *const _;
