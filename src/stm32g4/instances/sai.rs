#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Serial audio interface
//!
//! Used by: stm32g431, stm32g441, stm32g471, stm32g473, stm32g474, stm32g483, stm32g484

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g4::peripherals::sai::Instance;
pub use crate::stm32g4::peripherals::sai::{RegisterBlock, ResetValues};
pub use crate::stm32g4::peripherals::sai::{
    ACLRFR, ACR1, ACR2, ADR, AFRCR, AIM, ASLOTR, ASR, BCLRFR, BCR1, BCR2, BDR, BFRCR, BIM, BSLOTR,
    BSR, PDMCR, PDMDLY,
};

/// Access functions for the SAI peripheral instance
pub mod SAI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40015400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SAI
    pub const reset: ResetValues = ResetValues {
        BCR1: 0x00000040,
        BCR2: 0x00000000,
        BFRCR: 0x00000007,
        BSLOTR: 0x00000000,
        BIM: 0x00000000,
        BSR: 0x00000000,
        BCLRFR: 0x00000000,
        BDR: 0x00000000,
        ACR1: 0x00000040,
        ACR2: 0x00000000,
        AFRCR: 0x00000007,
        ASLOTR: 0x00000000,
        AIM: 0x00000000,
        ASR: 0x00000000,
        ACLRFR: 0x00000000,
        ADR: 0x00000000,
        PDMCR: 0x00000000,
        PDMDLY: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SAI_TAKEN: bool = false;

    /// Safe access to SAI
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
            if SAI_TAKEN {
                None
            } else {
                SAI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SAI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SAI_TAKEN && inst.addr == INSTANCE.addr {
                SAI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SAI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SAI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SAI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SAI: *const RegisterBlock = 0x40015400 as *const _;
