#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Cryptographic processor
//!
//! Used by: stm32f7x2, stm32f7x3

#[cfg(not(feature = "nosync"))]
pub use stm32f7::peripherals::cryp_v1::Instance;
pub use stm32f7::peripherals::cryp_v1::{RegisterBlock, ResetValues};
pub use stm32f7::peripherals::cryp_v1::{
    CR, DINR, DOUTR, IVR0, IVR1, IVR2, IVR3, KEYR0, KEYR1, KEYR2, KEYR3, KEYR4, KEYR5, KEYR6,
    KEYR7, SR, SUSP0R, SUSP1R, SUSP2R, SUSP3R, SUSP4R, SUSP5R, SUSP6R, SUSP7R,
};

/// Access functions for the CRYP peripheral instance
pub mod CRYP {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50060000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CRYP
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        SR: 0x00000000,
        DINR: 0x00000000,
        DOUTR: 0x00000000,
        KEYR0: 0x00000000,
        KEYR1: 0x00000000,
        KEYR2: 0x00000000,
        KEYR3: 0x00000000,
        IVR0: 0x00000000,
        IVR1: 0x00000000,
        IVR2: 0x00000000,
        IVR3: 0x00000000,
        KEYR4: 0x00000000,
        KEYR5: 0x00000000,
        KEYR6: 0x00000000,
        KEYR7: 0x00000000,
        SUSP0R: 0x00000000,
        SUSP1R: 0x00000000,
        SUSP2R: 0x00000000,
        SUSP3R: 0x00000000,
        SUSP4R: 0x00000000,
        SUSP5R: 0x00000000,
        SUSP6R: 0x00000000,
        SUSP7R: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut CRYP_TAKEN: bool = false;

    /// Safe access to CRYP
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
            if CRYP_TAKEN {
                None
            } else {
                CRYP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CRYP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CRYP_TAKEN && inst.addr == INSTANCE.addr {
                CRYP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to CRYP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CRYP: *const RegisterBlock = 0x50060000 as *const _;
