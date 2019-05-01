#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Cryptographic processor
//!
//! Used by: stm32f7x5, stm32f7x6, stm32f7x7, stm32f7x9

#[cfg(not(feature = "nosync"))]
pub use stm32f7::peripherals::cryp_v2::Instance;
pub use stm32f7::peripherals::cryp_v2::{RegisterBlock, ResetValues};
pub use stm32f7::peripherals::cryp_v2::{
    CR, CSGCM0R, CSGCM1R, CSGCM2R, CSGCM3R, CSGCM4R, CSGCM5R, CSGCM6R, CSGCM7R, CSGCMCCM0R,
    CSGCMCCM1R, CSGCMCCM2R, CSGCMCCM3R, CSGCMCCM4R, CSGCMCCM5R, CSGCMCCM6R, CSGCMCCM7R, DIN, DMACR,
    DOUT, IMSCR, IVLR0, IVLR1, IVRR0, IVRR1, KLR0, KLR1, KLR2, KLR3, KRR0, KRR1, KRR2, KRR3, MISR,
    RISR, SR,
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
        SR: 0x00000003,
        DIN: 0x00000000,
        DOUT: 0x00000000,
        DMACR: 0x00000000,
        IMSCR: 0x00000000,
        RISR: 0x00000001,
        MISR: 0x00000000,
        CSGCMCCM0R: 0x00000000,
        CSGCMCCM1R: 0x00000000,
        CSGCMCCM2R: 0x00000000,
        CSGCMCCM3R: 0x00000000,
        CSGCMCCM4R: 0x00000000,
        CSGCMCCM5R: 0x00000000,
        CSGCMCCM6R: 0x00000000,
        CSGCMCCM7R: 0x00000000,
        CSGCM0R: 0x00000000,
        CSGCM1R: 0x00000000,
        CSGCM2R: 0x00000000,
        CSGCM3R: 0x00000000,
        CSGCM4R: 0x00000000,
        CSGCM5R: 0x00000000,
        CSGCM6R: 0x00000000,
        CSGCM7R: 0x00000000,
        KLR0: 0x00000000,
        KRR0: 0x00000000,
        KLR1: 0x00000000,
        KRR1: 0x00000000,
        KLR2: 0x00000000,
        KRR2: 0x00000000,
        KLR3: 0x00000000,
        KRR3: 0x00000000,
        IVLR0: 0x00000000,
        IVRR0: 0x00000000,
        IVLR1: 0x00000000,
        IVRR1: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
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
