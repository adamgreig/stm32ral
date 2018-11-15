#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Serial audio interface
//!
//! Used by: stm32f405, stm32f407

#[cfg(not(feature = "nosync"))]
pub use stm32f4::peripherals::sai1::Instance;
pub use stm32f4::peripherals::sai1::{RegisterBlock, ResetValues};
pub use stm32f4::peripherals::sai1::{
    SAI_ACLRFR, SAI_ACR1, SAI_ACR2, SAI_ADR, SAI_AFRCR, SAI_AIM, SAI_ASLOTR, SAI_ASR, SAI_BCLRFR,
    SAI_BCR1, SAI_BCR2, SAI_BDR, SAI_BFRCR, SAI_BIM, SAI_BSLOTR, SAI_BSR,
};

/// Access functions for the SAI1 peripheral instance
pub mod SAI1 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40015800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SAI1
    pub const reset: ResetValues = ResetValues {
        SAI_ACR1: 0x00000040,
        SAI_BCR1: 0x00000040,
        SAI_ACR2: 0x00000040,
        SAI_BCR2: 0x00000040,
        SAI_AFRCR: 0x00000007,
        SAI_BFRCR: 0x00000007,
        SAI_ASLOTR: 0x00000000,
        SAI_BSLOTR: 0x00000000,
        SAI_AIM: 0x00000000,
        SAI_BIM: 0x00000000,
        SAI_ASR: 0x00000008,
        SAI_BSR: 0x00000008,
        SAI_ACLRFR: 0x00000000,
        SAI_BCLRFR: 0x00000000,
        SAI_ADR: 0x00000000,
        SAI_BDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut SAI1_TAKEN: bool = false;

    /// Safe access to SAI1
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
            if SAI1_TAKEN {
                None
            } else {
                SAI1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SAI1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SAI1_TAKEN && inst.addr == INSTANCE.addr {
                SAI1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to SAI1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SAI1: *const RegisterBlock = 0x40015800 as *const _;
